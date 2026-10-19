# 10 — Testing, performance and security

**Status:** Active
**Last updated:** {{DATE}}
**Source:** Product Factory knowledge pack (kmp-android-client)

## Testing

- Platform-neutral use-case, mapper, repository and ViewModel tests go in the
  shared test source sets of the owning module.
- Android integration and UI tests go in Android test source sets.
- Fakes stand in for product and backend boundaries; tests never encode a
  replacement contract.
- Cover protocol edges explicitly: Problem Details parsing, DTO fixtures,
  pagination and idempotency, realtime sequencing, reconnect and deduplication,
  token refresh concurrency.
- A compile-only result is not live acceptance. Runtime behaviour is accepted with
  [ANDROID_ACCEPTANCE_TEST.md](ANDROID_ACCEPTANCE_TEST.md).

Record here if the product restricts tests in this repo, and keep `tests:` in
`.factory/factory.yaml` consistent.

## Performance

- Blocking I/O off the main thread.
- Stable keys for lazy lists; bounded caches with explicit invalidation.
- Never rebuild navigation stacks or network clients during recomposition.
- Measure startup, scrolling, memory and network on release candidates.

## Security

- Never commit secrets, tokens, signing material or machine-local endpoints.
- Store sensitive local values only through the security and encrypted-datastore
  modules.
- Never log tokens, credentials, private content or personal data.
- Validate deep links, opaque IDs, capability state and backend errors before
  navigation or mutation.
- Keep release shrinking and obfuscation configured by build logic.
- Treat realtime and push payloads as invalidation hints; reconcile over REST.
- Follow [12-client-cryptography.md](12-client-cryptography.md).
