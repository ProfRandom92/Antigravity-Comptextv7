# Phase 6A agy-ct Agentic Deterministic CLI Handbook

## 1. Product Definition
`agy-ct` is a terminal-first, agent-facing and developer-centric Antigravity-CompText Rust SPARK CLI and workbench tool. It is designed to orchestrate local trace compression, sidecar generation, and operational context validation workflows. Unlike generic coding agents, `agy-ct` is a specialized workflow executor. It does not attempt to clone full Antigravity agent codebases, but focuses strictly on managing administrative AI pipeline states.

## 2. sparkctl Compatibility Strategy
`sparkctl` remains the historical compatibility CLI for legacy and hackathon integrations, maintaining its current behavior and CLI parameters. `agy-ct` is introduced as a parallel, production-grade tool. Common core models, validators, and serializers in the underlying `agy7rust` crate will be shared across both CLI binaries. This ensures that:
- Existing `sparkctl` features and APIs remain unchanged.
- Shared logic updates both CLIs simultaneously.
- Validation behavior remains consistent between the two CLIs.

## 3. Human UX vs Agentic Experience (AX)
`agy-ct` is designed for dual-mode operation:
- **Human UX:** Provides clear, colorful terminal status lines, warning notifications, and an intuitive nested subcommand taxonomy.
- **Agentic Experience (AX):** Provides structured JSON reporting, robust non-interactive flags, deterministic exit codes, and machine-parsable error payloads with diagnostic identifiers.
The tool implements an agentic terminal workflow design:
`Plan -> Run -> Verify -> Replay -> Context -> Report`

## 4. Non-Interactive Mode Design
When the `--non-interactive` flag is passed (or if no terminal/TTY is detected):
- The CLI disables interactive prompting, paging, and confirmation requests.
- If a destructive operation or missing argument requires input, the CLI must abort immediately, print a JSON or plain-text diagnostic payload, and exit with a non-zero code.
- Interactive elements like spinners or progress bars are hidden or converted to static log outputs.

## 5. Generate-Validate-Fix Workflow
`agy-ct` supports a multi-pass execution mode for automated script integration:
1. **Generate**: Create compressed sidecars or context files.
2. **Validate**: Perform strict schema verification, canonical ordering, and boundary checking.
3. **Fix**: If validation fails on fixable issues (such as unstable key sorting or non-canonical whitespaces), the CLI can run minor formatting adjustments if permitted, or emit precise machine-readable corrections for the caller to apply.

## 6. agy-ct Command Taxonomy
The CLI implements the following subcommand tree:
- `agy-ct run`: Automatically coordinates the full local step sequence.
- `agy-ct demo`: Runs a predefined end-to-end trace workflow.
- `agy-ct doctor`: Performs local workspace integrity and configuration diagnosis.
- `agy-ct validate`: Validates current project formatting, tests, and clippy rules.
- `agy-ct handoff`: Pre-checks local repository readiness for handoff.
- `agy-ct package compress`: Compress raw extraction files to `.spkg`.
- `agy-ct package inspect`: Read sidecar properties and headers from `.spkg`.
- `agy-ct package verify`: Run SHA-256 cryptographic verification of `.spkg`.
- `agy-ct package replay`: Deterministically reconstruct and replay the sidecar trace.
- `agy-ct package adversarial`: Verify robustness against tampered payload attributes.
- `agy-ct context build`: Generate a structured operational context from a package.
- `agy-ct context render`: Render operational context into token-light text.
- `agy-ct context validate`: Run structural validation and leak checks on a context.
- `agy-ct context all`: Execute context build, render, and validate tasks in sequence.
- `agy-ct schema check`: Validate raw trace files against target JSON schemas.
- `agy-ct report export`: Exporter for generated pipeline JSON reports.
- `agy-ct notebook bundle`: Bundles context state and text renderings into a unified documentation payload.

## 7. agy-ct run Automatic Workflow
The automatic executor `agy-ct run` coordinates the following order of local steps:
1. **workspace doctor**: Ensure that schemas and configs are present.
2. **package compress**: Generate the `.spkg` file from input traces.
3. **package verify**: Confirm cryptographic signature and hashes.
4. **package replay**: Replay sidecar tool order.
5. **schema check**: Verify data conforms to schema definitions.
6. **context build**: Compile the operational context.
7. **context render**: Generate the token-light rendered text.
8. **context validate**: Verify context structure and leak boundaries.
9. **report export**: Write the final pipeline execution JSON report.

## 8. Terminal Output Model, Stdout/Stderr, and Exit Codes
- **stdout**: Strictly reserved for primary payload data, requested exports, and JSON report strings. This ensures simple redirection to downstream tools.
- **stderr**: Used for progress bars, diagnostics, human-readable titles, warnings, and log messages.
- **Exit Codes**:
  - `0`: Success.
  - `1`: General runtime error.
  - `2`: Validation/leak boundary failure.
  - `3`: Missing configuration or schema.
  - `4`: Invalid file format or signature.
- **Compact Errors**: The CLI outputs compact, single-line error descriptions by default. Detailed diagnostic logs or debugging stack traces can be accessed via `--explain <ERROR_CODE>` or the `--verbose` flag.

## 9. Output Modes
The CLI supports the following output configurations:
- **Default**: Styled, human-readable terminal output.
- **`--plain`**: Text output without animations, progress tickers, or formatting blocks.
- **`--json` / `--output json`**: All outputs are emitted as structured JSON objects on stdout.
- **`--verbose`**: Outputs step-by-step diagnostic statements.
- **`--quiet`**: Suppresses all stderr info logs; only critical errors are output.
- **`--no-color`**: Disables ANSI color escapes.
- **`--non-interactive`**: Prevents prompts and forces immediate termination on input requests.

## 10. JSON Report Schema Draft
The JSON output generated by `report export` follows this structure:
```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "AgyCtPipelineReport",
  "type": "object",
  "properties": {
    "timestamp": { "type": "string" },
    "cli_version": { "type": "string" },
    "execution_mode": { "type": "string" },
    "status": { "type": "string", "enum": ["success", "failed"] },
    "steps": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "step_name": { "type": "string" },
          "status": { "type": "string" },
          "duration_ms": { "type": "integer" },
          "outputs": { "type": "object" },
          "error": { "type": "string" }
        },
        "required": ["step_name", "status", "duration_ms"]
      }
    },
    "metrics": {
      "type": "object",
      "properties": {
        "raw_bytes": { "type": "integer" },
        "compressed_bytes": { "type": "integer" },
        "compression_ratio": { "type": "number" },
        "leak_check_passed": { "type": "boolean" }
      }
    }
  },
  "required": ["timestamp", "cli_version", "status", "steps"]
}
```

## 11. Report/Notebook Export Strategy
Reports and documentation packages generated by the pipeline are exported as standalone, self-contained artifacts. The `notebook bundle` subcommand combines:
- Cryptographic signatures and package hashes.
- Token-light context renderings.
- Step-by-step metrics and timing statistics.
The exported payload is formatted as a single archive or plain-text document, making it easily readable by down-stream agents or administrative archivists.

## 12. Artifact Model
All files generated by the pipeline are mapped to deterministic output paths under the `artifacts/` folder:
- Compressed packages: `artifacts/spark/*.spkg`
- Build outputs: `artifacts/spark/*.json`
- Renderings: `artifacts/spark/*_render.txt`
- Reports: `reports/pipeline/*.json`

## 13. Context Cache Valve Strategy
To optimize performance in nested agent executions without introducing file concurrency issues, `agy-ct` uses a local context cache valve:
- Cached contexts are stored locally in a configured temp directory inside the project workspace.
- The cache key is derived from the SHA-256 hash of the input `.spkg` and the JSON schema.
- If a cached build output matches the key, the pipeline can skip the build phase and directly execute rendering or validation.
- The cache valve is strictly local and offline.

## 14. Workspace Boundary and Safety Rules
- **Safety Boundaries**: The CLI operates entirely locally. No network operations are run by default. No remote git actions (fetch, pull, commit, push) are executed.
- **No Destructive Action Without Confirmation**: Commands that overwrite existing packages or clean historical records must require confirmation unless `--non-interactive` or `--force` is specified.
- **Directory Traversal**: All operations are restricted to the local repository directory. No parent or sibling directory scanning is performed.
- **Secrets Isolation**: Under no circumstances should access tokens, user secrets, or private environment variables be logged or stored inside exported JSON reports.

## 15. Dependency Recommendations
- **Near-term Evaluation (Approved for blueprint only, no installation in this phase)**:
  - `clap`: For robust command-line argument parsing and nested subcommand trees.
  - `serde_json`: For structured JSON input/output and report generation.
  - `sha2`: For local SHA-256 hashing.
  - `anstream` & `anstyle`: For clean, style-aware terminal output handling.
  - `anyhow` or `miette`: For rich error reporting.
  - `thiserror`: For defining structured library error enumerations.
  - `assert_cmd` & `predicates`: For black-box integration tests of CLI binaries.
  - `insta`: For snapshot testing of rendered stdout outputs.
- **Later Evaluation Options**:
  - `indicatif` (progress bars), `dialoguer` (interactive prompts), `dag_exec` (pipeline graph runner), `asupersync` (async IO synchronization), `wasm_sandbox` / `wasmtime` (secure plugin runtime), `tokio` (asynchronous runtime), `ratatui` (TUI terminal layouts).

## 16. Safety, Leak & Claim Hygiene Guidelines
- **Wording Rules**:
  - Do NOT say "fully deterministic".
  - Do NOT say "100% safe".
  - Do NOT say "no risks".
  - Do NOT claim official SPARK compatibility.
  - Do NOT make EU AI Act compliance claims.
- **Required Claims Wording**:
  - "Offline behavior was deterministic in the validated test scope."
  - "Configured leak checks passed in the validated scope."
  - "No blocking risks found in the validated scope."

## 17. Phased Implementation Plan
- **Phase 6A**: Architecture blueprint and handbook definition (Current Phase).
- **Phase 6B**: CLI binary initialization, setup of clap command parser tree, and validation tests.
- **Phase 6C**: Implementation of `doctor`, `validate`, and `handoff` compatibility commands.
- **Phase 6D**: Implementation of `package` and `context` orchestration wrappers.
- **Phase 6E**: Implementation of `agy-ct run` step executor and JSON reporting.
- **Phase 6F**: Pre-release verification and final rollup snapshot of the `agy-ct` tool.
