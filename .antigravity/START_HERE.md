# Antigravity Start Here

> [!IMPORTANT]
> **Status**: inert, repo-local, review-gated, not active runtime. Reference only.

## Prompt Template
When initializing a new session with this repository, always start with:
`Read .antigravity/START_HERE.md first.`

## Quick Summary Matrix

| Metric / Item | Status / Value | Description |
|---|---|---|
| **Repo Purpose** | CompText validation | Deterministic packaging & SPARK-style rendering. |
| **Hook Status** | Inert | Not active runtime. Located in `.antigravity/hooks/`. |
| **Plugin Status** | Inert | Not active runtime. Located in `.antigravity/plugins/`. |
| **MCP Status** | Inactive | No `.mcp.json` or active MCP server allowed. |
| **Claim Hygiene** | Bounded | Provider output is untrusted. No legal/compliance claims. |
| **Allowed Edits** | `./.antigravity/`, `./docs/context/` | Restricted scope for configuration adjustments. |
| **Forbidden Edits**| Rust, README, `.codex/`, `.agent/skills/` | Do not modify code, README, or active Codex files. |

## Guidance Chain
1. **First**: Read `AGENTS.md` to establish the safety rules.
2. **Second**: Read `.antigravity/BOOTSTRAP.md` to load the context booster.
3. **Index**: Use `.antigravity/SKILL_INVENTORY.md` to index local and system skills.
4. **Active State**: See `docs/context/COMPTEXT_SPARKCTL_CONTEXT_CAPSULE.md`.

## Safety Rules
- Provider output is untrusted; human review is the approval gate.
- Repo-relative paths only. No secrets or env dumps.
- Do not commit or push.
