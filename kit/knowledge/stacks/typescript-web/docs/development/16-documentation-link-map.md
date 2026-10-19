# 16 — Documentation link map

**Status:** Active — agent routing authority
**Last updated:** {{DATE}}
**Source:** Product Factory knowledge pack (typescript-web)

## Knowledge order (mandatory)

1. This map, [../README.md](../README.md) and `AGENTS.md`.
2. The routed docs below.
3. Upstream owners (product, design, backend) through
   [../cross-project/README.md](../cross-project/README.md).
4. Authoritative online sources, only for what is still missing.
5. Durable findings go into the docs.

## Router

| Task type | Read first | Then, if needed |
|---|---|---|
| Any non-trivial task | `AGENTS.md`, the docs index, this file | The matching row |
| What the repo is now | `CURRENT_STATE.md` | Feature implementation status |
| Invariants | [00](00-engineering-principles.md) | [01](01-app-architecture.md) |
| Architecture, runtime flow | [01](01-app-architecture.md) | ADRs |
| TypeScript style | [02](02-typescript-language.md) | [15](15-code-samples-index.md) |
| UI, accessibility, RTL | [03](03-ui-design-system.md) | Design contracts via cross-project |
| API calls, contract change | [04](04-api-contracts-clients.md) | Backend OpenAPI via cross-project |
| Security | [05](05-security.md) | Backend security docs |
| Tests | [06](06-testing.md) | [09](09-tools-scripts-and-commands.md) |
| Build, container, release | [07](07-build-deployment.md) | [CICD](CICD.md) |
| Dependencies | [08](08-library-stack.md) | Release notes |
| Commands | [09](09-tools-scripts-and-commands.md) | Root `README.md` |
| Reviewing a UI change | [REVIEW_CHECKLIST](REVIEW_CHECKLIST.md) | [03](03-ui-design-system.md), [05](05-security.md) |
| Decisions | The decisions README | The ADR |
| History | The development log | `git log` |
