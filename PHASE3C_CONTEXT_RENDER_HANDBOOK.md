# Phase 3C Context-Render Implementation Handbook

## 1. Phase
Phase 3C context-render only

## 2. Objective
- Render an existing `context.json` operational context file into a deterministic, token-light plain-text layout.
- Save the rendered text to `artifacts/spark/context_render.txt`.
- Prevent any leak of raw payload values (no `applicant` raw values, `decision_recommendation` raw values, `notes`, or raw payload contents).

## 3. Strict Scope
**Allowed in Phase 3C:**
- Add context rendering logic.
- Add exactly one CLI command: `context-render`.
- Add unit/integration tests for the rendering routine.
- Generate `artifacts/spark/context_render.txt` only.

**Forbidden in Phase 3C:**
- No `context-validate` command.
- No MCP server implementation or integration.
- No database connections, active tool executors, or LLM clients.
- No schema or package code modifications.
- No reconstruction of raw payload or tracing history.
- No source code changes beyond registering `context-render`.

## 4. Allowed Future Implementation Paths
The subsequent implementation prompt may modify/create *only* the following files:
- `agy7rust/src/main.rs`
- `agy7rust/src/lib.rs`
- `agy7rust/src/commands/mod.rs`
- `agy7rust/src/commands/context_render.rs`
- `agy7rust/src/context/mod.rs`
- `agy7rust/src/context/render.rs`
- `agy7rust/tests/spark_roundtrip.rs`
- `artifacts/spark/context_render.txt`
- `agy7rust/PHASE3C_STATUS.md`

## 5. Forbidden Paths
Do not read, write, or modify:
- `agy7rust/src/commands/context_validate.rs`
- `agy7rust/src/context/validate.rs`
- `schemas/`
- `examples/`
- `README.md`
- Prior phase snapshots (`PHASE1_SPARK_SNAPSHOT.md`, `PHASE2_SCHEMA_SIDECAR_SNAPSHOT.md`, `PHASE3A_CONTEXT_MODEL_SNAPSHOT.md`, `PHASE3B_CONTEXT_BUILD_SNAPSHOT.md`)
- Local agent skills (`.agent/skills/*`)

## 6. Planned CLI
**Command:**
```bash
agy7rust context-render -i <context.json> -o <context_render.txt>
```

**Success Output:**
```text
OK: context-render passed
context: <context_id>
rendered_bytes: <rendered size>
```

**Failure Output:**
```text
ERROR: context-render failed
reason: <stable reason>
```

## 7. Deterministic Rendering Rules
- **Template Layout:** The output layout must use a stable text-only structure (e.g. key-value lines and bulleted items).
- **Element Sorting:** All lists (required field paths, satisfied paths, missing paths, constraints, non-claims, validation issues) must be rendered in alphabetical order.
- **Graph Edges:** Dependency edges, blockers, and recovery paths must be printed lexicographically by `source -> target`.
- **Newline and Padding:** Consistent indentation and a trailing newline must be enforced.
- **Repeatability:** Multiple rendering runs against identical `context.json` inputs must yield byte-identical `context_render.txt` files.

## 8. Leak Rules
The generated `context_render.txt` must NOT contain:
- `applicant` raw values (e.g. `"Nordwind Energie GmbH"`).
- `decision_recommendation` raw values (e.g. `"Zustimmung..."`).
- `extraction.notes` (`notes`) raw contents.
- `source_pdf_contents` or raw payload bytes.
- Full original extraction JSON structure.

## 9. Test Checklist
The integration test suite must assert:
- `context-render` successfully processes the built `context.json`.
- Output layout matches the deterministic formatting rules.
- Output file contains a trailing newline.
- Repeated rendering yields identical output hashes.
- Leak assertions pass (no raw payload values are printed in `context_render.txt`).
- No `context-validate` CLI command is registered.

## 10. Validation Checklist
Before completion, the following command pipeline must execute cleanly:
```powershell
cd C:\Users\contr\sandbox_workspace\Antigravity-Comptextv7-unified\agy7rust
cargo fmt
cargo fmt --all --check
cargo check
cargo test
cargo clippy -- -D warnings
cargo run -- context-render -i ..\artifacts\spark\context.json -o ..\artifacts\spark\context_render.txt
cargo run -- schema-check -i ..\examples\spark\extraction.json -s ..\schemas\genehmigung_v1.json
powershell -File .\demo_spark.ps1
```

## 11. Stop Conditions
Stop execution and report `blocked` if:
- Rendering code requires recreating or holding raw payload fields.
- Layout rendering relies on local system time, environment, or system-specific file separator constants.
- The requirements ask to integrate active LLM tokenization or text compression libraries.
- The prompt asks to register a `context-validate` command.
- Modifying command routing requires changes outside the CLI registration boundaries.

## 12. Return Format for Later Implementation
Report execution status using this exact block structure:
```text
PHASE: Phase 3C implementation - context-render only
STATUS: success | blocked
SKILLS_USED:
- ...
FILES_CHANGED:
- ...
COMMANDS_RUN:
- ...
TESTS:
- ...
CONTEXT_RENDER:
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
- Phase 3C audit / snapshot
```
