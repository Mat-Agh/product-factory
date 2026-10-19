# Architecture

**Status:** Active
**Last updated:** 2026-10-19

Documentation navigation: [docs index](../README.md) · [routing map](../development/16-documentation-link-map.md) · [complete inventory](../DOCUMENT_INVENTORY.md).

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

- [Parent README](../README.md)
- [Governing agent guide](../../AGENTS.md)
<!-- documentation-routing:end -->
