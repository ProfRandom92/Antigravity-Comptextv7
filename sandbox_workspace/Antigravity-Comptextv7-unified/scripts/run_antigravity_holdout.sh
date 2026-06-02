#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
export PYTHONPATH="$PWD/Comptextv7/src:$PWD/Comptextv7:$PWD/core:${PYTHONPATH:-}"
python -m pytest tests/test_blind_antigravity_trace_survival.py -q
