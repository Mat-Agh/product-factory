# COMP-015 — `pf` CLI

**Status:** Implemented — v1.0.0 passed 38 Rust tests, Clippy, and a standalone upgrade smoke test
**Last updated:** 2026-11-10
**Source:** [`cli/`](../../cli/) (Rust 2024 edition, one binary `pf`)

## Purpose

All factory tooling is one Rust binary, following the operator's
Rust-by-default rule ([USER.md](../operator/USER.md)). Project setup and job
control use Git. Specific features invoke their own tools: GitHub CLI for
remote health checks, Mermaid CLI for diagram validation, Graphify for code
graphs, `uv` to provision Graphify, and Hermes Agent for routed API workers.

## Install

For project installation and knowledge import, the standalone binary
carries the generic kit. Factory checkout commands such as
`pf mirror` still use a source repository.

```bash
cargo install --path cli                                                     # from a factory checkout
cargo install --git ssh://git@github.com/Mat-Agh/product-factory.git pf     # on any machine with repo access
```

`pf` must be on `PATH` wherever Claude Code runs a factory project. The guard
hook calls it, and it **fails closed** when it is missing: every Bash call is
blocked with the install hint above.

## Subcommands

| Command | Purpose | Contract |
|---|---|---|
| `pf guard` | Claude Code PreToolUse hook. Reads hook JSON on stdin; exit 2 blocks | [COMP-004](COMP-004-guard-hook.md) |
| `pf guard --self-test` | Run the 80 built-in policy cases | [COMP-004](COMP-004-guard-hook.md) |
| `pf docs-map [--fix] [--strict] [--allow <prefix>]` | `docs/` tree, generated README links and map | [COMP-012](COMP-012-docs-gate.md) |
| `pf diagrams check \| validate` | Lint inline Mermaid blocks; parse them with the Mermaid CLI | [COMP-014](COMP-014-diagrams.md) |
| `pf install project <path> \| module <path> --workspace <root> \| user \| all <path>` | Put the factory into a product or a user config | [COMP-007](COMP-007-installer.md) |
| `pf doctor [project]` | Health check; exit code = failures | [COMP-008](COMP-008-doctor.md) |
| `pf mirror add <name> <url> \| push \| bundle [dir]` | Backup remotes and offline bundles | [COMP-009](COMP-009-backup-tools.md) |
| `pf export` | Copy Claude Code memory into `.claude-export/<date>/` for review | [COMP-009](COMP-009-backup-tools.md) |
| `pf job claim \| prepare \| advance \| status \| show \| pause \| resume \| rework \| finish` | Durable job control, Git worktrees and evidence transitions | [Job lifecycle](../architecture/JOB_LIFECYCLE.md) |
| `pf job knowledge <id>` | Inspect committed change signals and candidate project docs for a prepared job | [Knowledge capture](../guide/02-commands.md#inspect-knowledge-capture) |
| `pf graph refresh [--modules]`, `query <question>` or `view` | Build, query or visualize a local Graphify index | [Knowledge structure](COMP-018-knowledge-structure.md#local-code-graph) |
| `pf providers setup`, `check` or `run <job-id> <role> --allow-api` | Configure or dispatch a model route | [Model routing](../guide/01-getting-started.md#model-routing) |
| `pf dashboard [--dir .factory] [--port 8000]` | Serve the dashboard on loopback | [COMP-006](COMP-006-backlog-and-dashboard.md) |
| `pf rules sync` | Generate path-scoped `.claude/rules/modules/<area>.md` from the descriptor | [COMP-016](COMP-016-session-integration.md) |
| `pf memory [--json] [--root <repo>]` | Show the repo's auto memory, for `/factory memory` | [COMP-016](COMP-016-session-integration.md) |
| `pf audit [path] [--modules] [--json]` | Inventory the existing agentic setup and flag clashes before adoption | [COMP-017](COMP-017-adoption.md) |
| `pf knowledge stacks \| plan \| init [--files-only]` | Per-stack knowledge packs: plan the gaps, import missing files and append missing sections (never overwrites) | [COMP-018](COMP-018-knowledge-structure.md) |

`--factory <path>` (global) names the factory checkout for `install`, `doctor`,
`mirror` and `export`. Without it, `pf` uses `$PRODUCT_FACTORY_HOME`, then an
ancestor of the current directory or of the `pf` binary, then the source tree
it was built from.

## Source layout

| File | Holds |
|---|---|
| `cli/src/main.rs` | Argument parsing (clap) and dispatch |
| `cli/src/guard.rs` | Guard policy, the shared `BLOCK`/`ALLOW` cases, the self-test |
| `cli/src/docs_map.rs` | Docs tree checks and generation |
| `cli/src/diagrams.rs` | Mermaid block lint and validation |
| `cli/src/install.rs` | Project and user install, settings merge, legacy migration |
| `cli/src/doctor.rs` | Health check |
| `cli/src/backup.rs` | Mirror and export |
| `cli/src/dashboard.rs` | Loopback static server |
| `cli/src/job.rs` and `cli/src/job_knowledge.rs` | Job state, committed change inspection and evidence gates |
| `cli/src/util.rs` | Paths, git, hashing, dates |

The CLI uses `clap`, `regex`, `serde_json`, `shlex`, and `libc` on Unix.

## Verification

```bash
cd cli && cargo test --release && cargo clippy --release && cargo fmt --check
```

Equivalence with the Python and bash tools it replaced was checked on
2026-10-19:
- **Guard:** identical exit code and message on the 73 policy cases plus 44
  extra edge-case inputs.
- **docs-map:** byte-identical generated files, apart from the "Generated by"
  line, which now names `pf`. Failure reports were identical on a deliberately
  broken tree.
- **Install, doctor, mirror, export:** run end to end, including migrating a
  product that had the legacy Python guard.

## Known limitations

- `pf install user` creates symlinks, so it needs a Unix-like system (Linux,
  macOS, WSL).
- `pf diagrams validate` needs Node.js and a Chrome or Chromium binary. `check`
  needs nothing extra.

## Cross-runtime commands

`pf install project <repo> [--root-only] [--install-graphify] [--configure-models]`
installs the runtime kits, can provision Graphify and can prompt for routing.
`pf graph refresh [--modules]` and `pf graph query <question>` operate on the
checkout selected by `--root` or the current directory. `pf providers setup`,
`pf providers check` and `pf providers run <job-id> <role> --allow-api` select,
validate and dispatch the active role's route. Pass `--root` to select the
workspace. [Model routing](../guide/01-getting-started.md#model-routing)
For local 9Router use, the CLI can read an owner-private router config from
`~/.config/product-factory/router.json`; environment values override it.

`pf job knowledge <id>` reports feature, spike and dependency signals, the
inspected commit and the changed knowledge paths. The command is read-only.
`pf job advance` enforces capture for jobs claimed with the updated kit.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Components](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
