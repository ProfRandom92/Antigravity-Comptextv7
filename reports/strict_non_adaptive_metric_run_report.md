# 🏆 Strict Non-Adaptive Metric Run - Turnierbericht

Dieses Dokument beschreibt die Ergebnisse des Replay-Integritäts-Turniers unter strengen, nicht-adaptiven Bewertungsfunktionen. Alle intelligenten Heuristiken, Konsonanten-Mappings bei der Verifikation und LCS-Metriken (Fuzzy Matching) wurden deaktiviert.

---

## 📊 Ergebnisse: Reale LMCache-Traces (10 Fixtures)

| Metrik / Gruppe | Group A (Raw) | Group B (CompText V7) | Group C (Naive Regex) | Group D (Static Summary) | Group E (Random Noise) |
| :--- | :---: | :---: | :---: | :---: | :---: |
| **Commitment Survival Rate** | 100.0% | **70.0%** | 100.0% | 70.0% | 70.0% |
| **Tool Sequence Accuracy** | 100.0% | **70.0%** | 100.0% | 70.0% | 70.0% |
| **Tamper Detection Rate** | 0.0% | **100.0%** | 0.0% | 0.0% | 0.0% |
| **Execution Hash Stability** | 100.0% | **100.0%** | 100.0% | 100.0% | 100.0% |
| **Avg. Size (Bytes)** | 2023.9 | **427.0** | 1376.3 | 43.0 | 1029.0 |

> [!NOTE]
> Die 70.0% für die Gruppen B, D und E bei Commitment Survival und Tool Sequence kommen daher, dass 7 der 10 realen Traces leere Erwartungswerte (`[]`) besaßen. Gemäß Spezifikation wird bei leeren Vorgaben ein Standardwert von `1.0` (100%) zurückgegeben. Für alle Traces mit tatsächlichen Werten sinkt die Erkennung unter dem harten String-Vergleich auf `0.0%`.

---

## 📊 Ergebnisse: Synthetische LMCache-Supplement-Fixtures (3 Fixtures)

| Metrik / Gruppe | Group A (Raw) | Group B (CompText V7) | Group C (Naive Regex) | Group D (Static Summary) | Group E (Random Noise) |
| :--- | :---: | :---: | :---: | :---: | :---: |
| **Commitment Survival Rate** | 100.0% | **0.0%** | 100.0% | 0.0% | 0.0% |
| **Tool Sequence Accuracy** | 100.0% | **0.0%** | 100.0% | 0.0% | 0.0% |
| **Tamper Detection Rate** | 0.0% | **100.0%** | 0.0% | 0.0% | 0.0% |
| **Execution Hash Stability** | 100.0% | **100.0%** | 100.0% | 100.0% | 100.0% |
| **Avg. Size (Bytes)** | 280.3 | **387.0** | 149.0 | 43.0 | 140.3 |

---

## 📊 Ergebnisse: Blind Antigravity Holdout-Szenarien (10 Szenarien)

| Metrik / Gruppe | Group A (Raw) | Group B (CompText V7) | Group C (Naive Regex) | Group D (Static Summary) | Group E (Random Noise) |
| :--- | :---: | :---: | :---: | :---: | :---: |
| **Commitment Survival Rate** | 100.0% | **10.0%** | 100.0% | 30.0% | 10.0% |
| **Tool Sequence Accuracy** | 100.0% | **100.0%** | 0.0% | 0.0% | 0.0% |
| **Tamper Detection Rate** | 0.0% | **100.0%** | 0.0% | 0.0% | 0.0% |
| **Execution Hash Stability** | 100.0% | **100.0%** | 100.0% | 100.0% | 100.0% |
| **Avg. Payload Size (Bytes)** | 218.9 | **197.9** | 101.3 | 62.6 | 126.0 |

---

## 🔬 Analyse des Leistungseinbruchs (CompText V7)

1. **Commitment Survival Rate:**
   * **Ursache:** Im KVTC-V7 Codec werden Wörter durch Konsonanten-Kompression (z.B. `exhaustion` -> `XHSTN`) und Domain-Shortcodes (z.B. `voltage` -> `VLT`) stark verkürzt dargestellt. 
   * **Auswirkung:** Da die Metrik nun einen exakten, unmodifizierten Zeichenkettenabgleich verlangt (`expected_commitment_hash == reconstructed_commitment_hash`), scheitert die Erkennung der komprimierten Werte bei allen nicht-leeren Commitments. Die Survival Rate sinkt somit auf den Basis-Defaultwert.

2. **Tool Sequence Accuracy:**
   * **Ursache:** Ähnlich wie bei Commitments werden in LMCache-Traces Toolnamen (z.B. `update_reservation_flights`) in der komprimierten Repräsentation unlesbar gemacht bzw. durch Konsonantenkürzungen signiert.
   * **Auswirkung:** Ohne Fuzzy- und Konsonanten-Rückabbildung schlägt der exakte Sequenzvergleich vollständig fehl. Die 100%ige Genauigkeit bei den Antigravity-Holdout-Szenarien bleibt bestehen, da die ECU-Namen (MCM, ACM, CPC) durch die sparse micro-frame Synopsis exakt im Header-Format unkomprimiert hinterlegt und durch den entsprechenden Decoder exakt ausgelesen werden.

3. **Integrität & Stabilität (Tamper Detection / Hash Stability):**
   * **Ursache:** Die kryptografische SHA-256/Blake2-Forensikschicht arbeitet unabhängig von String-Vergleichen rein auf der binären Struktur und dem SafePush-Ledger.
   * **Auswirkung:** Die Manipulationserkennung bleibt deterministisch bei **100%** und die Hash-Stabilität bei **100%**.

---

## 🛡️ Fazit

Unter harten, nicht-adaptiven Bewertungsmetriken zeigt sich der erwartete theoretische Grenzwert: Da CompText V7 für eine hohe Kompressionsrate auf verlustbehaftete Abbildungstechniken für Fließtext (Konsonantenkürzungen, burst windows) setzt, ist ein direkter Eins-zu-eins-Stringvergleich ohne Decoder-Logik nicht funktional. Die fundamentalen mathematischen Eigenschaften wie Determinismus (Hash-Stabilität) und Manipulationssicherheit (Tamper-Detection) bleiben hiervon unberührt.
