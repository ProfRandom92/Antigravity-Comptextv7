use crate::codec::schema::ContextPackage;
use crate::config::{Config, config_to_stable_json};
use crate::errors::RustCompTextError;
use crate::hash::sha256_hex;
use crate::ledger::entry::{LedgerEntry, compute_entry_hash_fields};

pub fn encode_package(package: &ContextPackage) -> Result<String, RustCompTextError> {
    let serialized = serde_json::to_string(package).map_err(RustCompTextError::Json)?;
    Ok(serialized)
}

pub fn build_plain_v1_package(input_text: &str) -> Result<ContextPackage, RustCompTextError> {
    let input_hash = sha256_hex(input_text.as_bytes());
    let payload_hash = input_hash.clone();

    let config = Config::default();
    let config_json = config_to_stable_json(&config)?;
    let config_hash = sha256_hex(config_json.as_bytes());

    let index = 0;
    let operation = "compress.plain.v1".to_string();
    let previous_hash = crate::ledger::chain::ZERO_HASH.to_string();
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

    let ledger_root = crate::ledger::chain::single_entry_root(&entry);

    Ok(ContextPackage {
        version: 1,
        config_hash,
        input_hash: entry.input_hash.clone(),
        payload_hash,
        ledger_root,
        entries: vec![entry],
        payload: input_text.to_string(),
    })
}
