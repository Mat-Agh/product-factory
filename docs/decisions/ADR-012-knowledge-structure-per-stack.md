# ADR-012 — One knowledge structure per stack, distilled from Connect

**Status:** Accepted — amended by ADR-013 (the knowledge moved to one library)
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
- Each stack overlay is a **knowledge pack** with real content: the stack's
  principles, layers, naming, persistence, messaging, security, cryptography,
  testing policy, build practice, library stack and code samples, distilled from
  the Connect handbooks and written generically. Project-state files (current
  state, history, status, area READMEs) stay **scaffolds**.
- `pf knowledge plan|init` **imports** the pack: missing files are created with
  full content, and an existing equivalent knowledge file gets the `##` sections
  it lacks, appended in a marked block for review. It recognises equivalents,
  reports numbering conflicts, never overwrites and never edits existing text.
  Adapting the import to the repo is a reviewed `docs` job. (Amended 2026-10-19:
  the first version created only missing placeholder files.)
- Keeping it true is part of every job: the protocol's knowledge-upkeep table,
  enforced by reviewer-code.
- **Refresh** stack knowledge against upstream releases through
  `/factory knowledge refresh`. Upgrades are separate, escalated jobs.

## Alternatives considered

- **Generate the docs fully.** Rejected: it would invent commands and facts about
  the repo. The packs carry stack knowledge that holds for any repo on the stack.
  Repo facts (commands, canonical sample, state) come from the code in a reviewed
  job.
- **Placeholder skeletons only.** Rejected after first use: empty headings teach
  nothing, and existing docs that lacked a topic stayed without it.
- **Overwrite or rewrite existing docs to the pack.** Rejected: the product's own
  docs take precedence (CONSTITUTION §8). Only missing sections are added, marked,
  and left for review.
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
