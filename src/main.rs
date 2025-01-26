mod cli;
mod grep;

use clap::Parser;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    if let Err(e) = get_args().and_then(run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

#[tokio::main]
pub async fn run(args: cli::Args) -> Result<()> {
    match args.command {
        cli::Command::Grep { flag } => {
            // Call the function from grep.rs
            grep::invoke(flag);

            Ok(())
        }
    }
}

pub fn get_args() -> Result<cli::Args> {
    Ok(cli::Args::parse())
}
