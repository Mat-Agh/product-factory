# COMP-009 — Mirror and memory export

**Status:** Implemented in Rust — bundle written and restored, export verified (2026-10-19); no second host added yet
**Last updated:** 2026-10-19
**Source:** [`cli/src/backup.rs`](../../cli/src/backup.rs)
**Decisions:** [ADR-006](../decisions/ADR-006-portable-distribution.md)

## `pf mirror`

| Command | Effect |
|---|---|
| `pf mirror add <name> <git-url>` | Adds a remote and pushes all branches and tags to it |
| `pf mirror push` | Pushes all branches and tags to every remote, and reports failures per remote |
| `pf mirror bundle [dir]` | Writes and verifies `product-factory-<YYYYMMDD>.bundle` (default `~/Backups/product-factory`) |

Restore from a bundle with `git clone product-factory-<date>.bundle product-factory`.

## `pf export`

Copies the global `CLAUDE.md` and every `~/.claude/projects/*/memory/*.md` into
`<factory>/.claude-export/<date>/`. It also writes an inventory of user-level
skills and agents that the factory does not manage. It copies no credentials,
sessions or transcripts. The folder is gitignored on purpose. Review it, fold
the durable facts into `docs/operator/USER.md` or a product's `AGENTS.md`, then
delete it.

## Verification

On 2026-10-19, a bundle was written and verified, then cloned back into a full
working tree. An export against a scratch config dir produced the expected
files and inventory. Adding a real second host is still pending (see
[IMPLEMENTATION_STATUS.md](../status/IMPLEMENTATION_STATUS.md)).

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Components](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
