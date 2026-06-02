use crate::codec::schema::ContextPackage;
use crate::errors::RustCompTextError;
use crate::ledger::verify::verify_package;

pub struct ReplayReport {
    pub steps_requested: u64,
    pub steps_replayed: u64,
    pub ledger_root: String,
}

pub fn replay_package(
    package: &ContextPackage,
    steps: u64,
) -> Result<ReplayReport, RustCompTextError> {
    // 1. Must call verify_package(package)? first
    verify_package(package)?;

    let entries_len = package.entries.len() as u64;
    let steps_replayed = if steps == 0 {
        0
    } else if steps > entries_len {
        entries_len
    } else {
        steps
    };

    Ok(ReplayReport {
        steps_requested: steps,
        steps_replayed,
        ledger_root: package.ledger_root.clone(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Config, config_to_stable_json};
    use crate::hash::sha256_hex;
    use crate::ledger::chain::ZERO_HASH;
    use crate::ledger::entry::{LedgerEntry, compute_entry_hash_fields};

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

    // 1. gültiges Paket replay ok
    #[test]
    fn test_replay_valid_ok() {
        let package = valid_test_package();
        let res = replay_package(&package, 10);
        assert!(res.is_ok());
        let report = res.unwrap();
        assert_eq!(report.steps_requested, 10);
        assert_eq!(report.steps_replayed, 1);
    }

    // 2. payload-manipuliertes Paket muss fehlschlagen
    #[test]
    fn test_replay_payload_tampered_fails() {
        let mut package = valid_test_package();
        package.payload = "tampered text".to_string();
        let res = replay_package(&package, 10);
        assert!(res.is_err());
    }

    // 3. steps == 0 ergibt steps_replayed == 0
    #[test]
    fn test_replay_steps_zero() {
        let package = valid_test_package();
        let report = replay_package(&package, 0).unwrap();
        assert_eq!(report.steps_replayed, 0);
    }

    // 4. steps > entries.len() ergibt steps_replayed == entries.len()
    #[test]
    fn test_replay_steps_larger() {
        let package = valid_test_package();
        let report = replay_package(&package, 100).unwrap();
        assert_eq!(report.steps_replayed, 1);
    }

    // 5. steps == 1 ergibt steps_replayed == 1
    #[test]
    fn test_replay_steps_one() {
        let package = valid_test_package();
        let report = replay_package(&package, 1).unwrap();
        assert_eq!(report.steps_replayed, 1);
    }

    // 6. ledger_root im Report entspricht package.ledger_root
    #[test]
    fn test_replay_ledger_root_matches() {
        let package = valid_test_package();
        let report = replay_package(&package, 1).unwrap();
        assert_eq!(report.ledger_root, package.ledger_root);
    }

    // 7. ungültige operation wird abgelehnt
    #[test]
    fn test_replay_invalid_operation_fails() {
        let mut package = valid_test_package();
        package.entries[0].operation = "compress.invalid.v1".to_string();
        package.entries[0].entry_hash = compute_entry_hash_fields(
            package.entries[0].index,
            &package.entries[0].operation,
            &package.entries[0].input_hash,
            &package.entries[0].output_hash,
            &package.entries[0].previous_hash,
        );
        package.ledger_root = package.entries[0].entry_hash.clone();

        let res = replay_package(&package, 10);
        assert!(res.is_err());
    }
}
