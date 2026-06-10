# Sparkctl Projektsteuerungsdatei

## 1. Zweck
- Diese Datei dient als lokale Steuerungsdatei für die Sparkctl-Entwicklungsarbeiten mit Antigravity.
- Sie ist kein Ersatz für `AGENTS.md`.
- `AGENTS.md` und die Skills unter `.agents/skills/**/SKILL.md` bleiben die vorrangig verbindlichen Arbeitsregeln.
- Diese Datei konkretisiert und priorisiert lediglich den aktuellen lokalen Backlog für diese Arbeitsumgebung.

---

## 2. Aktueller lokaler Stand
- **Branch:** `docs/spark-hackathon-governance`
- **Letzte relevante Commits:**
  - `dcfbd02` docs: sync alignment after context render
  - `d00132b` feat: wire context render command
- **Tests letzter Stand:** 69 PASS laut lokalem Audit/Teststand.

---

## 3. Wired Commands
Folgende Befehle des CLI `agy-ct` sind vollständig verdrahtet:
- `agy-ct package verify`
- `agy-ct package replay`
- `agy-ct package inspect`
- `agy-ct schema check`
- `agy-ct context validate`
- `agy-ct context build`
- `agy-ct context render`

---

## 4. Remaining Placeholders
Folgende Befehle des CLI `agy-ct` sind noch als Platzhalter deklariert:
- `agy-ct package compress`
- `agy-ct package adversarial`
- `agy-ct report export`
- `agy-ct notebook bundle`

---

## 5. Arbeitsmodus
- Lokale Arbeit ausschließlich innerhalb des Workspace-Verzeichnisses.
- Standardmäßiger Containment-Modus: `proceed-in-sandbox`.
- Vor Feature-Änderungen wird eine Read-only/Explore- oder Plan-only-Phase durchgeführt, falls der genaue Task-Kontext unklar ist.
- Bei klar definierten Backlog-Tasks darf lokal implementiert, validiert und lokal committet werden.
- **Kein Push.**
- **Keine PR.**
- **Kein Release.**

---

## 6. Erlaubte lokale Autonomie
- Durchführung von Read-only Audits im Workspace.
- Änderungen sind nur an Dateien erlaubt, die explizit im Scope der aktiven Backlog-Task aufgeführt sind.
- Ausführung der Rust-Validierung im Verzeichnis `agy7rust/`:
  - `cargo fmt --all --check`
  - `cargo check`
  - `cargo test`
  - `cargo clippy -- -D warnings`
- Nach Durchführung von `cargo test` wird `git status --short` auf gerufene Test-Seiteneffekte geprüft.
- Falls `reports/performance_baseline.json` geändert wurde, wird diese Datei sofort via `git restore reports/performance_baseline.json` zurückgesetzt.
- Erstellung lokaler Commits pro erfolgreich abgeschlossener und validierter Task.

---

## 7. Verboten ohne explizite menschliche Freigabe
- `git push`
- PR (Pull Request) erstellen
- Release-Aktionen ausführen
- Deploy-Vorgänge starten
- Branch veröffentlichen (Branch publish)
- Remote-Synchronisierung (Remote sync)
- `git pull` / `git fetch`
- Plugins modifizieren
- Hooks modifizieren
- Skills modifizieren
- `AGENTS.md` modifizieren
- Globale Systemeinstellungen oder User-Home-Konfigurationen ändern
- Secrets oder Umgebungsvariablen exportieren/dumpen
- Ordner `reports/` oder `artifacts/` committen (Ausnahme: genehmigte Dokumentenaktualisierungen)
- Reale Behörden-, Personen- oder Falldaten einführen
- Offizielle SPARK-/BMDS-/Compliance-/Forensic-/Legal-/EU-AI-Act-Claims hinzufügen

---

## 8. Commit-Regeln
- Lokale Commits sind erlaubt, wenn:
  - Die Task vollständig abgeschlossen ist.
  - Alle Validierungsschritte (Formatierung, Check, Linter, Tests) fehlerfrei durchlaufen wurden.
  - `git status --short` ausschließlich die erlaubten Scope-Dateien als verändert anzeigt.
  - Modifikationen an `reports/performance_baseline.json` zurückgesetzt wurden.
  - Keine Berichts- oder Artefaktdateien im Commit-Set enthalten sind.
- Commit-Messages müssen task-spezifisch sein. Zulässige Formate:
  - `feat: wire package compress command`
  - `docs: sync alignment after package compress`
  - `feat: wire package adversarial command`
  - `docs: sync alignment after package adversarial`
- **Kein Push, keine PR.**

---

## 9. Backlog

- [ ] **Task 01: agy-ct package compress verdrahten und testen**
  - **Scope:**
    - `agy7rust/src/bin/agy_ct.rs`
    - `agy7rust/tests/spark_roundtrip.rs`
  - **Akzeptanzkriterien:**
    - `PackageCommands::Compress` routet auf `compress::run`.
    - Ein Success-Test läuft vollständig im Temp-Verzeichnis ab.
    - Die Ausgabedatei `.spkg` existiert und ist nicht leer.
    - Fehlverhalten bei fehlenden oder beschädigten Eingaben wird abgefangen und liefert einen Fehler-Exit-Code.
    - Keine Erzeugung von Berichten oder dauerhaften Artefakten.
    - Keine echten Daten.
    - Rust-Güteprüfungen (`cargo fmt/check/test/clippy`) laufen fehlerfrei durch.
    - Lokaler Commit wird erstellt.

- [ ] **Task 02: docs/SPARK_ALIGNMENT.md nach package compress synchronisieren**
  - **Scope:**
    - `docs/SPARK_ALIGNMENT.md`
  - **Akzeptanzkriterien:**
    - `package compress` wird als "wired and functional" gelistet.
    - Die verbleibenden Platzhalter werden aktualisiert.
    - Der Teststatus wird auf den neuesten PASS-Wert aktualisiert.
    - Keine neuen Claims.
    - Lokaler Commit wird erstellt.

- [ ] **Task 03: agy-ct package adversarial verdrahten und testen**
  - **Scope:**
    - `agy7rust/src/bin/agy_ct.rs`
    - `agy7rust/tests/spark_roundtrip.rs`
  - **Akzeptanzkriterien:**
    - `PackageCommands::Adversarial` routet auf `adversarial::run`.
    - CLI-Integrationstest läuft isoliert im Temp-Verzeichnis.
    - Führt 5 simulierte Manipulationsprüfungen auf manipulierten `.spkg` Paketen aus und verifiziert den fehlerhaften Zustand.
    - Keine Erzeugung von Berichten oder dauerhaften Artefakten.
    - Keine echten Daten.
    - Rust-Güteprüfungen laufen fehlerfrei durch.
    - Lokaler Commit wird erstellt.

- [ ] **Task 04: docs/SPARK_ALIGNMENT.md nach package adversarial synchronisieren**
  - **Scope:**
    - `docs/SPARK_ALIGNMENT.md`
  - **Akzeptanzkriterien:**
    - `package adversarial` wird als "wired and functional" gelistet.
    - Die verbleibenden Platzhalter werden aktualisiert.
    - Keine neuen Claims.
    - Lokaler Commit wird erstellt.

- [ ] **Task 05: report export read-only analysieren**
  - **Scope:** Read-Only
  - **Akzeptanzkriterien:**
    - Realer CLI-Status geklärt.
    - Vorhandene Backend-Logik unter `agy7rust` ermittelt.
    - Für die Verdrahtung benötigte Dateien gelistet.
    - Keine Code- oder Dokumentationsänderungen vorgenommen.

- [ ] **Task 06: notebook bundle read-only analysieren**
  - **Scope:** Read-Only
  - **Akzeptanzkriterien:**
    - Realer CLI-Status geklärt.
    - Vorhandene Backend-Logik unter `agy7rust` ermittelt.
    - Für die Verdrahtung benötigte Dateien gelistet.
    - Keine Code- oder Dokumentationsänderungen vorgenommen.

- [ ] **Task 07: Final Local Audit**
  - **Scope:** Read-Only
  - **Akzeptanzkriterien:**
    - Workspace ist sauber.
    - Alle wired/placeholder Befehle stimmen mit Code und Dokumenten überein.
    - `README.md` und `docs/SPARK_ALIGNMENT.md` sind synchron.
    - Berichte und Artefakte sind unverändert.
    - Eventuelle verbleibende Remote/PR-Risiken sind erfasst.

- [ ] **Task 08: Remote/PR-Strategie abstimmen**
  - **Scope:** Read-Only bis zur Freigabe.
  - **Akzeptanzkriterien:**
    - Offene PRs berücksichtigt.
    - Kein Push oder PR ohne explizite Freigabe.

---

## 10. Nächster Task
- **Next:** `Task 01 — agy-ct package compress verdrahten und testen`.
- **Vor Start von Task 01:** Read-only Prüfung, ob die Signatur von `compress::run` und der Input/Output-Kontrakt stabil sind.

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
