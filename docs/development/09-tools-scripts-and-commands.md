# 09 — Tools and commands

**Status:** Active
**Last updated:** 2026-10-19

All factory tooling is one Rust binary, `pf` ([COMP-015](../components/COMP-015-pf-cli.md)).
Use it before inventing ad-hoc commands. `pf --help` and `pf <command> --help`
print usage.

## Subcommands

| Command | Purpose | Contract |
|---|---|---|
| `pf guard` | Claude Code PreToolUse hook (stdin JSON → exit 0 or 2) | [COMP-004](../components/COMP-004-guard-hook.md) |
| `pf guard --self-test` | Run the 80 policy cases | [COMP-004](../components/COMP-004-guard-hook.md) |
| `pf docs-map` | Docs tree check and live map generator | [COMP-012](../components/COMP-012-docs-gate.md) |
| `pf diagrams check \| render` | Mermaid sources to SVG; freshness check | [COMP-014](../components/COMP-014-diagrams.md) |
| `pf install project \| user \| all` | Put the factory into a product (copy) or a user config (links) | [COMP-007](../components/COMP-007-installer.md) |
| `pf doctor [project]` | Health check; exit code = failures | [COMP-008](../components/COMP-008-doctor.md) |
| `pf mirror add \| push \| bundle` | Backup remotes and offline bundles | [COMP-009](../components/COMP-009-backup-tools.md) |
| `pf export` | Copy Claude Code memory into the repo for review | [COMP-009](../components/COMP-009-backup-tools.md) |
| `pf dashboard` | Serve a product's `.factory/` on loopback | [COMP-006](../components/COMP-006-backlog-and-dashboard.md) |
| `pf install module` | Kit for one module repo of a multi-repo workspace | [COMP-016](../components/COMP-016-session-integration.md) |
| `pf rules sync` | Path-scoped module rules from the descriptor | [COMP-016](../components/COMP-016-session-integration.md) |
| `pf memory` | Show a repo's auto memory | [COMP-016](../components/COMP-016-session-integration.md) |
| `pf audit` | Inventory an existing project's agentic setup before adoption | [COMP-017](../components/COMP-017-adoption.md) |
| `pf knowledge stacks \| plan \| init` | The per-stack knowledge structure (state, history, handbook, areas) | [COMP-018](../components/COMP-018-knowledge-structure.md) |

## Samples

```bash
# Build and install pf
cargo install --path cli
cargo install --git ssh://git@github.com/Mat-Agh/product-factory.git pf

# Install the factory
pf install user
pf install user --with-guard           # also guard every project on this machine
pf install project ~/Work/MyApp
pf install all ~/Work/MyApp
pf install project ~/Work/Connect --no-ai-attribution   # workspace root
pf install module Daria-Connect-Back-End --workspace ~/Work/Connect
pf rules sync                           # in the workspace
pf memory                               # this repo's auto memory
pf audit ~/Work/Connect/Daria-Connect-Back-End   # adoption inventory (read-only)
pf audit . --modules --json             # whole workspace
pf knowledge plan --modules             # knowledge gaps per repo
pf knowledge init --root Daria-Connect-Federation   # import missing files and sections (inside a docs job worktree)
pf memory --json --root ~/Work/Connect
CLAUDE_CONFIG_DIR=/tmp/h pf install user

# Health
pf doctor
pf doctor ~/Work/MyApp

# Backups
pf mirror add gitlab git@gitlab.com:<you>/product-factory.git
pf mirror push
pf mirror bundle ~/Backups/product-factory

# Memory
pf export                               # then review .claude-export/<date>/ and delete it

# Gates (factory repo)
cd cli && cargo test --release && cargo clippy --release && cd ..
pf guard --self-test
pf docs-map --strict --allow kit/
pf docs-map --fix --strict --allow kit/
pf diagrams check
pf diagrams render                      # after editing a .mmd

# Try the guard by hand
echo '{"tool_name":"Bash","tool_input":{"command":"git merge main"}}' | pf guard; echo "exit=$?"   # exit=2 with reason

# In a product
pf docs-map --fix
pf dashboard                            # http://127.0.0.1:8000/dashboard.html
head -n1 .factory/jobs/<id>/*.md .factory/jobs/<id>/round-*/*.md
git worktree list
```

## Environment variables

| Variable | Read by | Effect |
|---|---|---|
| `PRODUCT_FACTORY_HOME` | `pf install`, `doctor`, `mirror`, `export` | Factory checkout (or pass `--factory <path>`) |
| `CLAUDE_CONFIG_DIR` | `pf install user`, `doctor`, `export` | Claude config dir (default `~/.claude`) |
| `FACTORY_GUARD=off` | `pf guard` | Disable the guard for a human-driven session (set before starting `claude`) |
| `MMDC`, `MERMAID_CHROME` | `pf diagrams render` | Mermaid CLI binary and Chrome binary to use |

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Factory development handbook](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
