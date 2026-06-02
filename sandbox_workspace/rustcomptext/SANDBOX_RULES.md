# Sandbox Rules for rustcomptext

## Erlaubter Schreibbereich
- Jegliche Schreiboperationen sind ausschließlich auf das Verzeichnis `./sandbox_workspace/rustcomptext/` beschränkt.

## Verbotene Änderungen
- Keine Änderungen an bestehenden CompText V7 Dateien außerhalb von `./sandbox_workspace/rustcomptext/`.
- Keine Änderungen an bestehenden Antigravity-Dateien.
- Keine Änderungen an `scripts/`.
- Keine Änderungen an `dashboard/`.
- Keine Änderungen an `.github/`.
- Kein `git push` und kein `merge` im übergeordneten oder lokalen Repository.
- Kein Löschen bestehender Dateien außerhalb von `./sandbox_workspace/rustcomptext/`.

## Deterministische Regeln
- Kein Netzwerkzugriff im Code.
- Keine LLM- oder API-Integrationen.
- Keine Verwendung stochastischer Elemente oder Randomness (Zufall).
- Keine Verwendung der Systemzeit in Programmausgaben, Hashes oder Paketen (um vollständige Reproduzierbarkeit zu gewährleisten).
