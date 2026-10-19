# Product Factory documentation

**Status:** Active
**Last verified:** 2026-10-19

This tree documents the Product Factory: what it is, how to use it, how it is
built, how to develop it, and what state it is in now. The rules themselves live
in [`CONSTITUTION.md`](../CONSTITUTION.md). The executable procedure lives in
[`kit/`](../kit/). These docs explain them and link to them. They never restate
them.

Agents: open [development/16-documentation-link-map.md](development/16-documentation-link-map.md)
first, and read only the routed documents.

## Start here

| You are… | Read, in order |
|---|---|
| New and have never used the factory | [guide/00-overview.md](guide/00-overview.md) → [guide/01-getting-started.md](guide/01-getting-started.md) → [guide/GLOSSARY.md](guide/GLOSSARY.md) |
| Running jobs day to day | [guide/02-commands.md](guide/02-commands.md) → [guide/03-reading-a-job.md](guide/03-reading-a-job.md) → [guide/06-troubleshooting.md](guide/06-troubleshooting.md) |
| Putting the factory on a product | [guide/04-add-a-project.md](guide/04-add-a-project.md) → [components/COMP-005-project-descriptor.md](components/COMP-005-project-descriptor.md) |
| Protecting against account or machine loss | [guide/05-portability-and-recovery.md](guide/05-portability-and-recovery.md) |
| Changing the factory itself | [../CONTRIBUTING.md](../CONTRIBUTING.md) → [development/README.md](development/README.md) |
| Checking what works today | [../CURRENT_STATE.md](../CURRENT_STATE.md) → [components/IMPLEMENTATION_STATUS.md](components/IMPLEMENTATION_STATUS.md) |
| Understanding why it is designed this way | [architecture/README.md](architecture/README.md) → [decisions/README.md](decisions/README.md) |

## Document areas

| Area | Contents | Index |
|---|---|---|
| [`guide/`](guide/README.md) | Operator handbook: overview, getting started, commands, reading a job, adding a project, portability, troubleshooting, glossary, examples | [guide/README.md](guide/README.md) |
| [`architecture/`](architecture/README.md) | Layers, runtime, job lifecycle and state machine, enforcement, research sources | [architecture/README.md](architecture/README.md) |
| [`components/`](components/README.md) | One contract per factory piece (`COMP-NNN-*.md`) and the implementation status of each | [components/README.md](components/README.md) |
| [`development/`](development/README.md) | Factory engineering handbook: principles, setup, layout, change procedures, testing, releases, tools, link map | [development/README.md](development/README.md) |
| [`decisions/`](decisions/README.md) | Accepted architecture decision records (`ADR-NNN`) | [decisions/README.md](decisions/README.md) |
| [ROADMAP.md](ROADMAP.md) | Phases, exit criteria, and what is never automated | — |
| [EVOLUTION_LOG.md](EVOLUTION_LOG.md) | Dated, append-only record of factory-level changes | — |
| [DOCUMENT_INVENTORY.md](DOCUMENT_INVENTORY.md) | Generated list of every document and its status | — |

## Documentation rules

- Each area has a README. Open it before you edit that folder.
- Every document starts with a `**Status:**` line and a `**Last updated:**` or
  `**Last verified:**` line.
- Change the docs **in the same commit** as the behaviour they describe, using
  the change map in [development/03-changing-the-factory.md](development/03-changing-the-factory.md).
- Before committing, run `python3 scripts/check-docs.py`. It checks links,
  headers and the inventory. After adding or removing a document, regenerate
  the inventory with `--inventory`.

<!-- documentation-routing:start -->
## Documentation routing

- [Parent README](../README.md)
- [Governing agent guide](../AGENTS.md)
- Child README indexes:

  - [architecture](architecture/README.md)
  - [components](components/README.md)
  - [decisions](decisions/README.md)
  - [development](development/README.md)
  - [guide](guide/README.md)
<!-- documentation-routing:end -->
