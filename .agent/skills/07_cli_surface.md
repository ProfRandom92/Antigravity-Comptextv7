# Skill 07: agy-ct CLI Surface Rules

## 1. CLI Subcommand Taxonomy
`agy-ct` must implement the following subcommand structure exactly:
- `run`
- `demo`
- `doctor`
- `validate`
- `handoff`
- `package`
  - `compress`
  - `inspect`
  - `verify`
  - `replay`
  - `adversarial`
- `context`
  - `build`
  - `render`
  - `validate`
  - `all`
- `schema check`
- `report export`
- `notebook bundle`

## 2. Clap Parser Behavior
- Subcommands must be defined using strict typed options inside Rust's `clap` crate.
- Command-line parsing must enforce correct parameter names, required arguments, and mutually exclusive options at the parser level.
- Clean `-h` and `--help` pages must be auto-generated for each command and subcommand.

## 3. Phase 6B Binary Scope Gating
- Phase 6B is strictly limited to initializing the `agy-ct` binary and constructing the parser tree.
- No execution logic, file operations, or validation pipeline workflows should be implemented in Phase 6B. All subcommands must return placeholder confirmation messages or print structural config layouts when run.

## 4. Preservation of sparkctl
- Under no circumstances should implementation changes to `agy-ct` alter or break any existing command surface or execution behavior of the compatibility CLI binary `sparkctl`.
- Codebase refactoring must maintain backward compatibility.
