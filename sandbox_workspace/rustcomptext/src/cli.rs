use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
#[command(name = "rustcomptext")]
#[command(about = "Rust CLI implementation of CompText V7", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, Clone, PartialEq, Eq)]
pub enum Commands {
    Compress {
        input: PathBuf,
        #[arg(long)]
        out: PathBuf,
    },
    Inspect {
        file: PathBuf,
    },
    Verify {
        file: PathBuf,
    },
    Replay {
        file: PathBuf,
        #[arg(long, default_value_t = 0)]
        steps: u64,
    },
    AdversarialTest {
        file: PathBuf,
    },
}
