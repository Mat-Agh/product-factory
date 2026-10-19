# 01 — App architecture

**Status:** Active
**Last updated:** {{DATE}}
**Source:** Product Factory knowledge pack (typescript-web)

## Workspace layout

| Path | Responsibility |
|---|---|
| `apps/<app>/` | One deployable browser surface |
| `packages/core/` | Typed representations of backend contracts and the fetch client |
| `packages/<name>/` | Other shared runtime code (UI kit, utilities) |
| `tooling/` | Shared lint, TypeScript and build configuration |
| `docs/` | Engineering knowledge |

## Rendering model

Record the model and why. A common, secure default for admin consoles is a
static export (for example Next.js App Router with `output: export`) served by a
hardened web server behind the backend edge:

- API calls are same-origin and browser-authenticated by backend-owned cookies;
- there is no Node.js BFF and no second session layer in this repo;
- client components only at interactive boundaries; layouts and metadata stay
  server components;
- no server actions or route handlers unless the topology changes and a security
  review approves it.

## Runtime flow

Write the boot sequence of each app as numbered steps: load the shell, obtain the
CSRF token, read status, choose the view (setup, login, console), load data in
parallel, perform mutations with CSRF and refresh authoritative data.

## Source layout

| Path | Responsibility |
|---|---|
| `apps/<app>/app/` | Layouts, metadata, error and not-found boundaries, global CSS |
| `apps/<app>/src/features/<feature>/` | One feature: views, state, panels |
| `packages/core/src/types.ts` | Backend contract types |
| `packages/core/src/client.ts` | Fetch, CSRF, redacted errors, typed endpoint functions |
