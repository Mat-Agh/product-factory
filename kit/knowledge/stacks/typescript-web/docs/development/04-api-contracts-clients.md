# 04 — API contracts and clients

**Status:** Active
**Last updated:** {{DATE}}
**Source:** Product Factory knowledge pack (typescript-web)

The backend OpenAPI is the API truth. Link it from `docs/cross-project/`.

## Client rules

- Relative URLs only. Same-origin cookies, no API keys or origins in downloadable
  JavaScript.
- One typed client in `packages/core`: a function per endpoint, typed request and
  response, Problem Details parsed into a typed error with a safe message.
- Mutations send the CSRF header; on a CSRF failure, refresh the token and retry
  once.
- Handle 401 (re-authenticate), 403 (show forbidden), 409/412 (concurrency:
  refetch), and empty responses explicitly.

| Surface | Prefix | Authentication |
|---|---|---|
| `<surface>` | `<prefix>` | `<session cookie + CSRF>` |

## Contract change order

1. Update the backend OpenAPI first (in its repo).
2. Update `packages/core` types and client.
3. Update mocked browser tests to assert method, path and body.
4. Verify redaction, 401/403 handling, concurrency and empty responses.
5. Record consumer impact in the cross-project ledger when required.
