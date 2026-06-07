# CompText Agent Governor Policy v1

CompText Agent Governor is a lightweight policy layer for Codex-style role, plugin, connector, and skill workflows. It does not replace repository instructions, project policy, or human review. It gives agents and reviewers a shared vocabulary for allowed modes, policy gates, and evidence requirements before an agent run is treated as reviewable.

The governor sits around the CompText pipeline:

Source or task -> Instructions -> Role mode -> Policy gates -> Plugin and connector boundary -> Evidence packet -> Human review -> Artifacts

Evidence packets remain the source of truth for what was requested, inspected, validated, and reviewed. Sites, dashboards, interactive views, comments, and annotations are presentation or review aids only unless their contents are also captured in the evidence packet or another declared artifact.

## Role Modes

### read_only_audit

Use `read_only_audit` when the agent is inspecting a repository, issue, pull request, artifact, or workflow without making changes.

Allowed behavior:

- read local instructions and scoped files
- inspect Git history and local diffs
- use approved read-only connectors
- produce findings, annotations, or reports for human review

Blocked behavior:

- no commits
- no pushes
- no pull requests
- no deployment
- no issue creation
- no source mutation

### sandbox_autonomous

Use `sandbox_autonomous` when the agent may make local, reviewable changes inside explicitly approved paths.

Allowed behavior:

- edit only approved local paths
- run local validation commands
- create local artifacts when the task allows them
- create a local commit when explicitly allowed by the task policy

Blocked behavior:

- no remote mutation
- no provider key handling
- no dependency updates unless separately approved
- no edits outside approved paths
- no bypass of human review

### pr_candidate

Use `pr_candidate` when local changes are ready to be reviewed as a possible pull request, but remote mutation has not been approved.

Allowed behavior:

- summarize the local branch
- report changed files and validation evidence
- prepare PR-ready wording as a draft artifact
- identify the approval needed for push or PR creation

Blocked behavior:

- no push
- no PR creation
- no merge
- no auto-approval
- no claim that review has completed

## Plugin And Connector Rules

Plugins and connectors are governed by task approval and repository policy.

- Prefer local files as the source of truth.
- Treat GitHub and source-control connectors as read-only unless the human explicitly approves mutation.
- Do not create issues, pull requests, remote branches, labels, comments, reactions, deployments, or releases without explicit approval.
- Do not use connectors to access or transmit secrets.
- Do not rely on hidden connector state as validation evidence unless the evidence is reported with enough detail for review.
- Provider output remains an untrusted proposal until human review.
- Tools may support presentation, inspection, or validation, but they do not create approval by themselves.

## Policy Gates

Every governed run should report these gates with `pass`, `fail`, `not_applicable`, or `deferred`, plus evidence.

| Gate | Required Evidence |
| --- | --- |
| `instructions_read` | Repository and task instructions read before edits or claims. |
| `secret_scan` | Targeted check that changed files do not expose secrets or tokens. |
| `artifact_schema_validation` | Schema validation for changed artifacts when a schema exists. |
| `canonical_hash_validation` | Recomputed hash or explicit not-applicable reason for canonical artifacts. |
| `claim_boundary_check` | Scan and review for blocked claims. |
| `replay_or_roundtrip_validation` | Replay, roundtrip, or equivalent local validation when behavior or artifacts changed. |
| `human_review` | Human approval state recorded as pending, approved, rejected, or not requested. |

The gates are evidence controls, not permission to auto-apply changes. A passing gate means the agent has produced reviewable support for that gate. It does not mean production readiness, legal status, certification, forensic certainty, or guaranteed correctness.

## Presentation Surfaces

Sites, interactive views, dashboards, and generated previews are presentation only. They can help reviewers inspect an evidence packet, diff, artifact manifest, or policy result, but they are not the authoritative record unless their data is captured in the evidence packet or declared artifacts.

Annotations are human-review inputs. Inline notes, comments, labels, review directives, and rendered callouts can guide attention, but they do not approve a change or replace the recorded human review decision.

## Claim Boundaries

CompText Agent Governor may claim bounded review workflow support, policy-gated operation, local validation evidence, and reviewable artifacts when those are actually implemented or documented.

It must not claim production-ready status, compliance or certification, legal evidentiary status, forensic proof, official SPARK compatibility, autonomous approval, replacement of human review, or guaranteed correctness.
