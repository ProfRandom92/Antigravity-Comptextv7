# Codex Handoff: SPARK Evidence Packet v1

## 1. Current Workspace Paths

- Working folder: `C:\Users\contr\Desktop\comptext-sparkctl-codex-work`
- Repo root: `C:\Users\contr\Desktop\comptext-sparkctl-codex-work\comptext-sparkctl`
- Rust crate root: `C:\Users\contr\Desktop\comptext-sparkctl-codex-work\comptext-sparkctl\agy7rust`
- Final ZIP path: `C:\Users\contr\Desktop\comptext-sparkctl-spark-ready.zip`

## 2. Current Git State

- Current branch: `main`
- Latest local commit: `c2f41e3`
- `git remote -v`:

```text
origin	https://github.com/ProfRandom92/comptext-sparkctl (fetch)
origin	DISABLED (push)
```

- Push URL status: `DISABLED`
- `git status --short` summary at handoff creation:

```text
 M AGENTS.md
 M agy7rust/src/bin/sparkctl.rs
 M agy7rust/src/codec/package.rs
 M agy7rust/src/lib.rs
 M agy7rust/src/sparkctl/mod.rs
 M agy7rust/tests/spark_roundtrip.rs
 M reports/performance_baseline.json
?? .agents/
?? agy7rust/src/sparkctl/spark_evidence.rs
?? artifacts/spark/evidence_packet_v1.json
?? docs/
?? reports/latest.json
```

No local implementation commit has been created yet.

## 3. Implemented Feature Summary

- Added SPARK Evidence Packet v1 as a local deterministic evidence envelope.
- Added `SparkEvidencePacketPreimage` for hash input fields only.
- Added `SparkEvidencePacketEnvelope` containing the preimage plus derived `canonical_json` and `canonical_hash`.
- `canonical_json` is computed from preimage fields only.
- `canonical_hash = sha256_hex(canonical_json)`.
- Added `sparkctl spark-evidence-demo --output ../artifacts/spark/evidence_packet_v1.json`.
- Added `sparkctl spark-evidence-validate --input ../artifacts/spark/evidence_packet_v1.json`.
- Added tamper validation tests for stale preimage, stale canonical JSON, stale hash, and missing required fields.
- Added docs for SPARK alignment, artifact contract, safety/claims, implementation report, README update proposal, and this handoff.

## 4. Intended Committed Files

- `AGENTS.md`
- `.agents/skills/**`
- `agy7rust/src/bin/sparkctl.rs`
- `agy7rust/src/codec/package.rs`
- `agy7rust/src/lib.rs`
- `agy7rust/src/sparkctl/mod.rs`
- `agy7rust/src/sparkctl/spark_evidence.rs`
- `agy7rust/tests/spark_roundtrip.rs`
- `artifacts/spark/evidence_packet_v1.json`
- `docs/SPARK_ALIGNMENT.md`
- `docs/ARTIFACT_CONTRACT.md`
- `docs/SAFETY_AND_CLAIMS.md`
- `docs/IMPLEMENTATION_REPORT.md`
- `docs/README_UPDATE_PROPOSAL.md`
- `docs/CODEX_HANDOFF_SPARK_EVIDENCE_PACKET_V1.md`

## 5. Files That Must NOT Be Committed

- `reports/latest.json`
- `reports/performance_baseline.json` if it is only validation churn
- `target/`
- `.env`
- caches
- secrets
- unrelated files

## 6. Validation Results Already Achieved

- `cargo fmt --all --check`: PASS
- `cargo test`: PASS
- `cargo clippy --all-targets --all-features -- -D warnings`: PASS
- `cargo run --bin sparkctl -- spark-evidence-validate --input ../artifacts/spark/evidence_packet_v1.json`: PASS
- Tamper tests: PASS

## 7. Critical Invariants

- Root `README.md` untouched.
- `canonical_json` is derived only from preimage fields.
- `canonical_hash` is computed only from `canonical_json`.
- No timestamp, random value, environment value, local absolute path, or machine-specific value is included in the canonical hash input.
- Provider output remains untrusted.
- Policy Gate remains before provider proposal.
- Human Review remains the approval boundary.
- Goal never bypasses Policy Gate, Provider Boundary, or Human Review.
- No provider calls were added.
- No GitHub writes were added.
- No compliance, legal, forensic, or production claims were added.

## 8. Remaining Next Step

No local implementation commit exists yet.

Recommended next safe sequence:

1. Clean generated report churn:
   - remove `reports/latest.json`
   - restore `reports/performance_baseline.json` if it is only validation churn
2. Create local branch `spark-evidence-packet-v1`.
3. Stage intended files only.
4. Commit locally with:

```text
feat(sparkctl): add SPARK Evidence Packet v1
```

5. Do not push without explicit push/PR approval.
