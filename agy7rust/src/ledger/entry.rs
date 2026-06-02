use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct LedgerEntry {
    pub index: u64,
    pub operation: String,
    pub input_hash: String,
    pub output_hash: String,
    pub previous_hash: String,
    pub entry_hash: String,
}

pub fn compute_entry_hash_fields(
    index: u64,
    operation: &str,
    input_hash: &str,
    output_hash: &str,
    previous_hash: &str,
) -> String {
    let mut data = String::new();
    data.push_str(&index.to_string());
    data.push('\n');
    data.push_str(operation);
    data.push('\n');
    data.push_str(input_hash);
    data.push('\n');
    data.push_str(output_hash);
    data.push('\n');
    data.push_str(previous_hash);
    data.push('\n');

    crate::hash::sha256_hex(data.as_bytes())
}

pub fn compute_entry_hash(entry_without_hash: &LedgerEntry) -> String {
    compute_entry_hash_fields(
        entry_without_hash.index,
        &entry_without_hash.operation,
        &entry_without_hash.input_hash,
        &entry_without_hash.output_hash,
        &entry_without_hash.previous_hash,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ledger_entry_roundtrip() {
        let entry = LedgerEntry {
            index: 10,
            operation: "hash".to_string(),
            input_hash: "in".to_string(),
            output_hash: "out".to_string(),
            previous_hash: "prev".to_string(),
            entry_hash: "ehash".to_string(),
        };
        let serialized = serde_json::to_string(&entry).unwrap();
        let deserialized: LedgerEntry = serde_json::from_str(&serialized).unwrap();
        assert_eq!(entry, deserialized);
    }

    #[test]
    fn test_entry_hash_determinism() {
        let hash1 = compute_entry_hash_fields(0, "op", "in", "out", "prev");
        let hash2 = compute_entry_hash_fields(0, "op", "in", "out", "prev");
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_entry_hash_changes_with_op() {
        let hash1 = compute_entry_hash_fields(0, "op1", "in", "out", "prev");
        let hash2 = compute_entry_hash_fields(0, "op2", "in", "out", "prev");
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_entry_hash_ignores_existing_hash_field() {
        let entry1 = LedgerEntry {
            index: 42,
            operation: "test".to_string(),
            input_hash: "hash_in".to_string(),
            output_hash: "hash_out".to_string(),
            previous_hash: "hash_prev".to_string(),
            entry_hash: "existing_incorrect_hash".to_string(),
        };
        let hash_derived = compute_entry_hash(&entry1);

        let entry2 = LedgerEntry {
            index: 42,
            operation: "test".to_string(),
            input_hash: "hash_in".to_string(),
            output_hash: "hash_out".to_string(),
            previous_hash: "hash_prev".to_string(),
            entry_hash: "some_other_existing_hash".to_string(),
        };
        let hash_derived_2 = compute_entry_hash(&entry2);

        assert_eq!(hash_derived, hash_derived_2);
        assert_ne!(hash_derived, "existing_incorrect_hash");
    }
}
