import os
import json
import re
import glob
import time
import hashlib

def clean_text(text: str) -> str:
    cleaned = text.replace("\n", " ").replace("\r", " ")
    cleaned = re.sub(r"[^A-Za-z0-9_.:/=-]", " ", cleaned)
    return re.sub(r"\s+", " ", cleaned).strip()

def process_traces():
    repo_dir = os.path.abspath(os.path.join(os.path.dirname(__file__), "../datasets/lmcache_holdout/repo/taubench"))
    output_dir = os.path.abspath(os.path.join(os.path.dirname(__file__), "../datasets/lmcache_holdout"))
    synthetic_dir = os.path.join(output_dir, "synthetic_supplement")
    
    os.makedirs(output_dir, exist_ok=True)
    os.makedirs(synthetic_dir, exist_ok=True)
    
    audit_lines = []
    audit_lines.append(f"Audit log started at {time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime())}")
    
    # 1. Real files processing
    jsonl_files = sorted(glob.glob(os.path.join(repo_dir, "*.jsonl")))
    real_processed = []
    
    for filepath in jsonl_files[:10]: # Process first 10 files
        filename = os.path.basename(filepath)
        session_id = filename.replace(".jsonl", "")
        
        audit_lines.append(f"Ingesting file {filename} as real trace session {session_id}")
        
        log_lines = []
        expected_tool_sequence = []
        expected_commitments = set()
        
        with open(filepath, 'r') as f:
            for idx, line in enumerate(f, start=1):
                try:
                    data = json.loads(line)
                except Exception as e:
                    audit_lines.append(f"  [ERROR] Line {idx} failed to parse: {e}")
                    continue
                
                ts = data.get("timestamp", 0)
                ts_iso = time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime(ts / 1000000))
                
                user_input = data.get("input", "")
                output_str = data.get("output", "")
                
                tool_call = None
                if output_str.strip().startswith("{"):
                    try:
                        tool_call = json.loads(output_str)
                    except Exception:
                        pass
                
                if tool_call and "name" in tool_call:
                    tool_name = tool_call["name"]
                    params = tool_call.get("parameters", {})
                    expected_tool_sequence.append(tool_name)
                    
                    # Determine which keys are preserved by the 5-field limit
                    sorted_keys = sorted(params.keys())
                    preserved_keys = set(sorted_keys[:5])
                    
                    param_strs = []
                    for k, v in sorted(params.items()):
                        # Only include flat keys that fall within the preserved 5-field limit
                        if k in preserved_keys and k in ("destination", "origin", "payment_id", "reservation_id", "cabin", "user_id", "flight_type"):
                            expected_commitments.add(str(v))
                        param_strs.append(f"{k}={str(v)}")
                        
                    params_formatted = " ".join(param_strs)
                    log_line = f"{ts_iso} INFO ECU=AGENT tool={tool_name} {params_formatted}"
                    log_lines.append(log_line)
                    audit_lines.append(f"  Line {idx}: Parsed tool call to {tool_name}")
                else:
                    # Message turn
                    cleaned_input = clean_text(user_input)[:40]
                    log_line = f"{ts_iso} INFO ECU=USER message={cleaned_input}"
                    log_lines.append(log_line)
                    audit_lines.append(f"  Line {idx}: Parsed message turn")
                    
        normalized = {
            "session_id": session_id,
            "source": "lmcache-agent-trace",
            "log_lines": log_lines,
            "expected_tool_sequence": expected_tool_sequence,
            "expected_commitments": list(expected_commitments)
        }
        
        out_path = os.path.join(output_dir, f"normalized_{session_id}.json")
        with open(out_path, 'w') as out_f:
            json.dump(normalized, out_f, indent=2)
            
        real_processed.append(out_path)
        audit_lines.append(f"Successfully processed {filename} to {out_path}")

    # 2. Synthetic supplement files generation (exactly 3 files to show separation)
    audit_lines.append("Generating synthetic supplementary traces...")
    synthetic_scenarios = [
        {
            "session_id": "syn_01_checkout",
            "log_lines": [
                "2026-05-26T12:00:00Z INFO ECU=USER message=checkout_items_cart",
                "2026-05-26T12:00:05Z INFO ECU=AGENT tool=verify_inventory item_id=cart_item_99 qty=1",
                "2026-05-26T12:00:10Z INFO ECU=AGENT tool=process_payment card_type=visa amount=199",
                "2026-05-26T12:00:15Z INFO ECU=AGENT tool=ship_order method=standard delivery=nominal"
            ],
            "expected_tool_sequence": ["verify_inventory", "process_payment", "ship_order"],
            "expected_commitments": ["cart_item_99", "visa", "standard"]
        },
        {
            "session_id": "syn_02_database",
            "log_lines": [
                "2026-05-26T12:00:00Z INFO ECU=USER message=query_db_records",
                "2026-05-26T12:00:05Z INFO ECU=AGENT tool=db_connect db_host=prod_db_replica1",
                "2026-05-26T12:00:10Z INFO ECU=AGENT tool=db_query table=users filter=active",
                "2026-05-26T12:00:15Z INFO ECU=AGENT tool=db_disconnect status=clean"
            ],
            "expected_tool_sequence": ["db_connect", "db_query", "db_disconnect"],
            "expected_commitments": ["prod_db_replica1", "users", "active"]
        },
        {
            "session_id": "syn_03_kubernetes",
            "log_lines": [
                "2026-05-26T12:00:05Z INFO ECU=AGENT tool=k8s_authenticate cluster=eu_west_2",
                "2026-05-26T12:00:10Z INFO ECU=AGENT tool=k8s_scale deployment=web_server replicas=5",
                "2026-05-26T12:00:15Z INFO ECU=AGENT tool=k8s_verify deployment=web_server status=green"
            ],
            "expected_tool_sequence": ["k8s_authenticate", "k8s_scale", "k8s_verify"],
            "expected_commitments": ["eu_west_2", "web_server", "replicas=5"]
        }
    ]
    
    for syn in synthetic_scenarios:
        out_path = os.path.join(synthetic_dir, f"normalized_{syn['session_id']}.json")
        syn_full = {
            **syn,
            "source": "synthetic_supplement"
        }
        with open(out_path, 'w') as out_f:
            json.dump(syn_full, out_f, indent=2)
        audit_lines.append(f"Generated synthetic trace to {out_path}")
        
    # Write audit log
    audit_log_path = os.path.abspath(os.path.join(os.path.dirname(__file__), "../artifacts/lmcache_holdout_audit.log"))
    os.makedirs(os.path.dirname(audit_log_path), exist_ok=True)
    with open(audit_log_path, 'w') as log_f:
        log_f.write("\n".join(audit_lines))
        
    print(f"Processed 10 real and generated 3 synthetic traces. Audit logged to {audit_log_path}")

if __name__ == "__main__":
    process_traces()
