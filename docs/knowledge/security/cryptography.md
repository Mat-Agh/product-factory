# Cryptography policy

**Status:** Active
**Last updated:** 2026-10-20

The approved suites and key-custody invariants for any product, server or client.
Where each control sits in code: [Spring](../kotlin/backend/spring/cryptography-by-layer.md),
[KMP client](../kotlin/client/security-and-cryptography.md),
[web](../typescript/web/security.md).

## Say what is protected

Server-side envelope encryption lets authorized services read data to perform
product behaviour. That is not end-to-end encryption, and no feature, document or
UI may claim otherwise. TLS, request envelopes, proof of possession, local
encryption and push encryption each solve a different threat.

## Approved suites

| Purpose | Suite |
|---|---|
| Restricted payload | AES-256-GCM, random 96-bit nonce, 128-bit tag, random per-record data key |
| Data-key protection | Versioned service or tenant wrapping keys; ordinary rotation rewraps keys without re-encrypting payloads |
| Sensitive auth request/response confidentiality | JWE `RSA-OAEP-256` + `A256GCM`, response signed and encrypted to a fresh per-request client key |
| Access, admin and audit signatures | RSA-3072 (PS256 for tokens and audit) or ECDSA P-256 (ES256 for device proofs) |
| Passwords | Argon2id, encoded parameters, at least 64 MiB / 3 iterations / parallelism 1, calibrated upward |
| Exact-match lookup over restricted values | HMAC-SHA-256 blind index with a field-and-purpose key |
| Opaque high-entropy token storage | SHA-256; HMAC-SHA-256 when the storage key is observable |
| Push payloads | RFC 8291 (P-256 ECDH, HKDF, AES-128-GCM) with VAPID ES256 |
| Backups | Encryption to an offline recipient key (for example `age` X25519) after compression |

## Key custody invariants

- Keys are generated on the target host with the OS CSPRNG. Release artifacts
  contain no installation key material.
- Private material lives in a root-owned secrets directory (`0700`, files `0400`)
  on encrypted storage, mounted only into the service that owns it.
- Each service has distinct, versioned keys per purpose; no key is reused across
  purposes or services.
- Missing, unreadable or mismatched keys fail startup. Existing keys are never
  silently regenerated.
- Raw data keys and revealed plaintext live in byte arrays scoped to the smallest
  use case and are zeroed on every exit; heap dumps are disabled in production.
- Rotation encrypts with the active version only and keeps declared historical
  decrypt versions until every dependent key is rewrapped.
- Environment flavors (beta, release) are isolation boundaries, never weaker
  security modes; each installation and flavor has its own key set.
- Hardware security modules, where used, hold non-extractable keys; an unavailable
  HSM fails new operations closed.

## Forbidden

Raw crypto primitives outside the one security module; Jasypt, `pgcrypto`,
transparent ORM encryption converters, deterministic encryption, shared static
production keys; constant or reused nonces; unversioned ciphertext; SHA-256 or
recoverable encryption for passwords; catching decryption failures and returning
empty or plaintext values; disabling TLS verification anywhere; describing
signing, hashing, encoding or redaction as encryption.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Security](README.md)
- Documentation home: [Product Factory documentation](../../README.md)
- Agent guide: [AGENTS.md](../../../AGENTS.md)
<!-- documentation-routing:end -->
