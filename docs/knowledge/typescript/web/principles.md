# Web engineering principles

**Status:** Active and mandatory
**Last updated:** 2026-10-20

## Boundaries

- Deployable browser surfaces live in `apps/`, reusable runtime code in `packages/`,
  shared build policy in `tooling/`, engineering knowledge in `docs/`.
- The backend owns APIs, sessions and policy. The browser never invents commercial
  or authorization policy; it renders what the backend allows.
- Product behaviour and design contracts are upstream; link them from feature docs.
- A shared package never imports an app. Apps that serve different trust domains
  (for example a customer console and a vendor console) never share a session
  client.

## Engineering rules

- Pin direct dependencies exactly; update the lockfile intentionally.
- Typed domain names, no `any`; parse uncertain external responses at the API
  boundary.
- Semantic HTML, explicit labels, live regions, visible focus, responsive layouts,
  reduced-motion support.
- Run independent requests in parallel; prevent duplicate mutations while an action
  is busy.
- Destructive actions are explicit and confirmed.
- Show safe error messages. Never render raw server payloads, tokens, stack traces
  or secrets.
- Global tokens and layout in one stylesheet; feature code colocated under its
  feature.
- Update `docs/` whenever a future developer would otherwise rediscover a decision.

## Quality gate

```bash
pnpm check        # lint, typecheck, unit tests, production build
pnpm test:e2e     # when user workflows change
git diff --check
```

Compile is not done. A UI change is done when the workflows are exercised in a
browser and the evidence is recorded.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Web apps](README.md)
- Documentation home: [Product Factory documentation](../../../README.md)
- Agent guide: [AGENTS.md](../../../../AGENTS.md)
<!-- documentation-routing:end -->
