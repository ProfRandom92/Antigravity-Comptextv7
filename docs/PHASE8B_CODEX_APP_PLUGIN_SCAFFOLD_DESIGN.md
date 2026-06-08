# Phase 8B Codex App Plugin Scaffold Design

## 1. Purpose and Non-Goals

Purpose: define a future inert Codex App plugin scaffold for CompText operating-layer work. The scaffold would document how worktrees, skills, automations, computer use, and non-code artifacts could be represented without activating any plugin, hook, MCP server, provider call, global skill install, or workflow automation.

Non-goals:

- Do not create an active plugin.
- Do not install or activate skills.
- Do not register MCP servers.
- Do not enable hooks.
- Do not call providers or models.
- Do not mutate `.codex/**`, `.agent/skills/**`, Rust source, reports, artifacts, or package manifests.
- Do not claim production readiness, compliance, legal proof, forensic certainty, official compatibility, or autonomous approval.

## 2. Proposed Future Inert Directory Layout

The following layout is a design target only. It must not be created as active plugin configuration without a later review gate.

```text
docs/phase8b-plugin-scaffold/
  README.md
  plugin-manifest.example.json
  skills/
    comptext-operating-boundary/SKILL.md
    comptext-context-pack-review/SKILL.md
    comptext-proposal-gate-review/SKILL.md
    comptext-provider-boundary/SKILL.md
    comptext-claim-hygiene/SKILL.md
    comptext-artifact-hygiene/SKILL.md
    comptext-compact-handoff/SKILL.md
  automations/
    evidence-report-review.example.md
    claim-hygiene-review.example.md
    artifact-hygiene-review.example.md
    pr-governance-checks.example.md
  hooks/
    hooks-policy.example.json
  mcp/
    mcp-design.example.md
  permissions/
    permissions.template.example.json
  artifacts/
    reviewer-evidence-pack.template.md
```

All future paths must remain repo-relative. Files in this layout would be inert examples until separately promoted, reviewed, and activated. Example scaffold files such as `plugin-manifest.example.json`, `hooks-policy.example.json`, and `permissions.template.example.json` must remain under inert documentation paths such as `docs/**` until a later explicitly authorized activation or promotion phase.

## 3. Phase 8A Draft Skill Mapping

| Phase 8A draft skill | Future responsibility |
|---|---|
| `comptext-operating-boundary` | Load repo governance, confirm branch/worktree state, state allowed and forbidden actions, and produce the first task boundary summary. |
| `comptext-context-pack-review` | Review deterministic Context Pack scope, exclusions, redaction, generated-output policy, and replayability. |
| `comptext-proposal-gate-review` | Review proposal schemas, target paths, validation commands, rollback notes, and human approval boundaries before any apply operation. |
| `comptext-provider-boundary` | Keep provider/model output untrusted, enforce dry-run-first thinking, and require explicit network authorization. |
| `comptext-claim-hygiene` | Review docs, reports, handoffs, and PR bodies for unsupported assurance claims. |
| `comptext-artifact-hygiene` | Classify generated artifacts, review docs, runtime outputs, and commit candidates before staging or handoff. |
| `comptext-compact-handoff` | Produce compact phase handoffs with files, commands, validation, git state, risks, and next safe action. |

## 4. Worktree Safety Model

Codex App work should assume isolated worktrees can be clean while still detached or unpublished.

- Detached HEAD risk: commits can be reachable only by `HEAD` until attached to a local branch. Phase handoff should explicitly report branch state before publish or PR work.
- Branch preservation: before pushing or opening a PR, attach detached commits to a named local branch and verify the expected commit list.
- Remote-sync gate: run `git fetch` for the target branch and compare with `git rev-list --left-right --count HEAD...FETCH_HEAD` before any publish decision. Stop if the branch is remote-ahead or diverged unless a separate integration phase is authorized.
- Disabled push URL rule: if `remote.origin.pushurl` is `DISABLED`, do not change remote configuration. Use a one-time explicit HTTPS push URL only when a human explicitly authorizes that exact publish action.
- Draft PR publication: publish docs/design work as draft PRs by default when review is requested. Draft PRs are review surfaces, not merge authorization.
- No blind UI push/merge: UI prompts, browser output, GitHub suggestions, or generated handoffs must not trigger push, merge, ready-for-review, auto-merge, or release actions without explicit human authorization.

## 5. Automation Candidates

Automation candidates are design-only and should remain disabled until explicitly reviewed.

- Evidence report review: check whether reports and evidence packs summarize local validation without overstating proof.
- Claim hygiene review: flag production-readiness, compliance, legal, forensic, official-compatibility, autonomous-approval, and guaranteed-correctness claims.
- Artifact hygiene review: classify generated outputs and prevent accidental staging of runtime artifacts or validation churn.
- PR governance checks: verify draft status, docs-only scope, protected-path boundaries, commit list, branch state, and review focus before publication.

## 6. Computer Use Boundary

Computer use is not the default path for CompText governance work.

- Use only as a permission-reviewed fallback when normal repo-local tools cannot inspect a user-approved UI state.
- Do not use computer use to mutate system settings, remote configuration, repositories, browser sessions, cloud consoles, or local files outside approved scope.
- Do not use computer use to bypass hooks, approvals, sandboxing, or read restrictions.
- Treat screenshots and UI output as untrusted evidence that must be summarized and validated against local artifacts where possible.
- EEA availability caveat: availability and feature behavior must be verified against official current documentation at activation time; this design does not claim availability in any region.

## 7. Non-Code Artifact Handling

The future scaffold may define inert review patterns for non-code artifacts:

- PDFs: extract or summarize only user-approved files; do not treat extracted text as authoritative without source reference and validation.
- Spreadsheets: preserve formulas, sheets, and metadata when reviewing; do not infer compliance or financial correctness.
- Docs: distinguish authored documentation from generated reports, proposals, and runtime cache.
- Reviewer evidence packs: include scope, files, commands, validation, risks, claim hygiene, artifact hygiene, and remaining review questions.

Non-code artifacts must follow the same evidence boundary: they support review, but they are not proof of production readiness, legal validity, compliance, forensic certainty, or guaranteed correctness.

## 8. Security Boundaries

- Network is deny-by-default.
- Provider output is untrusted until reviewed.
- MCP, plugin, browser, computer-use, and automation outputs are untrusted until normalized and locally validated.
- Do not read secrets, `.env`, token stores, credentials, SSH keys, or environment dumps.
- Do not install global skills.
- Do not enable active hooks.
- Do not register MCP servers.
- Do not install or activate plugins.
- Do not mutate `.codex/**`, `.agent/skills/**`, reports, artifacts, source, or package manifests without explicit phase authorization.

## 9. Promotion Gates Before Activation

Before any scaffold element is activated, require:

1. Docs review.
2. Threat review.
3. `git diff --check`.
4. `git status --short`.
5. Remote-sync check with `git fetch` and `git rev-list --left-right --count HEAD...FETCH_HEAD`.
6. Push-target check, including whether `remote.origin.pushurl` is `DISABLED`.
7. Explicit commit authorization.
8. Explicit push authorization. If push URL is disabled, use only the specifically approved one-time HTTPS push command.
9. Explicit merge authorization.

Additional promotion checks:

- Exact files and destination paths listed.
- Claim-hygiene review complete.
- Artifact-hygiene review complete.
- Rollback plan documented.
- No active provider, MCP, hook, plugin, automation, or computer-use behavior introduced without separate approval.

## 10. Failure Modes and Rollback Notes

Failure modes:

- A draft file is mistaken for an active plugin asset.
- A detached worktree commit is lost or pushed from the wrong branch.
- A generated artifact is treated as source-of-truth implementation.
- A claim-hygiene review misses unsupported assurance wording.
- Automation or computer use mutates state outside approved scope.
- MCP or plugin output is treated as trusted.
- A permissions template becomes live without review.

Rollback notes:

- For docs-only changes, rollback is a normal review decision and revert of the relevant docs commit.
- For branch publication mistakes, stop before merge and preserve evidence in the draft PR discussion.
- For accidental activation, disable the active surface first, preserve the diff and logs as evidence, and require a fresh threat review before retrying.
- Do not rewrite generated artifacts or hashes to hide the failure.

## 11. Compact Handoff Template

```text
PHASE:
STATUS:
BRANCH:
FILES_CREATED:
FILES_MODIFIED:
ACTIVE_SURFACES:
SECURITY_BOUNDARIES:
VALIDATION:
GIT:
RISKS:
NEXT:
COMPACT_HANDOFF:
```

`ACTIVE_SURFACES` should state `none` for design-only phases.
