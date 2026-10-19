# ADR-006 — Portable distribution with no account dependency

**Status:** Accepted
**Last updated:** 2026-10-19
**Date:** 2026-10-19

## Context

The operator requires that the factory, its rules and its configuration work
on any account, project or machine, including after the current Claude account
is lost. Hermes solved this with a git home and install/sync scripts. Claude
Code stores skills, agents and memory under `~/.claude`, on one machine.

## Decision

- The factory is a git repository, mirrored to several remotes and to offline
  bundles (`scripts/mirror.sh`).
- **Products get a copy** of the kit (`install.sh project`), committed in the
  product. Anyone who clones the product has a working factory, with
  `installed.lock` recording its source revision.
- **Machines get symlinks** (`install.sh user`), so `git pull` updates every
  project on that machine that uses the user install.
- Operator preferences live in `docs/operator/USER.md`, imported into Claude Code
  through `CLAUDE.md`. Vendor memory is exported for review, never treated as
  the source.
- No account ids, tokens or hosted state exist anywhere in the factory.

## Alternatives considered

- **Symlinks into products.** No drift, but they break on any other machine and
  for collaborators.
- **A Claude Code plugin or marketplace.** Convenient, but it ties
  distribution to one vendor channel. It can be added later as an extra
  channel, not as the only one.

## Consequences

Project copies can drift from the factory. `doctor.sh` reports drift, and
updating is an explicit `install.sh project` followed by review.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Architecture decisions](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
