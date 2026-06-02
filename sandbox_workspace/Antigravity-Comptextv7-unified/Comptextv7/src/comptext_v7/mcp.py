"""Minimal replay payload adapter used by the Antigravity bridge.

This shim keeps the Antigravity integration executable against the current
Comptextv7 core package. It intentionally performs deterministic structural
validation only; domain-specific contract validation remains delegated to
``src.validation.contract_validator``.
"""

from __future__ import annotations

import hashlib
import json
from typing import Any, Mapping

REQUIRED_KEYS = ("task", "trace", "state", "dependency_graph")

def _stable_hash(payload: Mapping[str, Any]) -> str:
    encoded = json.dumps(payload, sort_keys=True, separators=(",", ":"), default=str).encode("utf-8")
    return hashlib.sha256(encoded).hexdigest()

def build_replay_payload(context_data: Mapping[str, Any]) -> dict[str, Any]:
    payload = {key: context_data.get(key) for key in REQUIRED_KEYS}
    payload["integrity_hash"] = _stable_hash(payload)
    payload["schema"] = "comptext_v7.replay_payload.v1"
    return payload

def validate_replay_payload(payload: Mapping[str, Any]) -> dict[str, Any]:
    missing = [key for key in REQUIRED_KEYS if key not in payload or payload.get(key) is None]
    expected_hash = payload.get("integrity_hash")
    check_payload = {key: payload.get(key) for key in REQUIRED_KEYS}
    actual_hash = _stable_hash(check_payload)
    failure_labels: list[str] = []
    if missing:
        failure_labels.append("MISSING_REPLAY_PAYLOAD_FIELDS")
    if expected_hash != actual_hash:
        failure_labels.append("REPLAY_PAYLOAD_HASH_MISMATCH")
    return {
        "admissible": not failure_labels,
        "failure_labels": failure_labels,
        "integrity_hash": actual_hash,
    }
