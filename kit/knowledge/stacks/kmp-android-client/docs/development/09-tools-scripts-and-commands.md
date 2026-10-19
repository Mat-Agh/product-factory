# 09 — Tools, scripts and commands

**Status:** Active
**Last updated:** {{DATE}}
**Source:** Product Factory knowledge pack (kmp-android-client)

Replace every `<…>` once the real command has run successfully.

## Prerequisites

| Tool | Role |
|---|---|
| JDK compatible with the Gradle wrapper | Builds |
| Android Studio and SDK (API per the catalog) | Android builds, emulator |
| Xcode (macOS only) | iOS shell |
| `git` | Branching per [CICD.md](CICD.md) |

Setup per OS: [DEV_SETUP.md](DEV_SETUP.md).

## Everyday loop

```bash
cp secrets.defaults.properties secrets.properties   # ignored; fill only required keys
./gradlew projects
./gradlew :androidApp:assembleBetaDebug --no-daemon
```

## Gates

```bash
./gradlew check verifyLocalization --no-daemon
./gradlew :androidApp:assembleBetaDebug :androidApp:assembleProdDebug --no-daemon
git diff --check
```

Discover module tasks with `./gradlew tasks --all` and run the smallest affected
set. Never invoke tasks for modules absent from `settings.gradle.kts`.

## Repo scripts

| Script | Purpose |
|---|---|
| `<scripts/verify-*.sh>` | `<static privacy or policy check>` |
