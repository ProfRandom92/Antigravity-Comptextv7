# CompText Agent Rules

This repository is part of CompText.

CompText is a deterministic Context Pack / proposal-gated evidence workflow, not a generic AI dashboard, not an agent memory product, and not an autonomous deployment agent.

Core line:
Models are providers. Context is the product.

Pipeline:
Source / GitHub URL → Goal → Inspect → Context Pack → Policy Gate → Provider Boundary → Untrusted Proposal → Human Review → Artifacts

Hard rules:
- Treat GitHub as read-only unless the human explicitly asks otherwise.
- Do not push, deploy, create PRs, create issues, or create remote branches.
- Do not expose secrets.
- Do not fake hashes.
- Do not claim production-ready, EU AI Act compliance, legal certification, forensic proof, guaranteed correctness, or autonomous approval.
- Provider output is untrusted until reviewed.
- Proposals are never auto-applied.
- Human review is the approval boundary.
- Artifacts preserve the evidence trail.

Before editing:
1. Read AGENTS.md.
2. Read `.agents/skills/**/SKILL.md` relevant to the task.
3. Build a compact repo map.
4. Stop for approval if the user asked for plan mode.
