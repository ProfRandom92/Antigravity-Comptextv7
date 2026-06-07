# Safety And Claims

## Preserved Invariants

- Goal informs packaging and review criteria.
- Goal never bypasses the Policy Gate.
- Goal never authorizes auto-apply behavior.
- Provider output is untrusted until reviewed.
- Human Review is the approval boundary.
- Artifacts preserve the evidence trail.

## Allowed Claims

- deterministic canonical packaging
- bounded artifact manifest
- local integrity anchor when implemented
- SHA-256 hash of canonical JSON when actually computed
- human review trail
- policy result record
- provider-boundary record
- evidence packet for review
- prototype/demo workflow

## Blocked Claims

- production-ready
- EU AI Act compliant
- legally certified
- forensic proof
- guaranteed correctness
- guaranteed replay validity for arbitrary inputs
- certified government use
- autonomous approval
- replaces human review

## Runtime Boundaries

The evidence packet generator performs local file reads for manifest hashing only. It does not call providers, create tokens, deploy, push, create pull requests, create issues, create remote branches, or mutate GitHub.
