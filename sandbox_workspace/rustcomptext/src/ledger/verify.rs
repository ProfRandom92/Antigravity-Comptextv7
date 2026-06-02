use crate::codec::schema::ContextPackage;
use crate::config::{Config, config_to_stable_json};
use crate::errors::RustCompTextError;
use crate::hash::sha256_hex;
use crate::ledger::chain::ZERO_HASH;
use crate::ledger::entry::compute_entry_hash_fields;

fn is_valid_hash(hash: &str) -> bool {
    hash.len() == 64
        && hash
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
}

pub fn verify_package(package: &ContextPackage) -> Result<(), RustCompTextError> {
    // 1. package.version == 1
    if package.version != 1 {
        return Err(RustCompTextError::Verification(format!(
            "invalid version: expected 1, found {}",
            package.version
        )));
    }

    // 2. package.config_hash is 64 lowercase hex
    if !is_valid_hash(&package.config_hash) {
        return Err(RustCompTextError::Verification(format!(
            "invalid config_hash format: {}",
            package.config_hash
        )));
    }

    // 3. package.input_hash is 64 lowercase hex
    if !is_valid_hash(&package.input_hash) {
        return Err(RustCompTextError::Verification(format!(
            "invalid input_hash format: {}",
            package.input_hash
        )));
    }

    // 4. package.payload_hash is 64 lowercase hex
    if !is_valid_hash(&package.payload_hash) {
        return Err(RustCompTextError::Verification(format!(
            "invalid payload_hash format: {}",
            package.payload_hash
        )));
    }

    // 5. package.ledger_root is 64 lowercase hex
    if !is_valid_hash(&package.ledger_root) {
        return Err(RustCompTextError::Verification(format!(
            "invalid ledger_root format: {}",
            package.ledger_root
        )));
    }

    // 6. package.entries darf nicht leer sein
    if package.entries.is_empty() {
        return Err(RustCompTextError::Verification(
            "entries list is empty".to_string(),
        ));
    }

    // 7. payload_hash == sha256_hex(package.payload.as_bytes())
    let calculated_payload_hash = sha256_hex(package.payload.as_bytes());
    if package.payload_hash != calculated_payload_hash {
        return Err(RustCompTextError::Verification(format!(
            "payload hash mismatch: package has {}, calculated {}",
            package.payload_hash, calculated_payload_hash
        )));
    }

    // 18. config_hash muss gegen Default Config geprüft werden
    let config = Config::default();
    let config_json = config_to_stable_json(&config)?;
    let expected_config_hash = sha256_hex(config_json.as_bytes());
    if package.config_hash != expected_config_hash {
        return Err(RustCompTextError::Verification(format!(
            "config hash mismatch: package has {}, expected {}",
            package.config_hash, expected_config_hash
        )));
    }

    // Validate entries
    let mut prev_hash = ZERO_HASH.to_string();
    for (i, entry) in package.entries.iter().enumerate() {
        // 8. Alle LedgerEntry-Felder müssen valide sein
        if !is_valid_hash(&entry.entry_hash) {
            return Err(RustCompTextError::Verification(format!(
                "entry {} has invalid entry_hash: {}",
                i, entry.entry_hash
            )));
        }
        if !is_valid_hash(&entry.previous_hash) {
            return Err(RustCompTextError::Verification(format!(
                "entry {} has invalid previous_hash: {}",
                i, entry.previous_hash
            )));
        }
        if !is_valid_hash(&entry.input_hash) {
            return Err(RustCompTextError::Verification(format!(
                "entry {} has invalid input_hash: {}",
                i, entry.input_hash
            )));
        }
        if !is_valid_hash(&entry.output_hash) {
            return Err(RustCompTextError::Verification(format!(
                "entry {} has invalid output_hash: {}",
                i, entry.output_hash
            )));
        }
        if entry.operation.is_empty() {
            return Err(RustCompTextError::Verification(format!(
                "entry {} has empty operation",
                i
            )));
        }

        // 9. Ledger indices sind exakt fortlaufend ab 0
        if entry.index != i as u64 {
            return Err(RustCompTextError::Verification(format!(
                "entry {} has invalid index: expected {}, found {}",
                i, i, entry.index
            )));
        }

        // 10. Erster previous_hash == ZERO_HASH
        // 11. Für jeden weiteren Entry: previous_hash == vorheriger entry_hash
        if entry.previous_hash != prev_hash {
            return Err(RustCompTextError::Verification(format!(
                "entry {} previous_hash mismatch: expected {}, found {}",
                i, prev_hash, entry.previous_hash
            )));
        }

        // 12. entry_hash verification
        let calculated_entry_hash = compute_entry_hash_fields(
            entry.index,
            &entry.operation,
            &entry.input_hash,
            &entry.output_hash,
            &entry.previous_hash,
        );
        if entry.entry_hash != calculated_entry_hash {
            return Err(RustCompTextError::Verification(format!(
                "entry {} hash mismatch: entry has {}, calculated {}",
                i, entry.entry_hash, calculated_entry_hash
            )));
        }

        prev_hash = entry.entry_hash.clone();
    }

    // 13. ledger_root == letzter entry_hash
    let last_entry = &package.entries[package.entries.len() - 1];
    if package.ledger_root != last_entry.entry_hash {
        return Err(RustCompTextError::Verification(format!(
            "ledger_root mismatch: package has {}, last entry has {}",
            package.ledger_root, last_entry.entry_hash
        )));
    }

    // 14. package.input_hash == entries[0].input_hash
    if package.input_hash != package.entries[0].input_hash {
        return Err(RustCompTextError::Verification(format!(
            "package input_hash mismatch: package has {}, entry 0 has {}",
            package.input_hash, package.entries[0].input_hash
        )));
    }

    // 15. package.payload_hash == letzter entry.output_hash
    if package.payload_hash != last_entry.output_hash {
        return Err(RustCompTextError::Verification(format!(
            "package payload_hash mismatch: package has {}, last entry output_hash has {}",
            package.payload_hash, last_entry.output_hash
        )));
    }

    // 16. Für plain.v1 Single-Entry-Paket: entries[0].operation == "compress.plain.v1"
    if package.entries[0].operation != "compress.plain.v1" {
        return Err(RustCompTextError::Verification(format!(
            "invalid operation for plain.v1: {}",
            package.entries[0].operation
        )));
    }

    // 17. Für plain.v1 Single-Entry-Paket: entries.len() == 1
    if package.entries.len() != 1 {
        return Err(RustCompTextError::Verification(format!(
            "invalid entry list length for plain.v1: expected 1, found {}",
            package.entries.len()
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ledger::entry::LedgerEntry;

    fn valid_test_package() -> ContextPackage {
        let input_text = "test text";
        let input_hash = sha256_hex(input_text.as_bytes());
        let payload_hash = input_hash.clone();

        let config = Config::default();
        let config_json = config_to_stable_json(&config).unwrap();
        let config_hash = sha256_hex(config_json.as_bytes());

        let index = 0;
        let operation = "compress.plain.v1".to_string();
        let previous_hash = ZERO_HASH.to_string();
        let entry_hash = compute_entry_hash_fields(
            index,
            &operation,
            &input_hash,
            &payload_hash,
            &previous_hash,
        );

        let entry = LedgerEntry {
            index,
            operation,
            input_hash,
            output_hash: payload_hash.clone(),
            previous_hash,
            entry_hash,
        };

        ContextPackage {
            version: 1,
            config_hash,
            input_hash: entry.input_hash.clone(),
            payload_hash,
            ledger_root: entry.entry_hash.clone(),
            entries: vec![entry],
            payload: input_text.to_string(),
        }
    }

    #[test]
    fn test_verify_valid_ok() {
        let package = valid_test_package();
        assert!(verify_package(&package).is_ok());
    }

    #[test]
    fn test_verify_invalid_version() {
        let mut package = valid_test_package();
        package.version = 2;
        assert!(verify_package(&package).is_err());
    }

    #[test]
    fn test_verify_invalid_config_hash() {
        let mut package = valid_test_package();
        package.config_hash = "not-64-hex".to_string();
        assert!(verify_package(&package).is_err());
    }

    #[test]
    fn test_verify_invalid_input_hash() {
        let mut package = valid_test_package();
        package.input_hash = "not-64-hex".to_string();
        assert!(verify_package(&package).is_err());
    }

    #[test]
    fn test_verify_invalid_payload_hash() {
        let mut package = valid_test_package();
        package.payload_hash = "not-64-hex".to_string();
        assert!(verify_package(&package).is_err());
    }

    #[test]
    fn test_verify_invalid_ledger_root() {
        let mut package = valid_test_package();
        package.ledger_root = "not-64-hex".to_string();
        assert!(verify_package(&package).is_err());
    }

    #[test]
    fn test_verify_empty_entries() {
        let mut package = valid_test_package();
        package.entries.clear();
        assert!(verify_package(&package).is_err());
    }

    #[test]
    fn test_verify_invalid_index() {
        let mut package = valid_test_package();
        package.entries[0].index = 1;
        assert!(verify_package(&package).is_err());
    }

    #[test]
    fn test_verify_invalid_previous_hash() {
        let mut package = valid_test_package();
        package.entries[0].previous_hash =
            "1111111111111111111111111111111111111111111111111111111111111111".to_string();
        assert!(verify_package(&package).is_err());
    }

    #[test]
    fn test_verify_invalid_entry_hash() {
        let mut package = valid_test_package();
        package.entries[0].entry_hash =
            "2222222222222222222222222222222222222222222222222222222222222222".to_string();
        assert!(verify_package(&package).is_err());
    }

    #[test]
    fn test_verify_empty_operation() {
        let mut package = valid_test_package();
        package.entries[0].operation = "".to_string();
        assert!(verify_package(&package).is_err());
    }

    #[test]
    fn test_verify_invalid_operation() {
        let mut package = valid_test_package();
        package.entries[0].operation = "compress.invalid.v1".to_string();
        assert!(verify_package(&package).is_err());
    }

    #[test]
    fn test_verify_duplicate_or_extra_entry() {
        let mut package = valid_test_package();
        let mut entry2 = package.entries[0].clone();
        entry2.index = 1;
        entry2.previous_hash = entry2.entry_hash.clone();
        entry2.entry_hash = compute_entry_hash_fields(
            entry2.index,
            &entry2.operation,
            &entry2.input_hash,
            &entry2.output_hash,
            &entry2.previous_hash,
        );
        package.entries.push(entry2);
        assert!(verify_package(&package).is_err());
    }

    #[test]
    fn test_verify_non_hex_hash() {
        let mut package = valid_test_package();
        package.config_hash.replace_range(63..64, "g");
        assert!(verify_package(&package).is_err());
    }

    #[test]
    fn test_verify_uppercase_hash() {
        let mut package = valid_test_package();
        package.config_hash = package.config_hash.to_uppercase();
        assert!(verify_package(&package).is_err());
    }

    #[test]
    fn test_verify_too_short_hash() {
        let mut package = valid_test_package();
        package.config_hash.pop();
        assert!(verify_package(&package).is_err());
    }

    #[test]
    fn test_verify_payload_tampered() {
        let mut package = valid_test_package();
        package.payload = "tampered text".to_string();
        assert!(verify_package(&package).is_err());
    }

    #[test]
    fn test_verify_payload_hash_mismatch() {
        let mut package = valid_test_package();
        package.payload = "tampered text".to_string();
        package.payload_hash = sha256_hex(package.payload.as_bytes());
        assert!(verify_package(&package).is_err());
    }

    #[test]
    fn test_verify_entry_output_hash_mismatch() {
        let mut package = valid_test_package();
        package.payload = "tampered text".to_string();
        package.payload_hash = sha256_hex(package.payload.as_bytes());
        package.entries[0].output_hash = package.payload_hash.clone();
        assert!(verify_package(&package).is_err());
    }
}
