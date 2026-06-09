# Phase 8C MCP Design Only

## 1. Executive Summary

Phase 8C defines candidate Model Context Protocol (MCP) concepts for future CompText and `sparkctl` operating-layer work. This document is design-only. It does not configure, register, install, enable, or run any MCP server.

The design keeps the existing CompText line intact:

```text
Models are providers. Context is the product.
```

MCP is treated as a future context and tool boundary, not as an approval authority. Any MCP output would be untrusted until normalized, reviewed, and validated against local repository evidence. Human review remains the approval boundary before proposals are applied, configuration changes are promoted, or any active Codex surface is enabled.

The initial MCP direction is read-only and review-gated:

- read-only Git topology, diff, and history context
- read-only documentation and retrieval context
- read-only CI and log inspection
- local Context Pack inspection without execution
- proposal review support without applying changes

## 2. Why Phase 8C Is Design-Only

Phase 8C is intentionally limited to a single design document because the prior Phase 8 operating-layer materials require review-gated promotion before activation. Phase 8A draft skills are review artifacts, not installed skills. Phase 8B plugin scaffold material is inert and not an active plugin. The Phase 8 plan explicitly scopes Phase 8C to MCP concepts without live MCP configuration.

Design-only status prevents accidental expansion of authority. In particular, this phase does not:

- create `.mcp.json`
- edit `.codex/**`
- edit `.agent/skills/**`
- register MCP servers
- install or activate plugins
- enable hooks
- install global skills
- create automations
- use Computer Use
- call providers or models
- mutate source, reports, or artifacts

This preserves the current evidence workflow: Source or GitHub URL -> Goal -> Inspect -> Context Pack -> Policy Gate -> Provider Boundary -> Untrusted Proposal -> Human Review -> Artifacts.

## 3. Repo Operating Boundaries Inherited From AGENTS.md

The active `AGENTS.md` defines CompText as a deterministic Context Pack and proposal-gated evidence workflow. GitHub is read-only unless explicitly authorized by the human. Provider output is untrusted until reviewed. Proposals are never auto-applied. Human review is the approval boundary. Artifacts preserve the evidence trail.

Inherited hard boundaries:

- Do not expose secrets.
- Do not fake hashes.
- Do not claim production readiness, EU AI Act compliance, legal certification, forensic proof, guaranteed correctness, autonomous approval, or official compatibility without hard evidence and explicit permission.
- Do not modify root `README.md` without explicit approval.
- Do not commit `reports/latest.json`.
- Do not commit `reports/performance_baseline.json` when it is validation churn.
- Run cargo commands only inside `agy7rust/` unless explicitly approved.
- Keep changes scoped and reviewable.

Repo-local Codex Desktop governance adds:

- Project hooks live under `.codex/` and require Codex trust before enforcement.
- Existing hooks block unauthorized git writes, deploy/release actions, environment dumps, secret-file reads, `agy-ct run`, and `agy-ct benchmark`.
- Existing hooks warn on protected source, README, report, and `artifacts/spark/` references.
- The hook layer is a guardrail, not a complete security boundary.

Repo-local context-layer guidance adds:

- The SPARK Context Layer is not an orchestration framework or active workflow runner.
- It must not bundle the library as an MCP server in the current design.
- It must avoid raw dumps, external integrations, active execution loops, and token-heavy trace output.

## 4. Mapping To Codex App Concept Areas

This section aligns Phase 8C with current public Codex App concept areas from the official Codex documentation. It is concept alignment only, not activation.

| Codex App concept | Phase 8C interpretation |
|---|---|
| Worktrees | Phase 8C work must stay on the clean `phase8c-controlled-activation-plan` worktree and report branch/detached state before handoff. Future MCP output must not decide branch movement, merge, rebase, pull, push, or PR actions. |
| Local Environments | No local environment actions are created or changed. Future setup/action definitions would require a separate review gate because Codex stores project local-environment config under `.codex/`. |
| Review | The Codex review pane can inspect diffs, but it reflects all Git working-tree changes. Phase 8C keeps one docs-only change so review scope remains explicit. |
| Automations | Automations can run unattended and may use plugins and skills. Phase 8C denies automation creation because unattended MCP-backed work would need a separate threat review and sandbox review. |
| Computer Use | Computer Use can operate desktop apps and affect state outside the project. Phase 8C denies Computer Use and treats it as out of scope for governance design. |
| Commands | Codex commands can expose `/mcp`, `/review`, `/status`, goals, and skill invocation. Phase 8C does not use commands to activate MCP, goals, plugins, or automations. |
| Windows | The active environment is Windows/PowerShell. Any future MCP or hook design must include Windows command shape, path normalization, and sandbox behavior review. |
| Permissions | Future MCP activation must use least privilege, explicit tool allow/deny lists, prompt/approval modes, and secret-deny rules. Full-access assumptions are not acceptable for unattended MCP behavior. |
| Rules | Future command rules may complement hooks, but this phase creates no rules. Rules must be reviewed because shell wrappers can hide multiple commands. |
| Hooks | Hooks can match MCP tool names in `PreToolUse` and `PostToolUse`. Phase 8C reads current hooks but does not edit, trust, enable, or add hooks. |
| AGENTS.md | `AGENTS.md` remains the durable repo policy source. Phase 8C does not edit it. Any future MCP must obey the closest active AGENTS guidance. |
| MCP | MCP connects Codex to tools and context. Future MCP servers must be read-only by default, untrusted, allow-listed, and disabled until explicitly promoted. |
| Plugins | Plugins can bundle skills, apps, MCP servers, and lifecycle config. Phase 8C does not create or install a plugin. Phase 8B remains an inert scaffold design. |
| Skills | Phase 8A draft skills remain review-only docs. No global or repo-local skill activation is introduced by Phase 8C. |
| Subagents | Subagents require explicit user request and can add coordination risk. Phase 8C does not spawn subagents. Future MCP review subagents, if any, must be advisory only. |

Official concept references used for alignment:

- `https://developers.openai.com/codex/app`
- `https://developers.openai.com/codex/app/worktrees`
- `https://developers.openai.com/codex/app/local-environments`
- `https://developers.openai.com/codex/app/review`
- `https://developers.openai.com/codex/app/automations`
- `https://developers.openai.com/codex/app/computer-use`
- `https://developers.openai.com/codex/app/commands`
- `https://developers.openai.com/codex/app/windows`
- `https://developers.openai.com/codex/permissions`
- `https://developers.openai.com/codex/rules`
- `https://developers.openai.com/codex/hooks`
- `https://developers.openai.com/codex/guides/agents-md`
- `https://developers.openai.com/codex/mcp`
- `https://developers.openai.com/codex/plugins`
- `https://developers.openai.com/codex/skills`
- `https://developers.openai.com/codex/concepts/subagents`

## 5. MCP Trust Boundary

MCP output is not trusted authority. It is context or tool output that must pass through the CompText review path before it influences source changes, artifacts, reports, commits, or publication.

Minimum trust rules:

- Treat MCP data as untrusted input.
- Normalize paths to repo-relative form before use.
- Reject absolute paths, parent traversal, `.git/`, secret paths, generated runtime artifacts, and protected-path mutation requests.
- Redact secrets before writing or summarizing any MCP-derived evidence.
- Prefer deterministic local checks over MCP or model judgment.
- Do not let MCP tools apply proposals, approve proposals, grant PASS/FAIL, publish branches, mutate remotes, create PRs/issues, deploy, release, or call providers.
- Keep MCP tool sets allow-listed and deny-listed before activation.
- Require human review before any `.mcp.json`, `.codex/**`, plugin manifest, hook, rule, automation, or permission change.

## 6. Candidate MCP Servers

The following candidates are design targets only. None are implemented or configured by Phase 8C.

### 6.1 Read-Only Git Topology/Diff/History MCP

**Purpose:** Provide structured read-only views of repository topology, branch state, selected diffs, commit metadata, and file history for review and handoff.

**Allowed inputs:**

- repo root path
- current branch name
- explicit base ref such as `origin/main`
- explicit file path allow-list
- explicit commit hash or ref for read-only inspection

**Outputs:**

- branch and detached-state summary
- ahead/behind counts
- selected diff summaries
- commit metadata summaries
- file history summaries
- worktree list summaries

**Denied operations:**

- `git push`
- `git pull`
- `git merge`
- `git rebase`
- `git reset`
- `git commit`
- `git tag`
- remote creation or remote config edits
- branch deletion
- checkout/switch that changes active worktree state
- force push
- PR or issue write

**Trust boundary:** Git MCP output is evidence context only. Local Git commands and human review remain authoritative for publication decisions.

**Validation path:**

- Compare MCP summary with `git status --short`.
- Compare base state with `git rev-list --left-right --count HEAD...origin/main`.
- Compare changed files with `git diff --stat`.
- Confirm branch state with `git branch --show-current`.

**Failure modes:**

- stale refs if fetch is not run in a separately authorized step
- path confusion across multiple worktrees
- detached HEAD misreported as a branch
- generated artifacts included in change summaries
- remote write capability accidentally exposed

**Rollback notes:**

- Disable the MCP server or set it `enabled = false` before further use.
- Preserve the incorrect summary as review evidence.
- Re-run local Git verification from the active worktree.
- Do not rewrite history to hide a bad MCP summary.

### 6.2 Read-Only Docs/RAG MCP

**Purpose:** Provide bounded retrieval over approved project documentation, Phase 8 design docs, policy docs, and review-only skill drafts.

**Allowed inputs:**

- explicit docs path allow-list under `docs/**`
- explicit query text
- max result count
- optional phase filter such as `phase8a`, `phase8b`, or `phase8c`

**Outputs:**

- matched document paths
- short excerpts or summaries
- section headings
- citation metadata with repo-relative paths
- missing-document notices

**Denied operations:**

- source file edits
- report or artifact reads unless explicitly approved
- secret or environment reads
- global filesystem search
- provider/model calls
- indexing outside repo-approved docs
- writing embeddings or caches into commit paths

**Trust boundary:** Retrieval output is untrusted and may be incomplete or stale. It supports review but does not replace reading source docs.

**Validation path:**

- Spot-check cited paths with local file reads.
- Confirm retrieved docs are in allowed paths.
- Run `git diff --check` for any docs changes derived from retrieval.
- Keep generated indexes out of commit candidates unless separately approved.

**Failure modes:**

- stale index after docs change
- overbroad search leaking unrelated files
- long raw excerpts increasing token noise
- hallucinated citations or missing path metadata
- generated cache mistaken for source docs

**Rollback notes:**

- Delete or ignore generated local caches if they are outside approved commit scope.
- Rebuild any index only in a separately approved dry-run step.
- Fall back to direct local reads of named docs.

### 6.3 Read-Only CI/Log MCP

**Purpose:** Inspect validation logs, CI summaries, and local command outputs without triggering builds, workflows, uploads, or remote writes.

**Allowed inputs:**

- explicit log file path under approved docs or validation-output locations
- explicit CI run identifier for read-only inspection when authorized
- explicit command-output text pasted by the user
- max lines or section filters

**Outputs:**

- failed check summaries
- warning summaries
- command names and exit-code summaries
- links or identifiers for read-only CI evidence
- missing-log or stale-log notices

**Denied operations:**

- re-running CI
- approving CI
- uploading artifacts
- writing check statuses
- triggering workflows
- creating or updating PRs/issues
- reading secrets from CI logs
- deploy, release, or environment operations
- shell execution

**Trust boundary:** CI/log output is review input only. Logs can contain stale, truncated, or secret-adjacent data and must be summarized carefully.

**Validation path:**

- Confirm any local claims against actual local command outputs.
- Confirm no secret-looking values are quoted into docs or handoff.
- Keep validation claims bounded to observed commands and timestamps.

**Failure modes:**

- stale CI run inspected
- failed command summarized as passing
- secret-like strings exposed from logs
- remote write scope added through a CI integration
- generated logs treated as commit-ready artifacts

**Rollback notes:**

- Disable the MCP server or its write-capable tools.
- Remove secret-like text from summaries and preserve a redacted incident note.
- Re-run local validation when safe and authorized.

### 6.4 Local Context Pack MCP

**Purpose:** Expose normalized, redacted, deterministic Context Pack metadata for review without running providers, executing workflows, or mutating artifacts.

**Allowed inputs:**

- explicit Context Pack file path when approved
- explicit schema path
- requested metadata fields
- redaction profile name
- max rendered size

**Outputs:**

- schema version summary
- included/excluded path summary
- policy gate summary
- redacted provider metadata names, not values
- deterministic ordering summary
- change-detection hash metadata when already present

**Denied operations:**

- provider/model calls
- active workflow execution
- `agy-ct run`
- `agy-ct benchmark`
- artifact regeneration
- raw context dumps
- secret reads
- external API calls
- writing Context Packs or artifacts

**Trust boundary:** Context Pack MCP output is a review view, not the source of truth. Hashes are change-detection metadata only and are not legal, forensic, compliance, or correctness proof.

**Validation path:**

- Validate schema fields with existing local commands only when separately authorized.
- Verify paths are repo-relative.
- Verify excluded paths stay excluded.
- Compare MCP summary to direct file inspection for sampled fields.

**Failure modes:**

- raw payload exposed instead of summarized
- secret metadata value rendered
- generated artifact changed during inspection
- deterministic ordering claimed without evidence
- MCP server mistaken for an active SPARK Context Layer feature

**Rollback notes:**

- Disable the MCP server and discard generated views.
- Preserve existing artifacts without rewriting hashes.
- Re-run review from direct local files.
- Require a fresh design review before any future Context Pack MCP activation.

### 6.5 Proposal Review MCP

**Purpose:** Validate proposal shape, target paths, denied operations, rollback notes, and review readiness without applying changes.

**Allowed inputs:**

- explicit proposal document or JSON path
- explicit policy schema path
- allowed/forbidden path lists
- validation command allow-list
- max summary size

**Outputs:**

- schema conformance summary
- target path review
- denied-operation findings
- missing validation or rollback notes
- human-review checklist
- compact review handoff

**Denied operations:**

- applying proposals
- writing patches
- editing source
- editing reports or artifacts
- approving proposals
- granting PASS/FAIL authority
- provider/model calls
- shell execution
- git commit/push/merge/rebase/pull
- PR or issue writes

**Trust boundary:** Proposal Review MCP can identify review findings, but cannot approve, apply, or certify a proposal. Human review remains the approval gate.

**Validation path:**

- Compare proposal targets to repo-relative path policy.
- Check denied operations against AGENTS.md and hook policy.
- Confirm validation commands are safe and scoped.
- Run local validation only after explicit phase authorization.

**Failure modes:**

- path traversal missed
- proposal output treated as approval
- generated proposal staged by mistake
- validation command includes hidden mutation
- MCP report overstates safety or correctness

**Rollback notes:**

- Mark the MCP finding as advisory only.
- Do not apply the proposal.
- Correct the proposal in a separate reviewed docs or proposal phase.
- Preserve evidence of the rejected review if useful.

## 7. Explicitly Denied Initial MCP Tools

The initial MCP design must not include tools for:

- deploy
- release
- secrets manager output
- database writes
- provider/model calls
- issue/PR write
- git push
- git merge
- git rebase
- git pull
- shell execution
- hook activation
- plugin activation
- global skill install
- automation creation
- Computer Use
- remote branch creation
- environment dumps
- secret-file reads
- artifact regeneration

## 8. Interaction With Phase 8A Skills

Phase 8A draft skills remain review-only documentation under `docs/phase8a-draft-skills/`. They are not globally installed, not active repo skills, and not approved for automatic use outside this repo.

Future MCP tools may support these skill concepts only as advisory inputs:

- `comptext-operating-boundary`: MCP can provide read-only repo state summaries, but the skill boundary remains the policy source.
- `comptext-context-pack-review`: MCP can summarize Context Pack metadata, but cannot generate, mutate, or validate packs without explicit authorization.
- `comptext-proposal-gate-review`: MCP can check proposal shape, but cannot approve or apply proposals.
- `comptext-provider-boundary`: MCP must not call providers or blur provider/model output with local evidence.
- `comptext-claim-hygiene`: MCP summaries must use bounded language and flag unsupported assurance claims.
- `comptext-artifact-hygiene`: MCP must classify generated outputs as non-commit defaults unless exact files are approved.
- `comptext-compact-handoff`: MCP can help structure summaries, but final handoff remains human-reviewable and evidence-bounded.

Promotion from Phase 8A skill docs to active skills is a separate gate and is not part of Phase 8C.

## 9. Interaction With Phase 8B Plugin Scaffold

Phase 8B defines a future inert plugin scaffold and explicitly keeps `mcp/` as design notes only. Phase 8C fills in the MCP design portion of that scaffold without creating the scaffold files.

Future plugin interaction rules:

- Plugin-bundled MCP servers must remain disabled until reviewed.
- Plugin manifests must not bundle write-capable MCP tools in the initial CompText design.
- Plugin-bundled hooks or lifecycle config must pass hook trust review.
- Plugin marketplace entries are out of scope until a later activation phase.
- Plugin sharing, marketplace setup, or installation requires explicit human approval.
- The plugin scaffold cannot convert advisory MCP findings into approval, PASS/FAIL, or mutation authority.

## 10. Claim Hygiene

Allowed wording for Phase 8C:

- "design-only"
- "review-gated"
- "candidate MCP"
- "read-only candidate"
- "untrusted input"
- "local validation"
- "repo-relative path policy"
- "change-detection metadata"
- "tamper-sensitive checks where evidenced"
- "synthetic SPARK-style fixture"

Denied wording unless separately implemented, validated, and approved:

- production-ready
- compliant
- legally certified
- legal proof
- forensic proof
- guaranteed correct
- official SPARK compatible
- autonomous approval
- secure without qualification
- MCP capability is active
- plugin is active
- hooks are newly enabled

Any future MCP-derived statement must name the evidence source and distinguish local fact, design intent, and inference.

## 11. Artifact Hygiene

Phase 8C creates only this review document. Generated reports, benchmarks, runtime caches, Context Packs, provider requests, provider responses, proposals, and `artifacts/spark/*` remain non-commit defaults.

Artifact rules for future MCP work:

- Do not write MCP indexes, caches, logs, or summaries into commit paths by default.
- Do not regenerate artifacts during governance-only phases.
- Do not fake or edit hashes to satisfy a design claim.
- Do not stage generated reports or validation churn without explicit approval for exact files.
- Keep long evidence in files only when the phase explicitly allows it.
- Summarize command evidence in handoffs rather than pasting large logs.

## 12. Security Review Checklist

Before any MCP activation proposal, review:

- Is the MCP server read-only by default?
- Are all tools allow-listed?
- Are write-capable tools absent or disabled?
- Are denied tools explicitly blocked?
- Are secret paths and environment dumps denied?
- Are paths normalized and repo-relative?
- Are absolute paths and parent traversal rejected?
- Are generated artifacts excluded by default?
- Is provider/model execution impossible from the MCP server?
- Is shell execution impossible from the MCP server?
- Is network access documented and minimized?
- Are OAuth, bearer tokens, or environment variables avoided unless separately reviewed?
- Are tool timeouts and startup failure behavior documented?
- Is MCP output clearly untrusted?
- Does validation use local deterministic checks?
- Does the design avoid claims of production readiness, compliance, legal proof, forensic certainty, official compatibility, autonomous approval, or guaranteed correctness?
- Is rollback documented?
- Is human review the approval boundary?

## 13. Promotion Gates Before Any `.mcp.json` Or `.codex/**` Change

No `.mcp.json` or `.codex/**` change is allowed in Phase 8C. Before any future activation phase:

1. Open a separate phase request naming the exact MCP server and exact files.
2. Produce a threat review for the candidate server and tool set.
3. List every tool, input, output, denied operation, permission mode, and timeout.
4. Prove no deploy, release, PR/issue write, git write, shell execution, provider/model call, database write, or secret-output tool is present.
5. Define rollback as disabling or removing the MCP config without touching source artifacts.
6. Confirm AGENTS.md and repo-local governance alignment.
7. Confirm Phase 8A skill and Phase 8B plugin scaffold interactions.
8. Confirm claim hygiene and artifact hygiene.
9. Run `git status --short`.
10. Run `git diff --check`.
11. Run `git diff --stat`.
12. Require explicit human approval for the exact config files.
13. Require explicit human approval before commit.
14. Require explicit human approval before push.
15. Require explicit human approval before PR creation.

## 14. Open Review Questions

- Should CompText MCP candidates be split into separate servers or one server with strict tool namespaces?
- Should read-only Git MCP be allowed to run `git fetch`, or should fetch remain a manual, separately authorized shell step?
- Which docs paths are safe for a docs/RAG MCP index, and should reports remain excluded by default?
- Can CI/log inspection be useful without granting remote CI API access?
- What schema should define a Context Pack MCP response without exposing raw context payloads?
- Should Proposal Review MCP consume only JSON proposals, Markdown proposals, or both?
- What minimum hook coverage is required before MCP tools can be trusted to stay read-only?
- Should MCP outputs be written as temporary review artifacts or only returned in-thread?
- Which future phase owns threat review: Phase 8D dry-run installer, a new Phase 8C.1, or plugin scaffold promotion?
- What exact human approval phrase should unlock any `.codex/**` or `.mcp.json` change?

## 15. Compact Handoff Template

```text
PHASE: Phase 8C MCP Design Only
STATUS:
REPO:
BRANCH:
BASE:
ACTIVE_SURFACES: none
MCP_STATUS: design-only; no MCP registered
FILES_READ:
FILES_CREATED:
FILES_MODIFIED:
DENIED_OPERATIONS:
VALIDATION:
GIT:
CLAIMS:
ARTIFACTS:
RISKS:
NEXT:
COMPACT_HANDOFF:
```
