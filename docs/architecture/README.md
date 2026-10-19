# Architecture

**Status:** Active
**Last updated:** 2026-10-19

Documentation navigation: [docs index](../README.md) · [routing map](../development/16-documentation-link-map.md) · [documentation map](../README.md#documentation-map).

How the factory is put together and why. Each component's detailed contract is
in [`components/`](../components/README.md), and accepted decisions are in
[`decisions/`](../decisions/README.md).

| Document | Contents |
|---|---|
| [FACTORY_OVERVIEW.md](FACTORY_OVERVIEW.md) | Layers (factory repo, machine, product repo, git host), the runtime sequence of one job, contracts between components, roles and models, enforcement |
| [JOB_LIFECYCLE.md](JOB_LIFECYCLE.md) | Job state machine, lanes per type and tier, the artifact and verdict contract, concurrency and claiming |
| [SOURCES.md](SOURCES.md) | Research record: what came from the Hermes factory and from the Claude Code video, conflicts, and how each was resolved |

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
- In this folder:
  - [Factory overview](FACTORY_OVERVIEW.md)
  - [Job lifecycle](JOB_LIFECYCLE.md)
  - [Research sources and design synthesis](SOURCES.md)
<!-- documentation-routing:end -->
