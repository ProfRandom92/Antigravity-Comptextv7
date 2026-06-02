use crate::ledger::entry::LedgerEntry;
use serde::{Deserialize, Serialize};

pub const ZERO_HASH: &str = "0000000000000000000000000000000000000000000000000000000000000000";

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct LedgerChain {
    pub version: u32,
    pub entries: Vec<LedgerEntry>,
}

pub fn single_entry_root(entry: &LedgerEntry) -> String {
    entry.entry_hash.clone()
}
