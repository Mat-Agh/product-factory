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

## Step 0 — Audit and adopt (first, before anything is installed)

Connect already runs its own process. Five CI pipelines, git hooks, branch and
commit conventions, Hermes, Codex/Cursor/Junie rules and Firebase agent skills
are all in place. The factory adopts them (CONSTITUTION §8). It inventories them,
keeps them, configures itself to obey them, and only proposes edits where they
must change.

```bash
cd ~/Work/Connect
pf audit .                                   # workspace root
for m in Daria-Connect-Product Daria-Connect-Front-End Daria-Connect-Back-End Daria-Connect Daria-Connect-Federation; do pf audit "$m"; done
```

After step 1, run `/factory adopt` in Claude Code at the root. It reads every
file the audit lists and writes the reconciliation to `.factory/adoption.md`.
Nothing changes until you approve rows.

### What the audit found (2026-10-19, read-only)

| # | Finding | Evidence | Class | Reconciliation |
|---|---|---|---|---|
| 1 | **Branch names**: hooks and CI accept only `main`, `develop`, `feature/…`, `fix/…`, `docs\|chore\|refactor\|test\|ci\|build\|perf/…`, `release/x.y.z`, `hotfix/x.y.z`, `.hermes/<n>-…` and `codex/…`. A `factory/<job-id>` branch is **rejected**. This was a BLOCKER in Back-End and Front-End, and the same rules are in Client, Federation and Product | `build-logic/…/GitConventionRules.kt` (Back-End, Client, Federation), `tooling/ci/git-conventions.ts` (Front-End), `.ci/conventions.py` (Product) | configure the factory | `vcs.branch_template: "{kind}/{job-id}"` with `branch_kinds` (feature→`feature`, bug→`fix`, docs→`docs`, chore→`chore` …). Job branches become `feature/004-org-name`, which the conventions accept, and the guard then allows only those shapes. This is already in the draft descriptor. Optional later step: add a `factory/…` pattern to the conventions, as was done for `.hermes/` and `codex/` |
| 2 | **Commit subjects** must be Conventional Commits with a lowercase subject (`type(scope)!: …`), and author emails must be `@dariahamrah.ir` | same files, `.githooks/pre-commit` | configure the factory | `vcs.commit_convention: conventional`. Commits use your git identity, which is already `m.aghalarkhani@dariahamrah.ir` |
| 3 | **No AI attribution** anywhere (root `AGENTS.md`, mirrored in each child) | `AGENTS.md`, `CLAUDE.md` | configure the factory | `vcs.attribution: none`, and `pf install` writes Claude Code's `attribution` setting |
| 4 | **Approval-triggered auto-merge**: `merge_when_approved` waits for a human Approve, then merges once green | `.ci/approve.yml`, `.ci/merge_when_approved.py` (all five) | keep | Compatible. Your Approve is the merge decision, and the guard blocks agents from approving. Escalated MRs open as drafts and merge only after you mark them ready |
| 5 | **Git hooks** exist in every child (`.githooks/`: commit-msg, pre-commit, pre-push running `ciValidate`). They are **not active** locally, because `core.hooksPath` is unset | `.githooks/install.sh` | keep, then decide | Factory commits run whatever hooks are active, and CI enforces the same rules either way. Recommended: run `.githooks/install.sh` in each child, so factory pushes fail locally before CI |
| 6 | **Hook bypass switches**: `DARIA_HOOK_SKIP_GRADLE`, `GIT_CONVENTION_SKIP_COMMIT_SUBJECTS` | `.githooks/*` | align (enforced) | The guard now blocks setting any `*SKIP*`, `HUSKY` or `LEFTHOOK` variable around git writes, and `core.hooksPath` overrides |
| 7 | **Rule mirrors kept in sync by convention**: `.cursor/rules`, `.codex/rules`, `.agents/rules` (Back-End has 19 each), plus `CLAUDE.md` and `AGENTS.md` | root `CLAUDE.md` ("kept in sync by convention") | synchronise | Mirror the factory rule into each folder that holds rules, as `product-factory.md` (`.mdc` for Cursor), and add one "Product Factory" line to each `AGENTS.md` and `CLAUDE.md`. These are proposed edits, landed as one `chore` MR per child |
| 8 | **`CLAUDE.md` doesn't import `AGENTS.md`** (root and all children), so Claude Code doesn't auto-load `AGENTS.md` | each `CLAUDE.md` | synchronise | Add `@AGENTS.md` in the same MRs as row 7 |
| 9 | **Hermes** is named as the issue automation in root, Back-End, Front-End and Product instructions, issue templates and `docs/hermes.md`. Its `.hermes/<iid>-…` branches are allowed by the conventions | 16, 13 and 10 mentions | decide | Choose: (a) retire Hermes' Connect intake and name the factory as the delivery pipeline, or (b) keep Hermes for intake and Telegram only. Then update the instruction files to say so, as part of row 7's MRs. Never let both pick up the same issue |
| 10 | **Codex/Cursor config, Junie plans, Firebase agent skills** (`skills-lock.json`, `.agents/skills`) | root | keep | Unrelated to delivery. Left as they are |
| 11 | **Plaintext GitLab token** in root `.cursor/mcp.json`. It is gitignored and was never committed | `.cursor/mcp.json` | decide | Rotate the token and reference an environment variable instead. Agents must not read or copy it |
| 12 | **Broad pre-approvals** in root `.claude/settings.local.json` (`glab api *`, `git checkout *`) | `.claude/settings.local.json` | keep, then review | The guard still blocks merges, approvals, protected pushes and API writes to merge or protection endpoints. Trim the list if you no longer need it |
| 13 | **Docs routing markers in your own format** (31 Back-End docs, 1 Front-End doc, about 113 across the workspace). Front-End keeps its docs in `doc/` | `docs/**`, `doc/**` | decide | Keep your format (no `pf docs-map --fix`), or convert one repo at a time on a branch |
| 14 | **Inconsistency in your own conventions**: Product allows `.codex/…` branches, while the others allow `codex/…` | `.ci/conventions.py` and `GitConventionRules.kt` | decide | Not a factory concern. Worth unifying in a separate change |

## Step 1 — Install at the workspace root

```bash
cd ~/Work/Connect
pf install project . --no-ai-attribution
cp ~/Work/ProductFactory/docs/guide/examples/connect.factory.yaml .factory/factory.yaml
pf install project . --no-ai-attribution     # again, so the guard picks up the branch template
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

## Step 4b — Apply the adoption plan

In Claude Code at the root, run `/factory adopt`, and approve the rows you want
from the table above. Descriptor rows are applied directly. Rows 7–9 become one
`chore` job per child. Each lands as an MR on a `chore/NNN-…` branch with a
conventional subject and no AI attribution, and you approve it. Re-run
`pf audit` until no BLOCKER remains.

## Step 4c — Knowledge structure

Each child gets the knowledge structure of its stack (set in the descriptor:
Back-End and Federation `spring-kotlin-service`, Client `kmp-android-client`,
Front-End `typescript-web` in `doc/`, Product `product-docs` in `product/`).

```bash
pf knowledge plan --modules           # changes nothing: + import, = equivalent, ~ missing sections, ! numbering conflict
```

The plan run on copies of the children on 2026-10-19 is in
[COMP-018](../components/COMP-018-knowledge-structure.md#connect-today-plan-and-import-on-copies-2026-10-19).
In short:
- **Back-End** already has most of the handbook. It would get 8 files (history,
  sessions, architecture overview, templates) and the missing sections of 7
  existing docs, for example an event-envelope section in `05` and a migrations
  section in `04`.
- **Federation** has a thin handbook. It would get 31 files, including
  `CURRENT_STATE.md` and the full Spring handbook, plus the sections its `00`,
  `09`, `15` and `16` lack.
- **Client** would get 12 files and sections in 5 docs. It already uses `03` and
  `09` for other topics, so nothing is imported for those two numbers; renumber
  one of each pair or record the mapping in its handbook README.
- **Front-End** keeps its flat `doc/NN-*.md` chapters; they are recognised where
  the number and topic match. Four numbers (`02`, `03`, `08`, `09`) are used for
  other topics and are left to you.
- **Product** would get 15 files (the product handbook, design, feature matrix,
  history, templates); no existing doc lacks a pack section.

When you agree, run `/factory knowledge init`. It opens one `docs` job per child.
In each job worktree it imports the pack, then the engineer adapts it: scaffolds
filled from the code, merged sections kept, adjusted to Connect (for example the
Back-End no-test policy) or deleted, and the review markers removed. Each job
lands as an MR on a `docs/NNN-…` branch. Existing text is never overwritten or
edited.

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
worktree at `~/Work/Connect-factory/frontend/001-…` on branch `docs/001-…`, a code review, a gate, and
a GitLab MR to Front-End `develop` with no AI attribution. The MR's branch should be `docs/001-…` (Front-End accepts it), and its commit
subject should be `docs: …`. Then try one bug in
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
