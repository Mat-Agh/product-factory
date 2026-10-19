# 09 — Tools, scripts and commands

**Status:** Active
**Last updated:** 2026-10-19

Use these before inventing ad-hoc commands. Each script prints its usage when
run without arguments.

## Scripts

| Script | Purpose | Contract |
|---|---|---|
| `scripts/install.sh` | Install the factory into a product (copy) or a user profile (symlinks) | [COMP-007](../components/COMP-007-installer.md) |
| `scripts/doctor.sh` | Health check. Exit code = number of failures | [COMP-008](../components/COMP-008-doctor.md) |
| `scripts/mirror.sh` | Backup remotes and offline bundles | [COMP-009](../components/COMP-009-backup-tools.md) |
| `scripts/export-claude.sh` | Copy Claude Code memory into the repo for review | [COMP-009](../components/COMP-009-backup-tools.md) |
| `kit/.factory/docs-map.py` | Docs tree check and live map generator (also installed into products) | [COMP-012](../components/COMP-012-docs-gate.md) |
| `scripts/render-diagrams.sh` | Render Mermaid sources to SVG; `--check` for freshness | [COMP-014](../components/COMP-014-diagrams.md) |
| `tests/guard_test.py` | Guard hook policy tests | [COMP-004](../components/COMP-004-guard-hook.md) |

## Samples

```bash
# Install
scripts/install.sh user
scripts/install.sh project ~/Work/MyApp
scripts/install.sh all ~/Work/MyApp
WITH_USER_GUARD=1 scripts/install.sh user          # guard in every project on this machine
CLAUDE_CONFIG_DIR=/tmp/h scripts/install.sh user   # install into another Claude config dir

# Health
scripts/doctor.sh
scripts/doctor.sh ~/Work/MyApp

# Backups
scripts/mirror.sh add gitlab git@gitlab.com:<you>/product-factory.git
scripts/mirror.sh push
scripts/mirror.sh bundle ~/Backups/product-factory

# Memory
scripts/export-claude.sh        # then review .claude-export/<date>/ and delete it

# Gates
python3 tests/guard_test.py
python3 kit/.factory/docs-map.py --strict --allow kit/
python3 kit/.factory/docs-map.py --fix --strict --allow kit/
bash -n scripts/*.sh
scripts/render-diagrams.sh --check
scripts/render-diagrams.sh          # after editing a .mmd

# Try the guard by hand
echo '{"tool_name":"Bash","tool_input":{"command":"git merge main"}}' \
  | python3 kit/.claude/hooks/factory-guard.py; echo "exit=$?"   # exit=2 with reason

# Inspect a job in a product
head -n1 .factory/jobs/<id>/*.md .factory/jobs/<id>/round-*/*.md
jq . .factory/jobs/<id>/job.json
git worktree list
```

## Environment variables

| Variable | Read by | Effect |
|---|---|---|
| `CLAUDE_CONFIG_DIR` | install.sh, doctor.sh, export-claude.sh | The Claude config dir (default `~/.claude`) |
| `WITH_USER_GUARD=1` | install.sh user | Also wire the guard in the user `settings.json` |
| `FACTORY_GUARD=off` | factory-guard.py | Disable the guard for a human-driven session (set before starting `claude`) |
| `CLAUDE_PROJECT_DIR` | hook command | Set by Claude Code. Locates the project guard script |
| `MMDC`, `MERMAID_CHROME` | render-diagrams.sh | Mermaid CLI binary and Chrome binary to use |

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Factory development handbook](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
