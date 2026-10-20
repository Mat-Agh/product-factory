# Spring security contracts

**Status:** Active
**Last updated:** 2026-10-20

The security chapter for implementers. The threat model lives in
the repository's `docs/security/` area and [security](../../../security/README.md); cryptography in
[10-cryptography.md](cryptography-by-layer.md).

## API surfaces

Record each surface and its authentication posture.

| Prefix | Auth posture |
|---|---|
| `/api/v1` | Authenticated member sessions or tokens |
| `/ws/v1` | Short-lived single-use ticket obtained over REST |
| `/admin/v1` | Administration session cookie plus CSRF token (BFF) |
| `/internal/v1` | Service-to-service only; never exposed by the edge |

Internal service-to-service calls use mTLS where the platform supports it. Each
request is bound to the target service audience and an allow-listed method and
path scope. Actor and tenant context are added only by internal client adapters
and validated before controller work. Missing, duplicated, malformed, expired or
mismatched context fails closed.

## Authorization checklist (every resource operation)

1. An authenticated principal is present.
2. Tenant membership.
3. Capability or entitlement.
4. Role in the surface or workspace, when applicable.
5. Ownership or resource scope, when applicable.

Never accept tenant or member identity from a request body when it can come from
the authenticated principal.

## Facts vs decisions

| Kind | Mechanism |
|---|---|
| Past facts (created, updated, …) | Broker outbox and inbox |
| Current allow/deny decisions | Internal HTTP, fail closed |

## Credentials and tokens

- Passwords: Argon2id with upgrade-on-login; length limits, no composition rules,
  compromised-password screening.
- Refresh tokens: opaque, hashed at rest, rotated, replay-protected.
- Provider credentials, private content and personal data: contextual envelope
  encryption ([10](cryptography-by-layer.md)).
- Infrastructure links use authenticated TLS: PostgreSQL `verify-full`, broker
  TLS with per-service ACLs, Redis TLS with named users, object storage over HTTPS.
- Retryable creates and confirmations require an `Idempotency-Key` header.
- Browser mutations use rotating CSRF tokens and `HttpOnly`, `Secure`,
  `SameSite=Strict` cookies.

## Errors and observability

- Errors use RFC 9457 Problem Details with a stable `code` and a correlation ID.
- Logs, metrics, traces, diagnostics and error bodies never include tokens,
  secrets, provider payloads, private content, or personal data beyond approved
  identifiers.
- Catch errors only to translate them. Never log secret input or provider response
  bodies.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Spring Boot services](README.md)
- Documentation home: [Product Factory documentation](../../../../README.md)
- Agent guide: [AGENTS.md](../../../../../AGENTS.md)
<!-- documentation-routing:end -->
