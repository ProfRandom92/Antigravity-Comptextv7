# Phase 8 Cross-Repo Operating Layer Audit

## 1. Executive Summary

This audit reviewed `ProfRandom92/comptext-cli` as a temporary read-only source for reusable CompText Codex operating-layer patterns. The source repo already contains a mature operating model around deterministic Context Packs, proposal-gated mutation, provider isolation, hook/permission templates, local skill registries, provenance manifests, agent-state artifacts, token economy, and Antigravity plugin bundle design.

The recommended extraction path is documentation-first and review-gated: draft repo-local skills and design documents in `comptext-sparkctl`, then separately review any plugin, MCP, or global-install concept before implementation. Nothing from `comptext-cli` should be vendored or copied as executable source in this phase.

## 2. What comptext-cli Already Has

- A project-level `AGENTS.md` defining deterministic Context Packs, dry-run before network, proposal before apply, untrusted provider output, local validation, network deny-by-default, secrets redaction, and explicit git authorization gates.
- A Context Pack contract with schema fields for task, mode, read-first files, allowed/forbidden paths, validation commands, provider metadata, rendered context, and policy flags.
- Proposal Mode that writes structured proposal JSON under `proposals/` without mutating active source files.
- An Apply Gate design and implementation that blocks path traversal, absolute paths, `.git/`, `.comptext/`, `target/`, `reports/`, `.env`, and key/cert-like file targets before validation.
- Provider boundary docs and source code patterns for `dummy`, Ollama variants, and OpenAI-compatible profiles, with network fail-closed policy and redacted auth metadata.
- Hook and permission governance docs describing SessionStart, PreToolUse, PostToolUse, and PostPhase interceptors as planned policy targets.
- Local skill registries under `.agent/skills/` plus legacy compatibility metadata under `.agents/skills/`.
- Token economy guidance for read-first minimalism, one-skill-at-a-time loading, compact phase reports, and session-state handoff.
- Local provenance and agent-state contracts using repo-relative paths, SHA-256 change detection, schema versions, sorted evidence, and explicit no-assurance boundaries.
- Antigravity plugin bundle templates for manifest, skills, rules, hooks, MCP config, permissions, and advisory subagents.

## 3. Reusable Patterns for Codex Desktop

- Treat models, MCP servers, browser output, and plugin output as untrusted inputs until normalized, reviewed, and locally validated.
- Preserve a hard provider boundary: Context Pack first, policy gate second, provider call or dry-run third, proposal artifact fourth, human review fifth.
- Keep mutation authority out of provider output. Provider responses may propose changes, but they do not apply changes.
- Make all operating-layer paths repo-relative and reject absolute or parent-traversal paths.
- Separate evidence artifacts from source-of-truth configuration. Runtime reports and cache files should inform review but not silently become commit candidates.
- Use review gates for phase transitions. Do not imply completion or next-phase readiness from generated artifacts alone.
- Prefer deterministic local checks over LLM judging for PASS/FAIL decisions.

## 4. Candidate Global Skills

- `comptext-operating-boundary`: Load AGENTS, repo-local skills, allowed paths, forbidden actions, and return schema before any CompText work.
- `comptext-context-pack-review`: Audit context-pack inputs, exclusions, redaction, deterministic ordering, and generated-output hygiene.
- `comptext-proposal-gate-review`: Review proposals before apply, including schema shape, target paths, validation commands, rollback notes, and risk notes.
- `comptext-provider-boundary`: Enforce dry-run-first, provider-output-untrusted, network-deny-default, and auth-metadata-redaction rules.
- `comptext-claim-hygiene`: Block unsupported claims around production readiness, legal proof, compliance, forensic certainty, official compatibility, or autonomous approval.
- `comptext-artifact-hygiene`: Classify runtime cache, proposals, reports, benchmarks, and provenance files before staging or handoff.
- `comptext-handoff-compact`: Produce compact phase handoffs with files, commands, validation, risks, and next safe action.

## 5. Candidate Hooks

- Secret-read blocker for `.env`, `.env.*`, private keys, cert/key stores, credential files, and broad environment dumps.
- Git-write blocker for commit, push, pull, merge, rebase, tag, remote branch creation, PR creation, issue creation, release, and deploy commands unless explicitly authorized.
- Provider/network blocker that fails closed unless the active phase explicitly permits network/provider execution.
- Protected-path warning for source, README, generated reports, benchmarks, and evidence artifacts.
- Proposal-before-apply guard that blocks mutation flows lacking a reviewed proposal or explicit human authorization.
- Post-tool redaction filter for high-risk credential-looking output before it enters the model context.
- Post-phase validation reminder requiring local validation evidence and git status before handoff.

## 6. Candidate Plugin Assets

- A plugin manifest template that names CompText as control plane and Codex Desktop as execution surface.
- A rules document declaring deterministic control, no LLM judge, advisory-only subagents, untrusted MCP output, and repo-relative permissions.
- Permission templates with explicit operation types such as command, write_file, read_url, and MCP, but only after review.
- Hook policy templates as inert audit assets first; live hook enforcement should remain a separate reviewed phase.
- Advisory agent specs that can inspect and recommend but cannot approve, deny, or grant PASS/FAIL status.

## 7. Candidate MCP Ideas

- Read-only Git MCP for repo topology, diffs, and history context.
- Read-only project docs/RAG MCP for CompText docs and historical phase reports.
- Read-only CI/log MCP for validation output inspection.
- A local Context Pack MCP that exposes normalized, redacted, deterministic context artifacts without mutation tools.
- A proposal review MCP that validates proposal schema and path policy but does not apply changes.

## 8. Context-Budget / Handoff Strategy

- Load only the active repo guidance, the relevant skill, and the files named by the task.
- Avoid repo-wide rereads unless crossing task boundaries or source files changed.
- Keep handoffs structured and compact: phase, status, files changed/read, validation, risks, next action.
- Store long evidence in files and summarize it in chat instead of pasting full command logs.
- Prefer one operating skill per task to avoid instruction interference.

## 9. Claim-Hygiene Rules to Inherit

- Provider output is untrusted until reviewed.
- Local hashes are change-detection metadata, not legal, forensic, compliance, or correctness proof.
- MCP capability must not be claimed unless actually implemented and validated.
- Do not claim production readiness, legal proof, EU AI Act compliance, official SPARK compatibility, forensic certainty, autonomous approval, or guaranteed correctness.
- Use bounded wording: local validation, deterministic packaging where evidenced, tamper-sensitive checks, replayable metadata, and review-gated proposals.

## 10. Artifact-Hygiene Rules to Inherit

- Generated context packs, provider requests, provider responses, benchmark outputs, proposals, and agent-state cache files are runtime artifacts by default.
- Runtime artifacts should not be staged or committed without explicit human approval for exact files.
- Reports may be review evidence, but generated report churn is not automatically commit-worthy.
- Secret redaction must occur before artifacts are written.
- Artifact hashes must never be faked or edited to satisfy a narrative.

## 11. What Must NOT Be Copied

- Do not copy or vendor Rust source from `comptext-cli`.
- Do not copy `.git`, generated `.comptext/`, proposals, reports, benchmark outputs, or local runtime artifacts.
- Do not copy provider configs containing real endpoints or credential references as active config.
- Do not install global skills directly from the source repo.
- Do not enable live hooks from templates without separate review.
- Do not import MCP configs containing placeholder secret headers as active configs.
- Do not copy branding assets or README marketing material into operational guardrails.
- Do not copy scripts that create repos, push, publish, deploy, or otherwise perform remote write operations.

## 12. Recommended Next Phase

Proceed with Phase 8A only: draft repo-local skills in `comptext-sparkctl` as review documents, not global installs. The next phase should map each candidate skill to existing `comptext-sparkctl` governance files, remove source-repo-specific command names, and define validation that does not touch Rust source, reports, benchmarks, `.codex/**`, or generated artifacts.
