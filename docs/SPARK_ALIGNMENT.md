# SPARK Alignment

## Positioning

SPARK stands for "Schnellere Planung und Realisierung durch KI". In this repository, `sparkctl` is treated as the local artifact and evidence-package layer for CompText-style review workflows.

CompText is a deterministic Context Pack / proposal-gated evidence workflow:

Source / GitHub URL -> Goal -> Inspect -> Context Pack -> Policy Gate -> Provider Boundary -> Untrusted Proposal -> Human Review -> Artifacts

## SPARK Evidence Packet v1

The SPARK Evidence Packet v1 demo records a reviewable trail with:

- a review goal
- source and context summaries
- policy result
- provider-boundary status
- untrusted proposal text
- human review decision
- claim hygiene
- artifact manifest
- canonical JSON derived from the packet preimage
- SHA-256 hash computed from that canonical JSON

## Boundaries

The packet does not replace human review. Provider output remains untrusted until reviewed, and the goal does not bypass the Policy Gate or authorize auto-apply behavior. All planning workflows terminate at a mandatory human-in-the-loop review boundary.

The demo is local, offline, and bounded. It makes no provider calls and operates exclusively against synthetic/mock fixtures. It does not process real citizen or administrative case data.

We explicitly do **not** claim:
- production readiness or enterprise setup deployment
- regulatory compliance certification (such as the EU AI Act)
- legal or judicial proof admissibility
- official SPARK schema compatibility or compliance
- autonomous administrative decisions or approval
- data repair, recovery, or forensic certainty helpers

Legacy / future concepts such as XENTRY/OBD log sandwich models, consonant signature mapping, sparse micro-frame synopsis, and error correction layers are completely excluded from the active SPARK alignment scope.

## Local Development Checkpoint

This checkpoint records the current local development state. Current checkpoint is local-only; push/PR/release are outside this checkpoint.

### 1. Active Settings & Path Configurations
* **Active Agent Skills:** `.agents/skills` is established as the active Antigravity skill path for local workspace coordination.
* **Legacy Compatibility:** The `.agent/skills` directory remains as legacy/compatibility metadata only.

### 2. CLI Command Wiring
The following `agy-ct package` subcommands are fully wired and functional:
* **`agy-ct package verify`** — wired to `verify_cmd::run`
* **`agy-ct package replay`** — wired to `replay_cmd::run`
* **`agy-ct package inspect`** — wired to `inspect::run`

### 3. Replay UX Stream Separation
The `package replay` command implements a strict separation of output streams:
* `stdout` is reserved exclusively for the replayed machine-readable JSON trace.
* `stderr` outputs status messages, progress updates, and validation warnings.
* Global CLI flags `--quiet` (suppresses status output), `--plain` (strips ANSI escapes), and `--no-color` (disables terminal color codes) are explicitly supported.

### 4. Test and Placeholders Status
* **Validation Status:** The local Rust test suite executes successfully with **65 PASS** tests.
* **Remaining Placeholders:** The subcommands `package compress`, `package adversarial`, `context build`, `context render`, `context validate`, and `schema check` remain as placeholders.

### 5. Exclusion and Alignment Boundaries
* **XENTRY/OBD/X-Engine:** Legacy/future diagnostic components (including OBD-II logs and X-Engine sandwich parsers) remain completely excluded from the active scope.
* **Synthetic-Only:** All testing and demonstration data remains strictly synthetic.
* **Human-Review:** Mandated human-in-the-loop review boundaries remain enforced; provider output is untrusted.
* **Non-Certified:** No certifications, official compliance statements, or production/forensic readiness claims are asserted.

