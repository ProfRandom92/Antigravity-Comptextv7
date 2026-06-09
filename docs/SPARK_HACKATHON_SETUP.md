# SPARK Hackathon Setup and Governance Guidelines

This document outlines the local setup, operating boundaries, and validation guidelines for compiling and testing the `CompText-Sparkctl` codebase within the BMDS SPARK Hackathon alignment context.

## 1. Purpose

The purpose of this alignment setup is to ensure that development work is conducted in a verifiable, offline, and review-safe local sandbox. This setup coordinates planning context verification and trace metadata packaging without introducing external network side-effects or autonomous governance risks.

## 2. Sandbox Clone Instructions (Windows PowerShell)

To initialize and setup the sandbox workspace locally on Windows, execute the following commands in PowerShell:

```powershell
# Navigate to your local development workspace directory
cd "$HOME\development"

# Clone the repository locally
git clone https://github.com/ProfRandom92/comptext-sparkctl.git
cd comptext-sparkctl

# Verify that the local directory structure matches the expected repository map
ls
```

## 3. Antigravity Launch Steps

When launching an Antigravity agent session inside the clone:
1. Ensure the workspace settings are loaded from `.antigravitycli/settings.example.json`.
2. Write any local environment mappings exclusively to `.antigravitycli/workspace_config.json` (this file is git-ignored).
3. Confirm that the agent remains in **read-only git mode** and **offline mode**.

## 4. Command Boundaries

All command execution must strictly respect the repository policy limits:

### Safe Commands
These local validation commands can be run at any time inside the `agy7rust/` subdirectory to check code syntax and correctness:
* `cd agy7rust`
* `cargo fmt --all --check`
* `cargo check`
* `cargo test`
* `cargo clippy -- -D warnings`
* `cargo run --bin agy-ct -- --help`
* `cargo run --bin sparkctl -- --help`

### Review-Required Commands (Artifact/Evidence Generation)
These commands modify the local filesystem, generate planning context models, or compile run summaries. They require explicit human approval and should not be run automatically:
* `cargo run --bin agy-ct -- doctor` / `cargo run --bin sparkctl -- doctor`
* `cargo run --bin agy-ct -- validate` / `cargo run --bin sparkctl -- rust-validate`
* `cargo run --bin agy-ct -- context all` / `cargo run --bin sparkctl -- context-all`
* `cargo run --bin agy-ct -- demo` / `cargo run --bin sparkctl -- spark-demo`
* `cargo run --bin agy-ct -- handoff` / `cargo run --bin sparkctl -- handoff-check`
* `cargo run --bin sparkctl -- spark-evidence-demo --output <path>`
* `cargo run --bin sparkctl -- spark-evidence-validate --input <path>`

### Denied Commands
The following operations are strictly forbidden in this workspace and are blocked by hooks:
* `cargo run --bin agy-ct -- run` (restricted to prevent untracked report modifications)
* `cargo run --bin agy-ct -- benchmark` (restricted to prevent performance baseline drift)
* Any git mutating commands (`git commit`, `git push`, `git pull`, `git fetch`, `git merge`, `git rebase`, `git tag`)
* Any GitHub CLI write commands (`gh pr`, `gh issue`, `gh release`)
* Environment dumps (`env`, `printenv`, `Get-ChildItem env:`)
* Reading secrets or keys (`.env`, credentials, SSH files)
* Web server deployments (`vercel`, `netlify`, `wrangler deploy`, etc.)

## 5. Validation Plan

To validate the code offline, run the safe local check suite:
1. Run `cargo fmt --all --check` to verify code style formatting.
2. Run `cargo check` to verify compilation.
3. Run `cargo test` to execute the integration and unit tests.
4. Run `cargo clippy -- -D warnings` to enforce linter compliance.

Do not run `agy-ct run`, `agy-ct demo`, or `agy-ct benchmark` unless the final reports and artifacts are specifically requested for human review.

## 6. Synthetic-Data-Only Rule

All planning modules, extractors, and validators operate exclusively against static mock datasets (e.g. `examples/spark/extraction.json`). Processing of real citizen, administrator, or live public-sector cases is strictly prohibited.

## 7. Safety Non-Claims & Technology Exclusions

The `CompText-Sparkctl` workflow enforces the following non-claims and exclusions:
* **No Official SPARK Schema Compatibility:** All SPARK-oriented terminology is conceptual mock-up and synthetic schema alignment.
* **No EU AI Act Compliance Certification:** The toolkit serves as design support for record-keeping patterns only; it does not constitute certification or compliance.
* **No Legal or Judicial Proof:** Generated evidence packages do not replace official review, and have no forensic or judicial admissibility.
* **No Autonomous Approval:** System outputs are strictly untrusted proposals. All planning workflows terminate at a mandatory human-in-the-loop review boundary.
* **No Repair or Forensic Recovery:** The tool does not perform autonomous data recovery, error correction, or assert forensic certainty.
* **Legacy Exclusions:** Domain-specific concepts like XENTRY/OBD log X-Engines, consonant signatures, sparse micro-frame synopses, and the four-layer sandwich log parser are future/legacy design prototypes only and are not part of the active SPARK alignment claims.

