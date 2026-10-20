# Kotlin Multiplatform client

**Status:** Active
**Last updated:** 2026-10-20

Knowledge for a KMP app with Android as the active target and optional iOS.

| Page | Covers |
|---|---|
| [principles.md](principles.md) | Client invariants and the completion gate |
| [architecture.md](architecture.md) | Shells, shared, core, features; online-first data policy; request policy |
| [kmp-source-sets.md](kmp-source-sets.md) | Targets, layout, source-set rules |
| [android.md](android.md) | App shell, platform modules, flavors, local hosts and trust |
| [ios.md](ios.md) | The iOS target as far as it has been built |
| [presentation-mvi.md](presentation-mvi.md) | Screen, ViewModel, State, Action, Event; navigation; required states |
| [data-and-realtime.md](data-and-realtime.md) | Repositories, remote and local sources, realtime reconciliation |
| [library-stack.md](library-stack.md) | Every library: why, owning facade, what features must not import |
| [security-and-cryptography.md](security-and-cryptography.md) | TLS trust, auth envelopes, DPoP, local encryption, push, logging |
| [testing-and-performance.md](testing-and-performance.md) | Test placement, performance, client security rules |
| [acceptance.md](acceptance.md) | The Android runtime acceptance checklist |
| [setup.md](setup.md) | Workstation setup on Linux, macOS and Windows |
| [tools.md](tools.md) | Gradle tasks and gates |
| [code-samples.md](code-samples.md) | The canonical MVI page set |

Shared Kotlin rules: [language](../language.md), [Gradle](../gradle.md).

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Kotlin](../README.md)
- Documentation home: [Product Factory documentation](../../../README.md)
- Agent guide: [AGENTS.md](../../../../AGENTS.md)
- In this folder:
  - [Android acceptance checklist](acceptance.md)
  - [Android development](android.md)
  - [Client app architecture](architecture.md)
  - [Client code samples](code-samples.md)
  - [Client data and realtime](data-and-realtime.md)
  - [iOS](ios.md)
  - [KMP module structure and source sets](kmp-source-sets.md)
  - [Client library stack](library-stack.md)
  - [Presentation: MVI](presentation-mvi.md)
  - [Client engineering principles](principles.md)
  - [Client cryptography and secure transport](security-and-cryptography.md)
  - [Client development setup](setup.md)
  - [Client testing, performance and security](testing-and-performance.md)
  - [Client tools and commands](tools.md)
<!-- documentation-routing:end -->
