//! Tool for managing atsamd-rs/atsamd

mod error;
mod example;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about = "Manages atsamd-rs/atsamd", long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Manage examples provided for BSPs
    #[command(subcommand)]
    Example(example::Commands),
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Example(example_commands) => example::run(example_commands),
    }
    .unwrap();
}
