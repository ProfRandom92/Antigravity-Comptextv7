use crate::codec::decode::decode_package;
use crate::errors::RustCompTextError;
use crate::ledger::adversarial::run_adversarial_cases;
use crate::ledger::verify::verify_package;
use std::fs;
use std::path::Path;
use std::process;

pub fn run(file: &Path) -> Result<(), RustCompTextError> {
    let content_res = fs::read_to_string(file);
    let package = match content_res {
        Ok(c) => match decode_package(&c) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("adversarial-test: failed");
                eprintln!("reason: {}", e);
                process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("adversarial-test: failed");
            eprintln!("reason: {}", e);
            process::exit(1);
        }
    };

    // Verify original package first
    if let Err(e) = verify_package(&package) {
        eprintln!("adversarial-test: failed");
        eprintln!("reason: {}", e);
        process::exit(1);
    }

    let report = run_adversarial_cases(&package);

    if report.missed_cases.is_empty() {
        println!("adversarial-test: ok");
        println!(
            "tamper cases detected: {}/{}",
            report.detected_cases, report.total_cases
        );
        Ok(())
    } else {
        println!("adversarial-test: failed");
        println!(
            "tamper cases detected: {}/{}",
            report.detected_cases, report.total_cases
        );
        println!("missed:");
        for case in &report.missed_cases {
            println!("- {}", case);
        }
        process::exit(1);
    }
}
