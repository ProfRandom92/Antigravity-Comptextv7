#!/usr/bin/env python3
"""Repo-local Codex pre-tool policy for comptext-sparkctl."""

from __future__ import annotations

import json
import re
import shlex
import sys
from pathlib import PurePosixPath


BLOCKED_GIT = {
    "commit",
    "push",
    "pull",
    "merge",
    "rebase",
    "tag",
    "fetch",
}

BLOCKED_COMMANDS = {
    "gh pr",
    "gh issue",
    "gh release",
    "vercel",
    "netlify",
    "wrangler deploy",
    "fly deploy",
    "railway up",
    "render deploy",
}

SECRET_PATTERNS = (
    r"(^|[\s\\/])\.env(\.|$|[\s\\/])",
    r"(^|[\s\\/])\.npmrc($|[\s\\/])",
    r"(^|[\s\\/])\.pypirc($|[\s\\/])",
    r"(^|[\s\\/])\.netrc($|[\s\\/])",
    r"(^|[\s\\/])id_rsa($|[\s\\/])",
    r"(^|[\s\\/])id_ed25519($|[\s\\/])",
    r"(^|[\s\\/])credentials(\.|$|[\s\\/])",
    r"(^|[\s\\/])credential-store(\.|$|[\s\\/])",
    r"(^|[\s\\/])secrets?(\.|$|[\s\\/])",
)

PROTECTED_WARN_PATHS = (
    "README.md",
    "reports/latest.json",
    "reports/performance_baseline.json",
    "artifacts/spark/",
    "agy7rust/src/",
)

SAFE_CARGO = (
    ("cargo", "fmt", "--all", "--check"),
    ("cargo", "check"),
    ("cargo", "test"),
    ("cargo", "clippy"),
    ("cargo", "run", "--bin", "agy-ct", "--", "--help"),
)


def load_event() -> dict:
    try:
        event = json.load(sys.stdin)
    except json.JSONDecodeError:
        return {}
    if not isinstance(event, dict):
        return {}
    return event


def normalize_path(value: object) -> str:
    if not isinstance(value, str):
        return ""
    return value.replace("\\", "/").lstrip("./")


def bash_command(event: dict) -> str:
    tool_input = event.get("tool_input") or {}
    if isinstance(tool_input, dict):
        command = tool_input.get("command") or tool_input.get("cmd")
        if isinstance(command, str):
            return command
    return ""


def deny(reason: str) -> None:
    print(
        json.dumps(
            {
                "hookSpecificOutput": {
                    "hookEventName": "PreToolUse",
                    "permissionDecision": "deny",
                    "permissionDecisionReason": reason,
                }
            }
        )
    )
    raise SystemExit(0)


def warn(message: str) -> None:
    print(
        json.dumps(
            {
                "hookSpecificOutput": {
                    "hookEventName": "PreToolUse",
                    "additionalContext": message,
                }
            }
        )
    )
    raise SystemExit(0)


def split_command(command: str) -> list[str]:
    try:
        return shlex.split(command, posix=False)
    except ValueError:
        return command.split()


def is_safe_cargo(tokens: list[str], cwd: str) -> bool:
    cwd_path = normalize_path(cwd)
    in_rust_dir = cwd_path.endswith("/agy7rust") or PurePosixPath(cwd_path).name == "agy7rust"
    if not in_rust_dir or not tokens:
        return False
    lowered = tuple(token.lower() for token in tokens)
    return any(lowered[: len(prefix)] == prefix for prefix in SAFE_CARGO)


def command_has_secret_read(command: str) -> bool:
    lowered = normalize_path(command).lower()
    if re.search(r"\b(printenv|env)\b", lowered) or "get-childitem env:" in lowered:
        return True
    if re.search(r"\b(get-content|type|cat|more|less|gc|grep|egrep|fgrep|awk|sed|head|tail|jq|yq)\b", lowered):
        return any(re.search(pattern, lowered) for pattern in SECRET_PATTERNS)
    return False


def command_runs_blocked_agy_ct(command: str) -> bool:
    lowered = command.lower()
    direct = re.search(r"agy-ct(?:\.exe)?(?:\s+--)?\s+(run|benchmark)\b", lowered)
    cargo = re.search(r"--bin\s+agy-ct\s+--\s+(run|benchmark)\b", lowered)
    return bool(direct or cargo)


def main() -> None:
    event = load_event()
    command = bash_command(event)
    if not command:
        return

    lowered = command.lower()
    normalized_lowered = normalize_path(command).lower()
    tokens = split_command(command)
    token0 = tokens[0].lower() if tokens else ""
    token1 = tokens[1].lower() if len(tokens) > 1 else ""

    if command_has_secret_read(command):
        deny("Blocked by CompText policy: environment, .env, or credential reads are not allowed.")

    if token0 == "git" and token1 in BLOCKED_GIT:
        deny(f"Blocked by CompText policy: git {token1} is forbidden in this worktree.")

    if token0 == "gh" and token1 in {"pr", "issue", "release"}:
        deny(f"Blocked by CompText policy: GitHub {token1} writes are forbidden.")

    if command_runs_blocked_agy_ct(command):
        deny("Blocked by CompText policy: agy-ct run and agy-ct benchmark create generated artifacts.")

    if any(blocked in lowered for blocked in BLOCKED_COMMANDS):
        deny("Blocked by CompText policy: remote write, release, or deploy command is forbidden.")

    if token0 == "cargo" and not is_safe_cargo(tokens, event.get("cwd", "")):
        warn("Cargo command is outside the documented validation allowlist; run cargo only inside agy7rust/.")

    touched = [path for path in PROTECTED_WARN_PATHS if path.lower() in normalized_lowered]
    if touched:
        warn("Protected path mentioned; verify human approval and artifact hygiene before editing: " + ", ".join(touched))


if __name__ == "__main__":
    main()
