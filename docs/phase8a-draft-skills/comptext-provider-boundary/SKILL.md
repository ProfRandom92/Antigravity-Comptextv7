---
name: comptext-provider-boundary
description: Draft repo-local CompText provider-boundary skill. Use when reviewing model/provider integration boundaries, dry-run-first behavior, network deny-by-default policy, auth metadata redaction, and untrusted provider outputs.
---

# CompText Provider Boundary

Draft status: review-only Phase 8A skill candidate. Do not install globally.

## When To Use

Use this draft when reviewing provider configuration, dry-run behavior, network policy, provider output handling, or auth metadata treatment.

## Read First

- `AGENTS.md`
- `.agent/skills/09_codex_desktop_governance.md`
- `.agent/skills/10_generated_artifact_policy.md`
- Relevant provider-boundary docs or config examples named by the user

## Allowed Actions

- Read provider documentation and non-secret config examples.
- Verify dry-run-first behavior and explicit network authorization requirements.
- Check that auth values are represented only as metadata names, not secret values.
- Confirm provider responses are treated as untrusted and routed to proposal review before mutation.

## Forbidden Actions

- Do not execute provider/model calls.
- Do not enable network access.
- Do not read API keys, token stores, `.env`, credentials, or environment dumps.
- Do not install SDKs, plugins, MCP servers, or provider adapters.
- Do not edit Rust source, `.codex/**`, `.agent/skills/**`, reports, artifacts, `README.md`, or `AGENTS.md`.

## Output Contract

Return:

```text
PHASE:
STATUS:
PROVIDER_SCOPE:
DRY_RUN_BOUNDARY:
NETWORK_POLICY:
SECRET_REDACTION:
UNTRUSTED_OUTPUT_HANDLING:
MISSING_OR_WEAK_POINTS:
RISKS:
NEXT:
COMPACT_HANDOFF:
```

## Claim Hygiene

- Do not claim live provider support, MCP support, production readiness, official compatibility, compliance, legal proof, forensic certainty, or guaranteed correctness unless implemented and evidenced.
- Distinguish offline skeletons, dry-runs, and real provider execution.
- State that provider output is untrusted until reviewed.

## Artifact Hygiene

- Treat provider requests, responses, dry-run payloads, and benchmark outputs as runtime artifacts by default.
- Redact secrets before any artifact is written or summarized.
- Do not commit provider-generated artifacts without explicit approval for exact files.

## Compact Handoff Rule

Summarize provider kind, dry-run state, network gate, secret posture, output trust boundary, validation, risks, and next safe action.
