use crate::codec::decode::decode_package;
use crate::errors::RustCompTextError;
use crate::replay::runner::replay_package;
use std::fs;
use std::path::Path;
use std::process;

pub fn run(file: &Path, steps: u64) -> Result<(), RustCompTextError> {
    let result = (|| -> Result<_, RustCompTextError> {
        let content = fs::read_to_string(file)?;
        let package = decode_package(&content)?;
        let report = replay_package(&package, steps)?;
        Ok(report)
    })();

    match result {
        Ok(report) => {
            println!("replay: ok");
            println!("steps_requested: {}", report.steps_requested);
            println!("steps_replayed: {}", report.steps_replayed);
            println!("ledger_root: {}", report.ledger_root);
            Ok(())
        }
        Err(err) => {
            eprintln!("replay: failed");
            eprintln!("reason: {}", err);
            process::exit(1);
        }
    }
}
