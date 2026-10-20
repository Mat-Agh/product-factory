# Client development setup

**Status:** Active
**Last updated:** 2026-10-20

## Requirements

- A JDK compatible with the checked-in Gradle wrapper
- Android Studio and the Android SDK matching `gradle/libs.versions.toml`
- Git
- An emulator or a device
- macOS with Xcode for iOS work only

## Linux and macOS

```bash
cd <repo>
cp secrets.defaults.properties secrets.properties
./gradlew projects
./gradlew :androidApp:assembleBetaDebug --no-daemon
```

## Windows

```powershell
Copy-Item secrets.defaults.properties secrets.properties
.\gradlew.bat projects
.\gradlew.bat :androidApp:assembleBetaDebug --no-daemon
```

Set `ANDROID_HOME`, or put `sdk.dir` in an ignored `local.properties`. Fill only
the required keys in `secrets.properties`. Never commit `local.properties`,
`secrets.properties`, keystores, SDK paths or build output.

Install the backend's development CA on the emulator or device before signing in
([07](android.md)).

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Kotlin Multiplatform client](README.md)
- Documentation home: [Product Factory documentation](../../../README.md)
- Agent guide: [AGENTS.md](../../../../AGENTS.md)
<!-- documentation-routing:end -->
