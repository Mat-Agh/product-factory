# Web tools and commands

**Status:** Active
**Last updated:** 2026-10-20

## Prerequisites

- Node.js at the version the production builder uses
- Corepack and the pinned pnpm version
- Docker for container validation
- Playwright browsers for end-to-end tests

```bash
corepack enable
pnpm install --frozen-lockfile
pnpm exec playwright install chromium
```

## Commands

| Command | Purpose |
|---|---|
| `pnpm dev:<app>` | Development server |
| `pnpm lint` | ESLint |
| `pnpm typecheck` | Strict TypeScript across packages |
| `pnpm test` | Unit tests |
| `pnpm build` | Production static export, SRI, integrity manifest |
| `pnpm test:e2e` | Browser workflows |
| `pnpm check` | Lint, typecheck, unit tests and build |

A real local session is served through the backend edge because cookies, CSRF and
APIs are same-origin. Browser tests intercept endpoints and need no backend.

## First change

1. Read `AGENTS.md` and `docs/README.md`.
2. `git status --short --branch`; preserve existing work.
3. Identify the product behaviour and the backend OpenAPI that own the change.
4. Add or update a focused test with the implementation.
5. Run the gate in [00](principles.md).

Never create `.env` files containing credentials.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Web apps](README.md)
- Documentation home: [Product Factory documentation](../../../README.md)
- Agent guide: [AGENTS.md](../../../../AGENTS.md)
<!-- documentation-routing:end -->
