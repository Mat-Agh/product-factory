# API contracts

**Status:** Active
**Last updated:** {{DATE}}
**Source:** Product Factory knowledge pack (spring-kotlin-service)

Contracts are design-first and change in the same change as the code. OpenAPI
lives in [openapi/](openapi/README.md), AsyncAPI in [asyncapi/](asyncapi/README.md),
shared JSON schemas in [schemas/](schemas/README.md).

## Conventions

| Concern | Rule |
|---|---|
| Versioning | By path prefix (`/api/v1`, `/ws/v1`, `/admin/v1`). A breaking change is a new version |
| Field names | camelCase |
| Enum values | lowercase snake_case, one shared wire-enum contract |
| Timestamps | UTC ISO-8601 strings |
| IDs | UUIDs; list cursors are opaque strings |
| Lists | Cursor pagination with a bounded page size; never unbounded |
| Retries | Creates and confirmations accept `Idempotency-Key`; the same key returns the same result |
| Concurrency | Optimistic concurrency with `ETag` / `If-Match` or a version field where updates race |
| Identity | Tenant and actor come from the authenticated principal, never from the body |
| Empty responses | `204 No Content`, never `200` with an empty object |

## Errors

Every failure is RFC 9457 Problem Details (`application/problem+json`):

```json
{
  "type": "about:blank",
  "title": "Conflict",
  "status": 409,
  "code": "message_already_exists",
  "correlationId": "4b9d…"
}
```

- `code` is stable, snake_case, and documented in an error catalog.
- `correlationId` matches the logs and traces.
- Messages never include secrets, tokens, provider payloads or stack traces.
- Unexpected errors map to `internal_error` without detail.

## Contracts

| Surface | File | Owner service |
|---|---|---|
| Public REST | `openapi/<service>.yaml` | `<service>` |
| Events | `asyncapi/<service>.yaml` | `<service>` |

Contract change order: update the contract first, then the code, then the consumer
docs (sibling repos are told through `docs/cross-project/`).
