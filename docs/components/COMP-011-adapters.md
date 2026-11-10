# COMP-011 — Adapters

**Status:** Project runtime kits and provider routing implemented
**Last updated:** 2026-11-10
**Source:** [`docs/adapters/hermes.md`](../adapters/hermes.md), [`docs/adapters/other-agents.md`](../adapters/other-agents.md)
**Decisions:** [ADR-001](../decisions/ADR-001-claude-code-primary-runtime.md)

## Purpose

Adapters keep the factory usable without Claude Code. Role files are plain
markdown, so any capable agent can run one role at a time against the same job
folder and verdict contract.

| Adapter | Covers |
|---|---|
| Hermes | Shared coordinator skill, project context and independent workers with canonical role prompts |
| Codex | Shared coordinator skill and generated TOML roles with native model and effort overrides |
| Antigravity | Shared coordinator skill and generated Markdown subagents with Pro and Flash tiers |
| Gemini CLI, Cursor and other compatible hosts | Shared entry point and CLI worker dispatch |

## Known limitations

- Native project roles require a host that supports them and a trusted project where applicable. Routed worker models require an account with access and tool support.
- The guard hook works only in Claude Code. Elsewhere, branch protection and the
  agent's own approval mode are the controls.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Components](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
