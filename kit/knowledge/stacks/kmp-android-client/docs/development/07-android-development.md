# 07 — Android development

**Status:** Active
**Last updated:** {{DATE}}
**Source:** Product Factory knowledge pack (kmp-android-client)

## App shell

`androidApp/` is a thin shell. It owns the `Application` class (DI start,
logging, consent-gated observability), the `MainActivity` (splash, edge-to-edge,
Compose host, deep-link and intent forwarding, `ActivityResult` binders), and
OS-registered services (push, foreground calls) that resolve their dependencies
from DI. Vendor SDKs and business logic live in `:core:*` and `:feature:*`.

## Platform modules

Record which core module owns each platform capability:

| Concern | Module |
|---|---|
| Media picking | `:core:media_picker` |
| Permissions | `:core:permissions` |
| Push | `:core:push` |
| Realtime media | `:core:webrtc` |
| Logging | `:core:logging` |
| Activity binding helpers | `:core:platform` |

## Build variants

An `environment` flavor dimension (for example `beta` and `prod`) maps to backend
environments. Base URLs come from ignored local properties, must be HTTPS, and
derive the WSS origin rather than configuring it separately.

```bash
./gradlew :androidApp:assembleBetaDebug --no-daemon
./gradlew :androidApp:assembleProdDebug --no-daemon
```

## Platform boundaries

- Android APIs stay in `androidApp` or `androidMain`.
- Common code requests platform work through interfaces injected by DI.
- File, media, voice and WebRTC implementations own permission and lifecycle
  cleanup.
- Emulator loopback maps to the host machine (`10.0.2.2`); physical devices never
  use that mapping.

## Local development hosts

| Device | Host | Notes |
|---|---|---|
| Emulator | `https://localhost` | Install the development CA; DNS maps to the host |
| Physical device (USB or Wi-Fi) | The beta HTTPS origin | The certificate SAN must match the exact host |

Cleartext is disabled in every variant. Release trusts system CAs (plus any
documented, bundled CA); debug additionally trusts user-installed development CAs
through `network_security_config.xml` `debug-overrides`. No permissive trust
manager or hostname verifier exists in any variant.

After the backend recreates its services, wait until its health endpoint answers
200 before judging the client offline.
