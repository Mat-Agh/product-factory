# 07 — Putting a multi-repo workspace under the factory (Daria Connect)

**Status:** Active — procedure verified on a scratch workspace; not yet run on the real Connect workspace
**Last updated:** 2026-10-19

This page puts **Daria Connect** (`$HOME/Work/Connect`) under the factory. The
same steps work for any workspace that keeps several independent repos in one
folder. Connect's root is a GitHub repo of gitlinks, and its five children are
independent GitLab repos (Product, Front-End, Back-End, Client and Federation),
each with its own stack, `develop` branch, CI and `AGENTS.md`.

## What you get

- **One factory for the whole workspace.** The descriptor, backlog, job state
  and dashboard live at the workspace root (`~/Work/Connect/.factory/`).
- **Every Claude session uses it.** A session opened at the root *or inside any
  child* loads an always-on rule (`.claude/rules/product-factory.md`). That rule
  routes change requests through `/factory`, without you typing it. The skill
  is also set to trigger on change requests, and the guard hook blocks merges,
  tags and pushes outside `factory/*`.
- **Each child keeps its own stack.** A path-scoped rule per module
  (`.claude/rules/modules/<area>.md`) loads only when Claude touches that child.
  It points Claude to the child's own `AGENTS.md`/`CLAUDE.md` and gates, and to
  its test policy (no unit tests in Back-End and Federation).
- **Jobs stay inside one child.** Triage splits cross-child work into one job
  per child. Each job gets a worktree of that child outside the workspace
  (`~/Work/Connect-factory/<area>/<job-id>/`) and opens a GitLab MR to that
  child's `develop`. You approve and merge. You still bump the workspace
  gitlinks on GitHub `main` by hand, after the children are published.
- **No AI attribution**, as the workspace `AGENTS.md` requires.
  `vcs.attribution: none` makes `pf install` set Claude Code's `attribution`
  setting to empty, so commits and MRs carry no AI trailer or line.
- **Memory moves into git.** `/factory memory` turns the Connect auto memory
  (9 entries today) into reviewed `AGENTS.md`/docs edits, or retires them.

## Before you start

| Check | Command | Expected |
|---|---|---|
| `pf` installed | `pf --version` | `pf 0.2.0` or later. If not: `cd ~/Work/ProductFactory && cargo install --path cli` |
| User-level link | `pf install user` | `/factory` and the agents linked into `~/.claude` |
| GitLab CLI logged in to your host | `glab auth status --hostname gitrnd.dariarnd.ir` | logged in. If not: `glab auth login --hostname gitrnd.dariarnd.ir` |
| Clean trees on `develop` | `cd ~/Work/Connect && git submodule foreach 'git status --porcelain; git branch --show-current'` | nothing dirty, every child on `develop` |
| Hermes intake paused | Hermes cron for Connect (`$HOME/Work/Hermes`) | paused while you pilot, so two systems don't take the same issue |

## Step 1 — Install at the workspace root

```bash
cd ~/Work/Connect
pf install project . --no-ai-attribution
cp ~/Work/ProductFactory/docs/guide/examples/connect.factory.yaml .factory/factory.yaml
```

Open `.factory/factory.yaml` and check every `# verify` line: the product
one-liner, each child's `tests:` value, the docs paths and the acceptance
ledger. The draft is in [examples/connect.factory.yaml](examples/connect.factory.yaml).

## Step 2 — Install into each child

```bash
for m in Daria-Connect-Product Daria-Connect-Front-End Daria-Connect-Back-End Daria-Connect Daria-Connect-Federation; do
  pf install module "$m" --workspace .
done
pf rules sync
```

Each module install copies the skill, agents, always-on rule, guard hook and the
`attribution` setting into that child's `.claude/`. It does **not** touch the
child's `AGENTS.md` or `CLAUDE.md`. `pf rules sync` writes the five path-scoped
module rules at the root.

## Step 3 — Check

```bash
pf doctor ~/Work/Connect
```

Expect `ok` for the guard hook, the always-on rule, `AI attribution off`, every
`module … installed`, and no stale module rules. Expected `note` lines:
- **"CLAUDE.md does not import @AGENTS.md".** This is true today for the root and
  every child. With a `CLAUDE.md` present, Claude Code does not auto-load
  `AGENTS.md`. Add an `@AGENTS.md` line near the top of each `CLAUDE.md`, as one
  change per repo, following your rule that `CLAUDE.md`, `AGENTS.md` and the
  `.cursor`/`.codex`/`.agents` mirrors stay in sync. The module rules already
  tell Claude to read `AGENTS.md`, so the factory works before you do this.
- **"auto memories … review them with /factory memory".** That is step 5.
- **Setup / Dev server / Gates headings.** The root is a router, so this is
  expected there. Check that each child's `AGENTS.md` says how to set up a fresh
  checkout, run it locally and run its gates. Several say "Quality gate", which
  is fine.

## Step 4 — Docs: check, don't fix

```bash
pf docs-map                                                        # root docs/
(cd Daria-Connect-Back-End && pf docs-map)
(cd Daria-Connect-Front-End && pf docs-map --docs doc)             # Front-End keeps docs in doc/
```

**Do not run `--fix` yet.** Connect's docs already carry your own
`<!-- documentation-routing:start/end -->` blocks (113 files). `--fix` would
rewrite them all in the factory's format. If you want the factory format,
run `--fix` on a branch in one child, review the diff, and merge it yourself.
Until then, treat the report as information.

## Step 5 — Promote memory

Open Claude Code at `~/Work/Connect` and run:

```
/factory memory
```

It lists each auto memory with a verdict (team, personal, already in repo,
stale, or secret) in `.factory/memory-review.md`, and changes nothing until you
choose rows. Approved team items become a docs job, and so a reviewed MR in the
owning child. Approved personal items go to `CLAUDE.local.md`.

## Step 6 — Commit the setup

Commit in each child on a branch, open an MR to `develop`, and merge it yourself.
At the root, commit to `main`:

```bash
# in each child
git switch -c chore/product-factory develop
git add .claude/settings.json .claude/skills/factory .claude/agents .claude/rules
git commit -m "Add Product Factory kit"
# push and open the MR to develop the way you normally do

# at the root, after the children are merged
git add .claude/settings.json .claude/skills/factory .claude/agents .claude/rules .factory AGENTS.md .gitignore
git commit -m "Put the workspace under the Product Factory"
```

Stage these paths explicitly. `.claude/settings.local.json`, `launch.json` and
`.cc-writes` are personal or machine state and must stay out. These are your own commits. The factory does not commit for you.

## Step 7 — Pilot

Start small, from the root or from inside a child:

```
/factory Fix a typo in Daria-Connect-Front-End's README
```

You should see triage pick `module: frontend` and the trivial lane, then a
worktree at `~/Work/Connect-factory/frontend/001-…`, a code review, a gate, and
a GitLab MR to Front-End `develop` with no AI attribution. Then try one bug in
Back-End (QA reproduces first, and no unit tests are added), and one UI change
in Client (the designer reads Figma and stops if a frame is missing). Run
`/factory retro` after each one.

## Day to day

| You want | Do |
|---|---|
| Something built or fixed | Ask in any Connect session. The rule routes it through `/factory` |
| To work directly, without the factory | Say "outside the factory" or "directly". For merges or pushes you do yourself, start Claude with `FACTORY_GUARD=off claude` |
| Status | `/factory status`, or `cd ~/Work/Connect && pf dashboard` |
| Existing GitLab issues | `/factory issue <url>` |
| Release a child | `/factory release <version>`, then you tag and publish it, then bump the gitlink |
| Update the factory | `cd ~/Work/ProductFactory && git pull && cargo install --path cli`, then repeat steps 1–2 (`pf install project .` and the module loop) |

## Hermes and the factory

Both can run on Connect, but not on the same intake:
- Keep Hermes cron intake paused while you pilot.
- Afterwards, either retire Hermes' Connect intake, or keep Hermes for intake
  and Telegram only and let it hand work to the factory.

The factory uses `factory/*` branches and Hermes uses `.hermes/*`, so their
branches never collide.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Operator guide](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
