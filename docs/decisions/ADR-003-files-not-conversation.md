# ADR-003 — Context through files with first-line verdicts

**Status:** Accepted
**Last updated:** 2026-10-19
**Date:** 2026-10-19

## Context

Agents that pass context through conversation lose it at session boundaries,
cannot be audited, and need a model to interpret prose before deciding what
happens next. The video's factory has each agent read files and write one file
whose first line is a verdict. Hermes keeps evidence on board cards for the
same reason.

## Decision

- Every stage has exactly one artifact with one writer, in
  `.factory/jobs/<id>/`.
- The first line of each artifact is a fixed verdict (see
  [JOB_LIFECYCLE](../architecture/JOB_LIFECYCLE.md#artifact-contract)).
- The coordinator acts only on verdicts. A missing file or an unknown first line
  stops the job and goes to the human.
- Agents read the product's live files (`AGENTS.md`, docs, code) and never
  rely on a copy.

## Consequences

- A job can be resumed from disk by any new session, and audited after the
  fact.
- The verdict formats are a stable contract. Changing one is a breaking change.
- Job folders are local and gitignored. Anything durable must also go into the
  product repo, in the branch ([ADR-007](ADR-007-truth-in-product-repo.md)).

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Architecture decisions](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
