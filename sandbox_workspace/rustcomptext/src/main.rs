use clap::Parser;
use rustcomptext::cli::{Cli, Commands};
use std::process;

fn main() {
    let args = Cli::parse();
    let result = match args.command {
        Commands::Compress { input, out } => rustcomptext::commands::compress::run(&input, &out),
        Commands::Inspect { file } => rustcomptext::commands::inspect::run(&file),
        Commands::Verify { file } => rustcomptext::commands::verify::run(&file),
        Commands::Replay { file, steps } => rustcomptext::commands::replay::run(&file, steps),
        Commands::AdversarialTest { file } => rustcomptext::commands::adversarial::run(&file),
    };

    if let Err(err) = result {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}
