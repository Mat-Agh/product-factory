# API errors

**Status:** Active
**Last updated:** 2026-10-20

Every failure is RFC 9457 Problem Details (`application/problem+json`) with stable
machine fields:

```json
{
  "type": "https://example.invalid/problems/validation_failed",
  "title": "Bad Request",
  "status": 400,
  "detail": "Request validation failed",
  "instance": "/api/v1/example",
  "code": "validation_failed",
  "correlationId": "4a25e141-cde6-4f79-80f0-bd4d51e01c04",
  "errors": [{ "field": "email", "code": "email", "message": "must be a well-formed email address" }]
}
```

## Rules

- `code` is stable snake_case and listed in the repo's error catalog with its
  HTTP status and meaning. Clients branch on `code`, never on `detail`.
- `correlationId` matches logs and traces.
- Authorization and tenancy failures never reveal whether a cross-tenant resource
  exists: answer the same `404`/`403` for "absent" and "hidden".
- Login failures do not say which credential was wrong.
- Provider errors map to stable product codes; provider bodies and credentials are
  never echoed.
- Unexpected failures map to `internal_error` with no internals.
- An identity or authorization service outage fails closed (`503` or `403`), never
  open.

## Catalog shape

| Code | HTTP | Meaning |
|---|---:|---|
| `validation_failed` | 400 | Field or command validation failed |
| `invalid_credentials` | 401 | Login failed, without saying which credential |
| `access_denied` | 403 | Authenticated but not allowed |
| `not_found` | 404 | Absent, or hidden by tenant isolation |
| `version_conflict` | 409 | Optimistic concurrency conflict; refetch |
| `expired` | 410 | A time-limited capability elapsed |
| `policy_failed` | 422 | Input violates a domain rule |
| `dependency_unavailable` | 503 | A required dependency is down; retry |
| `internal_error` | 500 | Sanitized unexpected failure |

Domain-specific codes extend this list (for example `last_active_admin`,
`participant_busy`); name them after the rule, not the HTTP status.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [API contracts](README.md)
- Documentation home: [Product Factory documentation](../../README.md)
- Agent guide: [AGENTS.md](../../../AGENTS.md)
<!-- documentation-routing:end -->
