# Android acceptance checklist

**Status:** Active
**Last updated:** {{DATE}}
**Source:** Product Factory knowledge pack (kmp-android-client)

Use a configured emulator or device against a fresh development backend. Report
the exact build variant, device and API level, backend environment and SHA,
timestamps, commands and failures. Do not mark a feature accepted from compile
success alone.

## Build gates

```bash
./gradlew check verifyLocalization --no-daemon
./gradlew :androidApp:assembleBetaDebug :androidApp:assembleProdDebug --no-daemon
```

## Runtime coverage

Adapt the list to the product; keep every line that applies.

1. Record client and backend SHAs, origins, build hashes, devices and API levels,
   installed development CA fingerprint, timestamps and commands.
2. Provision real test accounts through the product's own flows. Exercise sign-in,
   refresh, sign-out, token rotation and host switching.
3. Exercise every changed screen in loading, empty, content, error, offline and
   unauthorized states.
4. Exercise creates with idempotency across an ambiguous retry, pagination, and
   optimistic updates reconciled with the server.
5. Exercise realtime: connect, heartbeat timeout, network loss, bounded reconnect,
   duplicate events, and REST reconciliation.
6. Exercise push in foreground, background and process-killed states; expiry,
   duplicates and sign-out cleanup.
7. Check compact and expanded layouts, every shipped language including RTL, screen
   reader labels, large text, clipping, contrast, system back and deep links.
8. Sign out and confirm subscriptions, sensitive state, connections and
   identity-bound caches are cleared.
9. Inspect storage, logcat, network logs and crash or analytics reports for leaked
   secrets or personal data.

Compare behaviour with the product requirements, design screens and backend
OpenAPI.

## Evidence log

| Date | Build | Devices | Backend | Result | Notes |
|---|---|---|---|---|---|
