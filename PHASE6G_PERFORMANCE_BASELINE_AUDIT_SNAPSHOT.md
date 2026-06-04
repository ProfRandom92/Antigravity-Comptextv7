# Phase 6G Performance Baseline Audit Snapshot

## 1. Files Inspected & Audited
- [PERFORMANCE_BASELINE.md](file:///C:/Users/contr/sandbox_workspace/Antigravity-Comptextv7-unified/git_post_push_verification/repo/PERFORMANCE_BASELINE.md)
- [reports/performance_baseline.json](file:///C:/Users/contr/sandbox_workspace/Antigravity-Comptextv7-unified/git_post_push_verification/repo/reports/performance_baseline.json)
- [PHASE6G_PERFORMANCE_BASELINE_SNAPSHOT.md](file:///C:/Users/contr/sandbox_workspace/Antigravity-Comptextv7-unified/git_post_push_verification/repo/PHASE6G_PERFORMANCE_BASELINE_SNAPSHOT.md)

## 2. Audit Findings & Checks

### Timing Measurement Method Audit
- Execution timings were gathered locally on Windows utilizing PowerShell's `Measure-Command` utility to track complete execution cycles from start to finish.
- Tested all commands to verify they execute successfully, complete without error flags, and exit with status 0.
- Executed double runs to confirm baseline consistency under dev profile setup.

### Artifact Integrity and Hashes Audit
- Measured file sizes in bytes using standard filesystem info calls.
- Validated that `artifacts/spark/extraction.spkg`, `artifacts/spark/context.json`, `artifacts/spark/context_render.txt`, and `reports/latest.json` are present and correct.
- Generated SHA-256 hashes using PowerShell `Get-FileHash` to register deterministic signature keys.

### JSON Schema Verification
- Validated that the output in `reports/performance_baseline.json` complies with JSON syntax rules.
- Run validator tool `python -m json.tool` to verify structural format validity.

## 3. Claim Hygiene
All assertions follow standard formatting rules:
- Offline behavior was deterministic in the validated test scope.
- Configured leak checks passed in the validated scope.
- No blocking risks found in the validated scope.
- **Required Wording:**
  - Performance baseline measured on local validation environment.
  - No performance optimization was performed in this phase.
  - Measurements are local and environment-specific.
- **Forbidden Claims Avoided:**
  - All forbidden claims, certifications, or official compatibility statements are strictly avoided.

## 4. Verification Checkups
- `cargo fmt --all --check` -> PASS
- `cargo check` -> PASS
- `cargo test` -> PASS
- `cargo clippy -- -D warnings` -> PASS
- `cargo run --bin agy-ct -- run` -> PASS
- `python -m json.tool ../reports/latest.json` -> PASS
- `python -m json.tool ../reports/performance_baseline.json` -> PASS
