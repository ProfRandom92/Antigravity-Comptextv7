use anyhow::Result;
use clap::{Parser, Subcommand};

#[path = "../sparkctl/mod.rs"]
mod sparkctl;

#[derive(Parser)]
#[command(name = "sparkctl")]
#[command(about = "SPARK Operational Context Layer CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Diagnose local project readiness")]
    Doctor,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Doctor => {
            sparkctl::doctor::run_doctor()?;
        }
    }

    Ok(())
}
