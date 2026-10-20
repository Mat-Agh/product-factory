# Client cryptography and secure transport

**Status:** Active
**Last updated:** 2026-10-20

The single client source of truth for transport and local cryptography. Backend
wire truth stays in its OpenAPI and security docs.

## Security boundary

State precisely what the product protects. If the server decrypts, validates,
stores and routes content, the product is server-visible and not end-to-end
encrypted. TLS, request envelopes, proof of possession, encrypted local state and
push encryption solve different threats; none of them is E2EE.

## TLS and origins

- All API, callback, media and realtime traffic uses HTTPS/WSS; cleartext is
  disabled in the manifest and network security configuration.
- Each flavor reads its own HTTPS base URL; WSS derives from it.
- Release trusts system roots (plus any documented bundled CA). If self-hosted
  servers without a public certificate must be supported, use an explicit,
  interactive trust-on-first-use pin of the exact SHA-256 fingerprint, re-prompting
  with a warning when it changes. Debug may add user-installed development CAs.
- Hostname verification is never disabled.
- A session is bound to its normalised origin. Switching host invalidates
  incompatible authentication and notification state.

## Authentication envelopes and proof of possession

- Authentication policy is explicit per request in the network facade (for example
  none, auth envelope, proof-of-possession token, action grant, external media).
- Where sensitive auth calls use JWE request/response envelopes, generate a fresh
  response key per request, bind method, path, time and request ID, verify the
  server's signature, and fail closed on any algorithm, key, time or status
  mismatch.
- A non-exportable installation key (Android Keystore P-256) signs device proofs,
  DPoP proofs and realtime hellos. DPoP proofs bind method, URL, token hash, time
  and a unique ID; retry at most once for a server nonce, then refresh single-flight.

## Local encryption

- Auth state and approved settings pass through the encrypted datastore and the
  platform keystore or keychain.
- Never persist passwords, one-time capabilities, OAuth codes, provider tokens the
  backend owns, presigned URLs or per-request private keys.
- Key loss, format mismatch or host mismatch invalidates the session; there is no
  silent downgrade to plaintext storage.

## Push and realtime

- Push payloads are opaque, bounded, versioned and short-lived; validate, deduplicate
  and reconcile over REST before presenting anything.
- Realtime frames are invalidation hints; authoritative state comes from REST.

## Logging and diagnostics

Redact authorization headers, proofs, envelope bodies, passwords, capabilities,
OAuth material, provider identities, push secrets, presigned URLs and media
credentials from logs, analytics, crash reports and diagnostics. Optional telemetry
stays off until explicit consent.

## Rotation and failure rules

Server keys are rediscovered only on expiry or an explicit rollover error. The
installation key is stable; its loss requires re-enrolment. Tokens are host- and
device-bound and refreshed single-flight. Any validation failure stops decoding;
no cryptographic or TLS downgrade path exists.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Kotlin Multiplatform client](README.md)
- Documentation home: [Product Factory documentation](../../../README.md)
- Agent guide: [AGENTS.md](../../../../AGENTS.md)
<!-- documentation-routing:end -->
