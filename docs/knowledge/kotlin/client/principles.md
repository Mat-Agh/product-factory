# Client engineering principles

**Status:** Active and mandatory
**Last updated:** 2026-10-20

Exceptions are recorded in
../architecture/CONFORMANCE_LEDGER.md.

## Architecture

- Platform shells are thin. Shared application composition lives in `:shared`.
- Reusable infrastructure belongs in `:core:*`; vertical behaviour belongs in
  `:feature:*`.
- Features depend on core facade interfaces, never on vendor SDKs directly.
- Product and design contracts are upstream. Client docs map them to the
  implementation; they never redefine them.
- Backend wire contracts are consumed through explicit DTOs and repositories.

## Feature structure

```text
feature/<name>/src/commonMain/kotlin/.../
  di/
  data/{dto,source,repository}/
  domain/{model,repository,usecase}/
  presentation/{component/page,navigation}/
```

Each screen page owns its `Screen`, `ViewModel`, `State`, `Action` and `Event`
types ([03](presentation-mvi.md)). One public top-level type per file.
Interfaces and implementations live in separate files.

## Kotlin and data rules

- Plain names are domain models; `*Dto` is a wire shape; `*Data` is persisted local
  data.
- Extension mappers named `toModel()`, `toDto()`, `toData()`.
- DTOs and persistence types never reach presentation code.
- Coroutines with `StateFlow` / `SharedFlow` and structured concurrency. No
  `GlobalScope`.
- Versions only in `gradle/libs.versions.toml`.

## UI and navigation

- Compose UI renders state and emits actions. Business work stays in use cases and
  repositories.
- Use the design-system module's tokens and components, never feature-local styling.
- Feature navigation owns typed keys and stack mutation. Cross-feature navigation
  goes through the shared shell contract.
- Localization keys stay paired across languages; verify RTL where a right-to-left
  language ships.

## Targets

- Android is the active implementation and verification target.
- iOS source sets are optional, enabled by a Gradle property.
- `commonMain` never imports Android or Apple platform APIs.
- Platform implementations are thin adapters behind common interfaces.

## Completion gate

The task-relevant tests, the Android compile and package gate, the localization
check, documentation checks and `git diff --check`. Compile is not done: runtime
behaviour is accepted with [ANDROID_ACCEPTANCE_TEST.md](acceptance.md).

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Kotlin Multiplatform client](README.md)
- Documentation home: [Product Factory documentation](../../../README.md)
- Agent guide: [AGENTS.md](../../../../AGENTS.md)
<!-- documentation-routing:end -->
