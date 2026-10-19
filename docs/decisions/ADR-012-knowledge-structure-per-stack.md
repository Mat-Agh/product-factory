# ADR-012 — One knowledge structure per stack, distilled from Connect

**Status:** Accepted
**Last updated:** 2026-10-19
**Date:** 2026-10-19

## Context

Before the factory, the operator built a knowledge practice across the five Daria
Connect repos:
- a `CURRENT_STATE.md` per repo;
- a docs router;
- numbered development handbooks (`00` principles, `09` tools, `15` code samples,
  `16` link map, plus stack chapters);
- architecture with a conformance ledger;
- ADRs, feature contracts with an implementation-status audit, and
  cross-project routers;
- dated session handoffs.

The practice was uneven between repos. Federation lacked most of it, Front-End
and Product used their own layouts, and no repo kept a running development
history. New projects had no template at all.

## Decision

- Capture the practice as **templates**: a `base/` shared by every repo and one
  overlay per stack (`spring-kotlin-service`, `kmp-android-client`,
  `typescript-web`, `product-docs`), plus `generic`. Shared numbers keep their
  meaning everywhere.
- Add a **development history** (`history/DEVELOPMENT_LOG.md`, one entry per
  merged change, written in the change's branch), next to `CURRENT_STATE.md`.
- `pf knowledge plan|init` installs the structure **only where it is missing**.
  It recognises equivalents, reports numbering conflicts, and never overwrites.
  Filling the new files is a reviewed `docs` job.
- Keeping it true is part of every job: the protocol's knowledge-upkeep table,
  enforced by reviewer-code.
- **Refresh** stack knowledge against upstream releases through
  `/factory knowledge refresh`. Upgrades are separate, escalated jobs.

## Alternatives considered

- **Generate the docs fully.** Rejected: it would invent commands and principles.
  The templates hold structure and prompts, and the content comes from the code
  in a reviewed job.
- **Force every repo onto `docs/`.** Rejected for adoption. A product's existing
  layout (`doc/`, `product/`) wins, and `docs_root` maps to it.

## Consequences

- Every new project starts with the full Connect-grade structure, and existing
  ones get exactly their gaps.
- Each job does a little more documentation, by design.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Architecture decisions](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
