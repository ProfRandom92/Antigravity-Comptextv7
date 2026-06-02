use crate::ledger::entry::LedgerEntry;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ContextPackage {
    pub version: u32,
    pub config_hash: String,
    pub input_hash: String,
    pub payload_hash: String,
    pub ledger_root: String,
    pub entries: Vec<LedgerEntry>,
    pub payload: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ledger::entry::LedgerEntry;

    #[test]
    fn test_context_package_roundtrip() {
        let entry = LedgerEntry {
            index: 1,
            operation: "test".to_string(),
            input_hash: "in".to_string(),
            output_hash: "out".to_string(),
            previous_hash: "prev".to_string(),
            entry_hash: "ehash".to_string(),
        };
        let package = ContextPackage {
            version: 1,
            config_hash: "conf".to_string(),
            input_hash: "in".to_string(),
            payload_hash: "pay".to_string(),
            ledger_root: "root".to_string(),
            entries: vec![entry],
            payload: "content".to_string(),
        };
        let serialized = serde_json::to_string(&package).unwrap();
        let deserialized: ContextPackage = serde_json::from_str(&serialized).unwrap();
        assert_eq!(package, deserialized);
    }
}
