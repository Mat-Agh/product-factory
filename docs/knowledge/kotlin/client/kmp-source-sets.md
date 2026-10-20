# KMP module structure and source sets

**Status:** Active
**Last updated:** 2026-10-20

## Targets

| Target | Source sets | Activation |
|---|---|---|
| Android | `commonMain`, `androidMain` | Always configured |
| iOS device and simulator | `commonMain`, `iosMain` | A Gradle property such as `<prefix>.includeIos=true` |

A KMP convention plugin configures Android for every module and adds the iOS
targets conditionally. A shared-module plugin exports the framework that the
Xcode shell consumes.

## Layout

```text
androidApp/         Android application shell
iosApp/             Xcode shell
shared/             shared application composition (DI start, root navigation, auth state)
core/*              reusable infrastructure (network, datastore, security, design system, …)
feature/*           vertical product slices
build-logic/        convention plugins
```

## Source-set rules

- Domain logic, repositories, MVI contracts and Compose UI go in `commonMain` when
  they are platform-neutral.
- Android permissions, services, media, pickers, notifications and WebRTC adapters
  go in `androidMain`.
- Darwin networking, Keychain and iOS UI bridges go in `iosMain`.
- Prefer a multiplatform dependency over a new `expect`/`actual` pair.
- Map platform values to common domain types at the boundary.

## iOS

See [ios.md](ios.md).

## iOS workflow (summary)

1. Enable the iOS targets in local Gradle properties.
2. On macOS, build the shared framework or let Xcode invoke Gradle from `iosApp/`.
3. Keep Swift limited to app entry, lifecycle and platform UI integration.
4. Run Android verification after shared-code changes, and iOS verification when
   iOS sources change.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Kotlin Multiplatform client](README.md)
- Documentation home: [Product Factory documentation](../../../README.md)
- Agent guide: [AGENTS.md](../../../../AGENTS.md)
<!-- documentation-routing:end -->
