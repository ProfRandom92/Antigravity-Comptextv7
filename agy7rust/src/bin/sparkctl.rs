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
    #[command(about = "Run local Rust quality checks (fmt, check, test, clippy)")]
    RustValidate,
    #[command(about = "Run complete context lifecycle (build, render, validate)")]
    ContextAll,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Doctor => {
            sparkctl::doctor::run_doctor()?;
        }
        Commands::RustValidate => {
            sparkctl::rust_validate::run_rust_validate()?;
        }
        Commands::ContextAll => {
            sparkctl::context_all::run_context_all()?;
        }
    }

    Ok(())
}
