# 11 — Build system and Gradle

**Status:** Active
**Last updated:** {{DATE}}
**Source:** Product Factory knowledge pack (kmp-android-client)

## Sources of truth

| Concern | File or directory |
|---|---|
| Module inventory | `settings.gradle.kts` |
| Versions and aliases | `gradle/libs.versions.toml` |
| Convention plugins | `build-logic/convention/` |
| Wrapper version | `gradle/wrapper/gradle-wrapper.properties` |
| Repositories | `settings.gradle.kts`, `build-logic/settings.gradle.kts` |
| Platform activation | `gradle.properties` |
| Local service and signing values | ignored `secrets.properties`, with `secrets.defaults.properties` as the key template |

Set `repositoriesMode = FAIL_ON_PROJECT_REPOS` so modules cannot add their own
repositories. Record the mirror or upstreams used here.

Every application, core and feature module applies its owning convention plugin.
Do not duplicate compiler, SDK, flavor, signing, localization or common dependency
policy in module build files.

## Targets and flavors

- Android is always configured for KMP modules; iOS targets are conditional.
- The `environment` flavor dimension provides the backend environments.
- Release signing and production values come from local files or CI secrets, never
  from git.
- Keep `compileSdk` and `targetSdk` matched to the CI image; record why when they lag.

## Common commands

```bash
./gradlew projects
./gradlew :androidApp:assembleProdDebug --no-daemon
./gradlew verifyLocalization --no-daemon
```
