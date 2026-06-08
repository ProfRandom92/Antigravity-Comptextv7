---
name: comptext-compact-handoff
description: Draft repo-local CompText compact-handoff skill. Use when ending or transferring CompText work with concise phase status, files, commands, validation, risks, next action, and claim/artifact boundaries.
---

# CompText Compact Handoff

Draft status: review-only Phase 8A skill candidate. Do not install globally.

## When To Use

Use this draft at the end of a phase, after validation, before review, or when transferring work to another session.

## Read First

- `AGENTS.md`
- `.agent/skills/12_agent_handoff_profile.md` if present
- `.agent/skills/09_codex_desktop_governance.md`
- `.agent/skills/10_generated_artifact_policy.md`
- Active phase docs or validation outputs named by the user

## Allowed Actions

- Summarize files read, files changed, commands run, validation, risks, and next safe action.
- Identify protected files that were not touched.
- Distinguish committed, staged, unstaged, untracked, and generated artifact state when git output is available.
- Keep command output concise and factual.

## Forbidden Actions

- Do not commit, push, create PRs/issues/releases, deploy, merge, pull, rebase, or tag unless explicitly authorized.
- Do not claim completion beyond available evidence.
- Do not hide failed validation or skipped checks.
- Do not read secrets or environment dumps.
- Do not edit `.codex/**`, `.agent/skills/**`, `README.md`, `AGENTS.md`, Rust source, reports, or artifacts unless explicitly authorized.

## Output Contract

Return:

```text
PHASE:
STATUS:
FILES_READ:
FILES_CHANGED:
COMMANDS_RUN:
VALIDATION:
GIT:
RISKS:
NEXT:
COMPACT_HANDOFF:
```

## Claim Hygiene

- Say exactly what was done, what was validated, what was skipped, and what remains risky.
- Avoid production readiness, compliance, legal proof, forensic certainty, official compatibility, autonomous approval, or guaranteed correctness claims.
- Call out inferred conclusions as inferences.

## Artifact Hygiene

- Name generated artifacts separately from source or review docs.
- Do not present generated artifacts as commit-ready without exact approval.
- Preserve evidence trail context while keeping the response short.

## Compact Handoff Rule

The final handoff should fit in a short review block unless the user asks for detail. Prefer high-signal bullets over pasted logs.
