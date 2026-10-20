# iOS

**Status:** Active — gathered knowledge is limited to an optional iOS target
**Last updated:** 2026-10-20

What has been learned so far comes from a KMP client where Android is the active
target and iOS is compile-safe but not released. Extend this page when an iOS
release happens.

## Shape

- `iosApp/` is an Xcode shell that hosts the shared Compose UI
  (`MainViewController` hosting the root screen).
- iOS targets (`iosArm64`, `iosSimulatorArm64`) are enabled by a Gradle property so
  Android-only developers and CI do not need macOS.
- A shared-module convention plugin exports the framework Xcode consumes.

## Source-set rules

- `iosMain` holds Darwin networking (the Ktor Darwin engine), Keychain-backed
  secure storage, and platform UI bridges.
- Swift stays limited to app entry, lifecycle and platform integration; business
  logic and reusable UI stay in shared Kotlin.
- Prefer a multiplatform library over a new `expect`/`actual` pair.

## Workflow

1. Enable the iOS targets in local Gradle properties.
2. On macOS, open `iosApp/` in Xcode, or build the shared framework with Gradle.
3. Run Android verification after any shared-code change, and iOS verification
   when iOS sources change.

## Not yet gathered

Background push and incoming-call parity on iOS, App Store signing and release,
and iOS acceptance. Do not claim them for a product until they are built and
verified; record the knowledge here when they are.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Kotlin Multiplatform client](README.md)
- Documentation home: [Product Factory documentation](../../../README.md)
- Agent guide: [AGENTS.md](../../../../AGENTS.md)
<!-- documentation-routing:end -->
