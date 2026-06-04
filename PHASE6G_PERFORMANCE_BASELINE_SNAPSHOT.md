# Phase 6G Performance Baseline Snapshot

## 1. Phase Name & Sandbox Root
- **Phase Name:** Phase 6G: Performance Baseline
- **Sandbox Root:** `C:\Users\contr\sandbox_workspace\Antigravity-Comptextv7-unified\git_post_push_verification\repo`

## 2. Created/Modified File Trees
- **Created Files (Local/Untracked):**
  - [PERFORMANCE_BASELINE.md](file:///C:/Users/contr/sandbox_workspace/Antigravity-Comptextv7-unified/git_post_push_verification/repo/PERFORMANCE_BASELINE.md)
  - [reports/performance_baseline.json](file:///C:/Users/contr/sandbox_workspace/Antigravity-Comptextv7-unified/git_post_push_verification/repo/reports/performance_baseline.json)
  - [PHASE6G_PERFORMANCE_BASELINE_SNAPSHOT.md](file:///C:/Users/contr/sandbox_workspace/Antigravity-Comptextv7-unified/git_post_push_verification/repo/PHASE6G_PERFORMANCE_BASELINE_SNAPSHOT.md)
  - [PHASE6G_PERFORMANCE_BASELINE_AUDIT_SNAPSHOT.md](file:///C:/Users/contr/sandbox_workspace/Antigravity-Comptextv7-unified/git_post_push_verification/repo/PHASE6G_PERFORMANCE_BASELINE_AUDIT_SNAPSHOT.md)
- **Modified Files:** None.
- **Excluded Build Folders:** `agy7rust/target/`

## 3. Execution Logs & Command Lists
Wall-clock timing measurements of CLI commands (unoptimized `dev` profile):
1. `cargo run --bin agy-ct -- run` -> **8.36 s** (8,360 ms)
2. `cargo run --bin agy-ct -- doctor` -> **0.39 s** (386 ms)
3. `cargo run --bin agy-ct -- validate` -> **18.48 s** (18,484 ms)
4. `cargo run --bin agy-ct -- context all` -> **1.73 s** (1,733 ms)
5. `cargo run --bin sparkctl -- doctor` -> **0.36 s** (364 ms)
6. `cargo run --bin sparkctl -- rust-validate` -> **14.49 s** (14,487 ms)
7. `cargo run --bin sparkctl -- context-all` -> **2.72 s** (2,720 ms)
8. `cargo run --bin sparkctl -- spark-demo` -> **2.65 s** (2,649 ms)
9. `cargo run --bin sparkctl -- handoff-check` -> **7.48 s** (7,476 ms)

## 4. Validation Test Run Status
- **Rust Test Suite:** 32 unit and integration tests executed inside `agy7rust/` under `cargo test` -> **PASS** (100% green, 32 passed, 0 failed).
- **Formatting Verification:** `cargo fmt --all --check` -> **PASS** (Zero formatting issues).
- **Compilation Check:** `cargo check` -> **PASS** (Clean build without warnings).
- **Clippy Check:** `cargo clippy -- -D warnings` -> **PASS** (Zero warnings/errors).

## 5. Deterministic Hash Signatures
The generated mock artifacts have the following SHA-256 hashes:
- `artifacts/spark/extraction.spkg` -> `AC3AC0F1E96CC3A208C6249D49EFDFB21044D93A284597BA7FF527DA6509BBEB`
- `artifacts/spark/context.json` -> `4EA7FE65FDAFB6972B9E759355F1D6DE0268479FEA510016DFA36BC73253DEFB`
- `artifacts/spark/context_render.txt` -> `C3A41070A6F331A14960EA693E0EBF5C3AB47960EF752E0897E107209779C320`

## 6. Leak Verification Evidence
- Configured leak checks passed in the validated scope.
- Subprocess outputs do not print or leak any sensitive configuration tokens, system environment variables, private paths, or raw input applicant payloads to `stdout` under `--json` or execution reports.

## 7. Adversarial Tamper Suite Statistics
- Replay and validation checkups correctly fail (exit status 2) when mock bytes in `.spkg` file or operational context `.json` files are manually altered.
- Security and tamper sensitivity boundaries perform exactly as described in Phase 3 Context Validate specifications.

## 8. Explicit Non-Claims & Risks
- **Required Wording:**
  - Performance baseline measured on local validation environment.
  - No performance optimization was performed in this phase.
  - Measurements are local and environment-specific.
- **Risks & Non-Claims:**
  - No claims of being "fully deterministic", "100% safe", or "no risks" are present.
  - Subprocess spawning introduces execution overhead (approx 0.2s - 0.4s per run) on local Windows environments.
  - Avoids claiming official SPARK JSON compatibility, EU AI Act certification, or forensic/judicial certainties.
