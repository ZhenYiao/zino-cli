use std::io::{BufRead, BufReader};
use clap_derive::Parser;
use notify::{Config, Event, RecursiveMode, Watcher};
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use std::thread::spawn;
use std::time::Duration;
use rust_i18n::t;
use tokio::sync::OnceCell;
use crate::i18n;
use crate::utils::zino_hello;

#[derive(Parser, Debug, Clone)]
pub struct ServeArgs {
    #[clap(long, default_value = "true")]
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
#[derive(Eq, PartialEq)]
pub enum TracingCommand {
    Start,
    Stop,
}
static mut CURRENT_PROCESS: Mutex<Option<Child>> = Mutex::new(None);
static SEND: OnceCell<Sender<ServeCommand>> = OnceCell::const_new();


impl ServeArgs {
    pub async fn serve_exec(self) -> anyhow::Result<()> {
        zino_hello();
        i18n::init_i18n()?;
        tracing::info!("{}", ansi_term::Color::Cyan.paint(t!("Starting server...")));
        let (tx, rx) = std::sync::mpsc::channel::<ServeCommand>();
        let (end_tx, end_rx) = std::sync::mpsc::channel::<()>();
        let (tracing_tx, tracing_rx) = std::sync::mpsc::channel::<TracingCommand>();
        let delay = self.delay.clone();
        SEND.get_or_init(||async {
            tx.clone()
        }).await;


        // Start the work thread
        spawn(move || {
            let mut next = true;
            Self::tracing(tracing_rx);
            while let Ok(cmd) = rx.recv() {
                if next {
                    next = false;
                    tracing_tx.send(TracingCommand::Stop).unwrap();
                    let work = Self::work_thread(cmd);
                    if work == ServeStatus::End {
                        end_tx.send(()).ok();
                        break;
                    } else if work == ServeStatus::Worked {
                        std::thread::sleep(Duration::from_secs(delay));
                        next = true;
                        tracing_tx.send(TracingCommand::Start).unwrap();
                    }
                } else {
                    continue;
                }
            }
        });
        SEND.get().unwrap().send(ServeCommand::Start(self.clone())).unwrap();


        if !self.hot_reload {
            while let Ok(()) = tokio::signal::ctrl_c().await {
                tracing::info!("{}", ansi_term::Color::Cyan.paint(t!("Stopping server...")));
                while let Ok(()) = end_rx.recv() {
                    tracing::info!("{}", ansi_term::Color::Cyan.paint(t!("Server stopped")));
                }
                break;
            }
            return Ok(());
        }
        // Watch for changes
        tracing::info!(
            "{}",
            ansi_term::Color::Cyan.paint(t!("Watching for changes..."))
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
                        tracing::info!("{}", ansi_term::Color::Blue.paint(t!("File changed...")));
                        SEND.get().unwrap().send(ServeCommand::Watch(self.clone())).unwrap();
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
            SEND.get().unwrap().send(ServeCommand::Stop).unwrap();
            tracing::info!("{}", ansi_term::Color::Cyan.paint(t!("Stopping server...")));
            while let Ok(()) = end_rx.recv() {
                tracing::info!("{}", ansi_term::Color::Cyan.paint(t!("Server stopped")));
            }
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
                    .arg("build")
                    .stdout(Stdio::piped())
                    .spawn()
                    .expect("failed to start cargo")
            } else {
                Command::new("cargo")
                    .arg("build")
                    .arg("--release")
                    .stdout(Stdio::piped())
                    .spawn()
                    .expect("failed to start cargo")
            };
        };
        let mut child = child();

        let _ = child.wait().expect("failed to wait on child");
        let child_run = || {
            if !args.release {
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
            }
        };
        let child_run = child_run();

        unsafe {
            CURRENT_PROCESS.get_mut().unwrap().replace(child_run);
        }
        return ServeStatus::Worked;
    }

    pub fn work_stop() -> ServeStatus {
        unsafe {
            if let Ok(Some(child)) = CURRENT_PROCESS.get_mut() {
                child.kill().ok();
            }
        }
        ServeStatus::End
    }

    pub fn tracing(rx: std::sync::mpsc::Receiver<TracingCommand>) {
        spawn(move || {
            let join_handle = Arc::pin(Mutex::new(None));
            while let Ok(rx) = rx.recv() {
                match rx {
                    TracingCommand::Start => {
                        join_handle.lock().as_mut().unwrap().replace(spawn(move || {
                            unsafe {
                                if let Ok(Some(child)) = CURRENT_PROCESS.get_mut() {
                                    let output = child.stdout.as_mut().expect("failed to get stdout");
                                    let mut reader = BufReader::new(output);
                                    loop {
                                        let mut line = String::new();
                                        match reader.read_line(&mut line) {
                                            Ok(0) => {
                                                break;
                                            }
                                            Ok(_) => {
                                                if
                                                line.contains("WARN")
                                                || line.contains("ERROR")
                                                || line.contains("INFO")
                                                || line.contains("DEBUG")
                                                || line.contains("TRACE"){
                                                    print!("{}\n", line.replace("\n", "").trim_start());
                                                }
                                                line.clear();
                                            }
                                            Err(e) => {
                                                eprintln!("failed to read line: {}", e);
                                                break;
                                            }
                                        }
                                        std::thread::sleep(Duration::from_millis(20));
                                    }
                                }
                            }
                        }));
                    }
                    TracingCommand::Stop => {
                        unsafe {
                            if let Ok(Some(child)) = CURRENT_PROCESS.get_mut() {
                                child.kill().ok();
                            }
                        }
                        join_handle.lock().as_mut().unwrap().take();
                    }
                }
            }
        });
    }
}
