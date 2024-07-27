use clap_derive::Parser;
use notify::{Config, Event, RecursiveMode, Watcher};
use std::process::{Child, Command, Stdio};
use std::sync::Mutex;
use std::thread::spawn;
use std::time::Duration;

#[derive(Parser, Debug, Clone)]
pub struct ServeArgs {
    #[clap(short, long, default_value = "true")]
    hot_reload: bool,
    #[clap(short, long, default_value = "false")]
    release: bool,
    #[clap(short, long, default_value = "2")]
    delay: u64,
}
#[derive(Clone, Debug)]
pub enum ServeCommand {
    Start(ServeArgs),
    Watch(ServeArgs),
    Stop,
}
#[derive(Eq, PartialEq)]
pub enum ServeStatus {
    End,
    Error,
    Worked,
}
static mut CURRENT_PROCESS: Mutex<Option<Child>> = Mutex::new(None);

impl ServeArgs {
    pub async fn serve_exec(self) -> anyhow::Result<()> {
        tracing::info!("{}", ansi_term::Color::Cyan.paint("Starting server..."));
        let (tx, rx) = std::sync::mpsc::channel::<ServeCommand>();
        let delay = self.delay.clone();
        // Start the work thread
        spawn(move || {
            let mut next = true;
            while let Ok(cmd) = rx.recv() {
                if next {
                    next = false;
                    let work = Self::work_thread(cmd);
                    if work == ServeStatus::End {
                        break;
                    } else if work == ServeStatus::Worked {
                        std::thread::sleep(Duration::from_secs(delay));
                        next = true;
                    }
                } else {
                    continue;
                }
            }
        });
        tx.send(ServeCommand::Start(self.clone())).unwrap();
        if !self.hot_reload {
            while let Ok(()) = tokio::signal::ctrl_c().await {
                tracing::info!("{}", ansi_term::Color::Cyan.paint("Stopping server..."));
                break;
            }
            return Ok(());
        }
        // Watch for changes
        tracing::info!(
            "{}",
            ansi_term::Color::Cyan.paint("Watching for changes...")
        );
        let current_dir = std::env::current_dir().unwrap();
        let current_dir_src = current_dir.join("src");
        let current_config = current_dir.join("./config");
        let current_cargo = current_dir.join("Cargo.toml");
        let config = Config::default()
            .with_poll_interval(Duration::from_secs(2))
            .with_compare_contents(true);
        let mut previous_change = std::time::SystemTime::now();
        let mut watcher = notify::poll::PollWatcher::new(
            move |res| {
                if let Ok(event) = res {
                    let event: Event = event;
                    if event.kind.is_modify() {
                        let now = std::time::SystemTime::now();
                        let duration = now.duration_since(previous_change).unwrap();
                        if duration.as_secs() < 2 {
                            return;
                        }
                        previous_change = now;
                        tracing::info!("{}", ansi_term::Color::Blue.paint("File modified"));
                        tx.send(ServeCommand::Watch(self.clone())).unwrap();
                    }
                }
            },
            config,
        )
        .ok()
        .unwrap();
        watcher
            .watch(&current_dir_src, RecursiveMode::Recursive)
            .ok();
        watcher
            .watch(&current_config, RecursiveMode::Recursive)
            .ok();
        watcher.watch(&current_cargo, RecursiveMode::Recursive).ok();
        // Handle ctrl+c
        while let Ok(()) = tokio::signal::ctrl_c().await {
            tracing::info!("{}", ansi_term::Color::Cyan.paint("Stopping server..."));
            break;
        }
        Ok(())
    }
    pub fn work_thread(serve_command: ServeCommand) -> ServeStatus {
        match serve_command {
            ServeCommand::Start(x) => Self::work_start(x),
            ServeCommand::Watch(x) => Self::work_watch(x),
            ServeCommand::Stop => Self::work_stop(),
        }
    }
    pub fn work_start(serve_args: ServeArgs) -> ServeStatus {
        return Self::work_watch(serve_args);
    }
    pub fn work_watch(args: ServeArgs) -> ServeStatus {
        unsafe {
            if let Ok(Some(child)) = CURRENT_PROCESS.get_mut() {
                child.kill().ok();
            }
        }
        let child = || {
            return if !args.release {
                Command::new("cargo")
                    .arg("run")
                    .stdout(Stdio::piped())
                    .spawn()
                    .expect("failed to start cargo")
            } else {
                Command::new("cargo")
                    .arg("run")
                    .arg("--release")
                    .stdout(Stdio::piped())
                    .spawn()
                    .expect("failed to start cargo")
            };
        };
        let mut child = child();

        let _ = child.wait().expect("failed to wait on child");
        unsafe {
            CURRENT_PROCESS.get_mut().unwrap().replace(child);
        }
        return ServeStatus::Worked;
    }
    pub fn work_stop() -> ServeStatus {
        // TODO
        ServeStatus::End
    }
}
