use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
pub enum Command {}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}
