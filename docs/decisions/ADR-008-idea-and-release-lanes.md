# ADR-008 — Idea and release lanes; launch material is drafts only

**Status:** Accepted
**Last updated:** 2026-10-19
**Date:** 2026-10-19

## Context

Both sources stop at merged code. The operator wants a **product** factory, from
idea to a market-ready product. Hermes deliberately kept marketing, outreach and
publishing out of the engineering gateway, because those actions are external
and hard to reverse.

## Decision

- **Idea lane:** product-manager discovery (problem, users, market with cited
  sources, metrics, risks, smallest viable release) → PRD → designer concept →
  tech-lead architecture ADR and roadmap → a docs PR. Roadmap items are
  appended to the backlog as proposals, and the human decides what enters the
  queue.
- **Release lane:** a release-manager readiness brief (evidence per merged PR,
  blockers, version) plus a launch kit on a branch: changelog, version bump,
  localized release notes, store-listing copy, help docs, and announcement
  **drafts**.
- Business decisions (pricing, positioning, free versus paid) are listed for the
  human and never made by agents.
- Publishing, posting, sending, tagging and deploying stay human-only.

## Consequences

The factory covers the product lifecycle without taking any external action.
Market feedback returns through issues and `/factory issue`.

<!-- documentation-routing:start -->
## Documentation routing

- [Parent README](README.md)
- [Governing agent guide](../../AGENTS.md)
<!-- documentation-routing:end -->
