# Example product `AGENTS.md`

**Status:** Example — copy the structure, replace every command with your own
**Last updated:** 2026-10-19

This is an illustrative `AGENTS.md` for a typical web product (a Next.js app with
Postgres). The factory's agents read **only** this file and
`.factory/factory.yaml` to learn about a product. Every heading below is one they
look for. Commands must be real and must have been run successfully. Do not
copy these placeholders as they are.

---

```markdown
# Agent instructions — Acme Web

Acme Web is the customer dashboard for Acme. Next.js 15 (App Router), TypeScript,
Prisma + Postgres, Tailwind with design tokens in `src/styles/tokens.css`.

## Setup
Fresh checkout (used for every factory worktree):
    cp .env.example .env.local        # dev values only; never real secrets
    npm ci
    npx prisma migrate deploy && npm run db:seed

## Dev server
    PORT=<port> npm run dev           # serves http://localhost:<port>
Test accounts (seeded): admin@acme.test / member@acme.test, password `dev-password`.
Never sign in with real accounts.

## Gates
Run for the areas you changed. All must pass.
    npm run lint -- <changed files>
    npm run typecheck
    npm test -- <related tests>       # Vitest; colocated *.test.ts
    npm run build
E2E (UI changes): npx playwright test e2e/<area>
Compile alone is not done.

## Conventions
- Server components by default; `"use client"` only for interactivity.
- Data access only through `src/server/repositories/*`; never Prisma in components.
- One component per file; colocate tests.
- Living samples: `src/app/(dashboard)/tenants/` (list + empty/error states),
  `src/server/repositories/tenantRepository.ts`.
- Copy is in `messages/en.json` and `messages/fa.json`; FA is RTL.
- Authorization: every server action calls `requireRole()` first. Fail closed.

## Docs map
| Area | Read |
|---|---|
| Product requirements | docs/product/ (PRD-NNN-*.md) |
| Architecture decisions | docs/adr/ |
| API routes | docs/api/README.md |
| Design system | docs/design/README.md, src/styles/tokens.css |

## Product Factory
(appended by scripts/install.sh — keep it)
```

---

## Why each section matters

| Section | Used by | If missing |
|---|---|---|
| Setup | coordinator (worktree setup), every agent that runs code | Worktrees fail to build, and jobs stop at the start |
| Dev server | engineer, reviewer-ux, reviewer-ui, tester | UI work cannot be seen. UX/UI reviews become guesses |
| Test accounts / fixtures | reviewer-ux, tester | Signed-in flows cannot be walked, and bugs end `NOT REPRODUCED` |
| Gates | engineer, reviewer-code, tester | "Done" has nothing to check against |
| Conventions + living samples | engineer, reviewer-code, tech-lead | Agents invent new patterns |
| Docs map | every agent | Agents read whole `docs/` trees, or miss the governing PRD |

<!-- documentation-routing:start -->
## Documentation routing

- [Parent README](../README.md)
- [Governing agent guide](../../../AGENTS.md)
<!-- documentation-routing:end -->
