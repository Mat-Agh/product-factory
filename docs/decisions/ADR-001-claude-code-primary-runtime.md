# ADR-001 — Claude Code is the primary runtime; roles stay tool-neutral

**Status:** Accepted
**Last updated:** 2026-10-19
**Date:** 2026-10-19

## Context

The earlier factory (Hermes) runs on Hermes Agent, with 9Router model routing,
Multica boards and cron. It works, but it depends on self-hosted services and
many moving parts. The ZazenCodes video shows that an equivalent pipeline can
be built from Claude Code's native skills and subagents, with no extra
services. The operator also needs the factory to survive the loss of any one
account or vendor.

## Decision

Claude Code is the primary runtime. The coordinator is a Claude Code skill, and
the roles are Claude Code subagents. All role definitions are plain markdown
whose bodies do not depend on Claude Code, so any agent (Hermes, Codex, Cursor)
can run one role at a time ([COMP-011](../components/COMP-011-adapters.md)).

## Alternatives considered

- **Keep Hermes as primary.** It needs more infrastructure, lacks native
  parallel worktrees, and couples the factory to self-hosted services.
- **A custom orchestrator (Agent SDK).** More control, but more code to own. It
  can be revisited for headless intake in phase 3.

## Consequences

- No servers to run. The factory is files plus Claude Code.
- The guard hook works only in Claude Code. Elsewhere, branch protection is the
  control.
- Model diversity (a cross-vendor reviewer) needs an explicit adapter
  ([ROADMAP](../status/ROADMAP.md) phase 5).

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Architecture decisions](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
