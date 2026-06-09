# Antigravity Bootstrap Context

> [!IMPORTANT]
> **Status**: inert, repo-local, review-gated, not active runtime. Reference only.

## 1. Bootstrap Protocol
To prevent context redundancy, follow this exact initial sequence:
1. Read `AGENTS.md` first for fundamental workspace rules.
2. Read `.antigravity/BOOTSTRAP.md` second to align config boundaries.
3. Use `.antigravity/SKILL_INVENTORY.md` as the index for local/system skills.

## 2. Repo Purpose & Scope
Deterministic trace packaging, replay-sidecar validation, and SPARK-style context artifact rendering for CompText.

## 3. Skill & Source Classifications
- **.agent/skills**: Repo guidance source (15 guidelines).
- **.agents/skills**: Compatibility metadata (9 legacy skill directories).
- **docs/phase8a-draft-skills**: Draft global skill candidates / design source only.
- **.antigravity**: Inert adapted workspace configuration directory.

## 4. Environment & Safety Boundaries
- **Hooks & Plugins**: Inert, repo-local, review-gated, not active runtime.
- **MCP Status**: Disabled/Not active. Do not create `.mcp.json`.
- **Claim Hygiene**: No production-ready, EU AI Act, legal, or official SPARK compatibility claims.
- **Allowed Edit Zones**: `./.antigravity/` and `./docs/context/` only.
- **Forbidden Edit Zones**: Rust sources (`./agy7rust/src/`), `README.md`, `.agent/skills/`, `.agents/skills/`, and `.codex/`.
- **Hard Rules**: Treat GitHub as read-only. No commits, pushes, deploys, secrets, or `.env` dumps.

## 5. Next Workflow Order
1. Load startup parameters from `.antigravity/START_HERE.md`.
2. Inspect `docs/context/COMPTEXT_SPARKCTL_CONTEXT_CAPSULE.md` for active project state.
3. Formulate plans for user approval before proposing edits.
