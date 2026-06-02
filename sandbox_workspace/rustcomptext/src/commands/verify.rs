use crate::codec::decode::decode_package;
use crate::errors::RustCompTextError;
use crate::ledger::verify::verify_package;
use std::fs;
use std::path::Path;
use std::process;

pub fn run(file: &Path) -> Result<(), RustCompTextError> {
    let result = (|| -> Result<(), RustCompTextError> {
        let content = fs::read_to_string(file)?;
        let package = decode_package(&content)?;
        verify_package(&package)?;
        Ok(())
    })();

    match result {
        Ok(()) => {
            println!("verify: ok");
            Ok(())
        }
        Err(err) => {
            eprintln!("verify: failed");
            eprintln!("reason: {}", err);
            process::exit(1);
        }
    }
}
