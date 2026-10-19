# 03 — Portability and recovery

**Goal:** losing any single thing, whether the Claude account, the GitHub account,
the laptop or the agent vendor, costs at most the work in flight on that one
thing. It never costs the factory, the rules or a product's history.

## Where every piece lives

| Piece | Primary home | Backup | Restore |
|---|---|---|---|
| Rules, roles, procedures | this repo on GitHub | mirror remotes and offline bundles (`scripts/mirror.sh`) | `git clone` from any mirror or the bundle |
| Operator profile (how you work) | `operator/USER.md` | same as above | `scripts/install.sh user` |
| Product truth (PRDs, ADRs, code, descriptor, backlog) | each product repo | that product's own mirrors | `git clone` |
| Work in flight | `factory/*` branches and PRs | any local clone has the branches | re-push from a clone, re-open the PR |
| Claude Code memory | `~/.claude` on one machine | `scripts/export-claude.sh` → review → fold into `USER.md` | re-installed from `USER.md`, not from the export |
| Secrets | password manager / `.env` (never git) | your password manager | re-enter |

## Rules that keep it portable

1. **Never edit the factory only in `~/.claude`.** The user install is a set of
   symlinks into this repo, so any edit there lands in the repo. Commit it.
2. **Durable facts go to git.** When an agent learns something about you, write
   it to `operator/USER.md`. When it learns something about a product, write it
   to that product's `AGENTS.md` or docs. Vendor memory is a cache.
3. **The project kit is a committed copy.** A teammate or a fresh account gets
   the factory by cloning the product, even without this repo.
   `.factory/installed.lock` records which factory revision it came from, and
   `doctor.sh` reports drift.
4. **Nothing in the factory is tied to an account.** There are no account IDs,
   no tokens and no hosted state. `vcs.host` can be GitHub or GitLab.

## Set up mirrors (do this once)

```bash
cd ~/Work/ProductFactory
scripts/mirror.sh add gitlab git@gitlab.com:<you>/product-factory.git   # or any second host
scripts/mirror.sh bundle                                               # offline copy in ~/Backups
```

After that, `scripts/mirror.sh push` updates every remote. Add the same second
remote to each product repo.

## Recovery runbooks

**Claude account lost.** Sign in with any other Claude account, or use
another agent.
```bash
git clone <any mirror>/product-factory.git ~/Work/ProductFactory
~/Work/ProductFactory/scripts/install.sh user
```
Product repos already contain their kit, so `/factory status` works right away.

**GitHub account lost.** Clone from the GitLab mirror or the bundle. Create the
repo under the new account, then
`git remote set-url origin <new>` and `scripts/mirror.sh push`. In each product,
push the `factory/*` branches to the new host and re-open the PRs.

**Laptop lost.** On the new machine, clone the factory, run
`install.sh user`, then clone the products. Unfinished job folders are lost by
design, but each job's branch and PR carry its summary. Re-run a job with
`/factory rework <job> "resume"` after recreating the worktree, or start
fresh.

**Agent vendor unavailable.** Use the role files directly from Codex, Cursor or
Hermes ([adapters/](../adapters/)). The procedure is markdown, so any capable
agent can run one role at a time.

## Health

`scripts/doctor.sh [project]` checks tools, auth, guard tests, mirrors, the user
links, the project install, kit drift and branch protection. Run it on any new
machine, and after every factory update.
