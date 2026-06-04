# Agent Instruction Manual — Antigravity × CompText v7

Welcome, Agent. You are pair programming inside the isolated SPARK Hackathon sandbox.

> [!IMPORTANT]
> **First Step:** You MUST read [.agent/skills/00_project_system.md](file:///.agent/skills/00_project_system.md) before performing any file reads, writes, edits, or terminal command executions.

## Protocol Highlights

1. **Read Guidelines First:** Open [.agent/skills/00_project_system.md](file:///.agent/skills/00_project_system.md) and choose the specific specialized skill matching your target task.
2. **Keep Changes Scoped:** Work only within the allowed write paths for the current approved phase. Never modify existing Python core, benchmarks, reports, or the original repository `README.md`.
3. **Phase-Gate Compliance:** Follow the `Implementation -> Audit -> Snapshot` loop. Do NOT advance to a new phase without explicit user approval.
4. **Cargo Restrictions:** You have permission to run `cargo fmt`, `cargo check`, `cargo test`, `cargo clippy`, and `cargo run` inside `agy7rust/` only. Active entry points: `sparkctl` (legacy compatibility checks) and `agy-ct` (production workflow orchestrator writing to untracked `reports/latest.json`).
5. **No Network or Git Remotes:** Web/network calls, git remote modifications, git fetch, git pull, or git push are strictly forbidden unless explicitly approved.
6. **Structured Output:** All step results must be reported using the structured phase block format.
