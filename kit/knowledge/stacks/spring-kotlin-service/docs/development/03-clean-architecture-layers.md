# 03 — Clean architecture layers

**Status:** Active
**Last updated:** {{DATE}}
**Source:** Product Factory knowledge pack (spring-kotlin-service)

Feature modules use four packages: `domain`, `data`, `presentation`, and the
mandatory `di`.

## Layout

```
┌─────────────────────────────────────────────┐
│  presentation/                              │
│  Controllers + DTOs + extension mappers     │
├─────────────────────────────────────────────┤
│  domain/                                    │
│  Models + repositories + outbound ports     │
├─────────────────────────────────────────────┤
│  data/                                      │
│  Entities + *JpaSource + repository impls   │
└─────────────────────────────────────────────┘
         ↑
    di/ wires beans (@Configuration / @Bean)
```

| Layer | Folder | Owns |
|---|---|---|
| Domain | `model/` | Immutable business types |
| Domain | `repository/` | Persistence interfaces used by use cases |
| Domain | `port/` | Non-persistence outbound capabilities (storage, delivery, provider, authorization) |
| Domain | `usecase/` | `usecase/<InterfaceName>/{*UseCase,*UseCaseImpl,*Command,*Result}` |
| Domain | `policy/`, `state/` | Optional pure rules and state machines |
| Data | `entity/` | `*Entity` JPA mappings |
| Data | `source/` | `*JpaSource` Spring Data interfaces |
| Data | `repository/` | `*RepositoryImpl` implementing domain interfaces |
| Data | `mapper/` | `toModel()` / `toEntity()` |
| Data | `adapter/` | Port implementations; internal HTTP and external clients |
| Data | `messaging/` | Broker consumers, schedulers |
| Presentation | `controller/` | HTTP/WS entry points, translation only |
| Presentation | `dto/` | Request and response wire types |
| Presentation | `mapper/` | `toCommand()` / `to*ResponseDto()` |

## Dependency direction

`presentation` and `data` depend on `domain`. Domain depends on nothing in `data`
or `presentation`. Controllers never import entities or `*JpaSource`. Use cases
orchestrate ports; they never open Hibernate sessions or call broker clients
directly.

## Ports and repositories

- `domain/repository/` is only for persistence interfaces.
- `domain/port/` holds every other outbound capability.
- Both are implemented in `data/` and wired in `di/`.

## DI

```kotlin
@Configuration
class MessagingConfiguration {
    @Bean
    fun sendMessageUseCase(
        messageRepository: MessageRepository,
        clock: Clock,
    ): SendMessageUseCase = SendMessageUseCaseImpl(
        messageRepository = messageRepository,
        clock = clock,
    )
}
```

Prefer constructor injection of interfaces. Wire `Clock.System` explicitly. When
an application must supply feature-specific configuration, it provides a domain
configuration model or port and lets the feature DI construct the implementation.

## Application module role

`application/<service>/` hosts the Boot main class, service-level security and
filter configuration that is not feature-local, the Flyway migrations for that
service's database, and thin edge controllers (BFF, provisioning). It composes
features; it does not own deep business logic.

## Core module layout

```
core/<module>/
  domain/   # interfaces, models, exceptions consumers depend on
  data/     # *Impl, entity/, source/, adapters, filters/handlers
  di/       # @Configuration / @Bean only
```

`core/contracts` may stay flat when it holds only stable wire abstractions.
