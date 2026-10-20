# Agent instructions in a repository

**Status:** Active
**Last updated:** 2026-10-20

## One source, many tools

- `AGENTS.md` at the repo root is the single instruction file: scope, setup, dev
  server, gates, conventions, the docs map, and what never to do. Agents of every
  tool read it.
- Claude Code reads `AGENTS.md` only when there is no `CLAUDE.md`, so `CLAUDE.md`
  contains `@AGENTS.md` (an import), plus nothing that contradicts it.
  Machine-local preferences go in the gitignored `CLAUDE.local.md`.
- Tool-specific rule folders (`.claude/rules/*.md`, `.cursor/rules/*.mdc`,
  `.codex/rules/*.md`) hold the same rules in each tool's format and are kept
  equivalent. A docs-first router rule points every tool at the link map.
- Path-scoped rules (`paths:` frontmatter in `.claude/rules/`) load only when the
  matching files are touched; use them for module-specific conventions in a
  workspace.

## Skills and subagents (Claude Code)

- Skills are searched upward only to the repository root; a nested child repository
  needs its own copy, installed per module.
- A skill's `description` and `when_to_use` decide when it triggers; describe the
  task, not the tool.
- Subagents get their own context and tool list; they do not see the session's auto
  memory, so everything they need must be in files.

## Hooks

- A `PreToolUse` hook on the shell tool can block commands reserved for humans
  (merge, tag, push to protected branches, deploy, bypass hooks). It must fail
  closed: when the guard binary is missing, block and print the install command.
- Hooks are configured in project `.claude/settings.json`; merge entries, never
  overwrite the file.

## Attribution

When a product forbids naming AI tools in commits or merge requests, configure the
tool (Claude Code `attribution` setting with empty commit and PR text) instead of
relying on agents remembering.

## Read live, never copy

Agent configuration routes to the live `AGENTS.md`, rules and docs of each repo. It
never copies them into skills or memory, where they would drift.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Agentic workflows](README.md)
- Documentation home: [Product Factory documentation](../../README.md)
- Agent guide: [AGENTS.md](../../../AGENTS.md)
<!-- documentation-routing:end -->
