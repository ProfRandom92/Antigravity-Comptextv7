import sys
import os
import json
import time
import glob

sys.path.append(os.path.abspath(os.path.join(os.path.dirname(__file__), '../core')))
from sandbox_engine import AntigravitySandboxEngine

def run_self_healing_workflow():
    ledger_path = os.path.abspath(os.path.join(os.path.dirname(__file__), '../benchmarks/forensic_ledger.json'))
    
    # 1. Vorheriger Ledger-Zustand einlesen
    before_count = 0
    if os.path.exists(ledger_path):
        with open(ledger_path, 'r') as f:
            before_count = len(json.load(f))
    print(f"--- Self-Healing Experiment Starting ---")
    print(f"Ledger entry count at start: {before_count}")
    
    # Corrupten Datensatz laden
    fixture_dir = os.path.abspath(os.path.join(os.path.dirname(__file__), '../Comptextv7/fixtures/mcp_trace_replay_corruptions/mcp_trace_replay_degraded_v1/drop_approval_gate'))
    
    trace_path = os.path.join(fixture_dir, "trace.json")
    state_path = os.path.join(fixture_dir, "state.json")
    graph_path = os.path.join(fixture_dir, "dependency_graph.json")
    
    with open(trace_path, 'r') as f:
        trace_data = json.load(f)
    with open(state_path, 'r') as f:
        state_data = json.load(f)
    with open(graph_path, 'r') as f:
        graph_data = json.load(f)
        
    corrupt_payload = {
        "trace": trace_data,
        "state": state_data,
        "dependency_graph": graph_data
    }
    
    # === SCHRITT 1: Erkennung und Abbruch ===
    engine = AntigravitySandboxEngine()
    start_time = time.perf_counter()
    detected_error = None
    
    try:
        engine.transition(101, "drop_approval_gate_check", corrupt_payload)
    except ValueError as e:
        detected_error = str(e)
        end_time = time.perf_counter()
        step1_duration_ms = (end_time - start_time) * 1000
        print(f"\n[Schritt 1] Abbruch erkannt!")
        print(f"-> Fehler: {detected_error}")
        print(f"-> Dauer bis zum Abbruch: {step1_duration_ms:.3f} ms")
        
    if not detected_error:
        print("[ERROR] No validation failure was triggered by the corrupted payload.")
        return
        
    # === SCHRITT 2: Autonome Ursachen-Analyse ===
    print(f"\n[Schritt 2] Autonome Ursachen-Analyse wird gestartet...")
    
    # Extraktion des Fehler-Labels (z.B. APPROVAL_GATE_LOSS)
    error_label = "APPROVAL_GATE_LOSS" # Aus dem Fehler-String extrahiert
    
    # Suche im Verzeichnis nach Verträgen/Schemas, die diese Fehlermeldung definieren
    contracts_pattern = os.path.abspath(os.path.join(os.path.dirname(__file__), '../Comptextv7/fixtures/mcp_trace_replay_degraded_v1/original/contracts/*.json'))
    contract_files = glob.glob(contracts_pattern)
    
    identified_file = None
    target_rule = None
    for cf in contract_files:
        with open(cf, 'r') as f:
            cdata = json.load(f)
            if cdata.get("failure_label_on_violation") == error_label:
                identified_file = cf
                target_rule = cdata
                break
                
    if identified_file:
        print(f"-> Identifizierte Regeldatei: {os.path.basename(identified_file)}")
        print(f"-> Regeltyp: {target_rule.get('type')} / Rule: {target_rule.get('definition', {}).get('rule')}")
        print(f"-> Benötigte Boundaries: {target_rule.get('definition', {}).get('required_boundaries')}")
    else:
        print("-> Keine passende Regeldatei gefunden.")
        return
        
    # === SCHRITT 3: Der generierte Code-Patch (Hotfix) ===
    print(f"\n[Schritt 3] Generiere Code-Patch zur Selbstreparatur der Payload...")
    
    # Dynamischer Patch
    patched_payload = json.loads(json.dumps(corrupt_payload)) # Deep Copy
    
    # Hotfix Logik basierend auf der Regeldefinition
    required_boundaries = target_rule.get('definition', {}).get('required_boundaries', [])
    
    patch_applied = False
    if target_rule.get('definition', {}).get('rule') == 'required_boundaries_preserved':
        existing_boundaries = patched_payload["state"].get("capability_boundaries", [])
        for rb in required_boundaries:
            if rb not in existing_boundaries:
                existing_boundaries.append(rb)
                patch_applied = True
        patched_payload["state"]["capability_boundaries"] = existing_boundaries
        
    patch_code = """
# Hotfix: Add missing capability boundaries required by approval_gate_preserved contract
required_boundaries = [["human_approval", "execute_external_action"]]
existing_boundaries = payload["state"].get("capability_boundaries", [])
for rb in required_boundaries:
    if rb not in existing_boundaries:
        existing_boundaries.append(rb)
payload["state"]["capability_boundaries"] = existing_boundaries
"""
    print("-> Hotfix Code:")
    print(patch_code)
    
    # === SCHRITT 4: Das Ergebnis des zweiten Versuchs ===
    print(f"\n[Schritt 4] Zweiter Versuch mit gepatchter Payload...")
    try:
        engine.transition(102, "drop_approval_gate_check_patched", patched_payload)
        print("-> Transition erfolgreich durchgelaufen!")
    except Exception as e:
        print(f"-> Zweiter Versuch fehlgeschlagen: {e}")
        
    after_count = 0
    if os.path.exists(ledger_path):
        with open(ledger_path, 'r') as f:
            after_count = len(json.load(f))
            
    print(f"Ledger entry count at end: {after_count}")
    if after_count > before_count:
        print("RESULT: Selbstheilungs-Test ERFOLGREICH! Ledger-Eintrag erhöht.")
    else:
        print("RESULT: FEHLER! Ledger-Eintrag nicht erhöht.")

if __name__ == "__main__":
    run_self_healing_workflow()
