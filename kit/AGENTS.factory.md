<!-- Product Factory: append this section to the product repo's AGENTS.md. -->

## Product Factory

This repo is run by the [Product Factory](https://github.com/Mat-Agh/product-factory).
Its descriptor is `.factory/factory.yaml`, and its rules are in
`.claude/skills/factory/CONSTITUTION.md`.

- **Documentation:** all docs live under `docs/`, one folder per area, each
  with a `README.md`. After adding or moving a doc, run
  `pf docs-map --fix`, which regenerates the README links and
  the map in `docs/README.md`.
- **Tooling:** the `pf` CLI must be on PATH. The guard hook calls `pf guard` and
  blocks Bash (fail closed) when `pf` is missing. Install it with
  `cargo install --git ssh://git@github.com/Mat-Agh/product-factory.git pf`.
- **Hard rules:** work on job branches (`vcs.branch_template`, default `factory/<job-id>`) in worktrees. Follow the repo's branch and commit conventions, and never bypass its hooks. Humans merge,
  tag, release and deploy. Never force-push or use `--no-verify`. Never stash or
  reset work you did not create. Never put secrets in git. Done means the gates
  pass plus recorded acceptance evidence. Compile is not done.

The factory needs these sections to exist in this file:

- **Setup**: commands for a fresh checkout (install dependencies, env files from
  `.env.example`).
- **Dev server**: the command and how to set its port.
- **Gates**: the lint, type-check, build and test commands, and which ones the
  module forbids (for example "no unit tests").
- **Conventions**: architecture rules, living samples to copy, naming.
- **Docs map**: which doc to read for which area.


<!-- product-factory-entrypoints -->

### Product Factory in your AI session

For a change request, load the coordinator before editing. Use `/factory` in
Claude Code or `$product-factory` in Codex. In Hermes, Antigravity or Gemini CLI, load the
`product-factory` project skill from `.agents/skills/`. In an IDE that only
loads this file, read `.claude/skills/factory/SKILL.md` directly. For status,
use `pf job status` or open the local board with `pf dashboard`. When a Factory
coordinator assigns you a role for an existing job, follow that role’s
handoff and write its assigned artifact.

In a workspace module, locate the parent workspace descriptor before claiming
a job. Follow the installed constitution and protocol referenced by the
coordinator. Codex role definitions are in `.codex/agents/factory-*.toml`.
For configured API workers, follow the coordinator's model routing procedure.
