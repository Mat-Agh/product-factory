# Client data

**Status:** Active
**Last updated:** {{DATE}}

## Model

The client's domain models and where each comes from. The backend OpenAPI owns
wire shapes; this page maps them.

| Domain model | Source DTO / endpoint | Feature |
|---|---|---|
| `<Model>` | `<Dto>` / `<GET /api/v1/...>` | `:feature:<name>` |

## Persistence

| Local data | Store | Encrypted | Cleared on |
|---|---|---|---|
| Auth state | Encrypted datastore | Yes | Sign-out, identity or host change |
| `<cache>` | `<store>` | `<yes/no>` | `<event>` |

What is never persisted: passwords, one-time capabilities, OAuth codes, provider
tokens the backend owns, presigned URLs, per-request keys.

## Relations

Caches are invalidated by realtime events and push wakes and rebuilt from REST.
Record each cache's invalidation trigger and staleness rule.
