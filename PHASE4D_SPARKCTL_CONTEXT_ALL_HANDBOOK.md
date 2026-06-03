# Phase 4D sparkctl context-all Planning Handbook

## 1. Objective
Define the implementation blueprint for the `sparkctl context-all` subcommand. This subcommand orchestrates the complete contextual lifecycle by running the internal tool sequence: `context-build`, `context-render`, and `context-validate` in order.

## 2. Strict Scope
- **Command:** `sparkctl context-all`
- **Exact Internal Command Sequence:**
  1. `context-build`
  2. `context-render`
  3. `context-validate`
- **Default Input/Output Paths:**
  - Input Package: `../artifacts/spark/extraction.spkg`
  - Input Schema: `../schemas/genehmigung_v1.json`
  - Output Context JSON: `../artifacts/spark/context.json`
  - Output Rendered Context TXT: `../artifacts/spark/context_render.txt`
- **Exit Code:** Exits with status `0` if all steps pass successfully, and a non-zero code if any step fails.
- **Stop on Failure:** The orchestration must stop on the first failed step (i.e. if `context-build` fails, it must not execute `context-render`).

## 3. Forbidden Scope
- **No Git Access:** The command must not run any git command or perform git operations.
- **No Network Usage:** Work must occur 100% offline.
- **No File Mutations Outside Declared Artifacts:** The command must update only `../artifacts/spark/context.json` and `../artifacts/spark/context_render.txt`.
- **No Directory Escapes:** Do not scan folders outside the sandbox directory.
- **No Compliance Claims:** No claims regarding EU AI Act or official SPARK schemas compatibility.
- **No Raw Payload Dumps:** Ensure logs do not print original payload fields (applicant, recommendation, notes).

## 4. Execution & Reporting Rules
- **Execution Directory:** The tool must execute local subcommands within the `agy7rust` workspace directory root path.
- **Diagnostics Formatting:** Output must show a clean list of checked subcommands with status indicator tags (e.g. `[BUILD] PASS`, `[RENDER] PASS`, `[VALIDATE] PASS`).

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
```

## 7. Stop Conditions
Stop implementation and return `blocked` if:
- Command execution requires external registry updates or internet connections.
- Validation checks require accessing remote credentials or secrets.

## 8. Return Format for Phase 4D Implementation
```text
PHASE: Phase 4D sparkctl context-all implementation
STATUS: success | blocked
COMMANDS_RUN:
- ...
FILES_CHANGED:
- ...
CONTEXT_ALL_OUTPUT:
- ...
VALIDATION:
- ...
RISKS:
- ...
NEXT:
- Phase 4E implementation or audit only after approval
```
