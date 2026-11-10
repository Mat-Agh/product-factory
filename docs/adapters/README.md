# Adapters

**Status:** Documented — procedures only, not exercised
**Last updated:** 2026-10-19

How to run the factory's roles without Claude Code, or alongside another agent
runtime. The role files in `kit/.claude/agents/` are plain markdown, so any
capable agent can follow one role at a time against the same job folder and
verdict contract.

| Adapter | Use it for |
|---|---|
| [hermes.md](hermes.md) | Hermes Agent: mapping to Hermes profiles and skills, running a factory role inside Hermes, and the convergence plan |
| [other-agents.md](other-agents.md) | Codex, Cursor, Gemini CLI: manual coordination, Codex as a cross-vendor review lens, guard behaviour outside Claude Code |

Contract: [COMP-011](../components/COMP-011-adapters.md).

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
- In this folder:
  - [Hermes adapter](hermes.md)
  - [Codex, Antigravity, Cursor, Gemini CLI and other agents](other-agents.md)
<!-- documentation-routing:end -->
