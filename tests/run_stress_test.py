import sys
import os
import json
import time

sys.path.append(os.path.abspath(os.path.join(os.path.dirname(__file__), '../core')))
from sandbox_engine import AntigravitySandboxEngine

def run_stress_test():
    # Ledger-Pfad ermitteln
    ledger_path = os.path.abspath(os.path.join(os.path.dirname(__file__), '../benchmarks/forensic_ledger.json'))
    
    # Vorherigen Ledger-Inhalt einlesen
    before_count = 0
    if os.path.exists(ledger_path):
        with open(ledger_path, 'r') as f:
            before_count = len(json.load(f))
            
    print(f"Ledger entry count before test: {before_count}")
    
    # 1. Corrupten Datensatz laden
    fixture_dir = os.path.abspath(os.path.join(os.path.dirname(__file__), '../Comptextv7/fixtures/mcp_trace_replay_corruptions/mcp_trace_replay_degraded_v1/drop_approval_gate'))
    
    trace_path = os.path.join(fixture_dir, "trace.json")
    state_path = os.path.join(fixture_dir, "state.json")
    graph_path = os.path.join(fixture_dir, "dependency_graph.json")
    
    try:
        with open(trace_path, 'r') as f:
            trace_data = json.load(f)
        with open(state_path, 'r') as f:
            state_data = json.load(f)
        with open(graph_path, 'r') as f:
            graph_data = json.load(f)
    except FileNotFoundError as e:
        print(f"Error loading fixtures from {fixture_dir}: {e}")
        sys.exit(1)
        
    corrupt_payload = {
        "trace": trace_data,
        "state": state_data,
        "dependency_graph": graph_data
    }
    
    print("\n--- Feeder starting: transitioning with corrupted payload ---")
    engine = AntigravitySandboxEngine()
    
    start_time = time.perf_counter()
    try:
        engine.transition(100, "drop_approval_gate_check", corrupt_payload)
        success = True
    except Exception as e:
        success = False
        duration_ms = (time.perf_counter() - start_time) * 1000
        print(f"\n[GATE TRIGGERED] Transition aborted successfully!")
        print(f"Exception details: {e}")
        print(f"Time taken to abort: {duration_ms:.3f} ms")
        
    # Ledger-Inhalt nach dem Test einlesen
    after_count = 0
    if os.path.exists(ledger_path):
        with open(ledger_path, 'r') as f:
            after_count = len(json.load(f))
            
    print(f"\nLedger entry count after test: {after_count}")
    
    if not success and before_count == after_count:
        print("RESULT: SafePush ledger successfully blocked from committing corrupted state!")
    else:
        print("RESULT: WARNING! Ledger block failed or transition succeeded without abort.")

if __name__ == "__main__":
    run_stress_test()
