# Antigravity-Comptextv7 Unified RC1

Interner Release-Kandidat, der den Antigravity-Bridge-Stand mit dem vollständigen `Comptextv7`-Core in **einer ZIP** bündelt.

## Enthalten

```text
core/                 Antigravity Bridge/Sandbox-Integration
tests/                Antigravity- und LMCache-Holdout-Prüfungen
datasets/             Holdout- und Replay-Daten
artifacts/            bestehende Ergebnisartefakte
reports/              menschenlesbare Auswertungen
benchmarks/           Audit-/Ledger-Daten
Comptextv7/           eingebetteter vollständiger Comptextv7-Main-Core
scripts/              zentrale Run-Scripts
docs/                 Original-README und Zusatzdokumente
```

## Schnellstart am PC

```bash
unzip Antigravity-Comptextv7-unified-rc1.zip
cd Antigravity-Comptextv7-unified
python -m venv .venv
source .venv/bin/activate  # Windows: .venv\Scripts\activate
pip install -U pip pytest
./scripts/run_all.sh
```

## Einzeltests

```bash
./scripts/run_antigravity_holdout.sh
./scripts/run_lmcache_replay.sh
```

## Aktueller Status

- Die ZIP ist als **interner Arbeits-/RC-Stand** gedacht.
- `Comptextv7/` ist bewusst eingebettet, damit Antigravity nicht gegen einen leeren oder veralteten Core läuft.
- Harte Termux-Pfade wurden auf projektrelative Pfade umgestellt.
- Ein minimaler `comptext_v7.mcp`-Adapter wurde ergänzt, damit die Antigravity-Sandbox gegen den aktuellen Core importierbar bleibt.

## Nicht als externer Release senden

Vor externer Weitergabe noch am PC prüfen:

```bash
./scripts/run_all.sh
git diff --stat
```

Danach README/Reports finalisieren und erst dann eine externe Release-ZIP bauen.
