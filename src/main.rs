#![allow(non_snake_case, non_upper_case_globals)]

use zino_cli::parse;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    parse().await
}
