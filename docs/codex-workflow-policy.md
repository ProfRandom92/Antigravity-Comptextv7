# Codex Workflow Policy

This policy describes how Codex-style role, plugin, connector, and skill workflows should operate under CompText Agent Governor Policy v1.

## Default Flow

1. Read task instructions and repository instructions.
2. Select a role mode: `read_only_audit`, `sandbox_autonomous`, or `pr_candidate`.
3. Identify approved paths, forbidden paths, and approval gates.
4. Use plugins, connectors, and skills only within the task boundary.
5. Produce local changes or review findings only when the mode allows them.
6. Run the smallest sufficient validation set.
7. Record evidence in the final report and, when applicable, in an evidence packet.
8. Leave human review as the approval boundary.

## Role Mode Selection

Use `read_only_audit` for inspection, review, claim checks, and security analysis when no local edit is approved.

Use `sandbox_autonomous` for local implementation inside explicitly approved paths. This mode can create local commits only when the task policy allows local commits.

Use `pr_candidate` after a validated local branch is ready for review. This mode prepares review evidence and next-step wording, but does not push or create a pull request without explicit approval.

## Connector And Plugin Controls

Approved plugins, connectors, and skills can support context gathering, security review, validation, or presentation. They must stay inside these controls:

- GitHub is read-only unless mutation is explicitly approved.
- Write-capable connector actions require explicit approval for the exact target.
- Provider calls are governed by policy and remain untrusted proposal sources.
- Secrets and tokens are not requested, created, pasted, committed, or exposed.
- Network or connector output is not treated as deterministic validation unless the final report describes the source and limits.
- Local files and committed artifacts are preferred as the source of truth.

## Policy Gate Checklist

Each run should report:

- `instructions_read`: which instructions, skills, and docs were read
- `secret_scan`: command or review evidence for changed files
- `artifact_schema_validation`: schema command or not-applicable reason
- `canonical_hash_validation`: recomputation command or not-applicable reason
- `claim_boundary_check`: unsafe-claim scan and manual review evidence
- `replay_or_roundtrip_validation`: command evidence or not-applicable reason
- `human_review`: approval state and remaining approval needed

Gate results are not blanket approval. They are inputs to human review.

## Evidence Packet Priority

Evidence packets remain the source of truth for governed workflow records. A site, interactive view, generated preview, issue annotation, or PR annotation can present the evidence, but it does not supersede the evidence packet.

If a view and an evidence packet disagree, reviewers should treat the evidence packet and local artifacts as authoritative until the discrepancy is resolved and recorded.

## Annotation Policy

Annotations are review inputs. They can identify risks, explain diffs, point to evidence, or ask for human decisions.

Annotations do not:

- approve a change
- replace policy gates
- replace validation
- prove compliance or certification
- authorize remote mutation

## Claim Hygiene

Workflow reports should use bounded language. Allowed framing includes reviewable evidence, local validation, policy gates, provider boundaries, and human-review inputs.

Blocked framing includes production-ready claims, compliance claims, legal or forensic certainty, official compatibility, autonomous approval, guaranteed correctness, and replacement of human review.
