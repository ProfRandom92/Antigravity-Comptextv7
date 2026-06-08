---
name: comptext-context-pack-review
description: Draft repo-local CompText Context Pack review skill. Use when auditing Context Pack design, inputs, exclusions, redaction, deterministic ordering, replayability, and generated-output hygiene without running providers or installing global skills.
---

# CompText Context Pack Review

Draft status: review-only Phase 8A skill candidate. Do not install globally.

## When To Use

Use this draft to review whether a Context Pack workflow is deterministic, bounded, redacted, replayable, and separated from provider execution.

## Read First

- `AGENTS.md`
- `.agent/skills/04_spark_context_layer.md` if present
- `.agent/skills/09_codex_desktop_governance.md`
- `.agent/skills/10_generated_artifact_policy.md`
- Relevant context-pack docs or schema files named by the user

## Allowed Actions

- Read context-pack documentation, schemas, and explicit test fixtures.
- Check that inputs, included files, excluded files, allowed write paths, forbidden actions, validation commands, provider metadata, and policy fields are documented.
- Verify that deterministic ordering, normalized metadata, redaction, and generated-output exclusion are specified.
- Recommend review-only fixes or follow-up tasks.

## Forbidden Actions

- Do not run provider/model calls.
- Do not generate new context packs unless explicitly authorized.
- Do not run `agy-ct run`, `agy-ct benchmark`, or cargo.
- Do not edit Rust source, `.codex/**`, `.agent/skills/**`, reports, artifacts, `README.md`, or `AGENTS.md`.
- Do not read secrets or environment dumps.

## Output Contract

Return:

```text
PHASE:
STATUS:
CONTEXT_PACK_SCOPE:
DETERMINISM_CHECKS:
REDACTION_CHECKS:
GENERATED_OUTPUT_POLICY:
MISSING_OR_WEAK_POINTS:
VALIDATION:
RISKS:
NEXT:
COMPACT_HANDOFF:
```

## Claim Hygiene

- Describe deterministic Context Pack behavior only when supported by docs, schemas, code, or validation output.
- Do not claim legal proof, compliance, forensic certainty, production readiness, official compatibility, or guaranteed correctness.
- Use "tamper-sensitive" or "change-detection" wording for local hashes.

## Artifact Hygiene

- Treat context packs, provider requests, provider responses, and generated cache files as runtime artifacts by default.
- Do not stage or commit generated artifacts without explicit approval for exact files.
- Do not rewrite artifacts to repair a review finding.

## Compact Handoff Rule

Summarize the reviewed Context Pack boundary, determinism gaps, artifact policy, validation performed, and next safe review action in a compact block.
