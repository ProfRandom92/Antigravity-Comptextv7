# SPARK Replay Failure Labels and Verification Standards

This document defines the structured failure labels, ledger validation criteria, and cryptographic verification policies implemented within the `CompText-Sparkctl` toolkit for the BMDS SPARK Hackathon.

## 1. Replay Failure Labels

During trace validation or package replay operations, any error or schema mismatch is mapped to one of the following structured failure labels:

### EVIDENCE_LOSS
* **Definition:** Critical payload elements, metadata, or step-sequence logs are missing or stripped.
* **Indicators:** Expected JSON fields are absent, the artifact manifest is missing entries, or the tool sequence chain is broken.

### CONSTRAINT_DRIFT
* **Definition:** Decoded values, commitments, or state hashes diverge from their initial baseline configuration.
* **Indicators:** Mismatch in the payload SHA-256 hash, key-ordering changes, or mismatch in the cryptographic signatures.

## 2. Ledger and Hash Chain Validation

To guarantee forensic auditability without live environment dependencies, the system enforces a strict cryptographic chain check:
1. **Entry Chaining:** Each ledger entry in the evidence package contains a `previous_hash` field that must match the `entry_hash` of the preceding entry.
2. **Root Anchoring:** The `ledger_root` field of the overall package envelope must equal the `entry_hash` of the final ledger transaction.
3. **Payload Mapping:** The `payload_hash` in the sidecar must match the SHA-256 checksum of the decoded payload.

## 3. Canonical JSON & Stable Serialization

To prevent environment-dependent hash drift (e.g. from key ordering in dictionaries or array whitespace variations), all hashes are computed over **Canonical JSON**:
* Keys must be recursively sorted in ascending ASCII order.
* Whitespace between tokens is completely stripped during canonical representation compilation.

## 4. Pre-Replay Validation Guardrail

To maintain strict execution safety:
* A package **must** successfully pass cryptographic signature checks (`verify_package_value`) before the step-simulation runner is allowed to parse and execute replay steps.
* Any verification failure halts the execution immediately and blocks replay.

## 5. Synthetic-Only SPARK Evidence Boundary

The verification and planning pipeline operates strictly inside a mock boundary:
* All input fixtures contain strictly synthetic planning data.
* The system must not process real citizen, administrator, or live public-sector case files.

## 6. Safety Exclusions

The current `CompText-Sparkctl` alignment scope excludes the following capabilities:
* **No Autonomous Recovery or Repair:** The system is strictly diagnostic and records evidence; it does not perform automated data repair.
* **Legacy Log Sandbox Exclusions:** Domain-specific XENTRY/OBD log X-Engines, consonant signatures, sparse micro-frame synopses, and the four-layer sandwich log parser are legacy design prototypes only and must not be asserted as active SPARK alignment features.
