# 04 — Add a project

**Status:** Active
**Last updated:** 2026-11-10

0. **If the project already has a process** (agent rules, hooks, CI, other
   automation), audit it first: `pf audit <path>`, then `/factory adopt` after
   install. See [COMP-017](../components/COMP-017-adoption.md).
1. **Install the kit** into the product repo. A standalone `pf` binary
   carries the kit, and a Factory checkout lets you test kit changes before
   building a release:
   ```bash
   pf install project ~/Work/MyApp
   ```
   This copies `.claude/skills/factory`, `.claude/agents/*`, the guard hook,
   `.factory/{factory.yaml,backlog.md,dashboard.html}`, adds `.factory/jobs/` to
   `.gitignore`, appends the Product Factory section to `AGENTS.md`, wires the
   hook into `.claude/settings.json` by merging rather than overwriting, and
   commits nothing.

2. **Review the draft** with `/factory init` in Claude Code or
   `$product-factory init` in Codex. Verify the detected stack and docs root,
   then fill in conventions, gates and VCS settings from the project. Preview
   the handbook import with `pf knowledge plan`.

3. **Fill `AGENTS.md`** with the real commands. The agents carry nothing
   project-specific. Everything they know about the repo comes from `AGENTS.md`
   and the descriptor. This step decides how well the factory works on the
   project.

4. **Protect the base branch** on the remote: require a PR and at least one
   approval, and block force-push. The factory cannot merge, and branch
   protection is what makes that true for the remote.

5. **Check and commit:**
   ```bash
   pf doctor ~/Work/MyApp
   ```
   Then commit `.claude/`, `.codex/agents/`, the factory skill under `.agents/`, `.factory/` (without `jobs/`), `AGENTS.md` and
   `CLAUDE.md` in the product repo yourself. A filled example of the `AGENTS.md`
   the factory needs is in [examples/AGENTS.example.md](examples/AGENTS.example.md).

6. **Start work.** Send a bounded request with an outcome and acceptance
   criteria, or use `issue` for an existing GitHub/GitLab issue. Use `next`
   for items queued in `.factory/backlog.md`. Open `pf dashboard` for the
   Kanban-style view of backlog, module progress, reviews and artifacts.
   A separate task board is optional. For Codex, confirm project trust and
   the worktree permissions described in [the runtime adapter](../adapters/other-agents.md).

## Multi-repo products

The full walkthrough, with commands, is [Multi-repo workspace guide](07-multi-repo-workspaces.md). In short:
Audit with `pf audit <workspace> --modules`. Install the workspace root,
review the generated module descriptor, then run
`pf install project <workspace>` from the workspace. The installer generates the module rules.

Set `layout: multi-repo` and `workspace_root` to the folder that holds the
module clones. Then add one `modules[]` entry per child repo, with its `area`
(used for the `repo::<area>` label), `path`, `agents_md`, `gates` and `tests`.
The [example descriptor](examples/workspace.factory.yaml) uses a docs module, a web app, a service and a mobile client. Install the kit at `workspace_root` and run
`/factory` there. Triage picks the module, and the job's worktree is created
from that module's clone, with the module's `vcs` settings, so the PR targets
that child repo.

## Several stacks in one Git history

Review the installer draft's module paths. Each job gets a separate worktree
and branch even when the modules share a Git repository. The job diff is checked
against its module path. Keep shared contracts, root lockfiles and other
cross-module changes in a root job and sequence their dependents. Worktrees
isolate changes on disk. They can still conflict when branches are integrated.

## Updating a project to a newer factory

```bash
cd ~/Work/ProductFactory
git pull --ff-only
cargo install --path cli --locked
pf --version
pf install project ~/Work/MyApp
pf doctor ~/Work/MyApp
```

For a binary install, replace `pf` with the verified release binary before
running the project installer. The installer preserves `factory.yaml` and
`backlog.md`. Review and commit the kit changes, then reopen your AI session.
New jobs are enrolled in knowledge capture after the kit update. If an
earlier installer also reports `pf 1.0.0`, verify its SHA-256 checksum against
the current release and check that `pf job knowledge --help` is available.
Previously claimed jobs retain their original evidence requirements.
[Release downloads](https://github.com/Mat-Agh/product-factory/releases)

## Runtime entry points and code graphs

Run `pf install project <repo> --install-graphify` from any shell.
It installs all runtime entry points in one pass. Independently opened module
repos get their own project instructions and generated roles. Shared-history
modules use the Git root's runtime kit and graph. For an existing install,
rerun the installer to pick up the entry points and run `pf providers setup`
to configure workspace model routes. [Setup](01-getting-started.md#model-routing)
The install prepares missing generic stack documentation in each repository
and leaves existing pages intact. It reports chapter numbering conflicts for
review. If Hermes and model routes are present, it trusts the project skill.
Use `pf graph --root <repo> view` to open the local 3D code network and
`pf graph --root <repo> query "<symbol>"` for bounded agent context.
For a workspace with independent repositories, run `pf graph --root <workspace> view --modules` and open `.factory/graphify/workspace-view.html`. Switch between Brain cloud and Thought sphere in the viewer.
Select a node to inspect its source preview, contained symbols, outgoing uses and incoming references. Shift+drag pans; the buttons and wheel control zoom.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Operator guide](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
