# Phase 8A Draft Skill Registry

This registry lists review-only draft global skill candidates created under `docs/phase8a-draft-skills/`.

These drafts are not installed globally, not active Codex skills, and not approved for automatic use outside this repo. They are design artifacts for review before any repo-local skill promotion, plugin scaffold, MCP design, dry-run installer, or global install.

## Shared Boundaries

- Drafts only.
- Do not install globally.
- Do not edit `.agent/skills/**`.
- Do not edit `.codex/**`.
- Do not edit `README.md`.
- Do not edit `AGENTS.md`.
- Do not edit Rust source.
- Do not touch reports or artifacts.
- Do not run cargo, provider/model calls, `agy-ct run`, or `agy-ct benchmark`.
- Do not commit or push unless separately authorized.
- Do not read secrets, token stores, credential files, `.env`, or environment dumps.

## Draft Skills

### `comptext-operating-boundary`

- Path: `docs/phase8a-draft-skills/comptext-operating-boundary/SKILL.md`
- Purpose: Load governance and confirm boundaries before CompText work.
- Primary output: phase/status/boundary handoff.

### `comptext-context-pack-review`

- Path: `docs/phase8a-draft-skills/comptext-context-pack-review/SKILL.md`
- Purpose: Review deterministic, redacted, replayable Context Pack workflows.
- Primary output: determinism, redaction, and generated-output review.

### `comptext-proposal-gate-review`

- Path: `docs/phase8a-draft-skills/comptext-proposal-gate-review/SKILL.md`
- Purpose: Review proposal schema, path safety, validation commands, and human gate before apply.
- Primary output: proposal-gate review summary.

### `comptext-provider-boundary`

- Path: `docs/phase8a-draft-skills/comptext-provider-boundary/SKILL.md`
- Purpose: Review dry-run-first provider boundaries, network deny-by-default, auth metadata redaction, and untrusted outputs.
- Primary output: provider-boundary review summary.

### `comptext-claim-hygiene`

- Path: `docs/phase8a-draft-skills/comptext-claim-hygiene/SKILL.md`
- Purpose: Review docs and handoffs for unsupported assurance claims.
- Primary output: claim findings and bounded replacement wording.

### `comptext-artifact-hygiene`

- Path: `docs/phase8a-draft-skills/comptext-artifact-hygiene/SKILL.md`
- Purpose: Classify generated artifacts and commit candidates before staging or handoff.
- Primary output: artifact classification and non-commit defaults.

### `comptext-compact-handoff`

- Path: `docs/phase8a-draft-skills/comptext-compact-handoff/SKILL.md`
- Purpose: Produce concise phase handoffs with files, commands, validation, git state, risks, and next action.
- Primary output: compact handoff block.

## Review Gate

Before any draft is promoted:

- Confirm the exact destination path.
- Confirm whether it remains repo-local or becomes global.
- Review claim hygiene and artifact hygiene.
- Validate that frontmatter contains only `name` and `description`.
- Confirm no active hook, plugin, MCP, provider, or installer behavior is introduced.
