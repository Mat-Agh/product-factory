# Multi-repository workspaces

**Status:** Active
**Last updated:** 2026-10-20

A product made of independent repositories (product docs, web, backend, mobile,
vendor plane) under one workspace folder.

- The workspace repository pins the children (gitlinks or submodules) and holds no
  product code. Children publish to their own remotes first; the workspace pin
  moves afterwards, by a human.
- Each child keeps its own conventions (branch names, commit format, hooks, CI,
  testing policy). Agents read the child's `AGENTS.md` before editing it and obey
  it; the factory adapts its branch template and commit convention per child.
- One change lands in one child. A cross-cutting change becomes one job per child,
  linked through the product's cross-project docs.
- Agent tooling is installed in every child (skills, agents, rules, guard), because
  sessions opened inside a child only see that child's configuration.
- A workspace descriptor lists each module's path, stack, docs root, owner role and
  gates, and generates path-scoped rules per module.
- Ownership is explicit: each child has a technical owner, and contract changes
  that cross children name every affected owner.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Agentic workflows](README.md)
- Documentation home: [Product Factory documentation](../../README.md)
- Agent guide: [AGENTS.md](../../../AGENTS.md)
<!-- documentation-routing:end -->
