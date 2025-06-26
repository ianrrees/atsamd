//! Tool for managing atsamd-rs/atsamd
//!
//! ## Implementation
//! At present, this tool is only used for managing the BSP examples, but it's
//! anticipated that usage will expand to tasks that we currently handle using
//! shell/Python scripts e.g. updating the PACs or various CI tasks.
//!
//! Each subcommand is associated with a `mod` which provides a `fn run(config:
//! toml::Table, commands: impl clap::Subcommand)` as the entry point from the
//! top-level program.

mod error;
mod example;

use clap::{Parser, Subcommand};
use error::Result;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::process::exit;
use toml::Table;

#[derive(Parser)]
#[command(version, about = "Manages atsamd-rs/atsamd", long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// Path to TOML configuration file
    #[arg(long, short, default_value = "manage.toml")]
    config_file: PathBuf,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Manage examples provided for BSPs
    #[command(subcommand)]
    Example(example::Commands),
}

/// Parses the config file, calling exit(2) on failure
fn read_config(path: &PathBuf) -> Table {
    let inner = |path| -> Result<Table> {
        let toml = read_to_string(path)?;
        Ok(toml.parse::<Table>()?)
    };

    inner(path)
        .map_err(|err| {
            eprintln!("Failed to parse {}: {err}", path.to_string_lossy());
            exit(2);
        })
        .unwrap()
}

fn main() {
    let cli = Cli::parse();

    let config = read_config(&cli.config_file);

    match &cli.command {
        Commands::Example(example_commands) => example::run(config, example_commands),
    }
    .map_err(|err| match err {
        error::Error::Logged => {
            exit(1);
        }
        err => {
            eprintln!("Command failed with error: {err}");
            exit(2);
        }
    })
    .unwrap();
}
