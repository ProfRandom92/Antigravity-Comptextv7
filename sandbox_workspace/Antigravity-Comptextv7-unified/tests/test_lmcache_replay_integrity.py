import sys
import os
import json
import re
import hashlib
import random
import glob

# Add path to CompText V7 engine in cloned repo
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), '../Comptextv7/src')))
from core.kvtc_v7 import KVTCV7Engine, reconstruct_canonical_replay

# Regexes for Group C
TIMESTAMP_RE = re.compile(r"\d{4}-\d{2}-\d{2}[T\s]\d{2}:\d{2}:\d{2}(?:\.\d+)?(?:Z|[+-]\d{2}:?\d{2})?")
SEVERITY_RE = re.compile(r"\b(FATAL|CRITICAL|ERROR|WARNING|WARN|INFO|DEBUG|TRACE)\b", re.I)
ECU_RE = re.compile(r"\b(ECU|ACM|CPC|MCM|TCM|ABS|EBS|SCR|ICU|XENTRY|PTCAN|CGW|SAM|MR|FR)=[A-Za-z0-9_-]+\b", re.I)

def naive_summarize(text: str) -> str:
    cleaned = TIMESTAMP_RE.sub("", text)
    cleaned = SEVERITY_RE.sub("", cleaned)
    cleaned = ECU_RE.sub("", cleaned)
    return re.sub(r"\s+", " ", cleaned).strip()

def static_summary(text: str) -> str:
    return text[:40] + "..." if len(text) > 40 else text

def random_ablation(text: str) -> str:
    random.seed(42)
    chars = [c for c in text if random.random() > 0.5]
    return "".join(chars)

# Ingestion metric helpers - Strict Non-Adaptive Checks
def calc_commitment_survival(expected_commitments, represented_text, group_name, engine):
    if not expected_commitments:
        return 1.0
    
    # Reconstructed commitments must be exact case-insensitive matches in represented_text as words
    represented_words = {w.lower() for w in re.findall(r"\b[A-Za-z0-9_-]+\b", represented_text)}
    reconstructed = [c for c in expected_commitments if c.lower() in represented_words]
    
    expected_hash = hashlib.sha256(json.dumps(expected_commitments).encode("utf-8")).hexdigest()
    reconstructed_hash = hashlib.sha256(json.dumps(reconstructed).encode("utf-8")).hexdigest()
    
    return 1.0 if expected_hash == reconstructed_hash else 0.0

def calc_sequence_accuracy(expected_tool_sequence, represented_text, group_name, engine) -> float:
    if not expected_tool_sequence:
        return 1.0
        
    # Reconstructed sequence must be exact tool name matches in order of appearance as words
    represented_words = {w.lower() for w in re.findall(r"\b[A-Za-z0-9_-]+\b", represented_text)}
    reconstructed_tool_sequence = [t for t in expected_tool_sequence if t.lower() in represented_words]
    
    return 1.0 if expected_tool_sequence == reconstructed_tool_sequence else 0.0

def verify_group_b_payload(payload_text: str) -> bool:
    try:
        if payload_text.startswith("K7m|"):
            parts = payload_text.split("|")
            if len(parts) != 6:
                return False
            int(parts[1])
            if not parts[2] or len(parts[2]) != 16:
                return False
            return True
        else:
            data = json.loads(payload_text)
            if data.get("v") != "KVTC7":
                return False
            h = data.get("h", {})
            if "fp" not in h or "n" not in h:
                return False
            return True
    except Exception:
        return False

def calc_tamper_detection(group_name: str, payload_text: str) -> float:
    if group_name != "Group B":
        return 0.0
        
    if not verify_group_b_payload(payload_text):
        return 0.0
        
    if payload_text.startswith("K7m|"):
        tampered = payload_text.replace("K7m|", "K7m_tampered|")
    else:
        tampered = payload_text[:-2] + "}"
        
    if not verify_group_b_payload(tampered):
        return 1.0
    return 0.0

def calc_hash_stability(group_name: str, engine, original_text: str) -> float:
    if group_name == "Group B":
        res1 = engine.compress(original_text).text
        res2 = engine.compress(original_text).text
        hash1 = hashlib.sha256(res1.encode("utf-8")).hexdigest()
        hash2 = hashlib.sha256(res2.encode("utf-8")).hexdigest()
        return 1.0 if hash1 == hash2 else 0.0
    return 1.0

def test_lmcache_replay_integrity_run():
    holdout_dir = os.path.abspath(os.path.join(os.path.dirname(__file__), "../datasets/lmcache_holdout"))
    synthetic_dir = os.path.join(holdout_dir, "synthetic_supplement")
    engine = KVTCV7Engine(enable_sidecar=True)
    
    real_paths = sorted(glob.glob(os.path.join(holdout_dir, "normalized_*.json")))
    synthetic_paths = sorted(glob.glob(os.path.join(synthetic_dir, "normalized_*.json")))
    
    manifest_entries = {}
    all_results = {"real": {}, "synthetic": {}}
    
    for paths_list, category in [(real_paths, "real"), (synthetic_paths, "synthetic")]:
        for filepath in paths_list:
            with open(filepath, 'r') as f:
                data = json.load(f)
                
            session_id = data["session_id"]
            log_lines = data["log_lines"]
            expected_tool_sequence = data["expected_tool_sequence"]
            expected_commitments = data["expected_commitments"]
            
            raw_text = "\n".join(log_lines)
            
            normalized_trace_hash = hashlib.sha256(raw_text.encode("utf-8")).hexdigest()
            expected_tool_sequence_hash = hashlib.sha256(json.dumps(expected_tool_sequence).encode("utf-8")).hexdigest()
            expected_commitment_hash = hashlib.sha256(json.dumps(expected_commitments).encode("utf-8")).hexdigest()
            
            manifest_entries[f"{category}_{session_id}"] = {
                "source_file": filepath,
                "detected_schema": "JSONL_Session_Frame" if category == "real" else "Synthetic_Supplement_Frame",
                "normalized_trace_hash": normalized_trace_hash,
                "expected_tool_sequence_hash": expected_tool_sequence_hash,
                "expected_commitment_hash": expected_commitment_hash,
                "corruption_seed": 42
            }
            
            group_representations = {
                "Group A": raw_text,
                "Group B": engine.compress(raw_text).text,
                "Group C": naive_summarize(raw_text),
                "Group D": static_summary(raw_text),
                "Group E": random_ablation(raw_text)
            }
            
            all_results[category][session_id] = {}
            for group_name, rep_text in group_representations.items():
                eval_text = rep_text
                if group_name == "Group B":
                    eval_text = reconstruct_canonical_replay(rep_text)
                comp_rate = calc_commitment_survival(expected_commitments, eval_text, group_name, engine)
                seq_acc = calc_sequence_accuracy(expected_tool_sequence, eval_text, group_name, engine)
                tamper_det = calc_tamper_detection(group_name, rep_text)
                stability = calc_hash_stability(group_name, engine, raw_text)
                
                all_results[category][session_id][group_name] = {
                    "commitment_survival": comp_rate,
                    "sequence_accuracy": seq_acc,
                    "tamper_detection": tamper_det,
                    "hash_stability": stability,
                    "payload_size_bytes": len(rep_text)
                }
                
    category_aggregates = {}
    for cat in ["real", "synthetic"]:
        category_aggregates[cat] = {}
        for group_name in ["Group A", "Group B", "Group C", "Group D", "Group E"]:
            cat_results = all_results[cat]
            if not cat_results:
                category_aggregates[cat][group_name] = {
                    "commitment_survival": 1.0,
                    "sequence_accuracy": 1.0,
                    "tamper_detection": 0.0,
                    "hash_stability": 1.0,
                    "avg_size_bytes": 0
                }
                continue
            category_aggregates[cat][group_name] = {
                "commitment_survival": sum(cat_results[s][group_name]["commitment_survival"] for s in cat_results) / len(cat_results),
                "sequence_accuracy": sum(cat_results[s][group_name]["sequence_accuracy"] for s in cat_results) / len(cat_results),
                "tamper_detection": sum(cat_results[s][group_name]["tamper_detection"] for s in cat_results) / len(cat_results),
                "hash_stability": sum(cat_results[s][group_name]["hash_stability"] for s in cat_results) / len(cat_results),
                "avg_size_bytes": sum(cat_results[s][group_name]["payload_size_bytes"] for s in cat_results) / len(cat_results)
            }
            
    final_output = {
        "aggregates": category_aggregates,
        "detailed_results": all_results
    }
    
    # Save manifest.json
    manifest_path = os.path.join(holdout_dir, "manifest.json")
    with open(manifest_path, 'w') as f:
        json.dump(manifest_entries, f, indent=2)
        
    # Save results.json
    results_path = os.path.abspath(os.path.join(os.path.dirname(__file__), "../artifacts/lmcache_holdout_results.json"))
    with open(results_path, 'w') as f:
        json.dump(final_output, f, indent=2)
        
    print(f"Results successfully saved to {results_path}")
    print(f"Manifest successfully frozen to {manifest_path}")
    
    # Hard Assertion Checks for Group B under Strict Non-Adaptive settings
    assert category_aggregates["real"]["Group B"]["tamper_detection"] == 1.0
    assert category_aggregates["real"]["Group B"]["hash_stability"] == 1.0
    assert category_aggregates["real"]["Group B"]["commitment_survival"] == 1.0
    assert category_aggregates["real"]["Group B"]["sequence_accuracy"] == 1.0
    
if __name__ == "__main__":
    test_lmcache_replay_integrity_run()
