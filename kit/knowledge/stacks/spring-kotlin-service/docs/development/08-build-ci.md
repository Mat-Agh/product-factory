# 08 — Build and CI

**Status:** Active
**Last updated:** {{DATE}}
**Source:** Product Factory knowledge pack (spring-kotlin-service)

## Reproducible baseline

Record the pinned baseline here and change it only in a dedicated change:

| Tool | Version | Pinned in |
|---|---|---|
| Spring Boot | `<x.y.z>` | `gradle/libs.versions.toml` |
| Kotlin | `<x.y.z>` | `gradle/libs.versions.toml` |
| JDK / GraalVM | `<n>` | toolchain in build logic |
| Gradle | `<x.y.z>` | `gradle/wrapper/gradle-wrapper.properties` |

- The Spring Boot BOM owns Spring, Jackson, Hibernate, Flyway, JDBC, broker client
  and Micrometer versions. Consume it as a native Gradle `platform(...)` in a
  convention plugin rather than the `io.spring.dependency-management` plugin, which
  pins the BOM's Kotlin version onto every configuration, including compiler and
  lint worker classpaths.
- When a lint or analysis tool lags the Kotlin version, hold it at the last
  compatible version and record why in this table.
- Supply-chain controls: wrapper checksum, pinned CI action and image digests,
  SBOM, provenance and signatures.

## Convention-plugin build

- `build-logic/` is an included build. Its plugins own the JVM target, compiler
  flags, explicit API, ktlint, detekt, the BOM platform, JPA/Spring/Boot setup,
  module dependencies and aggregate tasks.
- Project `build.gradle.kts` files contain only plugin declarations, so the
  dependency graph is reviewable in one place.
- `gradle/libs.versions.toml` owns external coordinates and plugin IDs for the main
  and included builds.
- An architecture verification task checks plugin-only build scripts, feature
  ownership, forbidden dependencies and layer boundaries.

## Build type and flavor

Keep build type (`development` / `production`) and deployment flavor (for example
`beta` / `release`) independent, and record both in JAR manifests and image labels.
Flavor-specific project names, secret directories and volumes prevent one
environment from reusing another's data.

## Commands

```bash
./gradlew verifyArchitecture classes ktlintCheck detekt --no-configuration-cache
./gradlew :application:<service>:bootJar
git diff --check
```

## CI gates

| Trigger | Gates |
|---|---|
| Merge request | Branch and commit conventions, architecture and docs checks, compile, ktlint, detekt, tests per policy |
| Integration branch | The above, then build, scan and sign service images |
| Release tag | Release build (native images where used), signed artifacts, release notes |

Pipeline detail lives in [CICD.md](CICD.md). Behaviour changes update OpenAPI,
AsyncAPI, Flyway, generated schema docs, the threat model and the module README
together.
