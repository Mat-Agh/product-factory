# Product Factory — how to work in this repository

<!-- Installed by `pf install`; edit it in the Product Factory repo, not here. -->

This repository is run by the **Product Factory**. Its rules are in
`.claude/skills/factory/CONSTITUTION.md`, and its descriptor is
`.factory/factory.yaml`. In a workspace module, the descriptor is the nearest one
in a parent folder.

## Route work through the factory

- When the operator asks for a change, run it through the **`factory` skill**:
  `/factory <request>`. Triage picks the lane, and a trivial change takes the
  short lane. This applies even when they do not type `/factory`. A change is
  anything that alters code, docs, design, configuration, product behaviour, or
  a release.
- Answer directly, without the factory, when the operator asks a question,
  wants an explanation, a code read, a review of something already written,
  `/factory status`, or says to work "outside the factory" or "directly".
- Never edit, commit or push on the base branch yourself. Factory work happens on
  `factory/<job-id>` branches in worktrees. Only humans merge, tag, release,
  deploy or publish.
- At the start of a working session, if the operator asks what is going on, run
  `/factory status`.

## Where knowledge goes

- Truth lives in git. Durable team knowledge, such as a convention, a command, a
  decision or a gotcha, goes into this repo's `AGENTS.md`, `docs/` or
  `.claude/rules/`, through a factory job. Write it to auto memory only as a
  short pointer.
- Auto memory (`~/.claude/projects/…/memory/`) is a per-machine cache. Factory
  subagents never see it. When it holds team knowledge, propose promoting it with
  `/factory memory`.
- Personal, machine-local preferences go in `CLAUDE.local.md`, which is
  gitignored.
- Documentation lives under `docs/`, one folder per area, each with a README.
  After adding or moving a doc, run `pf docs-map --fix`.

## Tooling

`pf` (the Product Factory CLI) must be on PATH. The guard hook calls `pf guard`,
and it blocks Bash on purpose when `pf` is missing.
