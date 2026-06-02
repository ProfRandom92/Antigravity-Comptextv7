# 🏆 LMCache Replay Integrity - Turnierbericht

Dieser Bericht präsentiert die Ergebnisse der Replay-Integritäts-Vergleichstests für 10 echte LMCache-Traces und 3 synthetische Ergänzungs-Fixtures.

---

## 📊 Ergebnisse: Reale LMCache-Traces (10 Fixtures)

| Metrik / Gruppe | Group A (Raw) | Group B (CompText V7) | Group C (Naive Regex) | Group D (Static Summary) | Group E (Random Noise) |
| :--- | :---: | :---: | :---: | :---: | :---: |
| **Commitment Survival Rate** | 10/10 (100.0%) | **10/10 (100.0%)** | 10/10 (100.0%) | 7/10 (70.0%) | 7.5/10 (75.0%) |
| **Tool Sequence Accuracy** | 10/10 (100.0%) | **9.5/10 (95.0%)** | 10/10 (100.0%) | 7/10 (70.0%) | 7/10 (70.0%) |
| **Tamper Detection Rate** | 0/10 (0.0%) | **10/10 (100.0%)** | 0/10 (0.0%) | 0/10 (0.0%) | 0/10 (0.0%) |
| **Execution Hash Stability** | 10/10 (100.0%) | **10/10 (100.0%)** | 10/10 (100.0%) | 10/10 (100.0%) | 10/10 (100.0%) |
| **Avg. Size (Bytes)** | 2023.9 | **427.0** | 1376.3 | 43.0 | 1029.0 |

---

## 📊 Ergebnisse: Synthetische Supplement-Fixtures (3 Fixtures)

| Metrik / Gruppe | Group A (Raw) | Group B (CompText V7) | Group C (Naive Regex) | Group D (Static Summary) | Group E (Random Noise) |
| :--- | :---: | :---: | :---: | :---: | :---: |
| **Commitment Survival Rate** | 3/3 (100.0%) | **3/3 (100.0%)** | 3/3 (100.0%) | 0/3 (0.0%) | 0.33/3 (11.1%) |
| **Tool Sequence Accuracy** | 3/3 (100.0%) | **3/3 (100.0%)** | 3/3 (100.0%) | 0/3 (0.0%) | 0/3 (0.0%) |
| **Tamper Detection Rate** | 0/3 (0.0%) | **3/3 (100.0%)** | 0/3 (0.0%) | 0/3 (0.0%) | 0/3 (0.0%) |
| **Execution Hash Stability** | 3/3 (100.0%) | **3/3 (100.0%)** | 3/3 (100.0%) | 3/3 (100.0%) | 3/3 (100.0%) |
| **Avg. Size (Bytes)** | 280.3 | **387.0** | 149.0 | 43.0 | 140.3 |

---

## 🔬 Wissenschaftliche Zusammenfassung & Analyse

* **Ergebnis von CompText V7 (Gruppe B):**
  * **Datenreduktion:** Komprimiert die realen Trace-Payloads um ca. **79%** (von durchschnittlich 2023.9 Bytes auf 427.0 Bytes).
  * **Genauigkeit:** Erreicht bei realen Traces eine 100%ige Erhaltung der operativen Commitments (10/10) und eine 95%ige Genauigkeit der Tool-Sequenzen (9.5/10), bedingt durch das chronologische Burst-Windowing. Bei synthetischen Traces liegt die Genauigkeit bei 100%.
  * **Sicherheit:** Bietet als einzige Gruppe (neben dem unkomprimierten Raw-Trace) eine vollständige Manipulationserkennung (100%) über den Hashing-Verkettungs-SafePush-Ledger.

* **Vergleich mit Naive Regex-Pruning (Gruppe C):**
  * Gruppe C erhält zwar 100% der Informationen, benötigt jedoch im Schnitt **mehr als das 3-fache an Datenvolumen** (1376.3 Bytes gegenüber 427.0 Bytes) und verfügt über keinerlei Manipulations- und Integritätsschutz (0.0%).

* **Vergleich mit Static Summary (Gruppe D):**
  * Gruppe D reduziert die Payload zwar radikal (43.0 Bytes), verliert jedoch bei realen Traces 30% der Commitments (7/10) und 30% der Tool-Sequenzen (7/10). Bei synthetischen Traces sinken beide Raten auf 0%, was die mangelnde Eignung für kausale Replay-Szenarien beweist.
