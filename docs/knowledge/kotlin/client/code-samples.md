# Client code samples

**Status:** Active
**Last updated:** 2026-10-20

Name one feature as the canonical MVI sample and keep it exemplary. Copy structure
and naming, not behaviour.

**Canonical feature:** `:feature:<name>` — base package
`feature/<name>/src/commonMain/kotlin/<package>/feature/<name>/`

## Page set

```text
presentation/component/page/<page>/
  screen/<Page>Screen.kt
  screen/<Page>ScreenViewModel.kt
  state/<Page>ScreenState.kt
  action/<Page>ScreenAction.kt
  action/<Page>ScreenActionHandler.kt
  event/<Page>ScreenEvent.kt
  event/<Page>ScreenEventHandler.kt
```

## Screen

```kotlin
val screenState by viewModel.screenState.collectAsStateWithLifecycle()
LoginScreenActionHandler(...) { onAction ->
    LoginScreenEventHandler(
        screenEvent = viewModel.screenEvent,
        onClearEvent = viewModel::onClearEvents,
    ) {
        LoginScreenContent(state = screenState, onAction = onAction)
    }
}
```

## Use case

```kotlin
class LoginUseCase(
    private val repository: AuthenticationRepository,
) {
    suspend operator fun invoke(
        email: String,
        password: String,
    ): Result<AuthSession> = repository.login(email, password)
}
```

Use cases depend on domain repository interfaces only.

## Repository

| Layer | File |
|---|---|
| Interface | `domain/repository/AuthenticationRepository.kt` |
| Implementation | `data/repository/AuthenticationRepositoryImpl.kt` |
| Remote source | `data/source/authentication_remote_source/` |

## Koin module

```kotlin
val authenticationModule = module {
    single<AuthenticationRepository> { AuthenticationRepositoryImpl(get(), get()) }
    factory { LoginUseCase(get()) }
    viewModelOf(::LoginScreenViewModel)
}
```

`:shared` includes the module at startup. Feature modules never construct an
`HttpClient` or vendor SDK.

## Where to look

| I need to… | Look at |
|---|---|
| Add a screen | The page set above, [03](presentation-mvi.md) |
| Call the backend | [04](data-and-realtime.md), the network facade |
| Add a dependency | [05](library-stack.md), [11](../gradle.md) |
| Add a platform capability | [01](kmp-source-sets.md), [07](android.md) |
| Store something sensitive | [12](security-and-cryptography.md) |

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Kotlin Multiplatform client](README.md)
- Documentation home: [Product Factory documentation](../../../README.md)
- Agent guide: [AGENTS.md](../../../../AGENTS.md)
<!-- documentation-routing:end -->
