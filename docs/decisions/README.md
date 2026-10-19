# Architecture decisions

**Status:** Active
**Last updated:** 2026-10-19

Documentation navigation: [development handbook](../development/README.md) · [routing map](../development/16-documentation-link-map.md) · [complete inventory](../DOCUMENT_INVENTORY.md).

Accepted decisions for the Product Factory. Only the decisions indexed below are
canonical. The research behind them is in
[architecture/SOURCES.md](../architecture/SOURCES.md). A new decision gets the
next number. A reversed decision is marked `Superseded by ADR-NNN` and kept.

| ADR | Decision | Status |
|---|---|---|
| [001](ADR-001-claude-code-primary-runtime.md) | Claude Code is the primary runtime; roles stay tool-neutral markdown | Accepted |
| [002](ADR-002-human-only-merge.md) | Humans merge; the gate opens a PR, or a draft PR on escalation | Accepted |
| [003](ADR-003-files-not-conversation.md) | Agents share context through files with first-line verdicts | Accepted |
| [004](ADR-004-review-lenses-and-qa.md) | Parallel review lenses, a separate QA stage, bounded rework | Accepted |
| [005](ADR-005-worktree-per-job.md) | One worktree, branch and `job.json` per job | Accepted |
| [006](ADR-006-portable-distribution.md) | Copied project kit, symlinked user install, mirrors: no account dependency | Accepted |
| [007](ADR-007-truth-in-product-repo.md) | Product truth stays in the product repo; descriptor versus `AGENTS.md` split | Accepted |
| [008](ADR-008-idea-and-release-lanes.md) | Idea and release lanes; launch material is drafts only | Accepted |

## Template

```markdown
# ADR-NNN — <decision>

**Status:** Proposed | Accepted | Superseded by ADR-NNN
**Last updated:** YYYY-MM-DD
**Date:** YYYY-MM-DD

## Context
## Decision
## Alternatives considered
## Consequences
```

<!-- documentation-routing:start -->
## Documentation routing

- [Parent README](../README.md)
- [Governing agent guide](../../AGENTS.md)
<!-- documentation-routing:end -->
