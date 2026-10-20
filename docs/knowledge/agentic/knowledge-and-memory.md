# Knowledge and memory

**Status:** Active
**Last updated:** 2026-10-20

## Truth lives in git

Durable knowledge (a convention, a command, a decision, a gotcha) goes into the
repo: `AGENTS.md`, `docs/`, or rules, through a reviewed change. A vendor's memory
(Claude auto memory, Cursor memories, an agent platform's memory store) is a
per-machine cache: useful for routing reminders, never the source of truth.

## Promotion

Periodically read the assistant's memory for the repo and sort each item:
- team knowledge → promote into the repo's docs through a docs change;
- personal preference → `CLAUDE.local.md` or the user-level profile;
- stale or wrong → delete.

Leave only short pointers in memory.

## Per-role memory

Agent platforms that give each role its own memory (as an earlier agent harness did with per-role profiles) keep
role context isolated; shared decisions go through the coordinator into files.
Subagents without memory are the safer default: they read the same files every
time.

## Account independence

Everything needed to run the factory and the products (rules, roles, operator
profile, knowledge) lives in git repositories with a mirror and offline bundles.
Losing an account or a machine must not lose the process. Exports of vendor memory
are reviewed and folded into the operator profile, never committed raw.

## Keeping knowledge current

- Every job updates the repo's current state, development log, tools, samples,
  library stack and status in the same branch.
- A periodic refresh checks upstream releases and docs for the libraries and tools
  the handbook lists, citing sources, and proposes upgrades as separate jobs.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Agentic workflows](README.md)
- Documentation home: [Product Factory documentation](../../README.md)
- Agent guide: [AGENTS.md](../../../AGENTS.md)
<!-- documentation-routing:end -->
