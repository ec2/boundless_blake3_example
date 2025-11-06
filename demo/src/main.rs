use anyhow::Result;

use clap::{Parser, Subcommand};
use std::str::FromStr;
use tracing_subscriber::{EnvFilter, filter::LevelFilter, prelude::*};

mod boundless;
mod risc0;
#[derive(Subcommand, Clone, Debug)]
enum Command {
    /// Commands for requestors submitting proof requests
    Boundless(Box<boundless::Args>),
    /// Commands for risc0 proof generation and verification
    Risc0(Box<risc0::Args>),
}

#[derive(Parser, Debug)]

struct Args {
    /// Subcommand to run
    #[command(subcommand)]
    command: Command,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging.
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::from_str("info")?.into())
                .from_env_lossy(),
        )
        .init();
    let args = Args::parse();
    match args.command {
        Command::Boundless(cmd) => cmd.run().await?,
        Command::Risc0(cmd) => cmd.run().await?,
    }
    Ok(())
}
