# Components

**Status:** Active
**Last updated:** 2026-10-19

Documentation navigation: [docs index](../README.md) · [routing map](../development/16-documentation-link-map.md) · [complete inventory](../DOCUMENT_INVENTORY.md).

One contract per factory piece: what it is for, its interface, its
configuration, how it is verified, and its known limits. Contracts link to the
source and never restate it. Current delivery status is in
[IMPLEMENTATION_STATUS.md](IMPLEMENTATION_STATUS.md).

| ID | Component | Source |
|---|---|---|
| [COMP-001](COMP-001-coordinator-skill.md) | `/factory` coordinator skill | `kit/.claude/skills/factory/SKILL.md` |
| [COMP-002](COMP-002-agent-protocol.md) | Agent protocol | `kit/.claude/skills/factory/protocol.md` |
| [COMP-003](COMP-003-role-agents.md) | Role agents (10) | `kit/.claude/agents/*.md` |
| [COMP-004](COMP-004-guard-hook.md) | Guard hook | `kit/.claude/hooks/factory-guard.py`, `kit/.claude/settings.factory.json` |
| [COMP-005](COMP-005-project-descriptor.md) | Project descriptor | `kit/.factory/factory.yaml` |
| [COMP-006](COMP-006-backlog-and-dashboard.md) | Backlog and dashboard | `kit/.factory/backlog.md`, `kit/.factory/dashboard.html` |
| [COMP-007](COMP-007-installer.md) | Installer | `scripts/install.sh`, `kit/AGENTS.factory.md` |
| [COMP-008](COMP-008-doctor.md) | Doctor | `scripts/doctor.sh` |
| [COMP-009](COMP-009-backup-tools.md) | Mirror and memory export | `scripts/mirror.sh`, `scripts/export-claude.sh` |
| [COMP-010](COMP-010-operator-profile.md) | Operator profile | `operator/USER.md` |
| [COMP-011](COMP-011-adapters.md) | Adapters | `adapters/*.md` |
| [COMP-012](COMP-012-docs-gate.md) | Documentation gate | `scripts/check-docs.py` |
| [COMP-013](COMP-013-constitution.md) | Constitution | `CONSTITUTION.md` |

Adding a component means adding a `COMP-NNN-*.md` here, a row in this table and
in `IMPLEMENTATION_STATUS.md`, and a route in the
[link map](../development/16-documentation-link-map.md).

<!-- documentation-routing:start -->
## Documentation routing

- [Parent README](../README.md)
- [Governing agent guide](../../AGENTS.md)
<!-- documentation-routing:end -->
