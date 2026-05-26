import os
import sys
import time
import json
import glob
from v7_bridge import CompTextV7Bridge

# Add paths for MCP context layer and validation
sys.path.append(os.path.abspath(os.path.join(os.path.dirname(__file__), '../Comptextv7/src')))
sys.path.append(os.path.abspath(os.path.join(os.path.dirname(__file__), '../Comptextv7')))
from comptext_v7.mcp import build_replay_payload, validate_replay_payload
from src.validation.contract_validator import ContractValidator

class AntigravitySandboxEngine:
    def __init__(self):
        self.bridge = CompTextV7Bridge()
        self.execution_history = []

    def transition(self, current_loop: int, task: str, payload_data: dict):
        start_time = time.perf_counter()
        
        # Falls die Payload-Daten die komplette MCP-Struktur haben, validieren wir sie:
        if isinstance(payload_data, dict) and "trace" in payload_data and "state" in payload_data and "dependency_graph" in payload_data:
            # 1. MCP replay-aware check
            context_data = {
                "task": task,
                "trace": payload_data["trace"],
                "state": payload_data["state"],
                "dependency_graph": payload_data["dependency_graph"]
            }
            replay_payload = build_replay_payload(context_data)
            validation_report = validate_replay_payload(replay_payload)
            
            if not validation_report.get("admissible", True):
                duration_ms = (time.perf_counter() - start_time) * 1000
                print(f"[ERROR] MCP Replay validation failed: {validation_report.get('failure_labels')}")
                print(f"Validation latency: {duration_ms:.3f} ms")
                raise ValueError(f"Aborted: Invalid MCP Trace state. Issue detected: {validation_report.get('failure_labels')}")
                
            # 2. Contract-based validation check against original/baseline
            fixture_prefix = "mcp_trace_replay_degraded_v1"
            base_fixture_dir = os.path.abspath(os.path.join(os.path.dirname(__file__), f"../Comptextv7/fixtures/{fixture_prefix}/original"))
            if os.path.exists(base_fixture_dir):
                original = {
                    **json.load(open(os.path.join(base_fixture_dir, "trace.json"))),
                    **json.load(open(os.path.join(base_fixture_dir, "state.json"))),
                    "dependency_graph": json.load(open(os.path.join(base_fixture_dir, "dependency_graph.json")))
                }
                
                # Flatten the incoming payload_data to match what ContractValidator expects
                reconstructed = {
                    **payload_data.get("trace", {}),
                    **payload_data.get("state", {}),
                    "dependency_graph": payload_data.get("dependency_graph", {})
                }
                
                contracts_dir = os.path.join(base_fixture_dir, "contracts")
                contracts = [json.load(open(f)) for f in sorted(glob.glob(os.path.join(contracts_dir, "*.json")))]
                
                validator = ContractValidator()
                results = validator.validate_contracts(original=original, reconstructed=reconstructed, contracts=contracts)
                failed_contracts = [res for res in results if not res.passed]
                
                if failed_contracts:
                    duration_ms = (time.perf_counter() - start_time) * 1000
                    labels = [res.failure_label for res in failed_contracts]
                    print(f"[ERROR] Contract validation failed: {labels}")
                    print(f"Validation latency: {duration_ms:.3f} ms")
                    raise ValueError(f"Aborted: Contract violation detected: {labels}")
        
        # Strukturierter, roher Zustand
        state_frame = {
            "loop": current_loop,
            "active_task": task,
            "data": payload_data,
            "environment": "Termux_Isolated_Sandbox"
        }
        
        # Kompression und Signierung via CompText v7
        v7_payload, state_hash = self.bridge.process_and_sign(state_frame)
        
        # Antigravity hält nur das hochdichte v7-Artefakt im aktiven Arbeitsgedächtnis
        self.execution_history.append(v7_payload)
        
        print(f"[AG 2.0 Loop {current_loop}] Zustand gesichert.")
        print(f"-> Hash: {state_hash[:16]}...")
        print(f"-> Payload-Dichte: {len(v7_payload)} Bytes\n")
