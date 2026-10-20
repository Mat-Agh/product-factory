# Client library stack

**Status:** Active
**Last updated:** 2026-10-20
**Authority:** `gradle/libs.versions.toml`

Add dependencies through the version catalog. Multiplatform dependencies go in
`commonMain`, Android-only in `androidMain`, Apple-only in `iosMain`. Dependencies
shared by several modules belong in a convention plugin. Features depend on core
facades, never vendor SDKs. For each library record **why**, the **owning facade**,
what **features must not import**, and a **sample** path.

| Library | Why | Owning facade | Features must not import |
|---|---|---|---|
| Compose Multiplatform + Material 3 | Shared UI with design-system tokens | `:core:design_system` | A second UI stack (Android-only Compose aliases) |
| Koin | Explicit composition root without annotation processing | `:shared` + feature `di/` | Hilt/Dagger; Koin Android APIs in `commonMain` |
| Navigation (typed keys) | Typed per-feature stacks composed by the shell | `:core:design_system` + `:shared` | Another feature's host |
| Ktor Client | Multiplatform HTTP and WebSocket with request policy | `:core:network`, `:core:realtime` | `io.ktor.client.*` engines or `HttpClient` |
| OkHttp (Android engine) | Interceptors, TLS behaviour | `:core:network` `androidMain` | `okhttp3.*` |
| Darwin engine (iOS) | Ktor on iOS | `:core:network` `iosMain` | NSURLSession types in `commonMain` |
| kotlinx.serialization | DTO JSON | Convention plugin; DTOs in `data/dto/` | Gson, Moshi, org.json |
| kotlinx.coroutines | Structured concurrency, flows | All modules | RxJava, `GlobalScope` |
| DataStore / multiplatform settings | Encrypted small state | `:core:datastore` + `:core:security` | Datastore APIs directly |
| Coil 3 | Multiplatform image loading | `:core:image` | `coil3.*`, Glide, Picasso |
| Napier / Timber | Logging with redaction | `:core:logging` | Logger APIs directly |
| Crash reporting / analytics | Optional, consent-gated observability | `:core:crash_reporting`, `:core:analytics` | Vendor SDKs directly |
| Debug HTTP inspector | Debug-only network inspection (no-op in release) | `:core:network` | The inspector API |
| JOSE library | Compact JWE/JWS and proof JWTs | `:core:network` | JOSE APIs directly |
| WebRTC | Realtime media | `:core:webrtc` | `org.webrtc.*` |

A catalog entry with no product use is marked "reserved, do not import" and stays
unused until a product contract needs it.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Kotlin Multiplatform client](README.md)
- Documentation home: [Product Factory documentation](../../../README.md)
- Agent guide: [AGENTS.md](../../../../AGENTS.md)
<!-- documentation-routing:end -->
