# Spring code samples

**Status:** Active
**Last updated:** 2026-10-20

Find the right template before writing code. Copy structure and naming, not
behaviour.

## Canonical living sample

Name one feature in this repo as the living template and keep it exemplary.
Prefer copying from it over inventing a layout.

| Concern | Path in the canonical feature |
|---|---|
| Mandatory DI | `<feature>/di/<Feature>Configuration.kt` |
| Use case contract and Impl | `<feature>/domain/usecase/<Name>UseCase/<Name>UseCase.kt`, `<Name>UseCaseImpl.kt` |
| Command and Result | `<feature>/domain/usecase/<Name>UseCase/<Name>Command.kt`, `<Name>Result.kt` |
| Persistence repositories | `<feature>/domain/repository/` |
| Outbound ports | `<feature>/domain/port/` |
| JPA sources | `<feature>/data/source/` |
| Repository implementations | `<feature>/data/repository/` |
| Extension mappers | `<feature>/data/mapper/`, `<feature>/presentation/mapper/` |
| Controllers and DTOs | `<feature>/presentation/controller/`, `<feature>/presentation/dto/` |

## By task

| I need to… | Look at |
|---|---|
| Create and wire a feature module | The canonical feature, [00](principles.md) §2–3, [03](layers.md) |
| Add a use case with Command/Result | The templates below, [02](../../language.md) |
| Add a persistence repository | [04](persistence.md) |
| Add a non-persistence outbound port | [03](layers.md) (`domain/port` + `data/adapter`) |
| Add a JPA entity and `*JpaSource` | [04](persistence.md) |
| Add a Flyway migration | The owning `application/<service>/…/db/migration/` and `docs/data/` |
| Publish or consume an event | [05](events-outbox-inbox.md) and the event catalog |
| Make an authorization decision | [06](security.md) |
| Protect a restricted field | [10](cryptography-by-layer.md) |

## By file type

### `*UseCase.kt` and `*UseCaseImpl.kt`

```kotlin
interface SendMessageUseCase {
    fun execute(command: SendMessageCommand): SendMessageResult
}

internal class SendMessageUseCaseImpl(
    private val messageRepository: MessageRepository,
    private val clock: Clock,
) : SendMessageUseCase {
    override fun execute(command: SendMessageCommand): SendMessageResult {
        // domain rules only, no *JpaSource
        …
    }
}
```

### `*Command.kt` and `*Result.kt`

```kotlin
data class SendMessageCommand(
    val tenantId: UUID,
    val authorId: UUID,
    val body: String,
    val idempotencyKey: String,
)

sealed interface SendMessageResult {
    data class Success(val message: Message) : SendMessageResult
    data class Rejected(val code: String) : SendMessageResult
}
```

### `*JpaSource.kt`

```kotlin
interface MessageJpaSource : JpaRepository<MessageEntity, UUID> {
    fun findByTenantIdAndId(
        tenantId: UUID,
        id: UUID,
    ): MessageEntity?
}
```

### `*Repository` and `*RepositoryImpl`

```kotlin
interface MessageRepository {
    fun findById(tenantId: UUID, id: UUID): Message?
    fun save(message: Message): Message
}

internal class MessageRepositoryImpl(
    private val messageJpaSource: MessageJpaSource,
) : MessageRepository {
    override fun findById(tenantId: UUID, id: UUID): Message? =
        messageJpaSource.findByTenantIdAndId(tenantId, id)?.toModel()

    override fun save(message: Message): Message =
        messageJpaSource.save(message.toEntity()).toModel()
}
```

### Extension mappers

```kotlin
fun MessageEntity.toModel(): Message = Message(
    id = id,
    tenantId = tenantId,
    body = body,
    createdAt = createdAt,
)

fun CreateMessageRequestDto.toCommand(
    tenantId: UUID,
    authorId: UUID,
    idempotencyKey: String,
): SendMessageCommand = SendMessageCommand(
    tenantId = tenantId,
    authorId = authorId,
    body = body,
    idempotencyKey = idempotencyKey,
)
```

### Feature `di/*Configuration.kt`

```kotlin
@Configuration
class MessagingConfiguration {
    @Bean
    fun clock(): Clock = Clock.System

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

### Thin controller

```kotlin
@RestController
@RequestMapping("/api/v1")
class MessageController(
    private val sendMessageUseCase: SendMessageUseCase,
) {
    @PostMapping("/messages")
    fun create(
        @RequestBody body: CreateMessageRequestDto,
        @RequestHeader("Idempotency-Key") idempotencyKey: String,
        principal: AuthenticatedPrincipal,
    ): MessageResponseDto {
        val result = sendMessageUseCase.execute(
            body.toCommand(
                tenantId = principal.tenantId,
                authorId = principal.memberId,
                idempotencyKey = idempotencyKey,
            ),
        )
        return when (result) {
            is SendMessageResult.Success -> result.message.toResponseDto()
            is SendMessageResult.Rejected -> throw ProblemException(result.code)
        }
    }
}
```

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Spring Boot services](README.md)
- Documentation home: [Product Factory documentation](../../../../README.md)
- Agent guide: [AGENTS.md](../../../../../AGENTS.md)
<!-- documentation-routing:end -->
