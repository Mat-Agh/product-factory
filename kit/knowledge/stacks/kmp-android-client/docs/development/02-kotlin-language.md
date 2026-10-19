# 02 — Kotlin language conventions

**Status:** Active
**Last updated:** {{DATE}}
**Source:** Product Factory knowledge pack (kmp-android-client)

## Naming

| Suffix | Meaning |
|---|---|
| Plain name | Domain model |
| `*Dto` | Wire shape (`@Serializable`) in `data/dto/` |
| `*Data` | Persisted local data |
| `*Repository` / `*RepositoryImpl` | Domain interface / data implementation |
| `*RemoteSource` / `*LocalSource` | Data-layer sources |
| `*UseCase` | One use case, `operator fun invoke` |
| `*Screen`, `*ScreenViewModel`, `*ScreenState`, `*ScreenAction`, `*ScreenEvent` | One MVI page |

## Mappers

Extension functions only: `toModel()`, `toDto()`, `toData()`. Mappers live in the
data layer; presentation maps domain models to UI state in the ViewModel.

## Coroutines

- `viewModelScope` in ViewModels; inject dispatchers at repository boundaries.
- `StateFlow` for state, `SharedFlow` (or a channel) for one-off events.
- Cancel and clean up in the owning scope; never `GlobalScope`.
- Blocking I/O never runs on the main thread.

## Style

- Two or more parameters or call arguments: one per line.
- No `!!` in production code.
- `internal` by default for feature types; only contracts are public.
- One public top-level type per file.
