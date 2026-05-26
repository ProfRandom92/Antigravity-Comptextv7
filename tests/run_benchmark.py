import sys
import os
import json

sys.path.append(os.path.abspath(os.path.join(os.path.dirname(__file__), '../core')))
from sandbox_engine import AntigravitySandboxEngine

def run_simulation():
    engine = AntigravitySandboxEngine()
    
    print("=== STARTE ANTIGRAVITY 2.0 × COMPTEXT V7 STRESSTEST ===")
    
    # 50 hochfrequente Schleifendurchläufe simulieren
    for i in range(1, 51):
        test_payload = {
            "node_target": f"sub_node_{i}_heidelberg",
            "status_code": 200,
            "noise_boilerplate_logs": "DEBUG: connection stable; INFO: packet acknowledged; TRACE: telemetry clear;" * 3
        }
        engine.transition(i, "execute_network_handshake", test_payload)

    # Integritätstest der Kette
    is_valid = engine.bridge.verify_integrity()
    print("==================================================")
    print(f"Forensische Kette validiert: {is_valid}")
    
    # Auswertung der Ersparnis
    raw_size_estimate = 50 * 450 # Ungefähre Bytes im JSON-Format
    with open(engine.bridge.ledger_path, 'r') as f:
        ledger = json.load(f)
    v7_size = sum(len(entry["v7_payload"]) for entry in ledger)
    
    print(f"Erwarteter Roh-Overhead:   {raw_size_estimate} Bytes")
    # Hier wird der echte Faktor sichtbar
    print(f"Tatsächlicher v7-Overhead:  {v7_size} Bytes")
    print(f"Effizienzgewinn:          {((raw_size_estimate - v7_size) / raw_size_estimate) * 100:.2f}% Token-Reduktion")

if __name__ == "__main__":
    run_simulation()
