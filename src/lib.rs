pub mod cli;

use clap::Parser;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
pub async fn run(_args: cli::Args) -> Result<()> {
    Ok(())
}

pub fn get_args() -> Result<cli::Args> {
    Ok(cli::Args::parse())
}
