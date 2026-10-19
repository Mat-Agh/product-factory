<!-- Product Factory: append this section to the product repo's AGENTS.md. -->

## Product Factory

This repo is run by the [Product Factory](https://github.com/Mat-Agh/product-factory).
Its descriptor is `.factory/factory.yaml`, and its rules are in
`.claude/skills/factory/CONSTITUTION.md`.

- **Claude Code:** `/factory <request>` runs the whole pipeline (triage → spec →
  design → plan → build → review lenses → QA → gate → PR). See
  `.claude/skills/factory/SKILL.md`.
- **Any other agent (Codex, Cursor, Gemini, Hermes):** act as one role at a time.
  Read `.claude/skills/factory/protocol.md`, then the role file
  `.claude/agents/<role>.md`, and follow it exactly. Roles are
  `product-manager`, `designer`, `tech-lead`, `engineer`, `reviewer-code`,
  `reviewer-security`, `reviewer-ux`, `reviewer-ui`, `tester` and
  `release-manager`. Job state lives in `.factory/jobs/<job-id>/`.
- **Documentation:** all docs live under `docs/`, one folder per area, each
  with a `README.md`. After adding or moving a doc, run
  `pf docs-map --fix`, which regenerates the README links and
  the map in `docs/README.md`.
- **Tooling:** the `pf` CLI must be on PATH. The guard hook calls `pf guard` and
  blocks Bash (fail closed) when `pf` is missing. Install it with
  `cargo install --git ssh://git@github.com/Mat-Agh/product-factory.git pf`.
- **Hard rules:** work on `factory/<job-id>` branches in worktrees. Humans merge,
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
