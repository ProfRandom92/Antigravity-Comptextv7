# SPARKctl Antigravity Enforcement Bridge

> [!IMPORTANT]
> **Status**: Active Project Reference. This document maps the synchronized autonomous-but-confirmation-gated governance workflow for Antigravity, distinguishing it from legacy Codex hooks and inert workspace configurations.

This enforcement bridge documents the active security boundaries, Antigravity permissions, local Git settings, and legacy Codex hook layers for the `comptext-sparkctl` workspace.

## 1. Antigravity Permission Model

Antigravity operates under a synchronized workflow allowing local autonomy combined with confirmation-gated remote publishing gates:

*   **ALLOWED (Autonomous Local Work)**:
    *   Local read-only git inspection (`git status`, `git diff`, `git branch`, etc.)
    *   Local staging (`git add`, `git restore --staged`)
    *   Local commits (`git commit`)
    *   Read-only GitHub commands (`gh pr list`, etc.)
*   **ASK / CONFIRMATION REQUIRED (Auth Boundaries)**:
    *   Remote publishing/push (`git push`)
    *   Pull Request write operations (`gh pr create`, `gh pr edit`, `gh pr ready`, `gh pr merge`)
    *   *Note: Push and PR creation are allowed only after a visible human confirmation or Auth prompt.*
*   **DENIED (Strictly Forbidden)**:
    *   Force-push (`git push --force`, `git push -f`)
    *   Destructive history manipulation (`git reset`, `git rebase`, `git merge`)
    *   Tagging and branch deletions (`git tag`, `git branch -d`, `git branch -D`)
    *   Direct raw GitHub API writes (`gh api`) or release modifications (`gh release`)
    *   Remote deploys or publications (`vercel`, `netlify`, etc.)
    *   Environment dumps (`env`, `printenv`, `Get-ChildItem Env:`)
    *   Credential or secret file reads (`.env`, SSH keys, etc.)
    *   Direct modifications to `.git/` internal metadata
    *   Execution of `agy-ct run` and `agy-ct benchmark` (which generate unreviewed outputs)

### Exact Verification Run Results (Historical Reference)

For context, the following baseline test results demonstrate the execution boundaries observed under initial read-only testing:

1.  **`git status --short`** -> **ALLOWED** (Executed successfully, returning untracked files).
2.  **`git diff --stat`** -> **ALLOWED** (Executed successfully).
3.  **`git branch --show-current`** -> **ALLOWED** (Returned `docs/project-governance-sync`).
4.  **`git push --dry-run origin HEAD:docs/project-governance-sync`** -> **DENIED** (Blocked by initial read-only client rule prior to confirmation authorization).
5.  **`git commit --dry-run`** -> **DENIED** (Blocked by initial read-only client rule prior to local autonomy authorization).
6.  **`gh pr list`** -> **DENIED** (Blocked by initial read-only client rule prior to configuration sync).

## 2. Existing Codex Hook Layer

The repository maintains an existing local hook layer designed specifically for Codex workspaces. 

*   **Hook Enablement**: Configured in [config.toml](file:///C:/Users/contr/sandbox/comptext-antigravity-work/comptext-sparkctl/.codex/config.toml) with `hooks = true`.
*   **Hook Matchers**: Configured in [hooks.json](file:///C:/Users/contr/sandbox/comptext-antigravity-work/comptext-sparkctl/.codex/hooks.json) to intercept command-line execution and invoke Python scripts.
*   **Tool Policy Checks**: Mapped to [pre_tool_use_policy.py](file:///C:/Users/contr/sandbox/comptext-antigravity-work/comptext-sparkctl/.codex/hooks/pre_tool_use_policy.py), which blocks:
    *   `git` write commands (`commit`, `push`, `pull`, `merge`, `rebase`, `tag`, `fetch`)
    *   `gh` CLI integrations (`pr`, `issue`, `release`)
    *   Deployments and environments (`vercel`, `netlify`, etc.)
    *   Credential, `.env`, and SSH key file reads.

### Runtime Boundary

> [!IMPORTANT]
> **Codex hooks are active only for Codex runtimes.** They do not intercept or govern the Antigravity agent execution.
>
> The local `.antigravity` configurations (such as [settings.comptext-sparkctl.json](file:///C:/Users/contr/sandbox/comptext-antigravity-work/comptext-sparkctl/.antigravity/settings.comptext-sparkctl.json)) and adapted hooks/plugins are mock templates and remain **completely inert** unless explicitly activated in a separately reviewed phase.
>
> Antigravity's active runtime enforcement is managed strictly via global client-side permissions combined with human confirmation/Auth prompts. The push/PR confirmation step is enabled because the human explicitly requested this workflow for cooperative delivery.

## 3. Local Hard Blocker (Git Push Configuration)

To prevent accidental, unconfirmed pushes to the upstream repository, the Git remote push URL for `origin` is disabled by default:

*   **Fetch URL**: `https://github.com/ProfRandom92/comptext-sparkctl.git`
*   **Push URL**: `DISABLED`

Under the synchronized workflow:
*   Local autonomous staging and commits can be made freely to keep changes scoped and trackable.
*   Pushes to remote branches require manual override, explicit Auth prompts, or temporary push URL authorization verified by the human operator.

## 4. Safe Workflow Execution

To respect the boundaries defined in [AGENTS.md](file:///C:/Users/contr/sandbox/comptext-antigravity-work/comptext-sparkctl/AGENTS.md) and [06_git_handoff.md](file:///C:/Users/contr/sandbox/comptext-antigravity-work/comptext-sparkctl/.agent/skills/06_git_handoff.md):

1.  **Local Inspection**: Running local read-only commands (`git status`, `git diff`, etc.) is fully supported.
2.  **Scoped Edits**: File modifications must be limited strictly to the assigned workspace directories (e.g., `docs/context/` or `agy7rust/` within task scope).
3.  **Local Commits**: Commits may be created autonomously by the agent once local formatting and testing validation passes.
4.  **Remote Publication**: Any push or PR creation must explicitly prompt the user for validation and Auth confirmation.

## 5. Claim Hygiene

This documentation adheres to the rules set forth in [SKILL.md](file:///C:/Users/contr/sandbox/comptext-antigravity-work/comptext-sparkctl/.agents/skills/09_codex_desktop_governance/SKILL.md) and [ANTIGRAVITY_MIGRATION_LEDGER.md](file:///C:/Users/contr/sandbox/comptext-antigravity-work/comptext-sparkctl/docs/context/ANTIGRAVITY_MIGRATION_LEDGER.md):
*   No claims of production readiness or enterprise deployment.
*   No assertions of legal certification, judicial admissibility, or forensic compliance.
*   No declarations of EU AI Act compliance.
*   No claims of official SPARK or BMDS status.
*   All artifacts, test environments, and ledgers remain synthetic evidence supports subject to human review.

---
**References**:
*   [AGENTS.md](file:///C:/Users/contr/sandbox/comptext-antigravity-work/comptext-sparkctl/AGENTS.md)
*   [06_git_handoff.md](file:///C:/Users/contr/sandbox/comptext-antigravity-work/comptext-sparkctl/.agent/skills/06_git_handoff.md)
*   [SKILL.md](file:///C:/Users/contr/sandbox/comptext-antigravity-work/comptext-sparkctl/.agents/skills/09_codex_desktop_governance/SKILL.md)
*   [PHASE8B_CODEX_APP_PLUGIN_SCAFFOLD_DESIGN.md](file:///C:/Users/contr/sandbox/comptext-antigravity-work/comptext-sparkctl/docs/PHASE8B_CODEX_APP_PLUGIN_SCAFFOLD_DESIGN.md)
