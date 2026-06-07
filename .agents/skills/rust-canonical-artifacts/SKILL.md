# Skill: Rust Canonical Artifacts

## Purpose
Keep Rust artifact packaging deterministic, testable, and honest.

## Use this skill when
- Editing Rust package/codec/serialization code.
- Working on `canonical_json`, `sha256_hex`, manifests, or roundtrip tests.

## Determinism rules
- Canonical serialization must be stable across runs.
- Avoid map iteration order unless explicitly sorted/canonicalized.
- Avoid nondeterministic timestamps in hash inputs unless intentionally part of the schema and tested.
- Avoid platform-specific path separators inside canonical hashes unless normalized.
- Hash only canonical bytes/string, not pretty-printed or debug output.

## Hash rules
- Never display `sha256` unless actually computed.
- Never call a placeholder hash an integrity anchor.
- If a hash is optional, represent missing hash explicitly.
- If docs mention SHA-256, say “over canonical JSON” only when implemented.

## Rust quality rules
- Prefer typed structs/enums over loose strings for contract-critical fields.
- Use serde derives consistently.
- Keep backwards compatibility if an existing package format exists.
- Add tests before broad refactors.
- Keep changes small.

## Validation
Prefer:
- `cargo fmt --check`
- `cargo test`
- `cargo clippy --all-targets --all-features -- -D warnings`

If clippy fails on pre-existing warnings, report honestly and fix only safe issues.
