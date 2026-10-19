# 02 — Kotlin language conventions

**Status:** Active
**Last updated:** {{DATE}}
**Source:** Product Factory knowledge pack (spring-kotlin-service)

The full reference for Kotlin usage in {{MODULE}}. Invariants:
[00-engineering-principles.md](00-engineering-principles.md).

## Types and time

- `java.util.UUID` for opaque identifiers in domain, persistence and wire mapping.
- `kotlin.time.Instant` for every timestamp. Never `java.time.Instant` or
  `java.util.Date` in new code.
- `kotlin.time.Duration` for intervals, timeouts, TTLs and policy windows.
- `kotlin.time.Clock` for testable time; `Clock.System` is wired in `di/`.
- Persist and serialize timestamps as UTC ISO-8601. Converters bridge to the
  database `timestamptz` type.

## Feature folder layout

```
feature/<name>/
  domain/
    model/
    repository/          # persistence interfaces
    port/                # non-persistence outbound capabilities
    usecase/
      <InterfaceName>/   # one folder per interface when several pairs share usecase/
    policy/ | state/     # optional, same nesting rule
  data/
    entity/
    source/              # *JpaSource Spring Data interfaces
    repository/          # *RepositoryImpl
    mapper/
    adapter/ | messaging/
  presentation/
    controller/
    dto/
    mapper/
  di/                    # mandatory: @Configuration / @Bean
```

## Structure rules

- Prefer `val`, data classes, and sealed interfaces for results and errors.
- One public type per file: DTOs, controllers, use cases, enums, entities, sources.
- Interface and `Impl` live in the same folder; related `*Command` and `*Result`
  types live with them.
- Every `*Impl` is `internal`, implements the exact same-named interface, and is
  referenced only from its owning `di/*Configuration.kt` bean, whose declared
  return type is the interface.
- Injected names carry the type suffix (`sendMessageUseCase`, not `sendMessage`;
  `messageRepository`, not `repo`).
- Every non-trivial use case has explicit input and output types.

## Mappers

Extension mappers only, no mapper classes:

- `<feature>.data.mapper`: `toModel()`, `toEntity()`
- `<feature>.presentation.mapper`: `to*ResponseDto()`, `toCommand()`

Framework types (HTTP clients, WebSocket sessions, servlet filters, mail senders,
serializers, configuration-properties) stay in `data`, `presentation` or `di`.
Domain ports expose capability-oriented Kotlin or JDK types. Spring transaction
annotations on use-case implementations are the one framework annotation the
domain tolerates, and only as a recorded pragmatic exception.

## Style

- Two or more constructor or function parameters: one per line. Two or more call
  arguments: one per line. Do not wrap right after `=`.
- No `!!` in production code.
- Explicit API mode for library-like core modules.
- ktlint and detekt configuration live in build logic, not per module.
