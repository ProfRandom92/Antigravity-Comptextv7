use crate::codec::package::{replay_package_value, sort_json_value};
use anyhow::{Context, Result};
use std::fs;

pub fn run(input_path: &str) -> Result<()> {
    let content = fs::read_to_string(input_path)
        .with_context(|| format!("Failed to read package file: {}", input_path))?;

    let package_val: serde_json::Value = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse package JSON from: {}", input_path))?;

    match replay_package_value(&package_val) {
        Ok(replay_val) => {
            let sorted_replay = sort_json_value(&replay_val);
            let pretty_str = serde_json::to_string_pretty(&sorted_replay)?;
            println!("{}", pretty_str);
            Ok(())
        }
        Err(e) => {
            eprintln!("replay failed: {}", e);
            Err(e)
        }
    }
}
