# Performance Baseline — CompText-Sparkctl

Performance baseline measured on local validation environment.
No performance optimization was performed in this phase.
Measurements are local and environment-specific.

## 1. Wall-Clock Execution Time Measurements

The following timings represent the wall-clock execution time of the CLI commands under the `dev` profile.

| Command | Execution Time (s) | Milliseconds | Notes |
| :--- | :---: | :---: | :--- |
| `cargo run --bin agy-ct -- run` | 8.36 s | 8,360 ms | Coordinates multiple validation subprocesses. |
| `cargo run --bin agy-ct -- doctor` | 0.39 s | 386 ms | Validates configuration directories and schema presence. |
| `cargo run --bin agy-ct -- validate` | 18.48 s | 18,484 ms | Executes full compilation, testing, formatting, and clippy gates. |
| `cargo run --bin agy-ct -- context all` | 1.73 s | 1,733 ms | Chains build, render, and validate contexts. |
| `cargo run --bin sparkctl -- doctor` | 0.36 s | 364 ms | Compatibility entry point for environmental verification. |
| `cargo run --bin sparkctl -- rust-validate` | 14.49 s | 14,487 ms | Invokes the test and style suite verification. |
| `cargo run --bin sparkctl -- context-all` | 2.72 s | 2,720 ms | Builds, renders, and validates context metadata. |
| `cargo run --bin sparkctl -- spark-demo` | 2.65 s | 2,649 ms | Executes compression and roundtrip context validation. |
| `cargo run --bin sparkctl -- handoff-check` | 7.48 s | 7,476 ms | Full workflow pre-handoff health verification. |

## 2. Generated Artifact Size Measurements

The following sizes represent the deterministic outputs of CLI workflows in bytes:

| Artifact Path | Size (Bytes) | Format / Description |
| :--- | :---: | :--- |
| `artifacts/spark/extraction.spkg` | 1,674 B | Binary SPARK-style compressed package. |
| `artifacts/spark/context.json` | 1,916 B | JSON operation context representation. |
| `artifacts/spark/context_render.txt` | 1,757 B | Rendered text context log. |
| `reports/latest.json` | 627 B | Orchestrator step run summary. |

## 3. Scope and Environment Configuration

- **OS Version**: Windows (sandbox environment)
- **Rust Toolchain**: cargo (dev profile, unoptimized + debuginfo)
- **Python Verification tool**: `python` `json.tool`
- **Execution Engine**: Local machine execution with subprocess coordination
