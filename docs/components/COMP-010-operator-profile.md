# COMP-010 — Operator profile

**Status:** Implemented
**Last updated:** 2026-10-19
**Source:** [`docs/operator/USER.md`](../operator/USER.md)

## Purpose

This is account-independent memory about the operator: identity, voice, what
"done" means, what not to do, and the projects. It replaces vendor memory as
the durable record. Vendor memory is a cache.

## Interface

- **Claude Code:** `pf install user` adds `@<factory>/docs/operator/USER.md` to
  `~/.claude/CLAUDE.md`, so it loads in every session.
- **Other agents:** reference it from their global instructions, or paste it in.
- **Sources:** distilled on 2026-10-19 from the Hermes `USER.md` and `SOUL.md`
  (`Mat-Agh/Hermes`, `config/hermes/memories/USER.md`).

## Rules

- Edit it here and commit it. Never record operator facts only in a vendor's
  memory.
- No secrets or private credentials. It is shared by every agent you use.
- Product-specific facts belong in that product's `AGENTS.md`, not here.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Components](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
