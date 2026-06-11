# Sparkctl Projektsteuerungsdatei

## 1. Zweck
- Diese Datei dient als lokale Steuerungsdatei für die Sparkctl-Entwicklungsarbeiten mit Antigravity.
- Sie ist kein Ersatz für [AGENTS.md](file:///C:/Users/contr/sandbox/comptext-antigravity-work/comptext-sparkctl/AGENTS.md).
- [AGENTS.md](file:///C:/Users/contr/sandbox/comptext-antigravity-work/comptext-sparkctl/AGENTS.md) und die Skills unter [00_project_system.md](file:///C:/Users/contr/sandbox/comptext-antigravity-work/comptext-sparkctl/.agent/skills/00_project_system.md) bleiben die vorrangig verbindlichen Arbeitsregeln.
- Diese Datei konkretisiert und priorisiert lediglich den aktuellen lokalen Backlog für diese Arbeitsumgebung.

---

## 2. Aktueller lokaler Stand
- **Branch:** `docs/project-governance-sync`
- **Letzte relevante Commits:**
  - `c9b9086` docs: polish README presentation
  - `b9191af` feat: complete Sparkctl CLI command surface
- **Tests letzter Stand:** 73 PASS dokumentiert laut lokalem Audit/Teststand in [SPARK_ALIGNMENT.md](file:///C:/Users/contr/sandbox/comptext-antigravity-work/comptext-sparkctl/docs/SPARK_ALIGNMENT.md).

---

## 3. Wired Commands
Folgende 11 Befehle des CLI `agy-ct` sind vollständig verdrahtet:
- `agy-ct package verify` — wired to `verify_cmd::run`
- `agy-ct package replay` — wired to `replay_cmd::run`
- `agy-ct package inspect` — wired to `inspect::run`
- `agy-ct package compress` — wired to `compress::run`
- `agy-ct package adversarial` — wired to `adversarial::run`
- `agy-ct report export` — wired to `report_export::run`
- `agy-ct notebook bundle` — wired to `notebook_bundle::run`
- `agy-ct schema check` — wired to `schema_check::run`
- `agy-ct context validate` — wired to `context_validate::run`
- `agy-ct context build` — wired to `context_build::run`
- `agy-ct context render` — wired to `context_render::run`

---

## 4. Remaining Placeholders
Folgende Befehle des CLI `agy-ct` sind noch als Platzhalter deklariert:
- Keine bekannten CLI-Platzhalter mehr.

---

## 5. Arbeitsmodus
- Lokale Arbeit ausschließlich innerhalb des Workspace-Verzeichnisses.
- Standardmäßiger Containment-Modus: `proceed-in-sandbox`.
- Vor Feature-Änderungen wird eine Read-only/Explore- oder Plan-only-Phase durchgeführt, falls der genaue Task-Kontext unklar ist.
- Bei klar definierten Backlog-Tasks darf lokal implementiert, validiert und lokal committet werden.
- **Push und PR-Erstellung:** Nur nach expliziter menschlicher Bestätigung / Auth-Abfrage erlaubt.
- **Verboten:** Force-Push, destruktive Git-Aktionen (Reset/Rebase), Deploys, Releases, Auslesen von Secrets.

---

## 6. Erlaubte lokale Autonomie
- Durchführung von Read-only Audits im Workspace.
- Änderungen sind nur an Dateien erlaubt, die explizit im Scope der aktiven Backlog-Task aufgeführt sind.
- Ausführung der Rust-Validierung im Verzeichnis `agy7rust/` (bei Bedarf):
  - `cargo fmt --all --check`
  - `cargo check`
  - `cargo test`
  - `cargo clippy -- -D warnings`
- Nach Durchführung von `cargo test` wird `git status --short` auf gerufene Test-Seiteneffekte geprüft.
- Falls `reports/performance_baseline.json` geändert wurde, wird diese Datei sofort via `git restore reports/performance_baseline.json` zurückgesetzt.
- Erstellung lokaler Commits pro erfolgreich abgeschlossener und validierter Task.

---

## 7. Verboten ohne explizite menschliche Freigabe und Bestätigung/Auth-Abfrage
- `git push` (Erfordert Bestätigung/Auth)
- PR (Pull Request) erstellen/ändern/mergen (Erfordert Bestätigung/Auth)
- Release-Aktionen ausführen
- Deploy-Vorgänge starten
- Branch löschen (`git branch -d` / `-D`)
- Destruktive Git-Verlauf-Operationen (`git reset`, `git rebase`, `git merge`)
- Raw-API-Zugriffe (`gh api`)
- Plugins modifizieren (außer im Rahmen eines genehmigten Entwurfs)
- Hooks modifizieren
- Skills modifizieren
- [AGENTS.md](file:///C:/Users/contr/sandbox/comptext-antigravity-work/comptext-sparkctl/AGENTS.md) modifizieren
- Globale Systemeinstellungen oder User-Home-Konfigurationen ändern
- Secrets oder Umgebungsvariablen exportieren/dumpen
- Ordner `reports/` oder `artifacts/` committen (Ausnahme: genehmigte Dokumentenaktualisierungen)
- Reale Behörden-, Personen- oder Falldaten einführen
- Offizielle SPARK-/BMDS-/Compliance-/Forensic-/Legal-/EU-AI-Act-Claims hinzufügen

---

## 8. Commit-Regeln
- Lokale Commits sind erlaubt, wenn:
  - Die Task vollständig abgeschlossen ist.
  - Alle Validierungsschritte (Formatierung, Check, Linter, Tests) fehlerfrei durchlaufen wurden (falls anwendbar).
  - `git status --short` ausschließlich die erlaubten Scope-Dateien als verändert anzeigt.
  - Modifikationen an `reports/performance_baseline.json` zurückgesetzt wurden.
  - Keine Berichts- oder Artefaktdateien im Commit-Set enthalten sind.
- Commit-Messages müssen task-spezifisch sein. Zulässige Formate:
  - `feat: wire package compress command`
  - `docs: sync Antigravity governance workflow`
- **Push und PR-Erstellung erst nach Freigabe und Auth-Bestätigung.**

---

## 9. Backlog

- [x] **Task 01: agy-ct package compress verdrahten und testen** (Erledigt / Obsolet laut [README.md](file:///C:/Users/contr/sandbox/comptext-antigravity-work/comptext-sparkctl/README.md))
- [x] **Task 02: docs/SPARK_ALIGNMENT.md nach package compress synchronisieren** (Erledigt / Obsolet laut [README.md](file:///C:/Users/contr/sandbox/comptext-antigravity-work/comptext-sparkctl/README.md))
- [x] **Task 03: agy-ct package adversarial verdrahten und testen** (Erledigt / Obsolet laut [README.md](file:///C:/Users/contr/sandbox/comptext-antigravity-work/comptext-sparkctl/README.md))
- [x] **Task 04: docs/SPARK_ALIGNMENT.md nach package adversarial synchronisieren** (Erledigt / Obsolet laut [README.md](file:///C:/Users/contr/sandbox/comptext-antigravity-work/comptext-sparkctl/README.md))
- [x] **Task 05: report export read-only analysieren** (Erledigt / Obsolet laut [README.md](file:///C:/Users/contr/sandbox/comptext-antigravity-work/comptext-sparkctl/README.md))
- [x] **Task 06: notebook bundle read-only analysieren** (Erledigt / Obsolet laut [README.md](file:///C:/Users/contr/sandbox/comptext-antigravity-work/comptext-sparkctl/README.md))

- [ ] **Task 07: Final Local Audit and PR Handoff Review**
  - **Scope:** Read-Only / Documentation
  - **Akzeptanzkriterien:**
    - Workspace ist sauber.
    - Alle wired Befehle stimmen mit Code und Dokumenten überein.
    - [README.md](file:///C:/Users/contr/sandbox/comptext-antigravity-work/comptext-sparkctl/README.md) und [SPARK_ALIGNMENT.md](file:///C:/Users/contr/sandbox/comptext-antigravity-work/comptext-sparkctl/docs/SPARK_ALIGNMENT.md) sind synchron.
    - Berichte und Artefakte sind unverändert.
    - Eventuelle verbleibende Remote/PR-Risiken sind erfasst.

- [ ] **Task 08: Remote/PR-Strategie abstimmen**
  - **Scope:** Read-Only bis zur Freigabe.
  - **Akzeptanzkriterien:**
    - Offene PRs berücksichtigt.
    - Push/PR wird nach Bestätigung / Auth-Freigabe ausgeführt.

---

## 10. Nächster Task
- **Next:** `Task 07 — Final Local Audit and PR Handoff Review`.

---

## 11. Claim-Grenzen
- **Synthetic-only:** Alle Tests laufen auf rein synthetischer Datenbasis ab.
- **Mandatory human review:** Alle Ergebnisse sind unverbindliche Vorschläge. Systementscheidungen ohne menschliche Freigabe sind ausgeschlossen.
- **Non-certified:** Keine Zertifizierungen oder Konformitätsgarantien nach dem EU AI Act oder anderen regulatorischen Rahmenwerken.
- **Kein offizieller SPARK-/BMDS-Status:** Die Software ist ein inoffizieller Prototyp.
- **Keine Produktivfreigabe:** Die Software ist nicht für den produktiven Einsatz bestimmt.
- **Keine Legal-/Forensic-Claims:** Keine rechtsverbindlichen Nachweise oder forensischen Sicherheitsgarantien.
- **Keine Echtdaten:** Keine echten Personen-, Fall- oder Behördendaten.
- **XENTRY/OBD ausgeschlossen:** Eventuelle Diagnose- oder Diagnoseschnittstellenlogs sind nicht Scope des Projekts.
