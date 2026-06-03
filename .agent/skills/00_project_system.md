# Agent Skill 00 — Project System

This skill defines the repository structure, active directories, permissions, and operating boundaries for the sandbox environment.

## 1. Operating Boundaries

- **Sandbox Root:** `C:\Users\contr\sandbox_workspace\Antigravity-Comptextv7-unified`
- **Allowed Write Paths:**
  - `agy7rust/` (Rust crate)
  - `examples/spark/` (Synthetic SPARK-style fixtures)
  - `artifacts/spark/` (Verification and demo outputs)
  - `.agent/skills/` (Local agent instructions)
- **Forbidden Paths:** Any parent directory (e.g. `C:\Users\contr`), desktop (`C:\Users\contr\Desktop`), sibling workspaces (e.g., `rustcomptext`), and the `.git` metadata of the system.
- **Search Boundaries:** Do NOT perform global searches, recursive searches, or file indexing outside the sandbox root.

## 2. Command Permissions

- **Cargo Access:** Running `cargo` command actions (`cargo fmt`, `cargo check`, `cargo test`, `cargo clippy`, `cargo run`) is strictly limited to the `agy7rust/` subdirectory.
- **Git Restrictions:** No git remotes config, git fetch, git pull, or git push commands are permitted.
- **Network Access:** All network calls and API connections are blocked. The project works entirely offline.

## 3. Standard Return Format

Every completed agent execution step must output the exact formatted block:

```text
PHASE: <phase_name>
STATUS: <success | blocked>
FILES_CHANGED:
- ...
COMMANDS_RUN:
- ...
TESTS:
- ...
RISKS:
- ...
NEXT:
- ...
```
