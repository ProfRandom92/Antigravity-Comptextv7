import os
import json
import hashlib

def generate_adversarial_traces():
    holdout_dir = "/data/data/com.termux/files/home/antigravity_v7_sandbox/datasets/antigravity_holdout"
    os.makedirs(holdout_dir, exist_ok=True)
    
    # 10 Szenarien generieren
    scenarios = {
        "scenario_01_standard": (
            "2026-05-26T12:00:00Z INFO ECU=MCM Startup successful voltage=24.2V\n"
            "2026-05-26T12:00:05Z DEBUG ECU=ACM Connection active status=stable\n"
            "2026-05-26T12:00:10Z INFO ECU=MCM Task Completed cycle=1"
        ),
        "scenario_02_parallel": (
            "2026-05-26T12:00:00Z INFO ECU=ACM TaskA start voltage=24.1V\n"
            "2026-05-26T12:00:00Z INFO ECU=CPC TaskB start temperature=85C\n"
            "2026-05-26T12:00:05Z WARN ECU=ACM TaskA complete status=success\n"
            "2026-05-26T12:00:06Z INFO ECU=CPC TaskB complete status=success"
        ),
        "scenario_03_bugfix": (
            "2026-05-26T12:00:00Z ERROR ECU=MCM DTC:P0301 misfire detected cylinder=1\n"
            "2026-05-26T12:00:05Z WARN ECU=MCM Guided test applied clear_faults\n"
            "2026-05-26T12:00:10Z INFO ECU=MCM DTC:P0000 fault cleared successful"
        ),
        "scenario_04_conflict": (
            "2026-05-26T12:00:00Z WARN ECU=CPC Boundary constraint limit=100\n"
            "2026-05-26T12:00:05Z ERROR ECU=CPC User command override limit=150\n"
            "2026-05-26T12:00:10Z CRITICAL ECU=CPC Safety cutoff triggered due to conflict"
        ),
        "scenario_05_tampered": (
            "2026-05-26T12:00:00Z INFO ECU=MCM Log entry authentic id=001\n"
            "2026-05-26T12:00:05Z INFO ECU=MCM Signatures valid count=5\n"
            "2026-05-26T12:00:10Z WARN ECU=MCM Verification complete success"
        ),
        "scenario_06_retry": (
            "2026-05-26T12:00:00Z WARN ECU=ACM Connection failed timeout=5s\n"
            "2026-05-26T12:00:05Z INFO ECU=ACM Retrying connection attempt=1\n"
            "2026-05-26T12:00:10Z INFO ECU=ACM Retrying connection attempt=2\n"
            "2026-05-26T12:00:15Z INFO ECU=ACM Connection reestablished success"
        ),
        "scenario_07_nested": (
            "2026-05-26T12:00:00Z INFO ECU=CPC Loop start depth=1\n"
            "2026-05-26T12:00:01Z INFO ECU=CPC Loop start depth=2\n"
            "2026-05-26T12:00:02Z INFO ECU=CPC Nested operation executed value=42\n"
            "2026-05-26T12:00:03Z INFO ECU=CPC Loop end depth=2\n"
            "2026-05-26T12:00:04Z INFO ECU=CPC Loop end depth=1"
        ),
        "scenario_08_bypass": (
            "2026-05-26T12:00:00Z WARN ECU=ACM Safety policy bypass requested by operator\n"
            "2026-05-26T12:00:05Z ERROR ECU=ACM Access granted override=active\n"
            "2026-05-26T12:00:10Z INFO ECU=ACM Critical task completed without governance"
        ),
        "scenario_09_exhaustion": (
            "2026-05-26T12:00:00Z WARN ECU=CPC Memory alert threshold=95%\n"
            "2026-05-26T12:00:05Z ERROR ECU=CPC Buffer overflow page=0x3F\n"
            "2026-05-26T12:00:10Z FATAL ECU=CPC Resource exhaustion shutdown initiated"
        ),
        "scenario_10_recovery": (
            "2026-05-26T12:00:00Z ERROR ECU=MCM Primary pump failure active=false\n"
            "2026-05-26T12:00:05Z INFO ECU=MCM Triggering backup pump active=true\n"
            "2026-05-26T12:00:10Z INFO ECU=MCM Recovery successful flow=nominal"
        )
    }
    
    manifest = {}
    for name, content in scenarios.items():
        filepath = os.path.join(holdout_dir, f"{name}.log")
        with open(filepath, "w") as f:
            f.write(content)
        
        # SHA256 berechnen
        sha256 = hashlib.sha256(content.encode("utf-8")).hexdigest()
        manifest[f"{name}.log"] = sha256
        
    # Manifest speichern
    manifest_path = os.path.join(holdout_dir, "frozen_manifest.json")
    with open(manifest_path, "w") as f:
        json.dump(manifest, f, indent=2)
        
    print(f"Generated 10 adversarial traces and frozen manifest at {manifest_path}")

if __name__ == "__main__":
    generate_adversarial_traces()
