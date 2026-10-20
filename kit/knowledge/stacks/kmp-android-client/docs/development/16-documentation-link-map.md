# 16 — Documentation link map

**Status:** Active — agent routing authority
**Last updated:** {{DATE}}

## Knowledge order (mandatory)

1. This map, [../README.md](../README.md) and `AGENTS.md`.
2. The routed docs below.
3. Upstream owners (product, design, backend) through
   [../cross-project/README.md](../cross-project/README.md).
4. Authoritative online sources, only for what is still missing.
5. Durable findings go into `docs/`.

## Router

| Task type | Read first | Then, if needed |
|---|---|---|
| Any non-trivial task | `AGENTS.md`, `docs/README.md`, this file | The matching row |
| What the app is now | `CURRENT_STATE.md` | `docs/features/IMPLEMENTATION_STATUS.md` |
| Invariants | [00](00-engineering-principles.md) | [01](01-kmp-module-structure.md) |
| Source sets, iOS | [01](01-kmp-module-structure.md) | [07](07-android-development.md) |
| Screens and MVI | [03](03-presentation-mvi.md) | [15](15-code-samples-index.md), the feature's UX spec |
| Backend calls, caching, realtime | [04](04-data-realtime.md) | Backend OpenAPI via cross-project |
| Dependencies | [05](05-library-stack.md) | [11](11-build-system-gradle.md) |
| Android shell, flavors, local hosts | [07](07-android-development.md) | [DEV_SETUP](DEV_SETUP.md) |
| Commands and gates | [09](09-tools-scripts-and-commands.md) | [CICD](CICD.md) |
| Tests, performance, security | [10](10-testing-performance-security.md) | [12](12-client-cryptography.md) |
| Gradle | [11](11-build-system-gradle.md) | `build-logic/` |
| TLS, keys, local encryption | [12](12-client-cryptography.md) | `docs/security/` if present |
| Runtime acceptance | [ANDROID_ACCEPTANCE_TEST](ANDROID_ACCEPTANCE_TEST.md) | Product acceptance docs |
| Integrations (push, OAuth, analytics) | `docs/integrations/README.md` | [05](05-library-stack.md) |
| A feature | `docs/features/README.md` | Its UX spec and status row |
| Decisions | `docs/decisions/README.md` | The ADR |
| History | `docs/history/DEVELOPMENT_LOG.md` | `git log` |
| iOS | [06](06-ios.md) | [01](01-kmp-module-structure.md) |
| App architecture, data policy | [../architecture/APP_ARCHITECTURE.md](../architecture/APP_ARCHITECTURE.md) | [04](04-data-realtime.md) |
| General principles, evidence, documentation | [knowledge/principles/](knowledge/principles/README.md) | |
| API conventions, errors, events | [knowledge/api/](knowledge/api/README.md) | |
| Security policy, classification, identity | [knowledge/security/](knowledge/security/README.md) | |
| CI/CD and workflows | [knowledge/cicd/](knowledge/cicd/README.md) | [CICD](CICD.md) |
| Agentic workflows | [knowledge/agentic/](knowledge/agentic/README.md) | |
