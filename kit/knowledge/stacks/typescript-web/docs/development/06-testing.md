# 06 — Testing

**Status:** Active
**Last updated:** {{DATE}}
**Source:** Product Factory knowledge pack (typescript-web)

## Layers

| Layer | Tool | Proves |
|---|---|---|
| Types | `tsc` | Contract and component-shape drift |
| Lint | ESLint (framework, React, hooks, accessibility, TypeScript rules) | Code-level rules |
| Unit | Vitest | Framework-neutral validation and API helpers |
| Browser | Playwright (desktop and mobile Chromium) | User workflows with intercepted endpoints |
| Build | Production build | Static export and SRI generation |
| Container | Image build and run | Shipped topology and headers |

Workflow changes require the browser tests. Tests assert outgoing mutation bodies
for contract-sensitive controls (roles, entitlements, membership). Unit tests are
never presented as browser-security coverage.

## Completion gate

```bash
pnpm check
pnpm test:e2e
git diff --check
```

For release candidates also build the image, check it runs as non-root, request
`/`, a hashed static asset and the 404 page, verify the security headers, and
validate the integrity manifest against the files in the image.
