# Release Notes — v0.1.0-rc1 internal

## Zweck

Erster einheitlicher ZIP-Stand für die weitere PC-Arbeit: Antigravity-Bridge + vollständiger Comptextv7-Core in einem Paket.

## Änderungen gegenüber den gelieferten ZIPs

- `Antigravity-Comptextv7/Comptextv7` war leer; wurde durch den vollständigen `Comptextv7-main`-Inhalt ersetzt.
- Cache-/Build-Artefakte bleiben ausgeschlossen.
- Zentrale Scripts ergänzt:
  - `scripts/run_all.sh`
  - `scripts/run_antigravity_holdout.sh`
  - `scripts/run_lmcache_replay.sh`
- Harte Termux-Pfade in Tests/Dataset-Scripts auf projektrelative Pfade umgestellt.
- `comptext_v7.mcp`-Kompatibilitätsadapter ergänzt.

## Offene PC-Arbeit

- Tests auf sauberem PC ausführen.
- Import-/Package-Layout final entscheiden: eingebetteter Core vs. Submodule/Dependency.
- README auf externe Zielgruppe kürzen.
- Release-Tag erst nach erfolgreichem Testlauf setzen.
