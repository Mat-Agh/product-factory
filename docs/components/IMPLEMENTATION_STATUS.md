# Product Factory implementation status

**Status:** Active audit record
**Last updated:** 2026-10-19
**Factory revision:** `main` after v1 documentation (see [EVOLUTION_LOG](../EVOLUTION_LOG.md))

This page is the evidence map of what exists, what has been verified, and what is
still open. Update it in the same commit as any change to a component or its
verification. "Implemented" means the code or procedure exists. "Verified" means
there is recorded evidence that it works.

## Summary

| Area | State |
|---|---|
| Rules, roles, procedure | Implemented, not yet exercised on a real product |
| Mechanical enforcement | Guard hook implemented and unit-tested. Branch protection depends on each product |
| Distribution and portability | Installer and doctor verified on a scratch repo. Mirrors and bundles not yet set up |
| Documentation | Complete for v1. `check-docs.py` passes |
| **Phase 1 exit (roadmap)** | **Not met.** No job has run end to end |

## Components

| Component | Implemented | Verified | Evidence | Remaining |
|---|---|---|---|---|
| [COMP-001](COMP-001-coordinator-skill.md) Coordinator skill | yes | no | — | End-to-end pilot: trivial, standard, bug, idea, release lanes; multi-repo worktrees; `SendMessage` resume |
| [COMP-002](COMP-002-agent-protocol.md) Agent protocol | yes | no | — | Observe in the pilot |
| [COMP-003](COMP-003-role-agents.md) Role agents | yes | partially | Frontmatter follows the Claude Code subagent format | Observe each role in the pilot; confirm they appear in `/agents` |
| [COMP-004](COMP-004-guard-hook.md) Guard hook | yes | unit | `tests/guard_test.py` 73/73 (2026-10-19) | Observe it blocking inside a live Claude Code session |
| [COMP-005](COMP-005-project-descriptor.md) Descriptor | yes (schema v1) | no | — | `/factory init` on a real repo; schema validator (optional) |
| [COMP-006](COMP-006-backlog-and-dashboard.md) Backlog and dashboard | yes | no | — | Claim race with two sessions; view the dashboard against live jobs |
| [COMP-007](COMP-007-installer.md) Installer | yes | yes (scratch) | Install, re-install and user link on a scratch repo, doctor 0 failures (2026-10-19) | Install on a real product |
| [COMP-008](COMP-008-doctor.md) Doctor | yes | yes | Machine and scratch-project runs (2026-10-19) | — |
| [COMP-009](COMP-009-backup-tools.md) Mirror and export | yes | syntax only | `bash -n` | Add the first mirror remote; write and restore a bundle |
| [COMP-010](COMP-010-operator-profile.md) Operator profile | yes | n/a | Distilled from Hermes `USER.md`/`SOUL.md` | Keep current |
| [COMP-011](COMP-011-adapters.md) Adapters | documented | no | — | Run one role from Codex; map to Hermes profiles |
| [COMP-012](COMP-012-docs-gate.md) Docs gate | yes | yes | `check-docs.py` passes (2026-10-19) | — |
| [COMP-013](COMP-013-constitution.md) Constitution | v1 | n/a | — | Revisit after the pilot retro |

## Open operator actions

| Action | Why | How |
|---|---|---|
| Add a second remote and an offline bundle | The account-loss guarantee needs a copy outside GitHub | [guide/05](../guide/05-portability-and-recovery.md) |
| Run the Phase 1 pilot | Nothing is verified end to end | [development/04](../development/04-testing-and-verification.md#3-end-to-end-pilot) |
| Decide on repo visibility | It was created private by default | GitHub settings |
| Enable branch protection on product repos | The guard is only a second line | [guide/04](../guide/04-add-a-project.md) |

<!-- documentation-routing:start -->
## Documentation routing

- [Parent README](README.md)
- [Governing agent guide](../../AGENTS.md)
<!-- documentation-routing:end -->
