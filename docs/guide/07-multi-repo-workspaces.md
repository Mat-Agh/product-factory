# 07 — Install Product Factory in a multi-repo workspace

**Status:** Active
**Last updated:** 2026-11-10

A workspace keeps one descriptor, backlog and dashboard at its root. Each
independent Git checkout gets the runtime kit. Jobs use the owning Git history
and create their worktrees beside the workspace. Modules that share a Git
history use the root kit and keep their changes inside the module's scope.

## Audit and install

Run these commands from the workspace root. Check out the submodules used by
your workspace before running the installer.

```bash
pf audit . --modules
pf knowledge plan --modules
pf install project .
```

Review the detected module paths, stacks and documentation roots in
`.factory/factory.yaml`. Review the existing rules, VCS conventions and quality
gates with `/factory adopt` in Claude Code or `$product-factory adopt` in Codex.
Use the [sample workspace descriptor](examples/workspace.factory.yaml) as a
reference for module overrides. The example uses synthetic paths and settings.

Once reviewed, install into the independent checkouts and generate module rules.

```bash
pf install project .
pf rules sync
pf doctor .
```

Use `pf install module <path> --workspace .` to update one independent checkout.
Use `pf knowledge plan --modules` to preview each handbook. Review and apply
imports on the owning repository's docs branch. Follow its existing conventions
for documentation links.

## Run and monitor work

Start an agent session in the workspace or an installed module. Include the
module, desired outcome and acceptance checks in the request. Independent jobs
can run in separate sessions under the workspace's active-job limit. Sequence
work that changes shared contracts or root files.

Run `pf dashboard` at the workspace root or in an installed module. The local
board shows queued work, job stages, module filters and review evidence. See
[Commands](02-commands.md) for issue intake, feedback and resuming a job.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Operator guide](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
