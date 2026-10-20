# Data classification

**Status:** Active
**Last updated:** 2026-10-20

Classify every new value before choosing its schema, API shape, cache or log
treatment. The class decides the control; encryption never replaces
authorization, row-level security, retention or redaction.

| Class | Examples | Storage | API and logs |
|---|---|---|---|
| Secret | Passwords, refresh tokens, OAuth credentials, private keys | Hash when verification-only; otherwise contextual envelope encryption | Never logged, never returned after creation |
| Restricted content | Private messages, calendar detail, setup drafts | Contextual envelope; no plaintext search | Never in logs, events, push payloads or errors |
| Personal data | Email, display name, biography | Envelope; a blind index only for an approved exact lookup | Approved identifiers only |
| Sensitive operational | Sync cursors, push endpoint keys | Envelope or purpose-specific HMAC | Redacted |
| Tenant and routing metadata | IDs, status, kind, timestamps, relationships | Plaintext where needed for RLS, integrity, routing, indexing | Allowed |
| Public | Hostnames, public keys, advertised capabilities | Plaintext with integrity controls | Allowed |

If a field must support equality lookup, sorting, range queries or uniqueness,
document why. Never switch to deterministic encryption to get it.

**Plaintext is not public.** Authorization, tenant isolation, least privilege,
redaction, retention and encrypted transport and storage still apply.

## Never in operational output

Logs, metrics labels, traces, errors, audit payloads, push messages, idempotency
records and event envelopes never contain: private content (plaintext or
ciphertext), passwords, tokens, cookies, authorization codes, provider
credentials, envelope nonces or wrapped keys, full personal or provider payloads,
or secret-bearing URLs, headers and request bodies.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Security](README.md)
- Documentation home: [Product Factory documentation](../../README.md)
- Agent guide: [AGENTS.md](../../../AGENTS.md)
<!-- documentation-routing:end -->
