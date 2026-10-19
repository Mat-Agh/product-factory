# Documentation inventory

**Status:** Generated completeness map
**Last regenerated:** 2026-10-19

All 50 Markdown documents under `docs/`. Regenerate with `python3 scripts/check-docs.py --inventory`; do not edit by hand.

| Document | Title | Status |
|---|---|---|
| [EVOLUTION_LOG.md](EVOLUTION_LOG.md) | Evolution log | Active — append-only |
| [README.md](README.md) | Product Factory documentation | Active |
| [ROADMAP.md](ROADMAP.md) | Roadmap | Active |
| [architecture/FACTORY_OVERVIEW.md](architecture/FACTORY_OVERVIEW.md) | Factory overview | Active |
| [architecture/JOB_LIFECYCLE.md](architecture/JOB_LIFECYCLE.md) | Job lifecycle | Active |
| [architecture/README.md](architecture/README.md) | Architecture | Active |
| [architecture/SOURCES.md](architecture/SOURCES.md) | Research sources and design synthesis | Active — research record |
| [components/COMP-001-coordinator-skill.md](components/COMP-001-coordinator-skill.md) | COMP-001 — `/factory` coordinator skill | Implemented — not yet run end to end on a real product |
| [components/COMP-002-agent-protocol.md](components/COMP-002-agent-protocol.md) | COMP-002 — Agent protocol | Implemented |
| [components/COMP-003-role-agents.md](components/COMP-003-role-agents.md) | COMP-003 — Role agents | Implemented — not yet run end to end on a real product |
| [components/COMP-004-guard-hook.md](components/COMP-004-guard-hook.md) | COMP-004 — Guard hook | Implemented — unit-tested (73/73); not yet observed inside a live Claude Code session |
| [components/COMP-005-project-descriptor.md](components/COMP-005-project-descriptor.md) | COMP-005 — Project descriptor | Implemented — schema version 1 |
| [components/COMP-006-backlog-and-dashboard.md](components/COMP-006-backlog-and-dashboard.md) | COMP-006 — Backlog and dashboard | Implemented — dashboard not yet viewed against live jobs |
| [components/COMP-007-installer.md](components/COMP-007-installer.md) | COMP-007 — Installer | Implemented — smoke-tested on a scratch repo (install, re-install, user link) on 2026-10-19 |
| [components/COMP-008-doctor.md](components/COMP-008-doctor.md) | COMP-008 — Doctor | Implemented — verified on this machine and a scratch project on 2026-10-19 |
| [components/COMP-009-backup-tools.md](components/COMP-009-backup-tools.md) | COMP-009 — Mirror and memory export | Implemented — syntax-checked; not yet run against a second host |
| [components/COMP-010-operator-profile.md](components/COMP-010-operator-profile.md) | COMP-010 — Operator profile | Implemented |
| [components/COMP-011-adapters.md](components/COMP-011-adapters.md) | COMP-011 — Adapters | Documented — procedures only, not exercised |
| [components/COMP-012-docs-gate.md](components/COMP-012-docs-gate.md) | COMP-012 — Documentation gate | Implemented |
| [components/COMP-013-constitution.md](components/COMP-013-constitution.md) | COMP-013 — Constitution | Active — v1 |
| [components/IMPLEMENTATION_STATUS.md](components/IMPLEMENTATION_STATUS.md) | Product Factory implementation status | Active audit record |
| [components/README.md](components/README.md) | Components | Active |
| [decisions/ADR-001-claude-code-primary-runtime.md](decisions/ADR-001-claude-code-primary-runtime.md) | ADR-001 — Claude Code is the primary runtime; roles stay tool-neutral | Accepted |
| [decisions/ADR-002-human-only-merge.md](decisions/ADR-002-human-only-merge.md) | ADR-002 — Humans merge; the gate opens a PR | Accepted |
| [decisions/ADR-003-files-not-conversation.md](decisions/ADR-003-files-not-conversation.md) | ADR-003 — Context through files with first-line verdicts | Accepted |
| [decisions/ADR-004-review-lenses-and-qa.md](decisions/ADR-004-review-lenses-and-qa.md) | ADR-004 — Parallel review lenses, separate QA, bounded rework | Accepted |
| [decisions/ADR-005-worktree-per-job.md](decisions/ADR-005-worktree-per-job.md) | ADR-005 — One worktree, branch and `job.json` per job | Accepted |
| [decisions/ADR-006-portable-distribution.md](decisions/ADR-006-portable-distribution.md) | ADR-006 — Portable distribution with no account dependency | Accepted |
| [decisions/ADR-007-truth-in-product-repo.md](decisions/ADR-007-truth-in-product-repo.md) | ADR-007 — Product truth stays in the product repo | Accepted |
| [decisions/ADR-008-idea-and-release-lanes.md](decisions/ADR-008-idea-and-release-lanes.md) | ADR-008 — Idea and release lanes; launch material is drafts only | Accepted |
| [decisions/README.md](decisions/README.md) | Architecture decisions | Active |
| [development/00-factory-engineering-principles.md](development/00-factory-engineering-principles.md) | 00 — Factory engineering principles | Active |
| [development/01-dev-setup.md](development/01-dev-setup.md) | 01 — Development setup | Active |
| [development/02-repository-layout.md](development/02-repository-layout.md) | 02 — Repository layout | Active |
| [development/03-changing-the-factory.md](development/03-changing-the-factory.md) | 03 — Changing the factory | Active |
| [development/04-testing-and-verification.md](development/04-testing-and-verification.md) | 04 — Testing and verification | Active |
| [development/05-releases-and-versioning.md](development/05-releases-and-versioning.md) | 05 — Releases and versioning | Active |
| [development/09-tools-scripts-and-commands.md](development/09-tools-scripts-and-commands.md) | 09 — Tools, scripts and commands | Active |
| [development/16-documentation-link-map.md](development/16-documentation-link-map.md) | 16 — Documentation link map | Active — agent routing authority |
| [development/README.md](development/README.md) | Factory development handbook | Active |
| [guide/00-overview.md](guide/00-overview.md) | 00 — How the Product Factory works | Active |
| [guide/01-getting-started.md](guide/01-getting-started.md) | 01 — Getting started | Active |
| [guide/02-commands.md](guide/02-commands.md) | 02 — Commands | Active |
| [guide/03-reading-a-job.md](guide/03-reading-a-job.md) | 03 — Reading a job | Active |
| [guide/04-add-a-project.md](guide/04-add-a-project.md) | 04 — Add a project | Active |
| [guide/05-portability-and-recovery.md](guide/05-portability-and-recovery.md) | 05 — Portability and recovery | Active |
| [guide/06-troubleshooting.md](guide/06-troubleshooting.md) | 06 — Troubleshooting | Active |
| [guide/GLOSSARY.md](guide/GLOSSARY.md) | Glossary | Active |
| [guide/README.md](guide/README.md) | Operator guide | Active |
| [guide/examples/AGENTS.example.md](guide/examples/AGENTS.example.md) | Example product `AGENTS.md` | Example — copy the structure, replace every command with your own |
