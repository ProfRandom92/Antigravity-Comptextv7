# Phase 3D Context-Validate Implementation Handbook

## 1. Phase
Phase 3D context-validate only

## 2. Objective
- Validate an existing `context.json` operational context file for structural correctness, logical consistency, and leak safety.
- Print structural audit results to the console.
- Prevent any leak of raw payload values during the validation process.

## 3. Strict Scope
**Allowed in Phase 3D:**
- Add context validation logic.
- Add exactly one CLI command: `context-validate`.
- Add unit/integration tests for the validation runner.
- Enforce structural rules and checks on `context.json`.

**Forbidden in Phase 3D:**
- No context rendering or context building changes.
- No MCP server implementation or integration.
- No database connections, active tool executors, or LLM clients.
- No schema or package code modifications.
- No source code changes beyond registering `context-validate`.

## 4. Allowed Future Implementation Paths
The subsequent implementation prompt may modify/create *only* the following files:
- `agy7rust/src/main.rs`
- `agy7rust/src/lib.rs`
- `agy7rust/src/commands/mod.rs`
- `agy7rust/src/commands/context_validate.rs`
- `agy7rust/src/context/mod.rs`
- `agy7rust/src/context/validate.rs`
- `agy7rust/tests/spark_roundtrip.rs`
- `agy7rust/PHASE3D_STATUS.md`

## 5. Forbidden Paths
Do not read, write, or modify:
- `agy7rust/src/commands/context_build.rs`
- `agy7rust/src/commands/context_render.rs`
- `schemas/`
- `examples/`
- `README.md`
- Prior phase snapshots (`PHASE1_SPARK_SNAPSHOT.md`, `PHASE2_SCHEMA_SIDECAR_SNAPSHOT.md`, `PHASE3A_CONTEXT_MODEL_SNAPSHOT.md`, `PHASE3B_CONTEXT_BUILD_SNAPSHOT.md`, `PHASE3C_CONTEXT_RENDER_SNAPSHOT.md`)
- Local agent skills (`.agent/skills/*`)

## 6. Planned CLI
**Command:**
```bash
agy7rust context-validate -i <context.json>
```

**Success Output:**
```text
OK: context-validate passed
context: <context_id>
valid: true
```

**Failure Output:**
```text
ERROR: context-validate failed
reason: <stable reason>
```

## 7. Validation Rules
- **Model Shape Validation:** Deserialize context JSON into `OperationalContext` and invoke `validate_model_shape()`.
- **Logical Validation:**
  - `validation.valid` must be `true` if and only if `missing_field_paths` is empty.
  - If `missing_field_paths` is not empty, `validation.valid` must be `false` and `validation.failure_labels` must contain `MISSING_REQUIRED_FIELD`.
  - The combination of `satisfied_field_paths` and `missing_field_paths` must match the set of `required_field_paths`.
- **Error Propagation:** If validation logic fails, report the failure with a stable reason (e.g. `context validation invalid: MISSING_REQUIRED_FIELD` or `validate_model_shape failed`).

## 8. Leak Rules
The validation execution path must NOT read, print, log, or leak:
- `applicant` raw values.
- `decision_recommendation` raw values.
- `extraction.notes` (`notes`) raw contents.
- `source_pdf_contents` or raw payload bytes.

## 9. Deterministic & Offline Rules
- Validation behavior must be deterministic in the validated test scope.
- No network/external calls are allowed.
- No system time, clock, or randomized inputs are allowed during validation check calculations.

## 10. Test Checklist
The integration test suite must assert:
- `context-validate` successfully processes a valid `context.json`.
- `context-validate` fails and reports correct errors for a context with missing fields or invalid shape.
- `context-validate` command is registered and mapped correctly.
- Validation checks are deterministic in the validated test scope.
- No leak of raw payload values occurs during validation execution.

## 11. Validation Checklist
Before completion, the following command pipeline must execute cleanly:
```powershell
cd C:\Users\contr\sandbox_workspace\Antigravity-Comptextv7-unified\agy7rust
cargo fmt
cargo fmt --all --check
cargo check
cargo test
cargo clippy -- -D warnings
cargo run -- context-validate -i ..\artifacts\spark\context.json
cargo run -- schema-check -i ..\examples\spark\extraction.json -s ..\schemas\genehmigung_v1.json
powershell -File .\demo_spark.ps1
```

## 12. Stop Conditions
Stop execution and report `blocked` if:
- Validation checks require accessing or holding original payload data.
- The workflow requires introducing new CLI subcommands.
- Logic errors occur that cannot be resolved without a general crate refactor.

## 13. Return Format for Later Implementation
Report execution status using this exact block structure:
```text
PHASE: Phase 3D implementation - context-validate only
STATUS: success | blocked
SKILLS_USED:
- ...
FILES_CHANGED:
- ...
COMMANDS_RUN:
- ...
TESTS:
- ...
CONTEXT_VALIDATE:
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
- Phase 3D audit / snapshot
```
