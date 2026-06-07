# Skill: PDF Extraction Contracts

## Purpose

Guide work on `PDF-EXTRACTION-V1` structured-data artifacts for SPARK-like administrative workflows.

## Use This Skill When

- Adding or reviewing PDF extraction fixtures.
- Updating `schemas/spark/pdf_extraction_v1.schema.json`.
- Validating external or manual extraction JSON before evidence packaging.
- Connecting PDF extraction artifacts to Context Pack or Evidence Packet workflows.

## Contract Boundary

`PDF-EXTRACTION-V1` is an adapter contract. It accepts structured JSON from manual or external extraction tools as input evidence.

It does not:

- implement OCR
- parse PDFs
- download source PDFs
- call providers
- create a Codex plugin bundle
- create an MCP server
- create hooks or commands
- claim official OpenAI plugin compatibility
- claim official SPARK compatibility
- replace human review

## Required Fields

Every artifact must include:

- `schema_version`
- `source_file`
- `document_type`
- `pages`
- `tables`
- `figures`
- `extracted_fields`
- `warnings`
- `tool_metadata`

`schema_version` must be `PDF-EXTRACTION-V1`.

`tool_metadata.converter` must be one of:

- `manual`
- `docling`
- `mineru`
- `marker`
- `pdftotext`
- `other`

`tool_metadata.extraction_mode` must be one of:

- `synthetic_fixture`
- `manual_fixture`
- `external_tool`

## Fixture Rules

Synthetic fixtures must not include protected personal data, real SPARK data, real Daimler data, real medical data, or real ePA data.

Do not commit source PDFs unless a future task explicitly approves that artifact and license boundary.

## Validation

Prefer local runtime validation with `validate_pdf_extraction_contract_value` and deterministic canonical hashing with the existing `canonical_json` and `sha256_hex` helpers.

Report Agent Governor gate states using exactly one of:

- `pass`
- `fail`
- `not_applicable`
- `deferred`

Use `not_applicable` only when a gate does not apply, and explain why. Use `deferred` when the gate is required but intentionally left for later human/tool review.

For Rust changes, run:

- `cargo fmt --all --check`
- `cargo test`
- `cargo clippy --all-targets --all-features -- -D warnings`

## Claim Boundaries

Use bounded wording: adapter contract, structured input evidence, manual fixture, external-tool output, review input, artifact manifest.

Do not claim production readiness, compliance or certification, legal evidentiary status, forensic certainty, official SPARK compatibility, official OpenAI plugin compatibility, autonomous approval, or guaranteed correctness.
