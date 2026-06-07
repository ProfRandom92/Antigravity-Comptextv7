# Skill: CompText Governance

## Purpose
Preserve the CompText product contract during any code, UI, CLI, documentation, or artifact work.

## Use this skill when
- Working on CompText Gateway, comptext-cli, comptext-sparkctl, Context Governor, or related docs.
- Changing pipeline, provider, review, artifact, or safety logic.
- Preparing material for reviewers, SPARK, public sector, or enterprise stakeholders.

## Product contract
CompText is a deterministic Context Pack / proposal-gated evidence workflow.

Core line:
Models are providers. Context is the product.

Canonical pipeline:
Source / GitHub URL → Goal → Inspect → Context Pack → Policy Gate → Provider Boundary → Untrusted Proposal → Human Review → Artifacts

## Required boundaries
- Provider output is untrusted until reviewed.
- Proposals are never auto-applied.
- Policy Gate decides whether provider calls are allowed.
- Human Review is the approval boundary.
- Artifacts preserve the evidence trail.
- GitHub/source repos remain read-only unless explicitly approved.

## Allowed claims
- deterministic context infrastructure
- bounded inspection
- proposal-gated workflow
- review boundary
- evidence trail
- provider-agnostic boundary
- artifact manifest
- local integrity anchor if actually implemented
- SHA-256 hash of canonical JSON if actually computed

## Blocked claims
- production-ready
- certified
- EU AI Act compliant
- legally compliant
- forensic proof or forensic certainty
- solved hallucinations
- guaranteed correctness
- guaranteed replay validity for arbitrary inputs
- autonomous enterprise agent
- autonomous approval
- universal AI memory

## Checklist before final answer
- Did the change preserve Goal → Context Pack → Policy Gate → Review → Artifacts?
- Is provider output clearly untrusted?
- Are claims bounded and review-safe?
- Are artifacts/data honest and not faked?
- Are source repos/GitHub writes blocked unless explicitly approved?
