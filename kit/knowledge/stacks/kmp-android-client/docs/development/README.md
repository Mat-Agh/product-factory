# {{MODULE}} development handbook

**Status:** Active
**Last updated:** {{DATE}}

The engineering handbook for {{MODULE}}, a Kotlin Multiplatform client with
Android as the active target and optional iOS. Agents route through
[16-documentation-link-map.md](16-documentation-link-map.md).
`settings.gradle.kts`, `build-logic/` and `gradle/libs.versions.toml` win when
code and prose disagree.

## Start here

| Step | Document | Purpose |
|---|---|---|
| 1 | `AGENTS.md` (repo root) | Scope, workflow, gates |
| 2 | [16-documentation-link-map.md](16-documentation-link-map.md) | The smallest doc set for the task |
| 3 | [00-engineering-principles.md](00-engineering-principles.md) | Invariants |
| 4 | [09-tools-scripts-and-commands.md](09-tools-scripts-and-commands.md) | Commands |
| 5 | [15-code-samples-index.md](15-code-samples-index.md) | The canonical MVI page set |

## Full index

| # | Document | Scope |
|---|---|---|
| 00 | [engineering-principles](00-engineering-principles.md) | Architecture, feature structure, data rules, UI, targets, completion gate |
| 01 | [kmp-module-structure](01-kmp-module-structure.md) | Targets, layout, source-set rules, expect/actual, iOS workflow |
| 02 | [kotlin-language](02-kotlin-language.md) | Naming, mappers, coroutines, style |
| 03 | [presentation-mvi](03-presentation-mvi.md) | Screen, ViewModel, State, Action, Event, navigation |
| 04 | [data-realtime](04-data-realtime.md) | DTOs, remote and local sources, repositories, realtime reconciliation |
| 05 | [library-stack](05-library-stack.md) | Each library: why, owning facade, what features must not import |
| 06 | [ios](06-ios.md) | The iOS target |
| 07 | [android-development](07-android-development.md) | App shell, platform modules, flavors, local hosts |
| 09 | [tools-scripts-and-commands](09-tools-scripts-and-commands.md) | Gradle tasks, gates, setup |
| 10 | [testing-performance-security](10-testing-performance-security.md) | Test placement, performance, client security rules |
| 11 | [build-system-gradle](11-build-system-gradle.md) | Sources of truth, convention plugins, repositories, flavors |
| 12 | [client-cryptography](12-client-cryptography.md) | TLS trust, proof of possession, local encryption, logging |
| 15 | [code-samples-index](15-code-samples-index.md) | Canonical feature walkthrough |
| 16 | [documentation-link-map](16-documentation-link-map.md) | Agent routing |
| — | [DEV_SETUP](DEV_SETUP.md) | Workstation setup (Linux, macOS, Windows) |
| — | [ANDROID_ACCEPTANCE_TEST](ANDROID_ACCEPTANCE_TEST.md) | Runtime acceptance checklist |
| — | [CICD](CICD.md) | This repo's pipeline |
| — | [../architecture/APP_ARCHITECTURE.md](../architecture/APP_ARCHITECTURE.md) | App architecture and data policy |
| — | [knowledge/](knowledge/README.md) | General knowledge imported from the factory library: principles, API, security, CI/CD, agentic workflows |
