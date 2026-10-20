# HTTP API conventions

**Status:** Active
**Last updated:** 2026-10-20

Wire rules for every HTTP API, whatever the server stack. Contracts are
design-first (OpenAPI 3.1) and change in the same change as the code; broker and
WebSocket payloads publish AsyncAPI and versioned JSON Schemas
([events](events.md)). Errors: [errors.md](errors.md).

## Surfaces and versions

Version by path prefix and keep surfaces separate:

| Prefix (example) | Audience | Authentication |
|---|---|---|
| `/api/v1` | Public REST clients | Member access token (proof-of-possession where supported) |
| `/ws/v1` | Public WebSocket | Short-lived single-use ticket obtained over REST |
| `/setup/v1` | One-time installation setup | A one-use setup credential exchanged for an HttpOnly session |
| `/admin/v1` | Browser administration (BFF) | Opaque HttpOnly session cookie plus CSRF token |
| `/internal/v1` | Service to service only | mTLS service identity; never exposed by the edge |

A breaking change is a new version. The edge aggregates only public routes.

## Wire rules

| Concern | Rule |
|---|---|
| Field names | camelCase |
| Enum values | lowercase snake_case through one shared wire-enum contract |
| IDs | Opaque UUID strings |
| Time | UTC ISO-8601 strings, produced by the presentation mapper from typed domain values |
| Lists | Opaque cursor pagination with a bounded page size; never database offsets on the wire |
| Retryable creates and confirmations | Mandatory `Idempotency-Key`; the same key returns the same result |
| Concurrency | Optimistic: `ETag`/`If-Match` or a version field, answered with 409/412 on conflict |
| Correlation | `X-Correlation-Id`, generated when absent or invalid, echoed in errors and logs |
| Identity | Tenant and actor come from the authenticated principal, never from the body |
| Empty responses | `204 No Content` |

## Resource naming

- The public API is resource-oriented: nouns in paths, CRUD as `GET`, `POST`,
  `PATCH`, `DELETE`.
- A non-CRUD operation is a `POST` to an action sub-resource
  (`POST /surfaces/{id}/leave`, `POST /invitations/accept`). That is the intended
  pattern, not a violation; do not contort it into nouns.
- The internal mesh may be RPC-shaped (`/internal/v1/decisions/...`), because it
  carries decisions and commands, not resources.

## Authentication patterns

- Access tokens: short-lived signed JWTs with issuer, audience, subject, session,
  tenant and role claims; bound to a device key (DPoP `cnf.jkt`) where clients
  support it.
- Refresh tokens: opaque, stored only as hashes, device-bound, rotated on every
  use; replay revokes the session family.
- Browser sessions: an opaque handle in a `__Host-` cookie with `Secure`,
  `HttpOnly`, `SameSite=Strict`, plus a server-issued CSRF token on mutations.
- Realtime: a single-use ticket fetched over authenticated REST, consumed by the
  WebSocket handler.

## Contract files

| Surface | File | Owner |
|---|---|---|
| Public REST | `docs/api/openapi/<service>.yaml` | The owning service |
| Events and frames | `docs/api/asyncapi/<service>.yaml` | The producing service |
| Shared schemas | `docs/api/schemas/*.schema.json` | `core/contracts` owner |

Contract change order: update the contract, then the code, then tell consumers
through the cross-project docs in the same change.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [API contracts](README.md)
- Documentation home: [Product Factory documentation](../../README.md)
- Agent guide: [AGENTS.md](../../../AGENTS.md)
<!-- documentation-routing:end -->
