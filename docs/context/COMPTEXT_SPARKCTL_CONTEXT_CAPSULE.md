# CompText Sparkctl Context Capsule

> [!IMPORTANT]
> **Status**: inert, repo-local, review-gated, not active runtime. Reference only.

This capsule acts as a persistent repository data store for Antigravity config state, avoiding document repetition in prompts.

## 1. Governance & Protocol Chain
- **Read Order**:
  1. `AGENTS.md` (Rules and core constraints)
  2. `.antigravity/BOOTSTRAP.md` (Context alignment)
  3. `.antigravity/START_HERE.md` (Active prompt template)
- **Index**: Reference `.antigravity/SKILL_INVENTORY.md` for local and system capabilities.

## 2. Skill Source Classifications
- `.agent/skills/` = repo guidance source
- `.agents/skills/` = compatibility metadata
- `docs/phase8a-draft-skills/` = draft source / design reference only
- `.antigravity/` = inert adapted workspace config

## 3. Project Configuration Matrix
- **Purpose**: Local packaging, validation, and metadata generation for CompText.
- **Hook Status**: Inert (no active runtime) example configs.
- **Plugin Status**: Inert (no active runtime) example configs.
- **MCP Status**: Inactive (no MCP configurations or servers).
- **Claim Hygiene**: Strict adherence to no production-ready, legal, compliance, or official SPARK claims. Provider output is untrusted and human review is the gate.
- **Edit Isolation**:
  - *Allowed*: `./.antigravity/` and `./docs/context/`.
  - *Forbidden*: `./agy7rust/src/` (Rust source), `./README.md`, `./.agent/skills/`, `./.agents/skills/`, and `./.codex/`.
- **Handoff Rules**: All paths must be repo-relative. No secrets, `.env` files, git pushes, or deployments.

## 4. Next Workflow Order
1. Apply the prompt prefix: `"Read .antigravity/START_HERE.md first."`
2. Follow the bootstrap and inventory files for contextual queries.
3. Obtain user approval before proposing changes to any workspace configs.
