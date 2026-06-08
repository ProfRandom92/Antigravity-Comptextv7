---
name: comptext-claim-hygiene
description: Draft repo-local CompText claim-hygiene skill. Use when reviewing docs, reports, handoffs, proposals, and generated text for unsupported claims about production readiness, compliance, legal proof, forensic certainty, official compatibility, autonomous approval, or guaranteed correctness.
---

# CompText Claim Hygiene

Draft status: review-only Phase 8A skill candidate. Do not install globally.

## When To Use

Use this draft before publishing, committing, or handing off CompText documentation or generated text that describes validation, security, determinism, evidence, SPARK-style fixtures, providers, MCP, hooks, or artifacts.

## Read First

- `AGENTS.md`
- `.agent/skills/05_claim_hygiene.md` if present
- `.agent/skills/09_codex_desktop_governance.md`
- `.agent/skills/10_generated_artifact_policy.md`
- The document or output under review

## Allowed Actions

- Read and review explicitly named docs, handoff text, reports, or generated outputs.
- Flag unsupported or overbroad claims.
- Suggest bounded replacement wording.
- Verify that local validation claims match command evidence.

## Forbidden Actions

- Do not invent validation evidence.
- Do not fake hashes or proof language.
- Do not edit protected files unless explicitly authorized.
- Do not claim production readiness, EU AI Act compliance, legal certification, forensic proof, official SPARK compatibility, autonomous approval, or guaranteed correctness.
- Do not read secrets or environment dumps.

## Output Contract

Return:

```text
PHASE:
STATUS:
CLAIMS_REVIEWED:
ALLOWED_CLAIMS:
UNSUPPORTED_CLAIMS:
RECOMMENDED_WORDING:
VALIDATION_EVIDENCE:
RISKS:
NEXT:
COMPACT_HANDOFF:
```

## Claim Hygiene

- Prefer precise wording: "local validation", "review-gated", "deterministic where evidenced", "tamper-sensitive", "change-detection metadata", and "synthetic SPARK-style fixture".
- Avoid absolute language such as "secure", "certified", "compliant", "forensic proof", "official", "guaranteed", or "autonomous approval" unless the active evidence explicitly supports it and project rules allow it.

## Artifact Hygiene

- Treat generated reports and handoff text as evidence summaries, not source-of-truth proof.
- Do not rewrite generated artifacts to support a claim.
- Keep exact command outputs summarized unless detailed logs are explicitly requested.

## Compact Handoff Rule

Return the smallest useful set of claim issues, replacement wording, validation evidence, residual risk, and next safe action.
