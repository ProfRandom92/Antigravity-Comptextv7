#!/usr/bin/env python3
"""Post-tool warning hook for protected CompText paths."""

from __future__ import annotations

import json
import sys


WATCHED = (
    "README.md",
    "agy7rust/src/",
    "reports/latest.json",
    "reports/performance_baseline.json",
    "artifacts/spark/",
)


def text_from(value: object) -> str:
    if isinstance(value, str):
        return value
    try:
        return json.dumps(value, sort_keys=True)
    except TypeError:
        return ""


def main() -> None:
    try:
        event = json.load(sys.stdin)
    except json.JSONDecodeError:
        return
    if not isinstance(event, dict):
        return

    data = " ".join(
        [
            text_from(event.get("tool_input")),
            text_from(event.get("tool_response")),
        ]
    ).replace("\\", "/")
    hits = [path for path in WATCHED if path in data]
    if not hits:
        return

    print(
        json.dumps(
            {
                "hookSpecificOutput": {
                    "hookEventName": "PostToolUse",
                    "additionalContext": (
                        "CompText artifact hygiene warning: review protected path changes before final handoff: "
                        + ", ".join(hits)
                    ),
                }
            }
        )
    )


if __name__ == "__main__":
    main()
