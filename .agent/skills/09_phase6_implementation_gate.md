# Skill 09: Phase 6 Implementation Gates

## 1. Phased Execution Roadmap
No cross-phase implementations are allowed. Development must follow these strict gates:
- **Phase 6B**: `agy-ct` binary configuration, dependencies registration, and `clap` tree definition only. No execution logic.
- **Phase 6C**: Compatibility command wrappers implementation (mapping `doctor`, `validate`, and `handoff` commands).
- **Phase 6D**: Automatic `run` and `demo` workflow orchestrator logic.
- **Phase 6E**: Structured output modes (`--json`, `--plain`) and execution JSON report exporter.
- **Phase 6F**: Context cache valve functionality and notebook bundle exporter.

## 2. Dependency Restriction
- The installation of heavy or complex libraries (`dag_exec`, `asupersync`, `wasm_sandbox`, `wasmtime`, `tokio`, `ratatui`) is deferred for future phases.
- No new packages or dependencies may be registered in `Cargo.toml` without explicit phase-gate approval.

## 3. Safety and Sandbox Bounds
- **Offline Operations**: By default, no subcommands may access the network.
- **Git Safety**: CLI commands must never run git commits or push operations.
- **No Destructive Overwrites**: Commands must prompt before overwriting files unless overridden by `--non-interactive` or `--force`.
- **Directory Bounds**: Commands must restrict all scans to the workspace directory. No parent or sibling directory searches are permitted.
