---
name: comptext-artifact-hygiene
description: Draft repo-local CompText artifact-hygiene skill. Use when classifying generated reports, context packs, proposals, benchmarks, provenance files, agent-state files, and spark artifacts before staging, handoff, or review.
---

# CompText Artifact Hygiene

Draft status: review-only Phase 8A skill candidate. Do not install globally.

## When To Use

Use this draft when a task creates, reviews, validates, stages, or references generated CompText artifacts or local evidence outputs.

## Read First

- `AGENTS.md`
- `.agent/skills/10_generated_artifact_policy.md`
- `.agent/skills/09_codex_desktop_governance.md`
- Relevant phase docs or artifact policy files named by the user

## Allowed Actions

- Classify files as source docs, review docs, runtime artifacts, validation churn, or protected generated outputs.
- Check whether exact artifact files were explicitly approved for staging or commit.
- Recommend validation commands that avoid regenerating artifacts during governance-only work.
- Summarize artifact provenance and risk without overstating assurance.

## Forbidden Actions

- Do not stage or commit generated artifacts without explicit approval for exact files.
- Do not touch `reports/latest.json`, `reports/performance_baseline.json`, or `artifacts/spark/*` unless explicitly authorized.
- Do not run `agy-ct run`, `agy-ct benchmark`, cargo, provider calls, deploy, release, push, PR, or issue commands unless explicitly authorized.
- Do not fake hashes or rewrite artifacts to make checks pass.
- Do not read secrets or environment dumps.

## Output Contract

Return:

```text
PHASE:
STATUS:
ARTIFACTS_REVIEWED:
CLASSIFICATION:
COMMIT_CANDIDATES:
NON_COMMIT_DEFAULTS:
VALIDATION:
RISKS:
NEXT:
COMPACT_HANDOFF:
```

## Claim Hygiene

- Describe artifacts as evidence trail material, not legal proof, compliance proof, forensic proof, or guaranteed truth.
- State local hashes as change-detection metadata only.
- Avoid claims of production readiness or autonomous approval.

## Artifact Hygiene

- Runtime outputs, benchmarks, proposals, context packs, provider payloads, and agent-state files are non-commit defaults.
- Review docs may be commit candidates if explicitly in scope and validation passes.
- Preserve deterministic outputs and do not mutate generated evidence without approval.

## Compact Handoff Rule

Summarize artifact classification, exact commit candidates, excluded files, validation, risk, and next safe action.
