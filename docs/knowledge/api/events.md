# Events and messaging contracts

**Status:** Active
**Last updated:** 2026-10-20

Facts that cross service boundaries travel on a broker; current authorization
decisions never do (they use authenticated internal HTTP and fail closed). The
Spring implementation is in
[outbox and inbox](../kotlin/backend/spring/events-outbox-inbox.md).

## Envelope

| Field | Meaning |
|---|---|
| `eventId` | Unique UUID; the consumer's inbox key |
| `eventType` | Past-tense name, for example `message.created` |
| `eventVersion` | Payload schema version |
| `producer` | The producing service |
| `tenantId` | Owning tenant |
| `correlationId` | From the originating request |
| `causationId` | Optional: the event that caused this one |
| `occurredAt` | UTC ISO-8601 |
| `payload` | Typed, minimal facts: identifiers, not restricted content |

## Rules

1. Producers publish only from a committed outbox row (same transaction as the
   domain write).
2. Consumers persist the event ID before side effects; a duplicate is a no-op.
3. Events are versioned, tenant-scoped, idempotent and schema-published.
4. Cross-service IDs are logical references, never foreign keys.
5. A new or changed event updates the AsyncAPI document, the JSON schema, the
   narrative event catalog and every listed consumer in the same change.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [API contracts](README.md)
- Documentation home: [Product Factory documentation](../../README.md)
- Agent guide: [AGENTS.md](../../../AGENTS.md)
<!-- documentation-routing:end -->
