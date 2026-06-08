# Phase 8 Global Skill Extraction Plan

## 1. Phase 8A: Repo-Local Draft Skills Only

Goal: convert the reusable operating-layer patterns into repo-local draft skill specifications for review.

Scope:
- Draft only under an explicitly approved docs or future skill-draft path.
- Map each candidate skill to existing `comptext-sparkctl` rules before writing any operational instructions.
- Preserve local boundaries: no provider calls, no installs, no global writes, no Rust source edits, no `.codex/**` edits, and no `.agent/skills/**` edits unless separately approved.

Candidate drafts:
- CompText operating boundary
- Context Pack review
- Proposal gate review
- Provider boundary
- Claim hygiene
- Artifact hygiene
- Compact handoff

Exit criteria:
- Each draft has purpose, trigger, allowed files, forbidden actions, validation, return schema, and claim boundaries.
- Review explicitly confirms which drafts may become repo-local skills.

## 2. Phase 8B: Plugin Scaffold Design Only

Goal: design a Codex Desktop / CompText plugin scaffold without creating an installable plugin.

Scope:
- Document manifest shape, permission categories, hook policy categories, advisory agent boundaries, and expected review gates.
- Treat hook and permission files as inert templates only.
- Keep all paths repo-relative.
- Do not create active plugin folders, enable hooks, or register MCP servers.

Design boundaries:
- CompText remains the deterministic control plane.
- Codex Desktop remains the execution surface.
- No LLM judge is introduced.
- Advisory agents cannot grant PASS/FAIL or bypass local validation.

Exit criteria:
- A reviewer can tell which assets would exist, which are inert, and which future step would activate them.

## 3. Phase 8C: MCP Design Only

Goal: design MCP concepts without configuring live MCP servers.

Scope:
- Specify read-only Git, docs/RAG, CI/log, Context Pack, and proposal-review MCP ideas.
- Define each MCP boundary as untrusted input requiring normalization and local validation.
- Exclude deploy, database-write, secrets-manager-output, issue/PR write, and provider-call tools from the initial design.

Exit criteria:
- Each MCP candidate lists purpose, input, output, denied operations, trust boundary, and validation path.
- No `.mcp.json`, `.codex/**`, or active connector config is modified.

## 4. Phase 8D: Optional Dry-Run Installer Only

Goal: design a dry-run installer that reports planned global skill/plugin actions without performing them.

Scope:
- Dry-run output only: planned source files, destination paths, conflicts, checksums, and denied files.
- No writes outside the repo.
- No global skill installation.
- No plugin registration.
- No network access.

Required checks:
- Reject absolute paths and parent traversal.
- Reject secret-looking files.
- Reject generated runtime artifacts.
- Reject executable hook activation.
- Report local hashes only as change-detection metadata.

Exit criteria:
- Dry-run output is reviewable and deterministic.
- Human review can approve, reject, or request edits before any global install phase exists.

## 5. Phase 8E: Explicit Review Gate Before Global Install

Goal: require a separate human decision before any global skill, plugin, hook, or MCP installation.

Gate requirements:
- Exact source files listed.
- Exact destination paths listed.
- Diff or generated preview available.
- Validation evidence recorded.
- Rollback plan documented.
- Claim-hygiene and artifact-hygiene review complete.

Blocked unless explicitly approved:
- Writing to global Codex skill directories.
- Editing `.codex/**`.
- Enabling live hooks.
- Registering MCP servers.
- Installing plugins.
- Creating commits, pushes, PRs, issues, releases, or deployments.

Recommended decision record:
- `APPROVE_GLOBAL_INSTALL`: no by default.
- `APPROVE_PLUGIN_SCAFFOLD`: no by default.
- `APPROVE_MCP_CONFIG`: no by default.
- `APPROVE_LIVE_HOOKS`: no by default.
