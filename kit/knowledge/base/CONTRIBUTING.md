# Contributing to {{MODULE}}

**Status:** Active
**Last updated:** {{DATE}}

This must match the hooks and CI exactly. The pipeline contract is in
[docs/development/CICD.md](docs/development/CICD.md).

## Branches

Branch from the integration branch with a name that matches the branch rule in
`CICD.md`. One change per branch.

## Commits

Follow the commit convention in `CICD.md`. Commit messages describe what changed
and why. Never bypass hooks (`--no-verify`, skip variables).

## Review and merge

Open a merge request into the integration branch. It needs a passing pipeline and
a human approval. Humans merge, tag, release and deploy.

## Before opening a change

- [ ] The quality gates in `docs/development/09-tools-scripts-and-commands.md` pass
- [ ] Contracts (API, events, schema) updated with the behaviour
- [ ] Docs updated: the feature doc, `docs/features/IMPLEMENTATION_STATUS.md`,
      `CURRENT_STATE.md` when what the repo contains changed
- [ ] A dated entry in `docs/history/DEVELOPMENT_LOG.md`
- [ ] No secrets, `.env` files or build output committed
- [ ] `git diff --check` is clean
