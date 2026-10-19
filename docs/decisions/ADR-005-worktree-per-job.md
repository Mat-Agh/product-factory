# ADR-005 — One worktree, branch and `job.json` per job

**Status:** Accepted
**Last updated:** 2026-10-19
**Date:** 2026-10-19

## Context

The operator's main checkout may be dirty or on another branch, and the factory
must never disturb it (CONSTITUTION §6). Parallel jobs must not share files,
ports or browser sessions. The video's factory moved from one shared
`board.json` to a `job.json` per job and a worktree per job for exactly this
reason.

## Decision

- Each job gets the branch `factory/<id>` in a worktree at
  `../<repo>-factory/<id>/`, created from `vcs.base_branch`.
- Job state stays in the main checkout under `.factory/jobs/<id>/`, and the
  coordinator is the only writer of `job.json`.
- Ids are claimed atomically with `mkdir`. Ports and browser sessions are
  derived from the job number and role.
- `/factory clean` removes worktrees, and branches only after merge, and only
  under the factory's own prefix and folder.

## Consequences

Several Claude Code sessions can run jobs in parallel without coordination
beyond the filesystem. Disk use grows with the number of open jobs, which
`clean` reclaims.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Architecture decisions](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
