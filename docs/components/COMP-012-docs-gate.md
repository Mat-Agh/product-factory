# COMP-012 — Docs map

**Status:** Implemented in Rust — byte-identical output to the Python original; verified on this repo (2026-10-19)
**Last updated:** 2026-10-19
**Source:** [`cli/src/docs_map.rs`](../../cli/src/docs_map.rs) (`pf docs-map`)
**Rule:** CONSTITUTION §1, documentation structure ([CONSTITUTION](../governance/CONSTITUTION.md))

## Purpose

This tool keeps a repository's documentation in one connected tree under
`docs/`, with every README linked to its parent, its documents and its
subfolders, and a map of the whole tree in `docs/README.md`. The links are
**generated**, so they cannot drift, and **checked**, so a stale or broken map
fails the gate. The same command runs on this repo and on every product. It
needs only `pf` on the PATH.

## Interface

```bash
pf docs-map                    # check (exit 1 on failure)
pf docs-map --fix              # create missing READMEs, regenerate routing blocks and the map, then check
pf docs-map --strict           # also: **Status:** header in every doc; no docs outside docs/
  --root <repo>   --docs <dir> (default docs)   --allow <prefix> (Markdown outside docs/ that is not documentation)
```

This repo runs it as `pf docs-map --strict --allow kit/`
(the kit's agent and skill files are payload, not docs).

## What it generates

| Where | Block | Content |
|---|---|---|
| Every folder `README.md` | `<!-- documentation-routing:start/end -->` | Parent README, documentation home, agent guide, every document in the folder (by title), every subfolder README |
| Every other document | the same markers | Parent folder README, documentation home, agent guide |
| `docs/README.md` | `<!-- documentation-map:start/end -->` | The full tree as a nested list of titles with each document's Status |
| A folder with no README | a stub `README.md` (`Status: Draft`) | for you to fill in |

Generated blocks replace whatever sits between their markers. Text outside the
markers is never touched.

## What it checks

1. Documentation outside `docs/`. The repo root allows only `README.md`,
   `AGENTS.md`, `CLAUDE.md` and tool-required files. This is a warning, or an
   error with `--strict`.
2. Every folder under `docs/` has a `README.md`.
3. Every routing block and the map match what would be generated now.
4. Every relative link in every tracked Markdown file resolves.
5. Every document under `docs/` is reachable by links from `docs/README.md`.
6. With `--strict`, every document has a `**Status:**` line.

It considers only git-tracked and untracked-but-not-ignored files, so
gitignored folders such as `.factory/jobs/` and `.claude-export/` are skipped.
It uses only the Python 3.8+ standard library.

## Where it runs

| Who | When |
|---|---|
| Factory developers | before every commit (a gate in `AGENTS.md`) and inside `pf doctor` |
| Every agent that adds, moves or renames a product doc | `--fix` in the worktree, committed with the change (protocol) |
| reviewer-code | a check failure is `CHANGES` (docs-sync gate) |
| `/factory init` | reports docs outside `docs/` and missing READMEs, and offers the move |
| `pf doctor <product>` | reports the product's map status |

## Known limitations

- It does not validate `#anchor` fragments or external URLs. Diagrams are
  validated by `pf diagrams render` ([COMP-014](COMP-014-diagrams.md)).
- Folder README titles come from their first `# ` heading, so keep headings
  meaningful.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Components](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
