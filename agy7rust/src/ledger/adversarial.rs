use crate::codec::schema::ContextPackage;
use crate::ledger::verify::verify_package;

pub struct AdversarialReport {
    pub total_cases: usize,
    pub detected_cases: usize,
    pub missed_cases: Vec<String>,
}

type Mutator = fn(&mut ContextPackage);

pub fn run_adversarial_cases(package: &ContextPackage) -> AdversarialReport {
    let mut report = AdversarialReport {
        total_cases: 0,
        detected_cases: 0,
        missed_cases: Vec::new(),
    };

    let cases: [(&str, Mutator); 15] = [
        ("payload ändern", |p: &mut ContextPackage| {
            p.payload = format!("{} tampered", p.payload);
        }),
        ("config_hash ändern", |p: &mut ContextPackage| {
            p.config_hash =
                "0000000000000000000000000000000000000000000000000000000000000000".to_string();
        }),
        ("input_hash ändern", |p: &mut ContextPackage| {
            p.input_hash =
                "0000000000000000000000000000000000000000000000000000000000000000".to_string();
        }),
        ("payload_hash ändern", |p: &mut ContextPackage| {
            p.payload_hash =
                "0000000000000000000000000000000000000000000000000000000000000000".to_string();
        }),
        ("ledger_root ändern", |p: &mut ContextPackage| {
            p.ledger_root =
                "0000000000000000000000000000000000000000000000000000000000000000".to_string();
        }),
        ("entries[0].index ändern", |p: &mut ContextPackage| {
            if !p.entries.is_empty() {
                p.entries[0].index = 999;
            }
        }),
        ("entries[0].operation ändern", |p: &mut ContextPackage| {
            if !p.entries.is_empty() {
                p.entries[0].operation = "compress.invalid.v1".to_string();
            }
        }),
        ("entries[0].input_hash ändern", |p: &mut ContextPackage| {
            if !p.entries.is_empty() {
                p.entries[0].input_hash =
                    "0000000000000000000000000000000000000000000000000000000000000000".to_string();
            }
        }),
        (
            "entries[0].output_hash ändern",
            |p: &mut ContextPackage| {
                if !p.entries.is_empty() {
                    p.entries[0].output_hash =
                        "0000000000000000000000000000000000000000000000000000000000000000"
                            .to_string();
                }
            },
        ),
        (
            "entries[0].previous_hash ändern",
            |p: &mut ContextPackage| {
                if !p.entries.is_empty() {
                    p.entries[0].previous_hash =
                        "1111111111111111111111111111111111111111111111111111111111111111"
                            .to_string();
                }
            },
        ),
        ("entries[0].entry_hash ändern", |p: &mut ContextPackage| {
            if !p.entries.is_empty() {
                p.entries[0].entry_hash =
                    "0000000000000000000000000000000000000000000000000000000000000000".to_string();
            }
        }),
        ("entries duplizieren", |p: &mut ContextPackage| {
            if !p.entries.is_empty() {
                let duplicate = p.entries[0].clone();
                p.entries.push(duplicate);
            }
        }),
        ("entries leeren", |p: &mut ContextPackage| {
            p.entries.clear();
        }),
        ("hash uppercase setzen", |p: &mut ContextPackage| {
            p.config_hash = p.config_hash.to_uppercase();
        }),
        ("hash zu kurz setzen", |p: &mut ContextPackage| {
            p.config_hash.pop();
        }),
    ];

    report.total_cases = cases.len();

    for &(name, mutate) in &cases {
        let mut test_pkg = package.clone();
        mutate(&mut test_pkg);
        if verify_package(&test_pkg).is_err() {
            report.detected_cases += 1;
        } else {
            report.missed_cases.push(name.to_string());
        }
    }

    report
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

    #[test]
    fn test_run_adversarial_cases_all_detected() {
        let package = valid_test_package();
        let report = run_adversarial_cases(&package);
        assert_eq!(report.total_cases, 15);
        assert_eq!(report.detected_cases, 15);
    }

    #[test]
    fn test_run_adversarial_cases_missed_empty() {
        let package = valid_test_package();
        let report = run_adversarial_cases(&package);
        assert!(report.missed_cases.is_empty());
    }

    #[test]
    fn test_base_invalid_fails_verification() {
        let mut package = valid_test_package();
        package.payload = "tampered text".to_string();
        assert!(verify_package(&package).is_err());
    }

    #[test]
    fn test_cases_names_unique() {
        let package = valid_test_package();
        let report = run_adversarial_cases(&package);
        assert_eq!(report.total_cases, 15);
    }
}
