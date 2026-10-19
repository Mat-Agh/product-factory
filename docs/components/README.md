# Components

**Status:** Active
**Last updated:** 2026-10-19

Documentation navigation: [docs index](../README.md) · [routing map](../development/16-documentation-link-map.md) · [documentation map](../README.md#documentation-map).

One contract per factory piece: what it is for, its interface, its
configuration, how it is verified, and its known limits. Contracts link to the
source and never restate it. Current delivery status is in
[IMPLEMENTATION_STATUS.md](../status/IMPLEMENTATION_STATUS.md).

| ID | Component | Source |
|---|---|---|
| [COMP-001](COMP-001-coordinator-skill.md) | `/factory` coordinator skill | `kit/.claude/skills/factory/SKILL.md` |
| [COMP-002](COMP-002-agent-protocol.md) | Agent protocol | `kit/.claude/skills/factory/protocol.md` |
| [COMP-003](COMP-003-role-agents.md) | Role agents (10) | `kit/.claude/agents/*.md` |
| [COMP-004](COMP-004-guard-hook.md) | Guard hook | `cli/src/guard.rs`, `kit/.claude/settings.factory.json` |
| [COMP-005](COMP-005-project-descriptor.md) | Project descriptor | `kit/.factory/factory.yaml` |
| [COMP-006](COMP-006-backlog-and-dashboard.md) | Backlog and dashboard | `kit/.factory/backlog.md`, `kit/.factory/dashboard.html`, `cli/src/dashboard.rs` |
| [COMP-007](COMP-007-installer.md) | Installer | `cli/src/install.rs`, `kit/AGENTS.factory.md` |
| [COMP-008](COMP-008-doctor.md) | Doctor | `cli/src/doctor.rs` |
| [COMP-009](COMP-009-backup-tools.md) | Mirror and memory export | `cli/src/backup.rs` |
| [COMP-010](COMP-010-operator-profile.md) | Operator profile | `docs/operator/USER.md` |
| [COMP-011](COMP-011-adapters.md) | Adapters | `docs/adapters/*.md` |
| [COMP-012](COMP-012-docs-gate.md) | Docs map (structure and live links) | `cli/src/docs_map.rs` |
| [COMP-013](COMP-013-constitution.md) | Constitution | `docs/governance/CONSTITUTION.md` |
| [COMP-014](COMP-014-diagrams.md) | Diagrams | `docs/assets/diagrams/*.mmd`, `cli/src/diagrams.rs` |
| [COMP-015](COMP-015-pf-cli.md) | `pf` CLI | `cli/` |
| [COMP-016](COMP-016-session-integration.md) | Project session integration and memory | `kit/.claude/rules/product-factory.md`, `cli/src/{rules,memory,install}.rs` |
| [COMP-017](COMP-017-adoption.md) | Adoption of an existing project | `cli/src/audit.rs`, `/factory adopt` |
| [COMP-018](COMP-018-knowledge-structure.md) | Knowledge structure per stack | `kit/knowledge/`, `cli/src/knowledge.rs` |

Adding a component means adding a `COMP-NNN-*.md` here, a row in this table and
in `IMPLEMENTATION_STATUS.md`, and a route in the
[link map](../development/16-documentation-link-map.md).

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
- In this folder:
  - [COMP-001 — `/factory` coordinator skill](COMP-001-coordinator-skill.md)
  - [COMP-002 — Agent protocol](COMP-002-agent-protocol.md)
  - [COMP-003 — Role agents](COMP-003-role-agents.md)
  - [COMP-004 — Guard hook](COMP-004-guard-hook.md)
  - [COMP-005 — Project descriptor](COMP-005-project-descriptor.md)
  - [COMP-006 — Backlog and dashboard](COMP-006-backlog-and-dashboard.md)
  - [COMP-007 — Installer](COMP-007-installer.md)
  - [COMP-008 — Doctor](COMP-008-doctor.md)
  - [COMP-009 — Mirror and memory export](COMP-009-backup-tools.md)
  - [COMP-010 — Operator profile](COMP-010-operator-profile.md)
  - [COMP-011 — Adapters](COMP-011-adapters.md)
  - [COMP-012 — Docs map](COMP-012-docs-gate.md)
  - [COMP-013 — Constitution](COMP-013-constitution.md)
  - [COMP-014 — Diagrams](COMP-014-diagrams.md)
  - [COMP-015 — `pf` CLI](COMP-015-pf-cli.md)
  - [COMP-016 — Project session integration and memory](COMP-016-session-integration.md)
  - [COMP-017 — Adoption of an existing project](COMP-017-adoption.md)
  - [COMP-018 — Knowledge structure per stack](COMP-018-knowledge-structure.md)
<!-- documentation-routing:end -->
