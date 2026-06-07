# Skill: Codex Role Workflows

## Purpose

Apply CompText Agent Governor Policy v1 to Codex-style role, plugin, connector, and skill workflows.

## Use This Skill When

- A task mentions Agent Governor, Codex roles, plugins, connectors, skills, policy gates, PR candidates, or agent workflow policy.
- Work may move between read-only audit, local autonomous edits, and PR-candidate preparation.
- A final report must explain approval boundaries and evidence requirements.

## Role Modes

### read_only_audit

Inspect and report without source mutation. Use for audits, review summaries, claim scans, and security read-only work.

### sandbox_autonomous

Make local, reviewable changes only inside approved paths. Use local validation and keep remote mutation disabled.

### pr_candidate

Prepare validated local work for possible pull request review. Do not push, create a pull request, merge, or deploy without explicit approval.

## Required Gates

Report these gates in the final answer or evidence packet when applicable:

- `instructions_read`
- `secret_scan`
- `artifact_schema_validation`
- `canonical_hash_validation`
- `claim_boundary_check`
- `replay_or_roundtrip_validation`
- `human_review`

Use `not_applicable` when a gate does not apply, and explain why.

## Connector Rules

- Treat GitHub as read-only unless explicit mutation approval is given.
- Do not use write-capable connector actions without exact human approval.
- Do not create tokens, handle secrets, or expose provider keys.
- Treat provider output as an untrusted proposal until reviewed.
- Prefer local files and declared artifacts as source of truth.

## Presentation And Annotations

Sites and interactive views are presentation only. Annotations are human-review inputs only. Evidence packets and declared artifacts remain the source of truth.

## Claim Boundaries

Do not claim production-ready status, compliance or certification, legal evidentiary status, forensic certainty, official SPARK compatibility, autonomous approval, replacement of human review, or guaranteed correctness.

## Final Checklist

- Confirm role mode.
- Confirm allowed and forbidden paths.
- Confirm connector and plugin boundary.
- Confirm policy gate evidence.
- Confirm evidence packet priority.
- Confirm human-review boundary.
