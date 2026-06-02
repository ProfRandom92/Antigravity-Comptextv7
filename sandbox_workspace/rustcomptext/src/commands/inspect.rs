use crate::codec::decode::decode_package;
use crate::codec::schema::ContextPackage;
use crate::errors::RustCompTextError;
use std::fs;
use std::path::Path;

pub fn run(file: &Path) -> Result<(), RustCompTextError> {
    // Read package from file
    let content = fs::read_to_string(file)?;

    // Decode package
    let package = decode_package(&content)?;

    // Render inspection
    let output = render_inspection(&package);

    // Print to stdout
    print!("{}", output);

    Ok(())
}

pub fn render_inspection(package: &ContextPackage) -> String {
    let mut out = String::new();
    out.push_str(&format!("version: {}\n", package.version));
    out.push_str(&format!("config_hash: {}\n", package.config_hash));
    out.push_str(&format!("input_hash: {}\n", package.input_hash));
    out.push_str(&format!("payload_hash: {}\n", package.payload_hash));
    out.push_str(&format!("ledger_root: {}\n", package.ledger_root));
    out.push_str(&format!("entries: {}\n", package.entries.len()));
    out.push_str("operations:\n");
    for entry in &package.entries {
        out.push_str(&format!("- {}: {}\n", entry.index, entry.operation));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ledger::entry::LedgerEntry;

    #[test]
    fn test_render_inspection_contains_version_hashes_and_counts() {
        let entry = LedgerEntry {
            index: 0,
            operation: "compress.plain.v1".to_string(),
            input_hash: "def".to_string(),
            output_hash: "ghi".to_string(),
            previous_hash: "prev".to_string(),
            entry_hash: "ehash".to_string(),
        };
        let package = ContextPackage {
            version: 1,
            config_hash: "abc".to_string(),
            input_hash: "def".to_string(),
            payload_hash: "ghi".to_string(),
            ledger_root: "jkl".to_string(),
            entries: vec![entry],
            payload: "secret payload".to_string(),
        };

        let output = render_inspection(&package);

        // Asserts
        assert!(output.contains("version: 1\n"));
        assert!(output.contains("config_hash: abc\n"));
        assert!(output.contains("input_hash: def\n"));
        assert!(output.contains("payload_hash: ghi\n"));
        assert!(output.contains("ledger_root: jkl\n"));
        assert!(output.contains("entries: 1\n"));
        assert!(output.contains("operations:\n- 0: compress.plain.v1\n"));

        // Payload must NOT be outputted
        assert!(!output.contains("secret payload"));
    }
}
