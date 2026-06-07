# PDF To Evidence Packet Adapter Contract

## Purpose

External or manual PDF extraction can produce a structured JSON artifact for `sparkctl` evidence workflows. `sparkctl` treats that JSON as input evidence, not as truth by itself.

The `PDF-EXTRACTION-V1` contract gives future converters and manual fixtures a deterministic shape for extracted administrative fields, page summaries, tables, figures, warnings, and converter metadata.

## Pipeline

PDF or text fixture -> PDF-EXTRACTION-V1 JSON -> Context Pack / Evidence Packet -> Policy Gate -> Human Review -> Artifact Manifest

The structured extraction can inform a Context Pack and be recorded in an Evidence Packet. It does not replace the Context Pack, the Policy Gate, or Human Review.

## Boundaries

- This PR does not implement OCR.
- This PR does not parse PDFs.
- This PR does not include protected SPARK data.
- This PR does not include real Daimler data.
- This PR does not include real medical or ePA data.
- This PR makes no production claim.
- This PR makes no compliance, legal, or forensic claim.
- Provider output remains untrusted until reviewed.
- Human review remains required.
- No real source PDF is committed.

## Converter Strategy

Docling, MinerU, Marker, pdftotext, and manual processes can be future producers of the same schema. This PR defines only the adapter contract.

The contract records converter name, converter version, and extraction mode so reviewers can distinguish synthetic fixtures, manual fixtures, and future external-tool outputs.

## Future Codex-Style Plugin Bundle Readiness

Future Codex-style plugin bundles could expose skills, commands, hooks, and declared artifacts around this contract.

This repository does not claim official OpenAI plugin compatibility or plugin-directory availability. A future bundle would still need human review, explicit connector boundaries, declared artifacts, and repository-specific approval before any write-capable action.

## Method Precedent

This follows prior CompText privacy-preserving synthetic fixture and replay-contract patterns. The fixture is synthetic and bounded so reviewers can inspect the contract without relying on protected source material.

## Review Use

Reviewers should check:

- the fixture uses `schema_version: PDF-EXTRACTION-V1`
- the source file path is descriptive but no source PDF is committed
- extracted fields are administrative and synthetic
- warnings state the fixture limits
- converter metadata identifies manual fixture preparation
- downstream evidence packets preserve the source and warning context
