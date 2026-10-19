# 04 — Add a project

**Status:** Active
**Last updated:** 2026-10-19

1. **Install the kit** into the product repo:
   ```bash
   ~/Work/ProductFactory/scripts/install.sh project ~/Work/MyApp
   ```
   This copies `.claude/skills/factory`, `.claude/agents/*`, the guard hook,
   `.factory/{factory.yaml,backlog.md,dashboard.html}`, adds `.factory/jobs/` to
   `.gitignore`, appends the Product Factory section to `AGENTS.md`, wires the
   hook into `.claude/settings.json` by merging rather than overwriting, and
   commits nothing.

2. **Run `/factory init`** in Claude Code inside the repo. It drafts
   `.factory/factory.yaml` from what it finds and marks guesses with
   `# verify`. It then lists what `AGENTS.md` is missing: setup, dev server and
   port flag, gates, conventions, docs map.

3. **Fill `AGENTS.md`** with the real commands. The agents carry nothing
   project-specific. Everything they know about the repo comes from `AGENTS.md`
   and the descriptor. This step decides how well the factory works on the
   project.

4. **Protect the base branch** on the remote: require a PR and at least one
   approval, and block force-push. The factory cannot merge, and branch
   protection is what makes that true for the remote.

5. **Check and commit:**
   ```bash
   ~/Work/ProductFactory/scripts/doctor.sh ~/Work/MyApp
   ```
   Then commit `.claude/`, `.factory/` (without `jobs/`), `AGENTS.md` and
   `CLAUDE.md` in the product repo yourself. A filled example of the `AGENTS.md`
   the factory needs is in [examples/AGENTS.example.md](examples/AGENTS.example.md).

6. **Pilot.** Run one trivial job and one standard job end to end before you use
   the factory for anything larger. Run `/factory retro` on both.

## Multi-repo products (Daria Connect style)

Set `layout: multi-repo` and `workspace_root` to the folder that holds the
module clones. Then add one `modules[]` entry per child repo, with its `area`
(used for the `repo::<area>` label), `path`, `agents_md`, `gates` and `tests`.
Example for Connect: `product` (no runtime code), `frontend`, `backend`
(`tests: no`), `client` (Android, never called Frontend) and `hub` (`tests: no`,
never shipped to customers). Install the kit at `workspace_root` and run
`/factory` there. Triage picks the module, and the job's worktree is created
from that module's clone, with the module's `vcs` settings, so the PR targets
that child repo.

## Updating a project to a newer factory

```bash
cd ~/Work/ProductFactory && git pull
scripts/doctor.sh ~/Work/MyApp        # shows drift
scripts/install.sh project ~/Work/MyApp
```
Your `factory.yaml` and `backlog.md` are kept. Review the diff in the product
repo and commit it.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Operator guide](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
