---
name: codex-token-saving
description: "Reduce context waste and avoid repo-drift during Codex Desktop work."
---

# Skill: Codex Token-Saving Workflow

## Purpose
Reduce context waste and avoid repo-drift during Codex Desktop work.

## Use this skill when
- Starting a new Codex Desktop session.
- Working under token limits.
- Deep-diving a large repo.

## Operating mode
- Do not scan the entire repo blindly.
- Start with local instructions and manifests.
- Use targeted `rg` searches.
- Read only files relevant to the current phase.
- Produce compact summaries.
- Avoid rereading large files.
- Batch validation after coherent edits.

## Standard phase pattern
1. Bootstrap workspace.
2. Read instructions.
3. Build compact repo map.
4. Identify smallest safe change.
5. Ask for approval if in plan mode.
6. Implement focused change.
7. Validate.
8. Report exact files and command results.

## Useful search terms for CompText
- `canonical_json`
- `sha256_hex`
- `package`
- `artifact`
- `manifest`
- `goal`
- `policy`
- `review`
- `provider`
- `boundary`
- `handoff`
- `roundtrip`
- `SPARK`
- `claim`
