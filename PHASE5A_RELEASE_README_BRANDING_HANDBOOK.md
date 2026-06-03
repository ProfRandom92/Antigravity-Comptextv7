# Phase 5A Release README & Branding Handbook

## 1. Objective
Define the plan for structured release-facing documentation, README enhancements, and branding asset planning for the completed `sparkctl` Command Layer.

## 2. Strict Scope
- Focus exclusively on defining the layout, sections, guidelines, and branding files for the upcoming Phase 5B implementation.
- Establish precise boundaries, safety wording, and claim hygiene.
- Create this handbook only; no modification of source code, testing configurations, or production files is authorized.

## 3. Forbidden Scope
- Do not modify `README.md` or any existing Markdown document in this phase.
- Do not modify Rust source code (`.rs` files) or dependencies (`Cargo.toml`).
- Do not modify tests or validation suites.
- Do not modify schemas, assets, or GitHub Actions workflows.
- Do not create image assets.
- Do not perform git actions (commit, push, pull, fetch, add remote).
- Do not use the network.

## 4. Release-Facing README Goals
The README update in Phase 5B must:
- Clearly communicate the product name: `sparkctl`.
- State the repository and project identity: `Antigravity-CompText v7 / SPARK Context Layer`.
- Provide a concise one-paragraph purpose summarizing `sparkctl` as the operations controller for the SPARK Context Layer.
- Highlight the completed command surface:
  - `sparkctl doctor`: Diagnostic tool for local workspace verification.
  - `sparkctl rust-validate`: Automated rust codebase formatting, checking, testing, and lint checking.
  - `sparkctl context-all`: Pipeline execution runner for building, rendering, and validating contexts.
  - `sparkctl spark-demo`: Demonstrates the full compression and context pipeline round-trip.
  - `sparkctl handoff-check`: Pre-handoff verification utility ensuring repository readiness.
- Include quickstart instructions executing from the `agy7rust` directory:
  - `cargo run --bin sparkctl -- doctor`
  - `cargo run --bin sparkctl -- rust-validate`
  - `cargo run --bin sparkctl -- context-all`
  - `cargo run --bin sparkctl -- spark-demo`
  - `cargo run --bin sparkctl -- handoff-check`
- Document key artifact outputs:
  - `artifacts/spark/extraction.spkg`
  - `artifacts/spark/context.json`
  - `artifacts/spark/context_render.txt`
- Explicitly state boundaries:
  - The CLI functions as a local/offline toolchain within its validated scope.
  - The toolchain does not verify remote CI or GitHub Actions environments.
  - No regulatory compliance claims are made.
  - No official SPARK compatibility claim is made.

## 5. Branding / Logo Integration Goals & Plan
- **Recommended Directory:** `assets/branding/`
- **Proposed Logo Filenames:**
  - `assets/branding/sparkctl-logo.png`
  - `assets/branding/sparkctl-logo-square.png`
  - `assets/branding/sparkctl-wallpaper.png`
- **README Logo Placement:** Top of README, centered or near the main title.
- **Alt Text:** `sparkctl logo`
- **Asset Approvals:**
  - Asset creation, image generation, and import require separate explicit approval.
  - Existing local logo files or generated images must be explicitly approved by the USER before copying into the repository workspace.

## 6. Safety Wording & Claim Hygiene Rules
To prevent unsupported statements:
- **Forbidden phrases:**
  - Do NOT use the term "fully deterministic".
  - Do NOT use the term "100% safe".
  - Do NOT use the term "no risks".
  - Do NOT make EU AI Act compliance claims.
  - Do NOT claim official SPARK compatibility.
- **Approved Wording:**
  - Use: `"Offline behavior was deterministic in the validated test scope."`
  - Use: `"Configured leak checks passed in the validated scope."`
  - Use: `"No blocking risks found in the validated scope."`

## 7. Local Validation Checklist
During Phase 5B:
1. Verify that `README.md` updates strictly document the approved CLI command surface and Quickstart directions.
2. Confirm no code or cargo configurations are modified.
3. Validate that safety wording rules are fully applied in the updated README.
4. Verify that logo alt text and asset image links match the approved paths.

## 8. Stop Conditions
- Stop immediately and reject changes if any modifications to Rust code, `Cargo.toml`, build scripts, or schemas are detected.
- Stop if any network access, remote branch fetch, or remote verification is attempted.

## 9. Implementation Return Format
The next phase (5B implementation) must return exactly:
```
PHASE: Phase 5B README and branding implementation
STATUS: success | blocked
COMMANDS_RUN:
- ...
FILES_MODIFIED:
- ...
FILES_CREATED:
- ...
BRANDING_ASSETS_ADDED:
- ...
SAFETY_WORDING_APPLIED:
- ...
RISKS:
- ...
NEXT:
- Commit and push Phase 5B README and branding implementation only after approval
```
