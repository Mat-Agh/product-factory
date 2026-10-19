# Operator profile

**Status:** Active
**Last updated:** 2026-10-19

This file is account-independent memory about the person who runs the factory.
`scripts/install.sh user` imports it into `~/.claude/CLAUDE.md` with an `@`
line. Other agents read it from `AGENTS.md`. Edit it here, in git, and never
only in a vendor's memory.

## Who

Matin Aghalarkhani (GitHub `Mat-Agh`). Operates the Product Factory and the
products it builds. UTC+3:30. Works with agents in English. Product UIs are
often EN and FA (RTL), with dark and light themes paired.

## How to work with me

- Lead with the answer, then the supporting fact. Complete sentences, concise,
  not clipped. No filler, no restating the ask, no metaphors or catchy labels.
  Do not replay the process I can already see.
- Push back when something is wrong, and say so when unsure. Agree because it is
  right, not because I said it.
- Do the work. Do not dump instructions on me.
- Read the live `AGENTS.md`, IDE rules and docs of the repo. Never copy them into
  agent memory or skills.

## Languages for tools

- **Rust by default.** Any script, CLI, hook, generator or helper program an
  agent writes and keeps is written in Rust (a cargo crate, or a subcommand of an
  existing Rust CLI), unless the operator says otherwise.
- Exceptions: product code follows the product repo's own stack and conventions
  (for example Kotlin in Daria Connect), and short throwaway shell one-liners run
  only to inspect something may stay in shell.
- When existing tooling is in another language, propose porting it to Rust
  rather than extending it.

## What done means

- The module's quality gates pass, and documented acceptance evidence exists.
  Compile is not done. A development Compose stack is not the production
  installer.
- Humans merge. Agents open the PR or MR and stop.

## What I don't want

- Duplicated docs. Link, do not copy.
- Invented product scope, invented credentials, invented commands.
- Unsolicited commits. Commit only when asked. No `--no-verify`. No force-push
  to protected branches.
- Secrets in git, memory or logs.
- Stashing, resetting or discarding work the agent did not create.
- Unit tests in modules whose conventions forbid them (for example Daria
  Connect Back-End and Hub).

## Projects

| Project | Where | Notes |
|---|---|---|
| Daria Connect | `$HOME/Work/Connect` (GitLab children, GitHub workspace) | Multi-repo: Product, Front-End, Back-End, Client (Android, never called "Frontend"), Hub (vendor plane, never shipped to customers). |
| Hermes harness | `$HOME/Work/Hermes` (`Mat-Agh/Hermes`) | The earlier factory implementation on Hermes Agent and 9Router. See `docs/adapters/hermes.md`. |
| Product Factory | `$HOME/Work/ProductFactory` (`Mat-Agh/product-factory`) | This repo. |

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Operator](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
