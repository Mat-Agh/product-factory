# ADR-004 — Parallel review lenses, separate QA, bounded rework

**Status:** Accepted
**Last updated:** 2026-10-19
**Date:** 2026-10-19

## Context

Hermes has one independent reviewer followed by a tester. The video has four
parallel reviewers (security, UX, UI, code), waits for all of them, and has no
separate QA. Both cap rework rounds.

## Decision

- Four lenses: **code** and **security** on every code change, and **UX** and
  **UI** only when the surface is `ui`. All lenses of a round are spawned
  together, and the round ends only when every required lens has reported.
- **QA is a separate stage after review**, because review reads the change and QA
  exercises the product against the acceptance list. Bugs also get a QA
  **repro** stage before any fix.
- Review and QA failures share one resume budget (`limits.max_resumes`,
  default 2). When it runs out, the job escalates with the outstanding findings.
- The docs-sync check is part of the code lens: a behaviour change with no doc
  change is `CHANGES`.
- Security runs on the deep model. The other lenses run on the build model.

## Alternatives considered

- **A single reviewer (Hermes).** Cheaper, but it misses security and UX issues.
- **No QA stage (video).** Rejected, because review is not acceptance, and
  "compile is not done" needs someone who walks the flow.

## Consequences

More tokens per job than either source. Lenses that do not apply are skipped to
contain the cost. Phase 4 measures whether the extra lenses earn their cost.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Architecture decisions](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
