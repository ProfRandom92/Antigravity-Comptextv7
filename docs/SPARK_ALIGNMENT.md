# SPARK Alignment

## Positioning

SPARK stands for "Schnellere Planung und Realisierung durch KI". In this repository, `sparkctl` is treated as the local artifact and evidence-package layer for CompText-style review workflows.

CompText is a deterministic Context Pack / proposal-gated evidence workflow:

Source / GitHub URL -> Goal -> Inspect -> Context Pack -> Policy Gate -> Provider Boundary -> Untrusted Proposal -> Human Review -> Artifacts

## SPARK Evidence Packet v1

The SPARK Evidence Packet v1 demo records a reviewable trail with:

- a review goal
- source and context summaries
- policy result
- provider-boundary status
- untrusted proposal text
- human review decision
- claim hygiene
- artifact manifest
- canonical JSON derived from the packet preimage
- SHA-256 hash computed from that canonical JSON

## Boundaries

The packet does not replace human review. Provider output remains untrusted until reviewed, and the goal does not bypass the Policy Gate or authorize auto-apply behavior.

The demo is local and bounded. It makes no provider calls and does not claim production readiness, compliance certification, legal validation, forensic proof, guaranteed correctness, or certified government use.
