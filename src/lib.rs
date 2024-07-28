#![allow(unreachable_code)]

mod new;
mod serve;
mod utils;
mod i18n;
mod core;

use ansi_term::Color::Red;
use clap::Parser;
rust_i18n::i18n!("locales");
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
    let resp = match Args::parse() {
        Args::New => new::new_parse().await,
        Args::Serve(serve) => serve.serve_exec().await,
    };
    match resp {
        Ok(_) => {}
        Err(e) => {
            println!("{}", Red.paint(format!("Error: {}", e)));
        }
    }
    Ok(())
}
