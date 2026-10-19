# Integrations

**Status:** Active
**Last updated:** {{DATE}}

One section per platform service or third-party SDK the client uses.

## Integrations

| Integration | Purpose | Owning module | Data it may see | Consent |
|---|---|---|---|---|
| Push distributor | Background wakes | `:core:push` | Opaque wake payloads | Notification permission |
| OAuth provider | Account linking | `:core:<provider>_auth` | Scopes in the contract | Explicit |
| Crash reporting | Stability | `:core:crash_reporting` | Redacted stack traces | Telemetry consent |
| Analytics | Product metrics | `:core:analytics` | Approved events only | Telemetry consent |

For each integration record: the exact scopes or permissions, the registration
steps an operator performs (signing-key fingerprints, client IDs), the failure and
revocation behaviour, and what must never reach it (tokens, private content,
personal data).
