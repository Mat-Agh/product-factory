# COMP-011 — Adapters

**Status:** Documented — procedures only, not exercised
**Last updated:** 2026-10-19
**Source:** [`docs/adapters/hermes.md`](../adapters/hermes.md), [`docs/adapters/other-agents.md`](../adapters/other-agents.md)
**Decisions:** [ADR-001](../decisions/ADR-001-claude-code-primary-runtime.md)

## Purpose

Adapters keep the factory usable without Claude Code. Role files are plain
markdown, so any capable agent can run one role at a time against the same job
folder and verdict contract.

| Adapter | Covers |
|---|---|
| Hermes | Maps the factory's roles to Hermes profiles and skills, how to run a factory role inside Hermes, and the convergence plan (generate Hermes profiles from `kit/` agents) |
| Codex, Cursor, Gemini CLI | Manual coordination (create the job and worktree, prompt one role per stage), Codex as a cross-vendor review lens, and guard behaviour outside Claude Code |

## Known limitations

- There is no automated coordinator outside Claude Code. The human runs the lane.
- The guard hook works only in Claude Code. Elsewhere, branch protection and the
  agent's own approval mode are the controls.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Components](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
