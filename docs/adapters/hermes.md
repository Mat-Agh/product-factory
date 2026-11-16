# Hermes adapter

**Status:** Active
**Last updated:** 2026-11-10

Open the installed checkout in Hermes and run `hermes skills trust` for the
project. Start a new session and load the `product-factory` skill. The project
context points Hermes to the same coordinator, constitution and roles as the
other runtimes. The session accepts work, claims jobs and monitors their
artifacts through `pf job status` or `pf dashboard`.
[Hermes project skills](https://hermes-agent.nousresearch.com/docs/user-guide/features/skills)

For the production install, 9Router routes and optional Honcho shared context,
follow [guide 08](../guide/08-hermes-and-honcho.md). The Factory installer
collects non-secret route names with `--configure-models`; Hermes owns
credentials and Honcho because they are machine-level services, never project
repository configuration.

Native roles can use Hermes’s `delegate_task` with the installed role file and
Factory handoff. For cross-provider workers, configure model routes for the workspace.
`pf providers run` starts a Hermes worker with the assigned role's canonical
instructions, worktree and artifact path. Use a direct provider key or a
9Router-compatible endpoint. [Model routing](../guide/01-getting-started.md#model-routing)

Hermes loads `SOUL.md` from its own instance or profile. Factory's
project coordinator instructions and per-worker role prompts are scoped to the
checkout, so opening another project uses that project's rules.
[Hermes context files](https://hermes-agent.nousresearch.com/docs/user-guide/features/context-files)

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Adapters](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
