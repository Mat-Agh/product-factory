# Operator

**Status:** Active
**Last updated:** 2026-10-19

Account-independent memory about the person who runs the factory.
[USER.md](USER.md) holds identity, voice, what "done" means, what never to do,
and the projects. `scripts/install.sh user` imports it into
`~/.claude/CLAUDE.md`, so every Claude Code session loads it, and other agents
can reference it directly.

Keep it here, in git. Vendor memory is a cache. To recover what an assistant
learned on one machine, run `scripts/export-claude.sh` (it writes to the
gitignored `.claude-export/`), then fold the durable facts into `USER.md`.

Contract: [COMP-010](../components/COMP-010-operator-profile.md).

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
- In this folder:
  - [Operator profile](USER.md)
<!-- documentation-routing:end -->
