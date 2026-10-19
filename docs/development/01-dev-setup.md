# 01 — Development setup

**Status:** Active
**Last verified:** 2026-10-19 (Ubuntu, Linux 7.0)

The factory is markdown plus one Rust CLI, `pf`, in `cli/`. Build and install it
with `cargo install --path cli`.

## Toolchain

| Tool | Version | Role |
|---|---|---|
| git | 2.40+ | worktrees, `merge-tree --write-tree` |
| Rust (cargo) | 1.85+ (edition 2024) | builds `pf`, the only factory tool |
| gh / glab | current | PR and MR operations, doctor checks |
| Claude Code | current | running and observing the factory |
| Node.js | 18+, optional | `pf diagrams render` (Mermaid CLI) |

Install commands per OS are in [guide/01-getting-started.md](../guide/01-getting-started.md#install-them).

## Clone and link

```bash
git clone git@github.com:Mat-Agh/product-factory.git "$HOME/Work/ProductFactory"
cd "$HOME/Work/ProductFactory"
cargo install --path cli   # builds and installs pf
pf install user           # your ~/.claude now points at this working tree
```

Because the user install uses symlinks, edits to `kit/` take effect in the
**next** Claude Code session on this machine. Projects that hold a copied kit
only change when you re-run `pf install project`.

## A scratch product for testing

Keep a throwaway repo for observing changes before they reach real products:

```bash
mkdir -p ~/Work/factory-sandbox && cd ~/Work/factory-sandbox
git init -b main && printf '# Sandbox\n\n## Setup\nnone\n\n## Dev server\nnone\n\n## Gates\nnone\n' > AGENTS.md
git add -A && git commit -m init
pf install project .
```

## Gates before every commit

```bash
(cd cli && cargo test --release && cargo clippy --release && cargo fmt --check)
pf guard --self-test
pf docs-map --strict --allow kit/
pf diagrams check
pf doctor
```

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Factory development handbook](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
