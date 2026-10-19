# Product Factory implementation status

**Status:** Active audit record
**Last updated:** 2026-10-19
**Factory revision:** `main` after v1 documentation (see [EVOLUTION_LOG](EVOLUTION_LOG.md))

This page is the evidence map of what exists, what has been verified, and what is
still open. Update it in the same commit as any change to a component or its
verification. "Implemented" means the code or procedure exists. "Verified" means
there is recorded evidence that it works.

## Summary

| Area | State |
|---|---|
| Rules, roles, procedure | Implemented, not yet exercised on a real product |
| Mechanical enforcement | Guard hook (`pf guard`, Rust) passes 80/80 (the original 73 match the Python guard on 117 inputs); follows the project branch template and blocks hook bypasses; fails closed without `pf`. Branch protection depends on each product |
| Distribution and portability | `pf install`, `pf doctor`, `pf mirror bundle` (with restore) and `pf export` verified on scratch setups. No second remote yet |
| Documentation | All docs under `docs/`, one connected tree. `pf docs-map --strict` and `pf diagrams check` pass |
| Tooling | One Rust CLI (`pf`); no Python, bash scripts or `jq`. 10 cargo tests, clippy clean |
| **Phase 1 exit (roadmap)** | **Not met.** No job has run end to end |

## Components

| Component | Implemented | Verified | Evidence | Remaining |
|---|---|---|---|---|
| [COMP-001](../components/COMP-001-coordinator-skill.md) Coordinator skill | yes | no | — | End-to-end pilot: trivial, standard, bug, idea, release lanes; multi-repo worktrees; `SendMessage` resume |
| [COMP-002](../components/COMP-002-agent-protocol.md) Agent protocol | yes | no | — | Observe in the pilot |
| [COMP-003](../components/COMP-003-role-agents.md) Role agents | yes | partially | Frontmatter follows the Claude Code subagent format | Observe each role in the pilot; confirm they appear in `/agents` |
| [COMP-004](../components/COMP-004-guard-hook.md) Guard hook | yes (Rust) | unit + differential | `cargo test`: 73 policy cases; identical decisions and messages to the Python guard on 117 inputs; the exact hook command tested with and without `pf` on PATH (2026-10-19) | Observe it blocking inside a live Claude Code session |
| [COMP-005](../components/COMP-005-project-descriptor.md) Descriptor | yes (schema v1) | no | — | `/factory init` on a real repo; schema validator (optional) |
| [COMP-006](../components/COMP-006-backlog-and-dashboard.md) Backlog and dashboard | yes | partially | `pf dashboard` served a job listing and `job.json` and refused `../../` escapes (2026-10-19) | Claim race with two sessions; view the dashboard against live jobs |
| [COMP-007](../components/COMP-007-installer.md) Installer | yes (Rust) | yes (scratch) | Fresh install, idempotent re-install, migration of a legacy Python install, user link with `--with-guard`; settings-merge unit test (2026-10-19) | Install on a real product |
| [COMP-008](../components/COMP-008-doctor.md) Doctor | yes (Rust) | yes | Machine, factory and scratch-project runs (2026-10-19) | — |
| [COMP-009](../components/COMP-009-backup-tools.md) Mirror and export | yes (Rust) | yes (local) | Bundle written, verified and cloned back; export against a scratch config (2026-10-19) | Add the first second-host mirror |
| [COMP-010](../components/COMP-010-operator-profile.md) Operator profile | yes | n/a | Distilled from Hermes `USER.md`/`SOUL.md` | Keep current |
| [COMP-011](../components/COMP-011-adapters.md) Adapters | documented | no | — | Run one role from Codex; map to Hermes profiles |
| [COMP-012](../components/COMP-012-docs-gate.md) Docs map | yes | yes | `pf docs-map --fix --strict` on this repo: 62 docs, every folder has a README, every doc is reachable from `docs/README.md` (2026-10-19) | Run on a real product (`/factory init`) |
| [COMP-014](../components/COMP-014-diagrams.md) Diagrams | yes | yes | 7 diagrams parse with Mermaid 10.9.8 and 11.17.2; SVGs rendered and checked as `<img>` in Chrome (2026-10-19) | Confirm display on GitHub from the operator's network |
| [COMP-015](../components/COMP-015-pf-cli.md) `pf` CLI | yes | yes | `cargo test` 10/10, `cargo clippy` 0 warnings, `cargo fmt` clean; every subcommand run end to end (2026-10-19) | Prebuilt binaries (optional, see ADR-009) |
| [COMP-016](../components/COMP-016-session-integration.md) Session integration and memory | yes | partially | Scratch workspace with two nested repos: workspace and module installs, `pf rules sync`, attribution off, doctor flags missing modules; `pf memory` read the real Connect auto memory (9 entries); 15 cargo tests (2026-10-19) | Observe a live Claude session routing a request through `/factory`; run `/factory memory` on Connect |
| [COMP-017](../components/COMP-017-adoption.md) Adoption | yes | partially | `pf audit` read-only on real Connect root, Back-End and Front-End (findings match manual review); scratch repo: foreign agent preserved, template cleared the blocker, guard policy verified (2026-10-19) | Run `/factory adopt` on Connect; land the mirror and `CLAUDE.md` MRs |
| [COMP-013](../components/COMP-013-constitution.md) Constitution | v1 | n/a | — | Revisit after the pilot retro |

## Open operator actions

| Action | Why | How |
|---|---|---|
| Add a second remote and an offline bundle | The account-loss guarantee needs a copy outside GitHub | [guide/05](../guide/05-portability-and-recovery.md) |
| Run the Phase 1 pilot | Nothing is verified end to end | [development/04](../development/04-testing-and-verification.md#3-end-to-end-pilot) |
| Decide on repo visibility | It was created private by default | GitHub settings |
| Enable branch protection on product repos | The guard is only a second line | [guide/04](../guide/04-add-a-project.md) |

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Status](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
