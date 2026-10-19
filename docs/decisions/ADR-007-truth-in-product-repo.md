# ADR-007 — Product truth stays in the product repo

**Status:** Accepted
**Last updated:** 2026-10-19
**Date:** 2026-10-19

## Context

Hermes learned that copying a product's docs and IDE rules into the harness
creates stale duplicates, and it moved to reading live files through a project
descriptor. The video's agents also carry nothing repo-specific and read
`AGENTS.md`.

## Decision

- The factory reads exactly two project-specific files: the product's
  `AGENTS.md` (**how**: setup, dev server, gates, conventions, docs map) and
  `.factory/factory.yaml` (**where** and **policy**: modules, VCS, docs paths,
  surfaces, risk, limits). Neither repeats the other.
- Durable outputs (PRDs, ADRs, design docs, contract and API docs, changelog)
  are written into the product repo **in the job branch**, and reach the base
  branch only through the human-merged PR.
- Job folders are scratch space. The PR body carries the summary.

## Consequences

- The factory is only as good as the product's `AGENTS.md`. `/factory init` and
  `doctor.sh` report the gaps, and
  [AGENTS.example.md](../guide/examples/AGENTS.example.md) shows the target.
- Losing a machine loses only the job notes, never product knowledge.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Architecture decisions](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
