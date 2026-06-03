# Agent Skill 05 — Claim Hygiene

This skill defines rules for project documentation and metadata claims to prevent overstatement of security or legal compliance.

## 1. Allowed System Claims

You may make the following claims in logs, reports, and documentation:
- **Synthetic SPARK-Style Fixture:** We operate against static mock datasets representing administrative structures.
- **Deterministic Packaging:** Packaging code creates identical byte outputs across repeated executions from the same input.
- **Replayable Metadata:** We extract canonical field paths and commitment tokens.
- **Tamper-Sensitive Hash Chain:** The package structure incorporates verification chains (payload SHA-256, sidecar final state hash, and package integrity hash).
- **Schema Sidecar Validation:** The CLI enforces required field presence and scalar types on input JSON templates.
- **Deterministic Replay Only:** The tool is designed exclusively for offline package packaging, verification, and schema checks; it does not perform active runtime execution, predictions, or online agent coordination.

## 2. Forbidden Claims (Strictly Prohibited)

Never write, log, or state the following claims:
- **SPARK JSON Compatibility:** Do not claim compatibility with official SPARK JSON extractors or schemas.
- **EU AI Act Compliance:** Do not claim the tool certifies or is compliant with the EU AI Act. Mention only "Art.-12-oriented record keeping support" as a design pattern.
- **Legal or Judicial Proof:** Do not claim that packages constitute court-admissible evidence, legally binding proofs, or legal validation.
- **Forensic Certainty:** Avoid terms like "100% forensic security" or "invulnerable tamper resistance". Use "tamper-sensitive validation".
- **MCP Integration:** Do not claim MCP capability or server features unless explicitly built in a future phase.
