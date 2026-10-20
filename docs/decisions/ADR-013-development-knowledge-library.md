# ADR-013 — One development knowledge library, imported through stack packs

**Status:** Accepted
**Date:** 2026-10-20
**Last updated:** 2026-10-20
**Amends:** [ADR-012](ADR-012-knowledge-structure-per-stack.md)

## Context

ADR-012 put stack knowledge into per-stack template folders under
`kit/knowledge/stacks/`. That made it hard to see, browse or extend as one body
of knowledge, duplicated pages shared by several stacks (Kotlin language, Gradle),
and left out what is not tied to one stack: CI/CD and pipelines, API contracts,
security policy, product practice, Rust tooling and agentic workflows. The
operator wants every piece of general knowledge gathered so far, for every
language and stack, in one place, and imported into projects from there.

## Decision

- `docs/knowledge/` is the **development knowledge library**, organised by language
  and stack: `principles/`, `kotlin/` (shared language and Gradle; `client/` for
  KMP, Android and iOS; `backend/spring/`), `typescript/` (`web/`), `rust/`, `api/`,
  `security/`, `product/`, `cicd/`, `agentic/`. It is the single source and is
  written generically.
- Each stack pack keeps only project **scaffolds** in `kit/knowledge/stacks/<stack>/`
  and a `pack.map` listing which library pages a repository of that stack imports
  and where they land: stack chapters become the numbered handbook
  (`docs/development/NN-…`), cross-cutting areas land under
  `docs/development/knowledge/`.
- `pf knowledge init` imports library pages without the factory's routing block,
  rewrites links between library pages to wherever their targets landed in the
  project (including existing equivalents), and keeps ADR-012's rules: create
  what is missing, append missing sections, never overwrite.
- Every library page must be mapped by at least one pack (enforced by a test).
- Knowledge not yet earned (Ktor server, iOS release) is named as a gap, not
  written.

## Alternatives considered

- **Keep per-stack copies.** Rejected: duplication, and cross-cutting knowledge has
  no home.
- **Import the library with its own folder layout only** (`docs/development/knowledge/**`).
  Rejected for stack chapters: existing repos keep numbered handbooks, and mapping
  onto those numbers lets the import merge into the chapters they already have.
- **Link to the factory instead of importing.** Rejected: agents and people in a
  product repo must find the knowledge in that repo, and the factory checkout may
  not be present.

## Consequences

- The library is browsable and reviewable as normal factory docs, checked by
  `pf docs-map`.
- Projects carry a copy; improvements reach them through `pf knowledge init` after
  the factory is pulled. Divergent local edits stay (nothing is overwritten); new
  sections are appended for review.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Architecture decisions](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
