# 04 — Data and realtime

**Status:** Active
**Last updated:** {{DATE}}
**Source:** Product Factory knowledge pack (kmp-android-client)

## Layers

| Layer | File |
|---|---|
| Domain interface | `domain/repository/<Name>Repository.kt` |
| Implementation | `data/repository/<Name>RepositoryImpl.kt` |
| Remote source | `data/source/<name>_remote_source/` |
| Local source | `data/source/<name>_local_source/` |
| Wire types | `data/dto/*Dto.kt` |

Repository interfaces return domain types. DTOs stay in `data/` and map with
`toModel()`.

## Remote

- Features call the network facade (for example an `Api` interface in
  `:core:network`) with an explicit request policy; they never construct an
  `HttpClient` or import engine types.
- Backend errors are parsed as RFC 9457 Problem Details and mapped to typed domain
  failures.
- Retryable creates send an `Idempotency-Key` and keep it across ambiguous retries.
- Lists use cursor pagination.

## Local

- Encrypted datastore sources for auth state and settings; bounded caches with
  explicit invalidation.
- Identity-bound caches are cleared on sign-out and identity change.

## Realtime

- One supervised socket owns heartbeat, bounded jittered reconnect, desired
  subscriptions and bounded event-ID deduplication.
- Realtime events and push wakes are invalidation hints. Authoritative state comes
  from REST, so reconnect marks views stale and refetches.
- Outbound frames carry a strictly increasing connection-local sequence where the
  protocol requires it.
