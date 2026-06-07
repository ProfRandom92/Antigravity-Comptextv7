# Skill: Security and Read-Only Boundaries

## Purpose
Prevent unsafe actions while working with Codex Desktop, GitHub, MCP/connectors, and local repositories.

## Use this skill when
- GitHub plugin is enabled.
- Codex Security plugin is enabled.
- Any MCP/connector is available.
- Working on CompText source repos or hackathon deliverables.

## Hard restrictions
- Do not push.
- Do not deploy.
- Do not create PRs.
- Do not create issues.
- Do not create remote branches.
- Do not create tokens.
- Do not write secrets.
- Do not paste secrets into code, docs, prompts, or tests.
- Do not install unofficial Codex UI/Android/remote-control packages.

## Git safety
After cloning, run:
`git remote set-url --push origin DISABLED`

Then show:
`git remote -v`

Treat GitHub as read-only even if credentials allow writes.

## MCP / connector rules
- Use MCP/connectors only for read-only context unless explicitly approved.
- Prefer local cloned files as source of truth.
- Do not use connectors to mutate GitHub or deployments.
- Do not rely on hidden external state for deterministic validation.

## Network/tooling caution
- Do not add provider calls for sparkctl.
- Do not add shell execution features to the product.
- Do not add arbitrary filesystem readers.
- Do not expose private provider keys to frontend/runtime output.
