# 16 — Documentation link map

**Status:** Active — agent routing authority
**Last updated:** {{DATE}}

Route agents to the smallest useful set of docs. Do not read every document
unless the task is repo-wide documentation work.

## Knowledge order (mandatory)

1. This map, [../README.md](../README.md) and `AGENTS.md`.
2. The routed docs in the table below.
3. Upstream owners through [../cross-project/README.md](../cross-project/README.md)
   when product, design or a sibling repo owns the decision.
4. Authoritative online sources, only for what is still missing.
5. Durable findings go into `docs/`, never only into an assistant's memory.

## Router

| Task type | Read first | Then, if needed |
|---|---|---|
| Any non-trivial task | `AGENTS.md`, `docs/README.md`, this file | The matching row |
| What the repo is now | `CURRENT_STATE.md` | `docs/features/IMPLEMENTATION_STATUS.md` |
| Engineering invariants | [00](00-engineering-principles.md) | [03](03-clean-architecture-layers.md), [02](02-kotlin-language.md) |
| Service or module ownership | [01](01-service-architecture.md) | `docs/architecture/OVERVIEW.md`, ADRs |
| Kotlin style, mappers, time | [02](02-kotlin-language.md) | [15](15-code-samples-index.md) |
| Persistence, Flyway, JPA | [04](04-persistence.md) | `docs/data/README.md` |
| Broker, outbox, inbox | [05](05-events-messaging.md) | `docs/api/asyncapi/`, event catalog |
| HTTP and WebSocket wire rules | `docs/api/README.md` | OpenAPI under `docs/api/openapi/` |
| Security, authz | [06](06-security-contracts.md) | [10](10-cryptography.md), `docs/security/README.md` |
| Cryptography | [10](10-cryptography.md) | `docs/security/` |
| Testing and acceptance | [07](07-testing-quality.md) | [08](08-build-ci.md), `CONTRIBUTING.md` |
| Build, CI, Gradle | [08](08-build-ci.md), [CICD](CICD.md) | `docs/operations/README.md` |
| Tools, scripts, local stack | [09](09-tools-scripts-and-commands.md) | Root `README.md`, `scripts/` |
| Code templates | [15](15-code-samples-index.md) | The canonical feature |
| Deployment and operations | `docs/deployment/README.md`, `docs/operations/README.md` | ADRs |
| A feature's contract and status | `docs/features/README.md` | `docs/features/IMPLEMENTATION_STATUS.md` |
| Architecture exceptions | `docs/architecture/CONFORMANCE_LEDGER.md` | [00](00-engineering-principles.md) |
| Decisions | `docs/decisions/README.md` | The ADR |
| Development history | `docs/history/DEVELOPMENT_LOG.md` | `git log` |
| Branching and review gate | `CONTRIBUTING.md` | [CICD](CICD.md) |

| Gradle build | [11](11-build-system-gradle.md) | [08](08-build-ci.md) |
| General principles, evidence, documentation | [knowledge/principles/](knowledge/principles/README.md) | |
| API conventions, errors, events | [knowledge/api/](knowledge/api/README.md) | |
| Security policy, classification, identity | [knowledge/security/](knowledge/security/README.md) | |
| CI/CD and workflows | [knowledge/cicd/](knowledge/cicd/README.md) | [CICD](CICD.md) |
| Agentic workflows | [knowledge/agentic/](knowledge/agentic/README.md) | |

Add a row for every feature doc and every recurring task. Update this map in the
same change that adds, moves or retires a doc.
