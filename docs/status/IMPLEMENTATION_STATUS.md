# Product Factory implementation status

**Status:** Active audit record
**Last updated:** 2026-11-15
**Factory revision:** `main` after v1 documentation (see [EVOLUTION_LOG](EVOLUTION_LOG.md))

This page is the evidence map of what exists, what has been verified, and what is
still open. Update it in the same commit as any change to a component or its
verification. "Implemented" means the code or procedure exists. "Verified" means
there is recorded evidence that it works.

## Summary

| Area | State |
|---|---|
| Rules, roles, procedure | Rust-backed state transitions verified with fixture artifacts. Live role execution remains open |
| Mechanical enforcement | Guard hook (`pf guard`, Rust) passes 80/80 (the original 73 match the Python guard on 117 inputs); follows the project branch template and blocks hook bypasses; fails closed without `pf`. Branch protection depends on each product |
| Distribution and portability | `pf install`, `pf doctor`, `pf mirror bundle` (with restore) and `pf export` verified on scratch setups. No second remote yet |
| Documentation | All docs under `docs/`, one connected tree. `pf docs-map --strict` and `pf diagrams check` pass |
| Tooling | One Rust CLI (`pf`); no Python, bash scripts or `jq`. 31 unit tests and 2 job integration tests, Clippy clean |
| **Phase 1 exit (roadmap)** | **Open.** Codex reached a verified local gate after terminal-assisted Git preparation. Claude Code delivery and hosted PR acceptance remain open |

## Components

| Component | Implemented | Verified | Evidence | Remaining |
|---|---|---|---|---|
| [COMP-001](../components/COMP-001-coordinator-skill.md) Coordinator skill | yes | mechanical paths | CLI integration verified a standard route with concurrent claims, independent worktrees, review rework, QA, gate and local delivery. A second case verified module-specific Git refs | Run native Claude Code and Codex roles against the v1 criteria |
| [COMP-002](../components/COMP-002-agent-protocol.md) Agent protocol | yes | partially | Graphify navigation tested on a disposable checkout | Observe agents following the graph and source checks in a live lane |
| [COMP-003](../components/COMP-003-role-agents.md) Role agents | yes | partially | Frontmatter follows the Claude Code subagent format | Observe each role in the pilot; confirm they appear in `/agents` |
| [COMP-004](../components/COMP-004-guard-hook.md) Guard hook | yes (Rust) | unit + differential | `cargo test`: 73 policy cases; identical decisions and messages to the Python guard on 117 inputs; the exact hook command tested with and without `pf` on PATH (2026-10-19) | Observe it blocking inside a live Claude Code session |
| [COMP-005](../components/COMP-005-project-descriptor.md) Descriptor | yes (schema v1) | no | — | `/factory init` on a real repo; schema validator (optional) |
| [COMP-006](../components/COMP-006-backlog-and-dashboard.md) Backlog and dashboard | yes | scratch | Concurrent processes claimed distinct backlog lines under the active-job limit. Dashboard serves module and review progress from durable job state | Observe live model-driven jobs |
| [COMP-007](../components/COMP-007-installer.md) Installer | yes (Rust) | yes | Claude, Codex, Antigravity, Gemini and Hermes entry points installed in a multi-module fixture. A binary-only Linux build installed both kits and passed doctor after removal of its build checkout. On 2026-11-15, the kit was installed in the real Connect workspace and five child repos; its reviewed descriptor and module rules passed `pf doctor` without failures | Run a live issue through delivery |
| [COMP-008](../components/COMP-008-doctor.md) Doctor | yes (Rust) | yes | Machine, factory and scratch-project runs, including Graphify and Hermes availability | — |
| [COMP-009](../components/COMP-009-backup-tools.md) Mirror and export | yes (Rust) | yes (local) | Bundle written, verified and cloned back; export against a scratch config (2026-10-19) | Add the first second-host mirror |
| [COMP-010](../components/COMP-010-operator-profile.md) Operator profile | yes | n/a | Distilled from Hermes `USER.md`/`SOUL.md` | Keep current |
| [COMP-011](../components/COMP-011-adapters.md) Adapters | Codex kit and manual adapters | partially | Generated Codex and Antigravity roles and shared Hermes/Gemini entry points. A synthetic Hermes worker exercised authorization, key checks and stage artifact handoff. The local Hermes resolver selected a test gateway and its key (2026-11-10) | The local Antigravity CLI listing did not show workspace agents in the scratch checkout. Verify discovery from an opened workspace, then run native dispatch and paid provider lanes |
| [COMP-012](../components/COMP-012-docs-gate.md) Docs map | yes | yes | `pf docs-map --fix --strict` on this repo: 62 docs, every folder has a README, every doc is reachable from `docs/README.md` (2026-10-19) | Run on a real product (`/factory init`) |
| [COMP-014](../components/COMP-014-diagrams.md) Diagrams | yes | yes | 29 inline Mermaid blocks lint clean and parse with Mermaid CLI 11; `validate` reports a broken block with file and line (2026-10-20) | Confirm live rendering on GitHub and in IntelliJ with the Mermaid extension |
| [COMP-015](../components/COMP-015-pf-cli.md) `pf` CLI | yes | yes | `v1.0.0` passed all 38 Rust tests, Clippy, formatting and a standalone upgrade smoke test. On 2026-11-15, the local private router config passed `pf providers check` for all three Connect routes after `cargo install --path cli`; Rust tests and Clippy passed | Live model-driven job and hosted delivery checks remain |
| [COMP-016](../components/COMP-016-session-integration.md) Session integration and memory | yes | partially | Scratch workspace with two nested repos: workspace and module installs, `pf rules sync`, attribution off, doctor flags missing modules; `pf memory` read a project auto-memory directory; 15 cargo tests (2026-10-19) | Observe a live Claude session routing a request through `/factory`; run `/factory memory` on an adopted project |
| [COMP-017](../components/COMP-017-adoption.md) Adoption | yes | partially | Read-only audit found five Connect modules. The real workspace descriptor now uses the accepted `codex/{job-id}` branch shape, attribution off, and generated module rules; `pf doctor` passed on 2026-11-15 | Resolve the knowledge import and docs-map notes, then run a live delivery and land the needed MRs |
| [COMP-018](../components/COMP-018-knowledge-structure.md) Knowledge structure | yes | partially | One development knowledge library (`docs/knowledge/`: principles, Kotlin client and Spring backend, TypeScript web, Rust, API, security, product, CI/CD, agentic) imported through stack pack maps; links between library pages follow the map; on copies of five module repositories every added link resolved and a rerun changed nothing; 8 knowledge tests incl. library coverage and a fresh-import link check per stack (2026-10-20) | Resolve five Client and Front-End numbering collisions, then run approved import and adapt jobs in an adopted workspace. The November docs-only copy verified flat Front-End and top-level Product handbooks and zero changes on a second import |
| [COMP-013](../components/COMP-013-constitution.md) Constitution | v1 | n/a | — | Revisit after the pilot retro |

## Knowledge capture evidence (2026-11-10)

- New jobs on the updated kit checked dependency changes against committed
  documentation. A missing page or reviewer check stopped advancement, and a
  job started on the older kit could still advance.
- Detection of new practices outside the committed signals remains an agent
  and reviewer judgment. Shared pack promotion remains a separate docs job.

## Live runtime evidence (2026-11-10)

- Codex's configured default was unavailable to the signed-in account. An
  account-listed model started the coordinator. Its restricted session
  paused a fixture job when Git denied creation of its branch ref.
- After the fixture worktree was prepared from the terminal, Codex resumed the
  recorded job and ran the installed code reviewer. The reviewer found a
  missing date in the development log, wrote `VERDICT: CHANGES`, and the
  coordinator advanced the job to build round 2. After the fixture correction,
  a second native review passed and the job reached the gate. The live test
  timed out during tech-lead validation. A new Codex session resumed the gate,
  wrote a real `READY` verdict and recorded the local delivery. The Codex
  fixture used terminal-assisted Git preparation and commits.
- Claude Code registered the skill and all ten project roles, then declined
  model use on the logged-in subscription. Live Claude delivery and hosted
  PR delivery still need acceptance evidence.

## Open operator actions

| Action | Why | How |
|---|---|---|
| Add a second remote and an offline bundle | The account-loss guarantee needs a copy outside GitHub | [guide/05](../guide/05-portability-and-recovery.md) |
| Run live Claude Code delivery and a hosted PR check | The signed-in Claude subscription declined model use on retry. Hosted PR delivery needs an approved test repository | [development/04](../development/04-testing-and-verification.md#3-end-to-end-pilot) |
| Decide on repo visibility | It was created private by default | GitHub settings |
| Enable branch protection on product repos | The guard is only a second line | [guide/04](../guide/04-add-a-project.md) |

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Status](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
