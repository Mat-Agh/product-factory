# 16 — Documentation link map

**Status:** Active — agent routing authority
**Last updated:** 2026-10-19

Use this map to route to the smallest useful set of documents. Read every
document only for repo-wide documentation work. The exhaustive list is
[DOCUMENT_INVENTORY](../DOCUMENT_INVENTORY.md).

```mermaid
flowchart TB
  Task["Task on the factory repo"] --> A["AGENTS.md"]
  A --> M["This map"]
  M --> S["Source of truth (CONSTITUTION, kit/…)"]
  M --> D["Routed docs"]
  S --> Change["Change + same-commit docs (03 change map)"]
  D --> Change
  Change --> Gates["Gates (04) → EVOLUTION_LOG + IMPLEMENTATION_STATUS"]
```

## Knowledge order

1. `AGENTS.md`, this map, and `docs/README.md`
2. The source file that defines the concept (table below)
3. The routed docs for the task
4. Hermes (`Mat-Agh/Hermes` docs) only for history or the Hermes adapter
5. Official Claude Code documentation for platform behaviour (skills,
   subagents, hooks)

## Router

| Task | Read first | Then if needed |
|---|---|---|
| Any change | `AGENTS.md`, [03-changing-the-factory](03-changing-the-factory.md) | [00-principles](00-factory-engineering-principles.md) |
| Rules / authority | `CONSTITUTION.md` | [COMP-013](../components/COMP-013-constitution.md), [ADR-002](../decisions/ADR-002-human-only-merge.md) |
| Coordinator, commands, lanes | `kit/.claude/skills/factory/SKILL.md` | [COMP-001](../components/COMP-001-coordinator-skill.md), [JOB_LIFECYCLE](../architecture/JOB_LIFECYCLE.md), [guide/02](../guide/02-commands.md) |
| Agent behaviour | `kit/.claude/agents/<role>.md`, `kit/.claude/skills/factory/protocol.md` | [COMP-002](../components/COMP-002-agent-protocol.md), [COMP-003](../components/COMP-003-role-agents.md) |
| Artifacts and verdicts | [JOB_LIFECYCLE → Artifact contract](../architecture/JOB_LIFECYCLE.md#artifact-contract) | SKILL verdict table, [guide/03](../guide/03-reading-a-job.md) |
| Guard hook | `kit/.claude/hooks/factory-guard.py`, `tests/guard_test.py` | [COMP-004](../components/COMP-004-guard-hook.md) |
| Descriptor | `kit/.factory/factory.yaml` | [COMP-005](../components/COMP-005-project-descriptor.md) |
| Dashboard / backlog | `kit/.factory/dashboard.html` | [COMP-006](../components/COMP-006-backlog-and-dashboard.md) |
| Install / update / doctor | `scripts/install.sh`, `scripts/doctor.sh` | [COMP-007](../components/COMP-007-installer.md), [COMP-008](../components/COMP-008-doctor.md), [05-releases](05-releases-and-versioning.md) |
| Portability / backups | `scripts/mirror.sh`, `scripts/export-claude.sh` | [guide/05](../guide/05-portability-and-recovery.md), [COMP-009](../components/COMP-009-backup-tools.md), [ADR-006](../decisions/ADR-006-portable-distribution.md) |
| Operator preferences | `operator/USER.md` | [COMP-010](../components/COMP-010-operator-profile.md) |
| Other runtimes | `adapters/*.md` | [COMP-011](../components/COMP-011-adapters.md) |
| Docs work | [docs/README](../README.md), `scripts/check-docs.py` | [COMP-012](../components/COMP-012-docs-gate.md) |
| Why it is designed this way | [decisions/README](../decisions/README.md) | [architecture/SOURCES](../architecture/SOURCES.md) |
| What works today | [CURRENT_STATE](../../CURRENT_STATE.md) | [IMPLEMENTATION_STATUS](../components/IMPLEMENTATION_STATUS.md) |
| Testing | [04-testing-and-verification](04-testing-and-verification.md) | — |
| Planning next work | [ROADMAP](../ROADMAP.md) | IMPLEMENTATION_STATUS "Remaining" column |

<!-- documentation-routing:start -->
## Documentation routing

- [Parent README](README.md)
- [Governing agent guide](../../AGENTS.md)
<!-- documentation-routing:end -->
