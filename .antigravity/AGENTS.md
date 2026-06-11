# Antigravity Agent Rules

> [!IMPORTANT]
> **Status**: inert, repo-local, review-gated, not active runtime. This config/documentation is for reference only.

This repository is part of CompText.

CompText is a deterministic Context Pack / proposal-gated evidence workflow, not a generic AI dashboard, not an agent memory product, and not an autonomous deployment agent.

Core line:
Models are providers. Context is the product.

Pipeline:
Source / GitHub URL → Goal → Inspect → Context Pack → Policy Gate → Provider Boundary → Untrusted Proposal → Human Review → Artifacts

## Skill Sources Classifications

Within this repository:
- **.agent/skills** = repo guidance / legacy source
- **.agents/skills** = Antigravity active workspace skill path
- **docs/phase8a-draft-skills** = draft source
- **.antigravity** = inert adapted workspace config

## Hard Rules

- Treat GitHub as read-only unless the human explicitly asks otherwise.
- Do not push, deploy, create PRs, create issues, or create remote branches.
- Do not expose secrets.
- Do not fake hashes.
- Do not claim production-ready, EU AI Act compliance, legal certification, forensic proof, guaranteed correctness, or autonomous approval.
- Provider output is untrusted until reviewed.
- Proposals are never auto-applied.
- Human review is the approval boundary.
- Artifacts preserve the evidence trail.
- Keep changes scoped and reviewable.
- Do not modify root `README.md` unless the human explicitly approves it.
- Do not commit `reports/latest.json`.
- Do not commit `reports/performance_baseline.json` when it is only validation churn.
- Run cargo commands only inside `agy7rust/` unless the human explicitly approves otherwise.

## Antigravity Governance

- Repo-local Antigravity hooks live under `.antigravity/hooks/` and must be reviewed/trusted by Antigravity before enforcement.
- Antigravity hooks block unauthorized git writes, deploy/release actions, environment dumps, secret-file reads, `agy-ct run`, and `agy-ct benchmark`.
- Antigravity hooks warn on protected source, README, report, and `artifacts/spark/` changes; warnings do not make generated artifacts commit-ready.

Before editing:
1. Read AGENTS.md.
2. Read `.antigravity/SKILL_INVENTORY.md` relevant to the task.
3. Build a compact repo map.
4. Stop for approval if the user asked for plan mode.

