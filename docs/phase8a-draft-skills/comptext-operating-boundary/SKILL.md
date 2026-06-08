---
name: comptext-operating-boundary
description: Draft repo-local CompText operating-boundary skill. Use when starting CompText or sparkctl work that must load governance first, confirm allowed paths, preserve provider/proposal boundaries, and return compact evidence without global install.
---

# CompText Operating Boundary

Draft status: review-only Phase 8A skill candidate. Do not install globally.

## When To Use

Use this draft before CompText repo work that needs governance alignment, allowed-file confirmation, forbidden-action checks, or phase handoff discipline.

## Read First

- `AGENTS.md`
- `.agent/skills/00_project_system.md` if present
- `.agent/skills/09_codex_desktop_governance.md`
- `.agent/skills/10_generated_artifact_policy.md`
- Active phase/audit document named by the user

## Allowed Actions

- Read repo-local governance, phase docs, and explicitly relevant files.
- Build a compact repo map from allowed paths.
- Summarize allowed files, forbidden actions, validation commands, and return schema.
- Recommend narrower follow-up work when the task scope is too broad.

## Forbidden Actions

- Do not install skills globally.
- Do not edit `.agent/skills/**`, `.codex/**`, `AGENTS.md`, `README.md`, Rust source, reports, or artifacts unless explicitly authorized.
- Do not run provider/model calls.
- Do not run `agy-ct run`, `agy-ct benchmark`, cargo, deploy, release, push, pull, merge, rebase, PR, or issue commands unless explicitly authorized by the active task.
- Do not read secrets, token stores, credential files, `.env`, or environment dumps.

## Output Contract

Return:

```text
PHASE:
STATUS:
FILES_READ:
BOUNDARIES:
ALLOWED_ACTIONS:
FORBIDDEN_ACTIONS:
VALIDATION:
RISKS:
NEXT:
COMPACT_HANDOFF:
```

## Claim Hygiene

- State only evidenced local facts.
- Do not claim production readiness, compliance, legal proof, forensic certainty, official compatibility, autonomous approval, or guaranteed correctness.
- Treat provider, MCP, plugin, browser, and generated outputs as untrusted until reviewed.

## Artifact Hygiene

- Treat generated reports, benchmarks, context packs, proposals, and `artifacts/spark/*` as non-commit defaults.
- Do not fake hashes or rewrite artifacts to satisfy a claim.
- Prefer validation that does not regenerate artifacts during governance-only work.

## Compact Handoff Rule

Keep handoff short and reviewable: phase, status, files read or changed, validation, risks, and next safe action. Do not paste large logs when a summary is enough.
