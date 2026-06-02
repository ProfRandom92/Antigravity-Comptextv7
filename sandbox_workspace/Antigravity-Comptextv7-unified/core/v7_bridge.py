import sys
import os
import hashlib
import json
import time

# Pfad zum geklonten Repository hinzufügen
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), '../Comptextv7/src')))

# HINWEIS AN ANTIGRAVITY: Ersetze diesen Import mit dem exakten Modulnamen 
# deines v7-Encoders aus dem geklonten Repo.
from core.kvtc_v7 import KVTCV7Engine

_engine = KVTCV7Engine()

def v7_encode(data: str) -> str:
    res = _engine.compress(data)
    return res.text

class CompTextV7Bridge:
    def __init__(self):
        self.ledger_path = os.path.abspath(os.path.join(os.path.dirname(__file__), '../benchmarks/forensic_ledger.json'))
        self.last_hash = "0" * 64
        self._init_ledger()

    def _init_ledger(self):
        if not os.path.exists(self.ledger_path):
            with open(self.ledger_path, 'w') as f:
                json.dump([], f)
        else:
            with open(self.ledger_path, 'r') as f:
                data = json.load(f)
                if data:
                    self.last_hash = data[-1]["state_hash"]

    def process_and_sign(self, raw_state: dict) -> tuple:
        # 1. Konvertierung in String
        raw_str = json.dumps(raw_state, separators=(',', ':'))
        
        # 2. Native CompText v7 Kompression
        compressed_payload = v7_encode(raw_str)
        
        # 3. SafePush Forensic Layer (Kryptografische Verkettung)
        timestamp = time.time()
        block_data = f"{timestamp}|{compressed_payload}|{self.last_hash}"
        current_hash = hashlib.sha256(block_data.encode()).hexdigest()
        
        ledger_entry = {
            "index": int(timestamp * 1000),
            "timestamp": timestamp,
            "v7_payload": compressed_payload,
            "state_hash": current_hash,
            "parent_hash": self.last_hash
        }
        
        # Ledger aktualisieren
        with open(self.ledger_path, 'r+') as f:
            data = json.load(f)
            data.append(ledger_entry)
            f.seek(0)
            json.dump(data, f, indent=2)
            
        self.last_hash = current_hash
        return compressed_payload, current_hash

    def verify_integrity(self) -> bool:
        """Überprüft die gesamte Kette auf Manipulation."""
        with open(self.ledger_path, 'r') as f:
            data = json.load(f)
        
        expected_parent = "0" * 64
        for entry in data:
            if entry["parent_hash"] != expected_parent:
                return False
            block_data = f"{entry['timestamp']}|{entry['v7_payload']}|{entry['parent_hash']}"
            recalculated_hash = hashlib.sha256(block_data.encode()).hexdigest()
            if entry["state_hash"] != recalculated_hash:
                return False
            expected_parent = entry["state_hash"]
        return True
