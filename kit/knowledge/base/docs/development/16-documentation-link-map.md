# 16 — Documentation link map

**Status:** Active — agent routing authority
**Last updated:** {{DATE}}
**Source:** Product Factory knowledge pack (generic)

Route each kind of task to the smallest set of docs. Update this map in the same
change that adds, moves or retires a doc.

## Knowledge order (mandatory)

1. `AGENTS.md`, this map, [../README.md](../README.md).
2. The routed docs below.
3. Sibling repos through [../cross-project/README.md](../cross-project/README.md)
   when they own the decision.
4. Authoritative online sources, only for what is still missing.
5. Durable findings go back into the docs, never only into an assistant's memory.

## Router

| Task | Read first | Then, if needed |
|---|---|---|
| Any change | `AGENTS.md`, this file | The matching row |
| What the repo is now | `CURRENT_STATE.md` | [../features/IMPLEMENTATION_STATUS.md](../features/IMPLEMENTATION_STATUS.md) |
| Principles | [00](00-engineering-principles.md) | [../architecture/CONFORMANCE_LEDGER.md](../architecture/CONFORMANCE_LEDGER.md) |
| Commands | [09](09-tools-scripts-and-commands.md) | [CICD](CICD.md) |
| Patterns | [15](15-code-samples-index.md) | The canonical module |
| Architecture | [../architecture/README.md](../architecture/README.md) | [../architecture/OVERVIEW.md](../architecture/OVERVIEW.md) |
| A decision | [../decisions/README.md](../decisions/README.md) | The ADR |
| Feature behaviour | [../features/README.md](../features/README.md) | Implementation status |
| History | [../history/DEVELOPMENT_LOG.md](../history/DEVELOPMENT_LOG.md) | `git log` |
| Branching and review | `CONTRIBUTING.md` | [CICD](CICD.md) |
