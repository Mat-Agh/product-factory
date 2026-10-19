# {{MODULE}} development handbook

**Status:** Active
**Last updated:** {{DATE}}

This is the engineering handbook for {{MODULE}}, a Spring Boot and Kotlin service
repository. These pages are reference material. Agents route through
[16-documentation-link-map.md](16-documentation-link-map.md) and read only the
chapters a task needs, never the whole handbook.

## Start here

| Step | Document | Purpose |
|---|---|---|
| 1 | `AGENTS.md` (repo root) | Repo scope, sibling repos, workflow, gates |
| 2 | [../README.md](../README.md) | Documentation index |
| 3 | [16-documentation-link-map.md](16-documentation-link-map.md) | The smallest doc set for the task |
| 4 | [00-engineering-principles.md](00-engineering-principles.md) | Module, layer, naming, API, event, security and build invariants |
| 5 | [09-tools-scripts-and-commands.md](09-tools-scripts-and-commands.md) | Gradle, Docker, scripts and acceptance commands |
| 6 | [15-code-samples-index.md](15-code-samples-index.md) | Copy-paste implementation patterns |

## Full index

| # | Document | Scope |
|---|---|---|
| 00 | [engineering-principles](00-engineering-principles.md) | Topology, layers, naming, API prefixes, outbox, fail-closed authz, quality gates, KISS/DRY/SOLID/YAGNI |
| 01 | [service-architecture](01-service-architecture.md) | Service ownership, the feature-to-service compile rule, facts vs decisions |
| 02 | [kotlin-language](02-kotlin-language.md) | Types, time, folder layout, suffixes, mappers, style |
| 03 | [clean-architecture-layers](03-clean-architecture-layers.md) | `domain` / `data` / `presentation` / `di` per feature, core module layout |
| 04 | [persistence](04-persistence.md) | Flyway, `*JpaSource` vs `*Repository`, tenant scope, pagination |
| 05 | [events-messaging](05-events-messaging.md) | Transactional outbox and inbox, event contracts |
| 06 | [security-contracts](06-security-contracts.md) | API surfaces, authorization checklist, credentials, errors, observability |
| 07 | [testing-quality](07-testing-quality.md) | The testing policy, static gates, manual acceptance, phase close |
| 08 | [build-ci](08-build-ci.md) | Convention plugins, version catalog, reproducibility, CI gates |
| 09 | [tools-scripts-and-commands](09-tools-scripts-and-commands.md) | Everyday loop, gates, generators, contract surfaces to keep in sync |
| 10 | [cryptography](10-cryptography.md) | Classification, envelope encryption, key custody, hashing, blind indexes, transport |
| 15 | [code-samples-index](15-code-samples-index.md) | Templates by task and by file type |
| 16 | [documentation-link-map](16-documentation-link-map.md) | Agent routing across docs, source paths and rules |
| — | [CICD](CICD.md) | Branching, conventions, pipeline by trigger |

Numbering is shared across stacks: `00` principles, `09` tools, `15` samples, `16`
link map. Stack chapters use the free numbers. When code and prose disagree,
`settings.gradle.kts`, `build-logic/` and `gradle/libs.versions.toml` win, and
the prose is fixed in the same change.
