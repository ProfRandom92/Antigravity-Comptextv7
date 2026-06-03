# Phase 3 Plan — SPARK Operational Context Layer

- **Phase:** Phase 3 Planning - SPARK Operational Context Layer
- **Status:** Planning Completed
- **Skills Used:**
  - `AGENTS.md` (General protocol entrypoint)
  - `.agent/skills/00_project_system.md` (Boundary management)
  - `.agent/skills/01_phase_gate.md` (Validation lifecycle)
  - `.agent/skills/04_spark_context_layer.md` (Context layer specifications)
  - `.agent/skills/05_claim_hygiene.md` (Security/regulatory claims hygiene)

---

## 1. Purpose
The SPARK Operational Context Layer acts as a mechanism to represent the prior task state, constraints, dependency boundaries, and validation properties of a planning or approval extractions process in a highly compact, deterministic, and replay-safe representation. It bridges the gap between raw package validation (Phase 1 & 2) and LLM-readable, token-light prompts.

---

## 2. Scope
- **Replay-Safe Operational Context:** The layer represents only metadata that governs process continuation (order, dependencies, errors, recovery paths, schema indicators).
- **Offline Validation:** All checks, hashes, and parsing happen locally in memory.
- **No Orchestration:** The layer does not initiate tool execution, run actions, or handle agents loops.
- **No External Integrations:** No semantic scoring, LLM judges, vector databases, RAG, or remote connections.
- **No MCP Server:** The codebase is designed as a library, not an active network server.

---

## 3. Non-Scope
- Active runner capabilities (task execution).
- Outer API bindings (LiteLLM, LangChain, etc.).
- Semantic matching or vector retrieval.
- Regulatory EU AI Act compliance checks (Technical Art.-12-oriented record-keeping patterns only).
- Official SPARK system compatibility.

---

## 4. Planned CLI Commands
- `context-build -p <package.spkg> -s <schema.json> -o <context.json>`
  Reads a valid `.spkg` package and a Phase 2 `.json` schema file and generates a structured, deterministic `context.json` outlining anchor validation, constraint status, and dependency mappings.
- `context-render -i <context.json> -o <context_render.txt>`
  Consumes a context JSON file and renders a token-optimized, human-readable prompt segment. This segment represents the progress state, dependencies, and recovery actions without leaking raw applicant details.
- `context-validate -i <context.json>`
  Evaluates the structure of the context file. It verifies that required schema anchors are present, dependency relationships conform to correct ordering, recovery protocols are defined, non-claims are registered, and the parent package hash matches.

---

## 5. Planned Context Model (JSON Schema)
The generated context file will contain:
- `context_id`: A deterministic string identifier (derived from hashes).
- `source_package_hash`: The SHA-256 hash of the parent package.
- `schema_name`: Name from the validating schema.
- `schema_version`: Version from the validating schema.
- `required_field_paths`: Paths designated by the schema.
- `satisfied_field_paths`: Required paths containing valid scalars.
- `missing_field_paths`: Required paths missing or empty.
- `constraints`: Active rules representing limits or parameters.
- `required_order`: Sequence rules mapping dependencies.
- `dependency_edges`: Explicit parent-child directed paths.
- `blockers`: Active blocks preventing task advancement.
- `recovery_paths`: Documented fallback plans.
- `validation`: Final validation result (true/false) plus error labels.
- `non_claims`: Clear declarations of mock/synthetic boundaries.

---

## 6. Planned Artifacts
- `artifacts/spark/context.json`
- `artifacts/spark/context_render.txt`
- `PHASE3_CONTEXT_LAYER_SNAPSHOT.md` (to be created at sandbox root after implementation)

---

## 7. Leak Rules
The rendered text output from `context-render` must NEVER output the following raw details:
- Full raw extraction JSON.
- Raw applicant value (e.g., `"Nordwind Energie GmbH"`).
- Raw decision recommendation string (e.g., `"Zustimmung mit Nebenbestimmungen..."`).
- Text contents of the `extraction.notes` property.
- Source PDF file paths or binary hashes.
- Raw payload payload bytes or original keys.

---

## 8. Determinism Rules
- Sort all object keys alphabetically (canonical JSON formatting) before hashing.
- Sequence all field path lists, commitment strings, and error arrays deterministically.
- Exclude timestamps, execution runtimes, UUID generators, and random values.
- Exclude machine-dependent local file system paths.

---

## 9. Rust Module Plan
We plan to introduce the following modules inside `agy7rust`:
- `agy7rust/src/codec/context.rs`: Context model serialization structures.
- `agy7rust/src/commands/context_build.rs`: Code for CLI building logic.
- `agy7rust/src/commands/context_render.rs`: Text generation code with leak protection.
- `agy7rust/src/commands/context_validate.rs`: Command to test context files.
- `agy7rust/src/context/mod.rs`: Registry exposing submodules.
- `agy7rust/src/context/model.rs`: Core structures (Context, Constraints, Edge).
- `agy7rust/src/context/render.rs`: Engine implementing token-light rendering.
- `agy7rust/src/context/validate.rs`: Evaluation rules (validating edges, hashes, anchors).

---

## 10. Test Plan
Unit and integration tests will verify the following scenarios:
1. **Successful context-build:** Creates valid context JSON from a valid package and schema.
2. **Stable context-build:** Repeated executions yield byte-for-byte identical context JSON files.
3. **Leak protection:** Confirms that rendered context strings contain no applicant name, decision recommendation, or extraction notes.
4. **Validation success:** Confirms that a complete, correct context file verifies successfully.
5. **Anchor failure:** Validates that missing required field anchors cause validation failure.
6. **Dependency failure:** Validates that out-of-order execution traces break dependency edge assertions.
7. **Recovery path failure:** Validates that missing fallback protocols trigger errors.
8. **Non-claim failure:** Validates that missing legal/forensic warnings trigger errors.
9. **Hash mismatch failure:** Validates that a tampered parent package hash fails validation.

---

## 11. Acceptance Criteria
- Code matches formatting guidelines (`cargo fmt`).
- Code builds cleanly (`cargo check`).
- All tests pass (`cargo test`).
- Lints pass warnings-as-errors checks (`cargo clippy -- -D warnings`).
- Phase 1 & 2 commands remain fully functional.
- Context outputs are deterministic and leak-safe.

---

## 12. Non-Claims
- **No MCP Capability:** The library is a CLI tool, not an active Model Context Protocol network host.
- **Not an Official SPARK Engine:** Built for synthetic evaluation, not official BMDS workflows.
- **No Legal completeness check:** Does not approve, assess, or audit planning applications legally.
- **No EU AI Act Compliance Certification:** Design patterns conform only to technical Art.-12-oriented record-keeping guidelines.
- **No Autonomous Execution:** Does not run tool loops or execute LLM decisions.
- **Deterministic Replay Only:** The tool does not perform live inferences, real-time predictions, or agent workflow runs; it performs deterministic validation of structured JSON traces and commitments.

---

## 13. Risks
- **Parsing Complexity:** Representing directed execution graphs (dependency edges) in a simplified flat JSON structure could increase error rates if schemas grow excessively complex.
- **Maintenance Overhead:** The separation between schema validation (Phase 2) and context validation (Phase 3) requires keeping duplicate path references aligned.

---

## 14. Next Gate
- Await user approval before starting Phase 3 implementation.
