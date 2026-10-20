# Persistence: source vs repository

**Status:** Active
**Last updated:** 2026-10-20

How the repository separates Spring Data access from domain persistence interfaces.

## Ownership

- One PostgreSQL database, Flyway history and credential per durable service.
- Migrations live in `application/<service>/src/main/resources/db/migration/`.
- Schema narrative lives in the repository's `docs/data/` area. Update it, and
  any generated ERD, whenever tables change.
- `ddl-auto=validate` in production. Flyway owns the schema.
- No cross-service foreign keys. Cross-service IDs are logical UUID references.

## Naming split

| Type | Package | Role |
|---|---|---|
| `*JpaSource` | `<feature>.data.source` | Spring Data `JpaRepository` and custom queries: the persistence adapter |
| `*Entity` | `<feature>.data.entity` | Table mapping |
| `*Repository` | `<feature>.domain.repository` | The use-case-facing persistence interface |
| `*RepositoryImpl` | `<feature>.data.repository` | Combines sources and mapping into domain models |

```kotlin
// domain/repository/MessageRepository.kt
interface MessageRepository {
    fun findById(tenantId: UUID, id: UUID): Message?
    fun save(message: Message): Message
}

// data/source/MessageJpaSource.kt
interface MessageJpaSource : JpaRepository<MessageEntity, UUID> {
    fun findByTenantIdAndId(tenantId: UUID, id: UUID): MessageEntity?
}

// data/repository/MessageRepositoryImpl.kt
internal class MessageRepositoryImpl(
    private val messageJpaSource: MessageJpaSource,
) : MessageRepository { … }
```

Never inject a `*JpaSource` into a use case or a controller.

## Tenant scope

Every tenant query is scoped by the authenticated tenant context, never by an ID
taken from the request body. Repository implementations keep the tenant filter.
Where PostgreSQL row-level security is used as a second boundary, the request
sets the tenant claim only inside the current transaction, and background workers
process one tenant per transaction. Tables that must be read before a tenant is
known (login, invitation, bootstrap) are explicit, documented exceptions.

## Migrations

- One migration per change, named `V<n>__<what>.sql`, never edited after merge.
- Constraints live in the database: `NOT NULL`, `CHECK`, unique and foreign keys
  inside one service.
- Index the tenant and time keys that queries use. Check the query plan of new
  list queries.
- A destructive change (drop, rename, type narrowing) gets an ADR and a rollout
  plan; it is never mixed with feature work.

## Pagination

Cursor pagination for lists. No unbounded result sets. Cursors are opaque on the
wire.

## Outbox and inbox tables

Each durable service owns `outbox_events` and `inbox_events` in its own database.
There is no central messaging database ([05](events-outbox-inbox.md)).

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Spring Boot services](README.md)
- Documentation home: [Product Factory documentation](../../../../README.md)
- Agent guide: [AGENTS.md](../../../../../AGENTS.md)
<!-- documentation-routing:end -->
