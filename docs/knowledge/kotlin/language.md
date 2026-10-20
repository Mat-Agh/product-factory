# Kotlin language conventions

**Status:** Active
**Last updated:** 2026-10-20

Shared by every Kotlin codebase: Spring services, KMP clients and Gradle build
logic. Stack chapters add their own layout rules
([client](client/README.md), [Spring backend](backend/spring/README.md)).

## Types and time

- `kotlin.time.Instant` for every timestamp; never `java.time.Instant` or
  `java.util.Date` in new code.
- `kotlin.time.Duration` for intervals, timeouts, TTLs and policy windows.
- `kotlin.time.Clock` for testable time; `Clock.System` is wired at the
  composition root (Spring `di/`, Koin module), never called inside domain code.
- Timestamps are persisted and serialized as UTC ISO-8601.
- Opaque identifiers are UUIDs (`java.util.UUID` on the JVM, a value type or
  `String` in common code); list cursors are opaque strings on the wire.

## Naming

| Suffix or name | Meaning |
|---|---|
| Plain name | Domain model (`Message`, `Call`) |
| `*Dto` | Wire shape (HTTP, WebSocket), `@Serializable` or Jackson-mapped |
| `*Entity` | Backend persistence row (JPA) |
| `*Data` | Client persisted local data |
| `*Repository` / `*RepositoryImpl` | Domain persistence interface / data implementation |
| `*Port` | Non-persistence outbound capability (backend) |
| `*Source` (`*JpaSource`, `*RemoteSource`, `*LocalSource`) | Data-layer access adapter |
| `*UseCase` / `*UseCaseImpl` | One use case |
| `*Command` / `*Result` | Explicit use-case input and output |
| `*Screen`, `*ScreenViewModel`, `*ScreenState`, `*ScreenAction`, `*ScreenEvent` | One client MVI page |

Injected names carry the type suffix: `sendMessageUseCase`, `messageRepository`,
never `sendMessage` or `repo`.

## Structure rules

- Prefer `val`, data classes, and sealed interfaces for results and errors.
- One public top-level type per file.
- Interfaces and implementations are explicit; implementations are `internal`
  and constructed only at the composition root, which exposes the interface.
- Every non-trivial use case has explicit input and output types.
- Domain code holds no framework, transport, storage or platform types.

## Mappers

Extension functions only, never mapper classes: `toModel()`, `toEntity()`,
`toDto()`, `toData()`, `toCommand()`, `to*ResponseDto()`. Mappers live in the
layer that owns the foreign type (data mappers in `data/`, wire mappers in
`presentation/` on the backend).

## Coroutines and flows

- Structured concurrency only: scopes owned by a lifecycle (`viewModelScope`, a
  request, a supervised service), never `GlobalScope`.
- `StateFlow` for state, `SharedFlow` or a channel for one-off events.
- Inject dispatchers at repository and adapter boundaries; blocking I/O never runs
  on the main thread.
- Cancellation is cooperative: do not swallow `CancellationException`.

## Style

- Two or more constructor or function parameters: one per line. Two or more call
  arguments: one per line. Do not wrap right after `=`.
- No `!!` in production code.
- Explicit API mode for library-like modules.
- ktlint and detekt are configured once in build logic ([gradle.md](gradle.md)),
  never per module.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Kotlin](README.md)
- Documentation home: [Product Factory documentation](../../README.md)
- Agent guide: [AGENTS.md](../../../AGENTS.md)
<!-- documentation-routing:end -->
