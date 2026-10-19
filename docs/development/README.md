# Factory development handbook

**Status:** Active
**Last updated:** 2026-10-19

Documentation navigation: [docs index](../README.md) · [routing map](16-documentation-link-map.md) · [documentation map](../README.md#documentation-map).

This handbook is for anyone, human or agent, who changes the factory itself.
Using the factory on a product is covered in the [operator guide](../guide/README.md).
Agents should route through [16-documentation-link-map.md](16-documentation-link-map.md)
and not read the whole handbook for ordinary work.

## Start here

| Step | Document | Purpose |
|---|---|---|
| 1 | [AGENTS.md](../../AGENTS.md) | Repo scope, gates, what not to do |
| 2 | [CONTRIBUTING.md](CONTRIBUTING.md) | Branches, commits, PRs, review |
| 3 | [16-documentation-link-map.md](16-documentation-link-map.md) | The smallest set of docs for your task |
| 4 | [00-factory-engineering-principles.md](00-factory-engineering-principles.md) | Invariants for changing the factory |
| 5 | [03-changing-the-factory.md](03-changing-the-factory.md) | "When you change X, also update Y" |

## Full index

| # | Document | Scope |
|---|---|---|
| 00 | [factory-engineering-principles](00-factory-engineering-principles.md) | Single source per concept, project-agnostic kit, stable contracts, fail closed, no invented knowledge |
| 01 | [dev-setup](01-dev-setup.md) | Toolchain per OS for developing the factory |
| 02 | [repository-layout](02-repository-layout.md) | Every folder and file, and who owns it |
| 03 | [changing-the-factory](03-changing-the-factory.md) | Change procedures and the docs-sync map; breaking versus non-breaking |
| 04 | [testing-and-verification](04-testing-and-verification.md) | Unit, smoke and end-to-end pilot procedures; how to record evidence |
| 05 | [releases-and-versioning](05-releases-and-versioning.md) | Factory versions, tags (human-only), updating products |
| 09 | [tools-scripts-and-commands](09-tools-scripts-and-commands.md) | Every script, with samples |
| 16 | [documentation-link-map](16-documentation-link-map.md) | Agent routing across docs, sources and rules |

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
- In this folder:
  - [00 — Factory engineering principles](00-factory-engineering-principles.md)
  - [01 — Development setup](01-dev-setup.md)
  - [02 — Repository layout](02-repository-layout.md)
  - [03 — Changing the factory](03-changing-the-factory.md)
  - [04 — Testing and verification](04-testing-and-verification.md)
  - [05 — Releases and versioning](05-releases-and-versioning.md)
  - [09 — Tools and commands](09-tools-scripts-and-commands.md)
  - [16 — Documentation link map](16-documentation-link-map.md)
  - [Contributing to the Product Factory](CONTRIBUTING.md)
<!-- documentation-routing:end -->
