#![allow(unreachable_code)]

mod new;
mod serve;
mod utils;

use crate::utils::zino_hello;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub enum Args {
    #[clap(name = "new", about = "Create a new project")]
    New,
    #[clap(name = "serve", about = "Serve the project")]
    Serve(serve::ServeArgs),
}

pub async fn parse() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    zino_hello();
    let resp = match Args::parse() {
        Args::New => new::new_parse(),
        Args::Serve(serve) => serve.serve_exec().await,
    };
    match resp {
        Ok(_) => {}
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    Ok(())
}
