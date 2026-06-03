# Phase 3B Context-Build Implementation Handbook

## 1. Phase
Phase 3B context-build only

## 2. Objective
- Build a deterministic `OperationalContext` from an existing `.spkg` package and schema JSON.
- Output deterministic context JSON to `artifacts/spark/context.json`.
- Prevent any raw payload or private field leak (no `applicant`, `decision_recommendation`, `extraction.notes` / `notes`, `source_pdf_contents`, or direct payload dumps).

## 3. Strict Scope
**Allowed in Phase 3B:**
- Add context builder logic.
- Add exactly one CLI command: `context-build`.
- Add unit/integration tests for the `context-build` routine.
- Generate `artifacts/spark/context.json` only.

**Forbidden in Phase 3B:**
- No `context-render` command.
- No `context-validate` command.
- No `context_render.txt` file generation.
- No MCP server implementation or integration.
- No new schema files.
- No Phase 3C/3D functionality.
- No broad package structure or API refactoring.

## 4. Allowed Future Implementation Paths
The subsequent implementation prompt may modify/create *only* the following files:
- `agy7rust/src/main.rs`
- `agy7rust/src/lib.rs`
- `agy7rust/src/commands/mod.rs`
- `agy7rust/src/commands/context_build.rs`
- `agy7rust/src/context/mod.rs`
- `agy7rust/src/context/model.rs`
- `agy7rust/src/context/build.rs`
- `agy7rust/tests/spark_roundtrip.rs`
- `artifacts/spark/context.json`
- `agy7rust/PHASE3B_STATUS.md`

## 5. Forbidden Paths
Do not read, write, or modify:
- `agy7rust/src/commands/context_render.rs`
- `agy7rust/src/commands/context_validate.rs`
- `agy7rust/src/context/render.rs`
- `agy7rust/src/context/validate.rs`
- `artifacts/spark/context_render.txt`
- `schemas/genehmigung_v1.json` (unless explicitly blocked/instructed)
- `examples/`
- `README.md`
- Phase snapshots (`PHASE1_SPARK_SNAPSHOT.md`, `PHASE2_SCHEMA_SIDECAR_SNAPSHOT.md`, `PHASE3A_CONTEXT_MODEL_SNAPSHOT.md`)
- Local agent skills (`.agent/skills/*`)

## 6. Planned CLI
**Command:**
```bash
agy7rust context-build -i <package.spkg> -s <schema.json> -o <context.json>
```

**Success Output:**
```text
OK: context-build passed
context: <context_id>
schema: genehmigung_v1
missing_fields: 0
```

**Failure Output:**
```text
ERROR: context-build failed
reason: <stable reason>
```

## 7. Planned Context-Build Behavior
- **Package Ingestion:** Read and parse `.spkg` using existing package verification/package logic.
- **Schema Ingestion:** Read schema JSON using existing schema parsing/checking logic.
- **Hash Derivation:** Derive `source_package_hash` directly from existing package integrity hash metadata.
- **Schema Metadata:** Derive `schema_name` and `schema_version` from the schema JSON.
- **Required Fields:** Copy `required_field_paths` from the schema.
- **Satisfied Fields:** Derive `satisfied_field_paths` from package field-path metadata.
- **Missing Fields:** Derive `missing_field_paths` as `required_field_paths` minus `satisfied_field_paths`.
- **Context Identifier:** Build a deterministic `context_id` by hashing `source_package_hash` + `schema_name` + `schema_version`.
- **Static Constraints:** Fill deterministic, conservative constraints:
  - `no_raw_payload_dump`
  - `schema_required_fields_must_exist`
  - `deterministic_replay_only`
  - `synthetic_fixture_only`
- **Execution Order:** Fill deterministic list:
  - `package_verified`
  - `schema_loaded`
  - `schema_checked`
  - `context_built`
- **Dependency Graph:** Fill deterministic dependency pairs:
  - `package_verified -> schema_checked`
  - `schema_loaded -> schema_checked`
  - `schema_checked -> context_built`
- **Blockers:** Fill deterministic blockers:
  - `missing_required_field -> context_built`
- **Recovery Paths:** Fill deterministic recovery paths:
  - `missing_required_field -> schema_check_failure_reported`
- **Non-Claims:** Copy `non_claims` from schema `non_claims` when present, plus conservative local non-claims.
- **Pre-write Validation:** Call `sort_stable()` and `validate_model_shape()` on the constructed context object before serialization.
- **Serialization:** Write canonical, stable pretty JSON with a terminating newline.

## 8. Determinism Rules
- **Key Ordering:** Stable alphabetical JSON key serialization.
- **List Ordering:** Stable lexicographical list order (via `sort_stable`).
- **No Volatile Elements:** Absolutely no timestamps, system clocks, randomized identifiers, or environmental paths.
- **Offline operation:** Zero network/web calls.
- **Identical Hash:** Repeated executions with identical package and schema files must yield byte-identical `context.json`.

## 9. Leak Rules
The generated `context.json` must NOT contain:
- `applicant` raw values.
- `decision_recommendation` raw values.
- `extraction.notes` (`notes`) raw content.
- `source_pdf_contents`.
- Hidden original payload payload bytes.
- Full raw input extraction JSON structure.

## 10. Test Checklist
The implementation validation suite must assert:
- `context-build` completes successfully from the default `extraction.spkg` + `genehmigung_v1.json`.
- The output parses and validates cleanly against `OperationalContext::validate_model_shape`.
- Repeated builds yield identical file hashes.
- Missing schema paths are correctly isolated in `missing_field_paths`.
- `context_id` remains stable and deterministic.
- Context JSON leak test successfully checks that no protected fields are present in `context.json`.
- No `context-render` or `context-validate` CLI commands are registered.

## 11. Validation Checklist
Before completion, the following command pipeline must execute cleanly:
```powershell
cd C:\Users\contr\sandbox_workspace\Antigravity-Comptextv7-unified\agy7rust
cargo fmt
cargo fmt --all --check
cargo check
cargo test
cargo clippy -- -D warnings
cargo run -- context-build -i ..\artifacts\spark\extraction.spkg -s ..\schemas\genehmigung_v1.json -o ..\artifacts\spark\context.json
cargo run -- schema-check -i ..\examples\spark\extraction.json -s ..\schemas\genehmigung_v1.json
powershell -File .\demo_spark.ps1
```

## 12. Stop Conditions
Stop execution and report `blocked` if:
- Existing package metadata format does not support metadata/field path extraction.
- Deriving package hashes requires loading or leaking raw payload contents.
- Schema parsing demands a general architectural crate refactoring.
- The workflow requires introducing `context-render` or `context-validate` commands.
- CLI command routing modification requires modifying other commands.
- Any private raw payload value is found written into `context.json`.

## 13. Return Format for Later Implementation
Report execution status using this exact block structure:
```text
PHASE: Phase 3B implementation - context-build only
STATUS: success | blocked
SKILLS_USED:
- ...
FILES_CHANGED:
- ...
COMMANDS_RUN:
- ...
TESTS:
- ...
CONTEXT_BUILD:
- ...
DETERMINISM_CHECK:
- ...
LEAK_CHECK:
- ...
DEMO:
- ...
RISKS:
- ...
NEXT:
- Phase 3B audit / snapshot
```
