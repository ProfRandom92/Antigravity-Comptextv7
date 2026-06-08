---
name: comptext-proposal-gate-review
description: Draft repo-local CompText proposal-gate review skill. Use when reviewing proposal-before-apply workflows, proposal schemas, target paths, validation commands, rollback notes, risk notes, and human review gates without applying changes.
---

# CompText Proposal Gate Review

Draft status: review-only Phase 8A skill candidate. Do not install globally.

## When To Use

Use this draft to review proposed mutation workflows before any source change is applied, especially when provider output, patch suggestions, or generated proposal files are involved.

## Read First

- `AGENTS.md`
- `.agent/skills/09_codex_desktop_governance.md`
- `.agent/skills/10_generated_artifact_policy.md`
- Relevant proposal schema, policy, or phase document named by the user

## Allowed Actions

- Read proposal docs, proposal examples, schemas, and explicitly named proposal artifacts.
- Check schema fields such as task, rationale, preconditions, affected files, operations, validation commands, rollback strategy, and risk notes.
- Verify target paths are repo-relative and do not cross protected boundaries.
- Report whether human review is clearly the approval boundary.

## Forbidden Actions

- Do not apply proposals.
- Do not run provider/model calls.
- Do not run `agy-ct run`, `agy-ct benchmark`, cargo, deploy, release, commit, push, PR, issue, merge, pull, or rebase commands unless explicitly authorized.
- Do not edit Rust source, `.codex/**`, `.agent/skills/**`, reports, artifacts, `README.md`, or `AGENTS.md`.
- Do not read secrets or environment dumps.

## Output Contract

Return:

```text
PHASE:
STATUS:
PROPOSAL_SCOPE:
SCHEMA_REVIEW:
PATH_REVIEW:
VALIDATION_REVIEW:
HUMAN_GATE:
MISSING_OR_WEAK_POINTS:
RISKS:
NEXT:
COMPACT_HANDOFF:
```

## Claim Hygiene

- Treat every provider proposal as untrusted until reviewed.
- Do not state that a proposal is safe, approved, production-ready, compliant, legally valid, or guaranteed correct.
- Use bounded terms such as "reviewable", "schema-shaped", "path-bounded", and "requires human approval".

## Artifact Hygiene

- Treat generated proposal files as runtime artifacts unless the user explicitly marks exact files as review artifacts.
- Do not stage or commit proposal outputs by default.
- Do not alter proposal contents to make validation appear successful.

## Compact Handoff Rule

End with a concise approval-boundary summary: proposal reviewed, files targeted, checks performed, blockers, and next safe action.
