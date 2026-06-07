# Artifact Contract

## SPARK Evidence Packet v1 Shape

The packet is modeled as two layers:

- `SparkEvidencePacketPreimage`: the review data that is canonicalized.
- `SparkEvidencePacketEnvelope`: the preimage plus derived `canonical_json` and `canonical_hash`.

The preimage contains:

- `schema_version`
- `local_id`
- `goal`
- `source_summary`
- `context_pack_summary`
- `policy_result`
- `provider_boundary_status`
- `untrusted_proposal`
- `human_review_decision`
- `claim_hygiene`
- `artifact_manifest`
- `warnings`
- `limitations`

The envelope contains all preimage fields plus:

- `canonical_json`
- `canonical_hash`

## Canonicalization

`canonical_json` is computed only from `SparkEvidencePacketPreimage`. It is not part of its own hash preimage.

`canonical_hash` is computed only as `sha256_hex(canonical_json)`.

## Validation

Validation rejects:

- changed preimage with stale `canonical_json`
- changed `canonical_json`
- changed `canonical_hash`
- missing required goal, review, policy, claim hygiene, warning, limitation, or artifact fields

Existing `.spkg` package compatibility is preserved. The SPARK Evidence Packet v1 is a separate evidence envelope and does not alter the existing package verification contract.
