---
name: sparkctl-evidence-packet
description: "Guide comptext-sparkctl work toward a SPARK Evidence Packet v1: a deterministic, reviewable artifact package for CompText."
---

# Skill: sparkctl Evidence Packet

## Purpose
Guide `comptext-sparkctl` work toward a SPARK Evidence Packet v1: a deterministic, reviewable artifact package for CompText.

## Use this skill when
- Working in `ProfRandom92/comptext-sparkctl`.
- Modifying artifact/package/codec/manifest/roundtrip code.
- Preparing SPARK Safe & Stable hackathon material.

## Target artifact
SPARK Evidence Packet v1 should contain, as data fields or manifest sections:
- `schema_version`
- `package_id` or `local_id`
- `goal`
- `source_summary`
- `context_pack_ref` or `context_pack_summary`
- `policy_result`
- `provider_boundary_status`
- `untrusted_proposal`
- `human_review_decision`
- `claim_hygiene`
- `artifact_manifest`
- `canonical_hash` only if computed from canonical JSON
- `warnings` / `limitations`

## Enums
Policy Gate result:
- `ALLOW`
- `REVIEW_NEEDED`
- `BLOCK`

Provider Boundary status:
- `DEMO`
- `UNAVAILABLE`
- `AVAILABLE`
- `BLOCKED_BY_POLICY`

Human Review decision:
- `PASS`
- `NOTES`
- `BLOCKED`

## Rules
- Goal informs packaging and review criteria.
- Goal never bypasses Policy Gate.
- Goal never authorizes auto-apply.
- Provider output is untrusted until reviewed.
- Human Review is the approval boundary.
- Artifacts preserve the evidence trail.
- Do not fake hashes.
- If a hash is shown, compute it from canonical JSON.

## Tests to prefer
- canonical JSON deterministic output
- SHA-256 stable for known canonical input
- package roundtrip verify
- goal included in manifest
- policy result included
- provider boundary status included
- review decision included
- claim hygiene included
- SPARK Evidence Packet demo validates
