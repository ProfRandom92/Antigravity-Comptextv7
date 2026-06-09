# Antigravity Migration Ledger

> [!IMPORTANT]
> **Status**: inert, repo-local, review-gated, not active runtime. This ledger is for local reference and documentation only.

This file documents the conceptual mapping and legacy source classifications between Codex and Antigravity for the CompText project context.

## Skill Sources Classifications

- **.agent/skills** = repo guidance source
- **.agents/skills** = compatibility metadata
- **docs/phase8a-draft-skills** = draft source
- **.antigravity** = inert adapted workspace config

## Codex to Antigravity Mapping Rules

We map the original Codex infrastructure concepts to their Antigravity equivalents in all new documentation and configurations:

| Codex Concept | Antigravity Concept | Description |
|---|---|---|
| Codex App | Antigravity | The core agent system workspace and host application. |
| Codex CLI | Antigravity CLI | The CLI execution layer for local agent orchestration. |
| Codex skills | Antigravity skills | The repository-relative or system-level capabilities. |
| Codex hooks | Antigravity hooks | Pre-tool and post-tool execution policy validation hooks. |
| Codex plugin | Antigravity plugin | Custom plugins extending workspace capabilities. |
| Codex workflow | Antigravity workflow | The pipeline executing the Context Pack process. |

## Migration Status

- **Migration Mode**: Manual documentation and alignment mapping.
- **Hook Integration**: Mapped, but disabled (inert, not active runtime).
- **Skill Discovery**: Completed; mapped to local directories and system-level capabilities.
- **Legacy Alignment**: Legacy and draft skill sources classified and cataloged.
