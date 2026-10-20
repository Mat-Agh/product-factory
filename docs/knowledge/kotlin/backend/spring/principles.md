# Spring service engineering principles

**Status:** Active and mandatory
**Last updated:** 2026-10-20

> **Authority.** New server code in the repository follows these conventions unless an
> explicit, documented exception is recorded in
> ../architecture/CONFORMANCE_LEDGER.md.
> Every other chapter applies this one. When they disagree, this one wins.

## 1. Why this architecture exists

The product is a modular platform, not a monolith with features bolted on. The
backend architecture optimises for:

| Goal | Mechanism |
|---|---|
| Parallel team development | Feature modules with strict service ownership |
| Predictable coupling | `core/*` never depends on features; features never import other services' features |
| Safe side effects | Domain write and outbox row in one transaction; inbox record before consumer side effects |
| Fail-closed authorization | Current decisions over authenticated internal HTTP; never soft-fail open |
| Build-time policy | Compile, lint, static and security gates, contract diffs, Flyway per service database |
| Tenant isolation | One database and credential per durable service; tenant-scoped queries |

This is practical Clean Architecture with strong module governance, not layering
for its own sake.

## 2. Module topology

```
Deployables:  application/<service>     one Spring Boot app per service
Features:     feature/<name>            vertical slices, each owned by one service
Core:         core/<module>             contracts, security, persistence, events, web, observability
```

Dependency rules (non-negotiable):

1. `application/*` depends on the features it owns and on `core/*`, never the reverse.
2. Features depend on `core/*`, never on features owned by another service.
3. A feature compiles into exactly one owning service. Cross-service feature
   imports and database reads are forbidden.
4. `core/*` never depends on features. Core-to-core dependencies are acyclic.
5. One package root, one environment-variable prefix and one image prefix for
   the product. Record them in `AGENTS.md`.

Record the service ownership table (service → the data and behaviour it owns) in
[01-service-architecture.md](service-architecture.md).

## 3. Layer responsibilities

Inside each feature module (details in [03](layers.md)):

| Layer | Holds | Never holds |
|---|---|---|
| `domain/` | `model/`, `repository/` (persistence interfaces), `port/` (other outbound capabilities), `usecase/`, optional `policy/` and `state/` | Spring Data, JPA entities, HTTP clients, framework configuration |
| `data/` | `entity/` (`*Entity`), `source/` (`*JpaSource`), `repository/` (`*RepositoryImpl`), `mapper/`, `adapter/`, `messaging/` | Business rules, wire DTOs |
| `presentation/` | `controller/`, `dto/` (`*Dto`), `mapper/` | Entities, `*JpaSource`, business rules |
| `di/` (mandatory) | `@Configuration` classes and explicit `@Bean` wiring | Logic |

Dependency rule: `presentation → domain ← data`. Do not rely on package-scan
accidents for non-trivial wiring.

## 4. Naming and model semantics

| Suffix or name | Meaning | Example |
|---|---|---|
| `*Dto` | Wire format (HTTP, WebSocket) | `CreateMessageRequestDto` |
| `*Entity` | JPA persistence row | `MessageEntity` |
| `*JpaSource` | Spring Data interface in `data/source/` | `MessageJpaSource` |
| `*Repository` | Persistence interface in `domain/repository/` | `MessageRepository` |
| `*RepositoryImpl` | Data-layer implementation | `MessageRepositoryImpl` |
| `*Port` | Non-persistence outbound interface in `domain/port/` | `ObjectStoragePort` |
| `*UseCase` / `*UseCaseImpl` | Use-case contract and implementation | `SendMessageUseCase` |
| `*Command` / `*Result` | Explicit use-case input and output | `SendMessageCommand` |
| Plain name | Domain model | `Message` |

Mapping uses extension functions only, never mapper classes:

```kotlin
fun MessageEntity.toModel(): Message = …
fun Message.toEntity(): MessageEntity = …
fun CreateMessageRequestDto.toCommand(): SendMessageCommand = …
fun Message.toResponseDto(): MessageResponseDto = …
```

## 5. Presentation and use-case contract

| Concern | Rule |
|---|---|
| Controllers | Translate HTTP ↔ Command/Result/DTO only |
| Use cases | Interface and `*Impl` in the same folder (a subfolder named after the interface when several pairs share `usecase/`); one public type per file |
| I/O | Every non-trivial use case has explicit `Command` and/or `Result` types |
| Injected names | Carry the type suffix: `sendMessageUseCase`, `messageRepository`, `messageJpaSource` |
| Time | `kotlin.time.Instant`, `Duration`, `Clock` (`Clock.System` wired in `di/`) |
| IDs | `java.util.UUID`; opaque cursors on the wire |

Every `*Impl` is `internal`, implements the same-named interface, is referenced
only by its owning `di/*Configuration.kt`, and every `@Bean` returns the
interface type. Application modules supply configuration and ports; they never
construct another module's implementation.

API surfaces are versioned by prefix, for example `/api/v1` (public REST),
`/ws/v1` (public WebSocket), `/admin/v1` (administration BFF). Record the real
prefixes in `docs/api/README.md`. Wire fields are camelCase, enum values
lowercase snake_case, timestamps UTC ISO-8601.

## 6. Persistence, events and security

- Each durable service owns one database, its Flyway history and its credential.
  Transactions never span services. Cross-service IDs are logical references,
  not foreign keys. ([04](persistence.md))
- Commit domain state and the outbox row in the same transaction. Events are past
  tense, versioned, tenant-scoped, idempotent and schema-published. Consumers
  record the inbox ID before side effects. ([05](events-outbox-inbox.md))
- Current authorization decisions use authenticated internal HTTP (mTLS where the
  platform has it) and fail closed. Every resource operation checks membership,
  capability or entitlement, role and ownership as applicable. Identity comes from
  the authenticated principal, never from a request body. ([06](security.md))
- Passwords use Argon2id. Refresh tokens are opaque, hashed, rotated and
  replay-protected. Restricted fields use contextual envelope encryption.
  ([10](cryptography-by-layer.md))
- Logs, metrics, traces and errors never contain tokens, secrets, provider
  payloads or private content.

## 7. Build logic and quality gates

- `gradle/libs.versions.toml` is the single source of dependency versions. No
  versions in module build files. ([08](build-and-ci.md))
- Gates: compile, ktlint, detekt, architecture/contract/docs/schema validation,
  dependency and secret scans. The testing policy is recorded in
  [07](testing-and-quality.md).
- Contracts change with behaviour: OpenAPI, AsyncAPI, Flyway and generated schema
  docs are updated in the same change.
- Compile is not done. Done means the gates pass and the acceptance evidence is
  recorded.

## 8. Library stack

| Concern | Choice |
|---|---|
| Runtime | Spring Boot (Kotlin) |
| DI | Spring `@Configuration` / `@Bean` under feature `di/` |
| HTTP | Spring WebMVC; design-first OpenAPI under `docs/api/openapi/` |
| Realtime | Spring WebSocket and AsyncAPI |
| Messaging | Kafka with a transactional outbox and inbox |
| Persistence | PostgreSQL, JPA and Flyway, one database per durable service |
| Serialization | Jackson on the wire |
| Security | Spring Security; mTLS for the internal mesh |
| Time | `kotlin.time.*` |

Record deviations and the reasons in this table.

## 9. Design discipline — KISS, DRY, SOLID, YAGNI (review-blocking)

A reviewer rejects code that violates these without a linked exception in the
conformance ledger.

| Principle | What it means here |
|---|---|
| KISS | The smallest construct that satisfies the contract. No speculative abstraction. A use case that needs a paragraph to explain is split. |
| DRY | One owner for every rule, shape and constant. Shared wire shapes and enums live in `core/contracts`. Do not copy validation, mappers or DTOs between features. Coincidental similarity is not duplication. |
| SOLID | One reason to change per type. Depend on `domain/repository` and `domain/port` interfaces, never on `data/` implementations. Ports stay narrow and substitutable. |
| YAGNI | Build only what a current product contract requires. No unused parameters, flags, endpoints or DTO fields. Delete dead code in the change that orphans it. |

Concrete conventions this codifies:

- **Validation is domain-owned.** Use cases and `domain/policy` validate inputs and
  raise a typed error with a stable snake_case code. Bean-validation annotations
  on DTOs are only for purely structural checks shared by every caller.
- **Wire timestamps are pre-formatted strings** produced by the presentation
  mapper; the domain keeps `kotlin.time.Instant`.
- **Wire enums implement one shared contract** whose wire value is
  `name.lowercase()`.
- **One error envelope everywhere**: RFC 9457 Problem Details with `code` and
  `correlationId`; a catch-all maps the unexpected to `internal_error` without
  leaking internals.

## 10. Invariants checklist (before every merge request)

- [ ] The feature belongs to exactly one service; no cross-service import or database read
- [ ] Layers respected; no entities or `*JpaSource` in `domain/` or `presentation/`
- [ ] Persistence interfaces in `domain/repository/`, other outbound capabilities in `domain/port/`, implementations in `data/`
- [ ] The feature has `di/` with explicit `@Configuration` / `@Bean` wiring
- [ ] One public type per file; interface and `Impl` together; every `Impl` internal and returned as its interface
- [ ] Domain code has no HTTP client, servlet filter, mail sender, configuration-properties type or JPA entity
- [ ] Mapping uses extension functions only
- [ ] Non-trivial use cases expose `Command` / `Result`
- [ ] Domain write and outbox committed together; inbox before consumer side effects
- [ ] Cross-service authorization fails closed
- [ ] API prefixes correct; OpenAPI, AsyncAPI and Flyway updated with behaviour
- [ ] Gates pass; acceptance evidence recorded; no secrets committed
- [ ] KISS/DRY/SOLID/YAGNI hold, or a written exception is linked in the ledger
- [ ] Failures answer with Problem Details (`code` and `correlationId`)

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Spring Boot services](README.md)
- Documentation home: [Product Factory documentation](../../../../README.md)
- Agent guide: [AGENTS.md](../../../../../AGENTS.md)
<!-- documentation-routing:end -->
