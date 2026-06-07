# Agent Task Template

## Task

Describe the requested agent work in one bounded paragraph.

## Role Mode

Select one:

- `read_only_audit`
- `sandbox_autonomous`
- `pr_candidate`

## Approved Paths

- `path/or/file`

## Forbidden Actions

- no push
- no pull request creation
- no merge
- no deploy
- no dependency update
- no secret handling
- no claim that human review has been replaced
- no unauthorized write actions, including creating comments, issues, labels, remote branches, releases, deployments, or GitHub metadata changes

## Plugins, Connectors, And Skills

List approved plugins, connectors, and skills. Mark write-capable connectors as read-only unless human approval is explicitly granted for a specific action.

## Policy Gates

| Gate | Expected Evidence |
| --- | --- |
| `instructions_read` | Instructions, skills, and docs read. |
| `secret_scan` | Changed-file scan or manual no-secret review. |
| `artifact_schema_validation` | Schema validation or not-applicable reason. |
| `canonical_hash_validation` | Recomputed hash or not-applicable reason. |
| `claim_boundary_check` | Unsafe-claim scan and manual review. |
| `replay_or_roundtrip_validation` | Replay, roundtrip, or not-applicable reason. |
| `human_review` | Pending, approved, rejected, or not requested. |

## Evidence Packet

State whether an evidence packet is required. If required, name the packet path and schema.

Evidence packets remain the source of truth. Sites, previews, dashboards, annotations, and comments are presentation or human-review inputs only.

## Claim Boundaries

Do not claim production-ready status, compliance or certification, legal evidentiary status, forensic certainty, official SPARK compatibility, autonomous approval, replacement of human review, or guaranteed correctness.

## Validation

List the smallest sufficient validation commands for the approved paths.

## Human Review

Record the requested human decision and any approval needed for push, PR creation, merge, or deploy.
