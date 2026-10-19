# ADR-002 — Humans merge; the gate opens a PR

**Status:** Accepted
**Last updated:** 2026-10-19
**Date:** 2026-10-19

## Context

In the video's factory, the approver agent merges to `main` when it approves.
The Hermes factory and the operator's standing rule say humans merge, and
Hermes only opens the MR. A community review of the video's factory found that
"never merge" written only as prose was unenforced, and added a hook guard and
branch protection.

## Decision

- The final gate (tech-lead in gate mode) produces `READY` or `ESCALATE`. Both
  open a PR. `ESCALATE` opens a **draft** PR, and its body carries the exact
  question for the human.
- No agent merges, approves, tags, releases or deploys. This is enforced by
  CONSTITUTION §5, the guard hook ([COMP-004](../components/COMP-004-guard-hook.md))
  and branch protection on the remote.
- The approver's escalation list from the video is kept and extended:
  money, identity, auth, tenancy, crypto, deploy/CI, migrations, outbound
  messages, dependencies, product decisions, and `risk.escalate_paths`.

## Alternatives considered

- **Auto-merge on APPROVE (as in the video).** Faster, but it contradicts the
  operator's authority boundary, and a merge is the hardest step to undo.
- **Auto-merge for trivial changes only.** Rejected for now. It can be revisited
  only with metrics showing a near-zero escaped-defect rate (phase 4).

## Consequences

- The best outcome of a job is "PR open". Lead time includes human review.
- The guard also blocks the operator's own Claude sessions in factory projects.
  `FACTORY_GUARD=off` is the documented override for human sessions.

<!-- documentation-routing:start -->
## Documentation routing

- [Parent README](README.md)
- [Governing agent guide](../../AGENTS.md)
<!-- documentation-routing:end -->
