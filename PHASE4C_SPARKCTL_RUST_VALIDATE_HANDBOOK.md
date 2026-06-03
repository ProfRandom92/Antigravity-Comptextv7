# Phase 4C sparkctl rust-validate Planning Handbook

## 1. Objective
Define the implementation blueprint for the `sparkctl rust-validate` subcommand. This subcommand executes local crate quality checks and outputs formatted validation status results.

## 2. Strict Scope
- **Command:** `sparkctl rust-validate`
- **Actions Wrapped:**
  1. `cargo fmt --all --check`
  2. `cargo check`
  3. `cargo test`
  4. `cargo clippy -- -D warnings`
- **Output:** Outputs a clear block stating pass/fail status for each test runner step.
- **Exit Code:** Exits with status `0` if all checks pass, and a non-zero code if any subcommand fails.

## 3. Forbidden Scope
- **No Git Access:** The command must not run any git command or perform git operations.
- **No Network Usage:** Work must occur 100% offline.
- **No File Mutations:** The command must verify configurations but not modify source code files.
- **No Directory Escapes:** Do not scan folders outside the sandbox directory.
- **No Compliance Claims:** No claims regarding EU AI Act or official SPARK schemas compatibility.
- **No Raw Payload Dumps:** Ensure logs do not print original payload fields (applicant, recommendation, notes).

## 4. Execution & Reporting Rules
- **Execution Directory:** The tool must execute local cargo commands within the `agy7rust` workspace directory root path.
- **Diagnostics Formatting:** Output must show a clean list of checked subcommands with status indicator tags (e.g. `[FMT] PASS`, `[TEST] FAIL`).

## 5. Safety & Security Boundaries
- Validation runs local cargo subcommands only via std::process::Command.
- Environment variables or randomized system inputs must not affect check calculations.

## 6. Validation Checklist
Before completion, verification must confirm:
```bash
cargo fmt --all --check
cargo check
cargo test
cargo clippy -- -D warnings
sparkctl doctor
sparkctl rust-validate
```

## 7. Stop Conditions
Stop implementation and return `blocked` if:
- Cargo command execution requires external registry updates or internet connections.
- Validation checks require accessing remote credentials or secrets.

## 8. Return Format for Phase 4C Implementation
```text
PHASE: Phase 4C sparkctl rust-validate implementation
STATUS: success | blocked
COMMANDS_RUN:
- ...
FILES_CHANGED:
- ...
RUST_VALIDATE_OUTPUT:
- ...
VALIDATION:
- ...
RISKS:
- ...
NEXT:
- Phase 4C audit/snapshot only after approval
```

## 9. Assurances
- No blocking risks found in the validated scope.
