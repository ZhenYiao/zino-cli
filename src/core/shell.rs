use std::fmt::Debug;
use std::io::Write;
use std::sync::LazyLock;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::JoinHandle;

pub static SHELL: LazyLock<Shell<String>> = LazyLock::new(|| Shell::new());

pub struct Shell<T>{
    pub tx: Sender<T>,
    pub join_handle: JoinHandle<()>
}


impl <T>Shell<T>
where T: Send + 'static + Debug
{
    pub fn new() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        let handle = Self::handle(rx);
        Self {
            tx,
            join_handle: handle,
        }
    }
    pub fn send(&self, command: T) {
        self.tx.send(command).unwrap();
    }
    pub fn stop(self) {
        drop(self.tx);
        self.join_handle.join().unwrap();
    }
    pub fn handle(rx: Receiver<T>) -> JoinHandle<()> {
        return std::thread::spawn(move || {
            let stdout = std::io::stdout();
            loop {
                match rx.recv() {
                    Ok(command) => {
                        write!(&stdout, "{:?}", command).ok();
                    }
                    Err(_) => {
                        break;
                    }
                }
            }
        })
    }
}