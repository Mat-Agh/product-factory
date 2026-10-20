# Spring tools and commands

**Status:** Active
**Last updated:** 2026-10-20

The operational toolkit for the repository. Use these commands before inventing new
ones. Replace every `<…>` with the repo's real command once it has been run
successfully, and delete rows that do not apply.

## Prerequisites

| Tool | Role |
|---|---|
| JDK (or GraalVM for native images) | Compile, run and package services |
| Gradle wrapper (`./gradlew`) | Build and static analysis; never a system Gradle |
| Docker and Compose | The local stack: database, broker, cache, object storage, edge, services |
| `git` | Branching per CICD.md |

Environment variables share one prefix. Secrets are Docker secret files or
generated material outside git, never committed `.env` files.

## Everyday loop

```bash
<generate local development secrets>          # outside git
./gradlew verifyArchitecture classes ktlintCheck detekt
docker compose config --quiet                 # validate Compose before up
<build service jars, then compose up --build --wait>
```

`docker compose build` packages the JARs that already exist; it does not compile
Kotlin. Build the JARs immediately before `compose up`, or use the repo's start
script that enforces the order.

After a recreate, wait until the edge health endpoint answers 200 before judging a
client "offline". Never disable certificate or hostname verification to make a
local call work.

Module-scoped iteration:

```bash
./gradlew :application:<service>:classes :application:<service>:ktlintCheck
./gradlew :feature:<name>:compileKotlin :feature:<name>:detekt
```

## Quality and architecture gates

```bash
./gradlew verifyArchitecture classes ktlintCheck detekt --no-configuration-cache
<docs link check>
git diff --check
```

## Generators

| Generator | Run after | Output |
|---|---|---|
| `<schema/ERD generator>` | Flyway changes | `docs/data/` |
| `<API docs generator>` | OpenAPI changes | `docs/api/` |

Review a generator's diff before committing and keep unrelated changes out.

## Contract surfaces to keep in sync

| Change type | Update in the same change |
|---|---|
| HTTP API | The owning `docs/api/openapi/*.yaml` and the controller map |
| Broker or WebSocket | `docs/api/asyncapi/*` and the event catalog |
| Persistence | Flyway, `docs/data/` and generators |
| Feature behaviour | `docs/features/IMPLEMENTATION_STATUS.md` and the feature doc |

## Library notes

Record how this repo uses each infrastructure library, not a tutorial: what Redis
holds and for how long, which broker topics exist, which JOSE library is the only
one, and so on. One subsection per library.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Spring Boot services](README.md)
- Documentation home: [Product Factory documentation](../../../../README.md)
- Agent guide: [AGENTS.md](../../../../../AGENTS.md)
<!-- documentation-routing:end -->
