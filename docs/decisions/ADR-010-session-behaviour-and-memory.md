# ADR-010 — Sessions default to the factory; auto memory is promoted into git

**Status:** Accepted
**Last updated:** 2026-10-19
**Date:** 2026-10-19

## Context

The operator wants every Claude session in a product, including one opened
inside a single child of a multi-repo workspace, to use the factory without
being told. Claude Code has two kinds of project memory. The first is
instruction files (`CLAUDE.md`, `.claude/rules/`, `AGENTS.md`), which live in
git. The second is auto memory (`~/.claude/projects/<project>/memory/`), which
stays on one machine and is not loaded into subagents. In a multi-repo workspace, every child
is a nested git repo, so Claude Code treats each one as its own project and
looks for skills only up to that repo's root.

## Decision

- Install an **always-on rule** (`.claude/rules/product-factory.md`) in every
  factory repo and module repo. It routes change requests through `/factory`,
  and keeps questions direct. The skill gets a `when_to_use` trigger to match.
- For multi-repo workspaces, install a **module kit** in each child
  (`pf install module`), and generate **path-scoped module rules** at the root
  (`pf rules sync`). The workspace keeps the single descriptor, backlog and job
  state, and the skill finds the descriptor in a parent folder.
- Treat auto memory as a **cache**. `/factory memory` sorts it into team,
  personal, already-in-repo, stale and secret, and promotes approved team
  knowledge to `AGENTS.md`, docs or rules through a reviewed docs job. Personal
  items go to `CLAUDE.local.md`. Nothing moves without approval.
- Put the attribution policy in the descriptor (`vcs.attribution`), and let
  `pf install` write Claude Code's `attribution` setting.

## Alternatives considered

- **Relocate auto memory into git with `autoMemoryDirectory`.** The value must
  be an absolute path, so it cannot be shared through a committed project
  setting. It would also put unreviewed notes in git, so it was not adopted.
  Pointing it at the private factory repo for backup remains an option.
- **Rewrite each child's `CLAUDE.md`.** That would collide with the operator's
  convention of syncing `CLAUDE.md`, `AGENTS.md` and the IDE mirrors. The
  factory adds files under `.claude/` instead, and only recommends an
  `@AGENTS.md` import.

## Consequences

- Each module repo carries a copy of the kit, and `pf doctor` reports drift.
- Routing is instruction-level. A determined direct edit is still possible, and
  the guard hook and branch protection remain the enforced controls.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Architecture decisions](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
