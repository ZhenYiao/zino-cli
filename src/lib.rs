#![allow(unreachable_code)]

mod new;
mod serve;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub enum Args {
    #[clap(name="new", about="Create a new project")]
    New,
    #[clap(name="serve", about="Serve the project")]
    Serve(serve::ServeArgs),
}

pub async fn parse() -> anyhow::Result<()>{
    tracing_subscriber::fmt().init();
    let resp = match Args::parse() {
        Args::New => {
            new::get_user_selected()
        }
        Args::Serve(x) => {
            x.exec().await
        }
    };
    match resp{
        Ok(_) => {}
        Err(e) => {
            println!("Error: {}",e);
        }
    }
    Ok(())
}
