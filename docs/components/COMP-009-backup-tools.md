# COMP-009 — Mirror and memory export

**Status:** Implemented — syntax-checked; not yet run against a second host
**Last updated:** 2026-10-19
**Source:** [`scripts/mirror.sh`](../../scripts/mirror.sh), [`scripts/export-claude.sh`](../../scripts/export-claude.sh)
**Decisions:** [ADR-006](../decisions/ADR-006-portable-distribution.md)

## `mirror.sh`

| Command | Effect |
|---|---|
| `mirror.sh add <name> <git-url>` | Adds a remote and pushes all branches and tags to it |
| `mirror.sh push` | Pushes all branches and tags to every remote, and reports per-remote failures |
| `mirror.sh bundle [dir]` | Writes and verifies `product-factory-<YYYYMMDD>.bundle` (default `~/Backups/product-factory`). Restore with `git clone <bundle> product-factory` |

## `export-claude.sh`

Copies the global `CLAUDE.md` and every `~/.claude/projects/*/memory/*.md` into
`operator/claude-export/<date>/`, plus an inventory of user-level skills and
agents the factory does not manage. It copies no credentials, sessions or
transcripts. The folder is gitignored on purpose. Review it, fold the durable
facts into `operator/USER.md` or a product's `AGENTS.md`, then delete it.

## Verification

`bash -n` passes for both scripts. Live runs are pending the first mirror
setup (see [IMPLEMENTATION_STATUS.md](IMPLEMENTATION_STATUS.md)).

<!-- documentation-routing:start -->
## Documentation routing

- [Parent README](README.md)
- [Governing agent guide](../../AGENTS.md)
<!-- documentation-routing:end -->
