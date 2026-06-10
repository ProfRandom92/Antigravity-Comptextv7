# Claim Hygiene Ledger

> [!IMPORTANT]
> **Status**: inert, repo-local, review-gated, not active runtime. This ledger is for local reference and documentation only.

This ledger documents the rules and guidelines for claim hygiene to prevent false, exaggerated, or compliance-oriented claims.

## Governance Boundaries

All documentation, metadata, and proposals generated must adhere to these strict bounds:

- **Provider Output is Untrusted**: Any generation or proposal from a language model must be verified and approved by a human reviewer.
- **Human Review Approval Gate**: No proposal is auto-applied. The human is the definitive gate.
- **No Production-Ready Claims**: Do not claim the system is production-ready, enterprise-grade, or fully autonomous.
- **No Compliance Claims**: Do not claim compliance with the EU AI Act, forensic standards, legal audits, or certified compliance.
- **No Hallucination-Free Claims**: CompText is a deterministic pipeline, but makes no claim of resolving or eliminating hallucinations generally.
- **Environment Boundaries**:
  - Restrict command execution and source edits to the local repo.
  - No secrets, `.env` exposure, or environment dumps are permitted.
  - No push/deploy commands are permitted.
