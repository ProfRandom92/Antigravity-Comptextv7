# Phase 4E sparkctl spark-demo Planning Handbook

## 1. Objective
Define the implementation blueprint for the `sparkctl spark-demo` subcommand. This subcommand runs the complete end-to-end SPARK package, build, render, and validation pipeline using only existing local CLI commands.

## 2. Strict Scope
- **Command:** `sparkctl spark-demo`
- **Exact Demo Command Sequence:**
  1. `cargo run -- compress -i ../examples/spark/extraction.json -o ../artifacts/spark/extraction.spkg`
  2. `cargo run -- context-build -i ../artifacts/spark/extraction.spkg -s ../schemas/genehmigung_v1.json -o ../artifacts/spark/context.json`
  3. `cargo run -- context-render -i ../artifacts/spark/context.json -o ../artifacts/spark/context_render.txt`
  4. `cargo run -- context-validate -i ../artifacts/spark/context.json -s ../schemas/genehmigung_v1.json`
- **Default Demo Paths:**
  - Input JSON: `../examples/spark/extraction.json`
  - Schema: `../schemas/genehmigung_v1.json`
  - Output Package: `../artifacts/spark/extraction.spkg`
  - Output Context JSON: `../artifacts/spark/context.json`
  - Output Rendered Context: `../artifacts/spark/context_render.txt`
- **Exit Code:** Exits with status `0` if all steps pass successfully, and a non-zero code if any step fails.
- **Stop on Failure:** The orchestration must stop on the first failed step.

## 3. Forbidden Scope
- **No Git Access:** The command must not run any git command or perform git operations.
- **No Network Usage:** Work must occur 100% offline.
- **No File Mutations Outside Declared Artifacts:** The command must update only:
  - `../artifacts/spark/extraction.spkg`
  - `../artifacts/spark/context.json`
  - `../artifacts/spark/context_render.txt`
- **No Directory Escapes:** Do not scan folders outside the sandbox directory.
- **No Compliance Claims:** No claims regarding EU AI Act or official SPARK schemas compatibility.
- **No Raw Payload Dumps:** Ensure logs do not print original payload fields (applicant, recommendation, notes).

## 4. Execution & Reporting Rules
- **Execution Directory:** The tool must execute local subcommands within the `agy7rust` workspace directory root path.
- **Diagnostics Formatting:** Output must show a clean list of checked subcommands with status indicator tags (e.g. `[COMPRESS] PASS`, `[BUILD] PASS`, `[RENDER] PASS`, `[VALIDATE] PASS`).

## 5. Safety & Security Boundaries
- Validation runs local crate subcommands only via direct execution or library orchestration.
- Offline behavior was deterministic in the validated test scope.
- Configured leak checks passed in the validated scope.
- No blocking risks found in the validated scope.

## 6. Validation Checklist
Before completion, verification must confirm:
```bash
cargo check
cargo test
sparkctl doctor
sparkctl rust-validate
sparkctl context-all
sparkctl spark-demo
```

## 7. Stop Conditions
Stop implementation and return `blocked` if:
- Command execution requires external registry updates or internet connections.
- Validation checks require accessing remote credentials or secrets.

## 8. Return Format for Phase 4E Implementation
```text
PHASE: Phase 4E sparkctl spark-demo implementation
STATUS: success | blocked
COMMANDS_RUN:
- ...
FILES_CHANGED:
- ...
SPARK_DEMO_OUTPUT:
- ...
VALIDATION:
- ...
RISKS:
- ...
NEXT:
- Phase 4E audit/snapshot only after approval
```
