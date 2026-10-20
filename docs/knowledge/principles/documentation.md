# Documentation

**Status:** Active
**Last updated:** 2026-10-20

How knowledge is written, found and kept true in every repository.

## Where docs live

- All documentation lives under the docs root (`docs/` by default), one folder per
  area, each with a `README.md` that indexes it. Only `README.md`, `AGENTS.md`,
  `CLAUDE.md`, `CURRENT_STATE.md`, `CONTRIBUTING.md` and `INSTALL.md` stay at the
  repo root.
- Every document starts with a title, a `**Status:**` line and a
  `**Last updated:**` line.
- READMEs link to each other so the docs form one connected map from
  `docs/README.md`; no orphan documents. Generate the links where tooling exists
  (`pf docs-map --fix`).
- Diagrams are sources (Mermaid, Draw.io, DBML) rendered to committed images,
  regenerated in the same change; never hand-edit generated output.

## Routing

- A documentation link map (`development/16-documentation-link-map.md`) routes
  each kind of task to the smallest set of docs. Agents and people read that,
  not whole trees.
- **Knowledge order:** the link map and docs index; the routed docs; upstream
  owners through `cross-project/`; authoritative online sources only for what is
  still missing; then write durable findings back into the docs.

## Writing rules

- One home per fact. Link, never copy. A copied rule drifts.
- Active scope and history are separate: superseded text moves to history or is
  deleted, not left beside the current rule.
- Decisions are ADRs with a status (Proposed, Accepted, Superseded).
- Open questions are named, owned and dated.
- Product owns **what**; implementation repos own **how** (APIs, schema, build).
- A change that alters behaviour, contracts, schema, configuration or commands
  updates the matching docs in the same change. Missing doc updates block review.

## The knowledge every repo keeps

`CURRENT_STATE.md` (what it is now), `docs/history/DEVELOPMENT_LOG.md` (how it got
here, one dated entry per merged change), the numbered handbook (`00` principles,
`09` tools and commands, `15` code samples, `16` link map, stack chapters),
architecture with a conformance ledger, decisions, features with an
implementation-status page, cross-project routers, and temporary sessions that
are promoted or deleted.

## Verification before completion

Inventory Markdown, scan for stale terms and references, validate local links,
parse diagram sources, regenerate generated output, and run `git diff --check`.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Principles](README.md)
- Documentation home: [Product Factory documentation](../../README.md)
- Agent guide: [AGENTS.md](../../../AGENTS.md)
<!-- documentation-routing:end -->
