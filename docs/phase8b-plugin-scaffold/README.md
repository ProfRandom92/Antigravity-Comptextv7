# Phase 8B Plugin Scaffold Design Folder

This folder is a design placeholder for a future inert Codex App plugin scaffold. It is not an active plugin, not a registered MCP configuration, not a hook installation, and not a global skill install.

## Scope

- Design documentation only.
- Repo-relative paths only.
- Review-gated promotion only.
- No runtime/source changes.
- No provider/model calls.
- No active automations.
- No computer-use actions.
- No MCP registration.
- No hook activation.
- No plugin installation.

## Intended Future Scaffold Areas

- `skills/`: future copies or promoted forms of Phase 8A draft skills after review.
- `automations/`: disabled examples for evidence report review, claim hygiene review, artifact hygiene review, and PR governance checks.
- `hooks/`: inert policy examples only.
- `mcp/`: design notes only; no live server registration.
- `permissions/`: example permission templates only.
- `artifacts/`: reviewer evidence pack templates for non-code artifacts.

These paths are not created in Phase 8B. They describe a possible future layout.

Future example files such as `plugin-manifest.example.json`, `hooks-policy.example.json`, and `permissions.template.example.json` must remain under inert documentation paths such as `docs/**` until a later explicitly authorized activation or promotion phase.

## Codex App Concepts Reflected

- Worktrees: require branch-state reporting, detached HEAD detection, branch preservation, and draft PR publication as separate explicit steps.
- Skills: keep Phase 8A drafts review-only until promoted by an explicit gate.
- Automations: treat scheduled or repeated checks as candidates, not active behavior.
- Computer use: not default; permission-review only; no mutation outside approved scope; verify current regional availability before activation.
- Non-code artifacts: PDFs, spreadsheets, docs, and reviewer evidence packs are review inputs, not proof artifacts.

## Security Boundaries

- Deny network by default.
- Treat provider, MCP, plugin, automation, browser, and computer-use outputs as untrusted.
- Do not read secrets, `.env`, credentials, token stores, SSH keys, or environment dumps.
- Do not mutate `.codex/**`, `.agent/skills/**`, Rust source, reports, artifacts, package manifests, or remote configuration.
- Do not install global skills, activate hooks, register MCP, install plugins, deploy, release, merge, push, or update PRs without explicit authorization.
- If a remote push URL is intentionally `DISABLED`, leave it unchanged. Use a one-time explicit HTTPS push URL only when the exact publish action is separately authorized.

## Promotion Gate

Before this design becomes anything active:

1. Review the docs.
2. Run a threat review.
3. Run `git diff --check`.
4. Run `git status --short`.
5. Run the remote-sync gate with `git fetch` and `git rev-list --left-right --count HEAD...FETCH_HEAD`.
6. Confirm generated artifacts and validation churn are not commit candidates.
7. Confirm any disabled push URL remains unchanged.
8. Get explicit commit authorization.
9. Get explicit push authorization.
10. Get explicit merge authorization.

Activation requires a separate phase request. Phase 8B only records the design.
