# 01 — Development setup

**Status:** Active
**Last verified:** 2026-10-19 (Ubuntu, Linux 7.0)

The factory is markdown, bash and Python 3 standard library. It has no build
step and no package dependencies.

## Toolchain

| Tool | Version | Role |
|---|---|---|
| git | 2.40+ | worktrees, `merge-tree --write-tree` |
| bash | 3.2+ (the macOS default works) | scripts |
| python3 | 3.9+ | guard hook, tests, docs gate |
| jq | 1.6+ | installer settings merge |
| gh / glab | current | PR and MR operations, doctor checks |
| Claude Code | current | running and observing the factory |
| shellcheck | optional | lint scripts (`shellcheck scripts/*.sh`) |

Install commands per OS are in [guide/01-getting-started.md](../guide/01-getting-started.md#install-them).

## Clone and link

```bash
git clone git@github.com:Mat-Agh/product-factory.git "$HOME/Work/ProductFactory"
cd "$HOME/Work/ProductFactory"
scripts/install.sh user      # your ~/.claude now points at this working tree
```

Because the user install uses symlinks, edits to `kit/` take effect in the
**next** Claude Code session on this machine. Projects that hold a copied kit
only change when you re-run `install.sh project`.

## A scratch product for testing

Keep a throwaway repo for observing changes before they reach real products:

```bash
mkdir -p ~/Work/factory-sandbox && cd ~/Work/factory-sandbox
git init -b main && printf '# Sandbox\n\n## Setup\nnone\n\n## Dev server\nnone\n\n## Gates\nnone\n' > AGENTS.md
git add -A && git commit -m init
~/Work/ProductFactory/scripts/install.sh project .
```

## Gates before every commit

```bash
python3 tests/guard_test.py
bash -n scripts/*.sh
python3 scripts/check-docs.py
scripts/doctor.sh
```

<!-- documentation-routing:start -->
## Documentation routing

- [Parent README](README.md)
- [Governing agent guide](../../AGENTS.md)
<!-- documentation-routing:end -->
