# Codex, Antigravity, Cursor, Gemini CLI and other agents

**Status:** Active
**Last updated:** 2026-11-10

The role files are plain markdown. The Claude Code frontmatter (`name`, `tools`,
`model`) is metadata that other agents can ignore. Any agent that reads
`AGENTS.md` sees the Product Factory section the installer appended, and that
section points it to the roles.

## Codex

Run `pf install project <repo>` to include independent workspace checkouts. In a new Codex session with the project configuration trusted,
invoke `$product-factory init` to review the draft, or
`$product-factory <request>` to coordinate work. The installed skill reads the
shared coordinator and dispatches the `factory-<role>` custom agents under
`.codex/agents/`. [Codex subagents](https://learn.chatgpt.com/docs/agent-configuration/subagents)

Deep roles request high reasoning effort and build roles request medium. Both
inherit the session model by default. Set `models.codex_deep` and
`models.codex_build` to model IDs available on your configured provider,
then reinstall the kit. The selected models must support the role's effort
level and the runtime's tools.

The coordinator needs an approved path for Git metadata writes during
`pf job prepare` and commits. The job files live in the main checkout and
the job worktree lives in a sibling directory. In a restricted session,
use the runtime's approval prompts for the fixture or product's worktree
and Git commands. If the runtime cannot request approval, run the reported
`pf job prepare` command in the workspace terminal and resume the recorded
job. Build and gate commands still need the relevant worktree permissions.

Use a model available on the signed-in account. Codex's model picker or
`codex debug models` can help diagnose a stale default. The Factory descriptor
controls optional role overrides, while the runtime selects the coordinator's
model.

## Antigravity

The installed `AGENTS.md` and `.agents/skills/product-factory/SKILL.md` are
discovered at the workspace. Use the Factory skill for a change. The
coordinator can dispatch `factory-<role>` subagents generated under
`.agents/agents/`, with Pro for deep roles and Flash for build roles, or use
the configured cross-provider route.
[Antigravity custom subagents](https://antigravity.google/docs/subagents?tab=cli)

## Gemini CLI, Cursor and other compatible hosts

Load the `product-factory` skill or follow the instructions in `AGENTS.md`.
Use `pf job` for state and `pf providers run` for independent routed roles.
A host needs project filesystem access and a terminal that can invoke `pf`.
For a native role in a host without subagent dispatch, run the role from a
separate session using the coordinator's handoff, then validate the result
with `pf job advance`.

## Guard outside Claude Code

The hook works only in Claude Code. Elsewhere, branch protection on the remote
is the control, and the agent's own approval mode (for example Codex
`--ask-for-approval`) is the second line.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Adapters](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
