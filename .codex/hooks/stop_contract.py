#!/usr/bin/env python3
"""Stop hook that nudges incomplete CompText handoffs without making claims."""

from __future__ import annotations

import json
import sys


REQUIRED_LABELS = (
    "PHASE:",
    "STATUS:",
    "FILES_CHANGED:",
    "COMMANDS_RUN:",
    "VALIDATION:",
    "GIT:",
    "SECRETS:",
    "CLAIMS:",
    "RISKS:",
    "NEXT:",
)


def main() -> None:
    try:
        event = json.load(sys.stdin)
    except json.JSONDecodeError:
        print(json.dumps({"continue": True}))
        return
    if not isinstance(event, dict):
        print(json.dumps({"continue": True}))
        return

    message = event.get("last_assistant_message") or ""
    missing = [label for label in REQUIRED_LABELS if label not in message]
    if missing:
        print(
            json.dumps(
                {
                    "decision": "block",
                    "reason": "Complete the CompText handoff block before stopping. Missing labels: "
                    + ", ".join(missing),
                }
            )
        )
        return

    print(json.dumps({"continue": True}))


if __name__ == "__main__":
    main()
