# Antigravity Skill Inventory

> [!IMPORTANT]
> **Status**: inert, repo-local, review-gated, not active runtime. This config/documentation is for reference only.

This inventory provides a complete comparison and registry of the skill sources in the repository, mapped to Antigravity guidelines.

## Skill Sources Classifications

- **.agent/skills** = repo guidance / legacy source
- **.agents/skills** = Antigravity active workspace skill path
- **docs/phase8a-draft-skills** = draft source
- **.antigravity** = inert adapted workspace config

---

## 1. Repo Guidance Source (`.agent/skills/`)

These 15 files represent local repository guidance guidelines:
- `00_project_system.md`: Project system initialization guidelines.
- `01_phase_gate.md`: Phase transition verification.
- `02_rust_validation.md`: Cargo checks and testing protocols.
- `03_artifact_validation.md`: Validation rules for generated assets.
- `04_spark_context_layer.md`: Context pipeline documentation.
- `05_claim_hygiene.md`: Claim constraints.
- `06_git_handoff.md`: Handoff protocols.
- `07_cli_surface.md`: Command surface rules.
- `08_agentic_output_contract.md`: Code quality expectations.
- `09_codex_desktop_governance.md` -> **09_antigravity_governance.md**: Hook rules and boundaries.
- `09_phase6_implementation_gate.md`: Gate guidelines for Phase 6.
- `10_generated_artifact_policy.md`: Generated output management rules.
- `10_spark_evidence_review.md`: Evidence package checking instructions.
- `11_comptext_validate.md`: Local validation commands execution guide.
- `12_agent_handoff_profile.md`: Profile specifications for handoffs.

---

## 2. Compatibility Metadata (`.agents/skills/`)

These 9 folders contain compatibility metadata (with `SKILL.md` files):
- **comptext-governance**: Mapped to Antigravity workflow governance for CompText integrity.
- **codex-role-workflows** -> **antigravity-role-workflows**: Guidelines for role orchestration.
- **codex-token-saving** -> **antigravity-token-saving**: Strategies to minimize prompt size and trace context.
- **pdf-extraction-contracts**: Structural rules for processing and validation of PDF files.
- **reviewer-final-report**: Guidelines for creating evidence reports for human review.
- **rust-canonical-artifacts**: Rules for generating serialized Rust structures.
- **security-readonly-boundaries**: Enforcing read-only rules for source control and GitHub.
- **spark-hackathon-alignment**: Specific prompt engineering and validation guides for the SPARK challenge.
- **sparkctl-evidence-packet**: Structuring SPARK evidence packets for review gates.

---

## 3. Draft Source (`docs/phase8a-draft-skills/`)

These 7 subdirectories contain draft global skill candidates for review only:
- **comptext-operating-boundary**: Load governance and confirm boundaries before CompText work.
- **comptext-context-pack-review**: Review deterministic, redacted, replayable Context Pack workflows.
- **comptext-proposal-gate-review**: Review proposal schema, path safety, validation commands, and human gate before apply.
- **comptext-provider-boundary**: Review dry-run-first provider boundaries, network deny-by-default, auth metadata redaction, and untrusted outputs.
- **comptext-claim-hygiene**: Review docs and handoffs for unsupported assurance claims.
- **comptext-artifact-hygiene**: Classify generated artifacts and commit candidates before staging or handoff.
- **comptext-compact-handoff**: Produce concise phase handoffs with files, commands, validation, git state, risks, and next action.

---

## 4. System-Level Antigravity Skills

The following core Antigravity capabilities are available to this assistant:
- **kernel-dev**: Logic for Multimodal Graph VM and Bytecode Execution.
- **replay-engine**: Kernel Record-Replay (KRR) and Determinism Logic.
- **workflow-skill-creator**: Packages a completed workflow into a reusable Antigravity skill.
- **uv**: Python package manager helper.
- **Scientific Databases**: ensembl-database, pubchem-database, uniprot-database, gnomad-database, clinical-trials-database, etc.
- **Literature Search**: literature-search-arxiv, literature-search-openalex, pubmed-database, etc.

