# ADR-011 — Adopt an existing process: inventory, preserve, configure, then align

**Status:** Accepted
**Last updated:** 2026-10-19
**Date:** 2026-10-19

## Context

Established products arrive with a working agentic process: rule
files for several tools that are kept in sync by convention, git hooks, five CI
pipelines with approval-triggered auto-merge, strict branch and commit
conventions, a no-AI-attribution rule, an existing issue automation (Hermes),
and plaintext credentials in local tool config. A naive install would break
it. Some repositories reject the factory's `factory/<job-id>` branches
outright, and an install could overwrite same-named agents.

## Decision

- **Adoption is the first step** of putting any existing project under the
  factory (CONSTITUTION §8). `pf audit` inventories the setup, and
  `/factory adopt` reads it, classifies every rule, and records the
  reconciliation in `.factory/adoption.md`.
- **Precedence:**
  1. The constitution's authority and safety rules.
  2. The product's own engineering and process rules, which the factory obeys
     by configuring itself (branch template, commit convention, attribution,
     risk paths).
  3. Factory defaults.
- **Never delete.** Existing files are kept. Same-named non-factory agents and
  skills are kept as `*.pre-factory*`. Edits to the product's files are proposed
  diffs, landed as reviewed jobs.
- **The guard follows the project.** Job-branch shape comes from the descriptor.
  Hook-bypass switches and `core.hooksPath` overrides are blocked.
- **Synchronise governance** into the product's other agent surfaces through
  their own mirroring convention, so Codex, Cursor and the rest work under the
  same rules.

## Alternatives considered

- **Require projects to adopt the factory's conventions** (for example, add a
  `factory/` branch pattern everywhere). This was rejected as the default: it
  edits five repos' CI before the factory has proven itself. It remains an
  optional later step, using reviewed `.hermes/` or `codex/`
  patterns.
- **Overwrite and let git history preserve the old files.** Rejected: silent
  loss of working process, and against CONSTITUTION §6.

## Consequences

- `/factory init` on an existing project takes longer. It reads everything first.
- Branch names vary per project. Tooling must read them from the descriptor and
  never assume `factory/`.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Architecture decisions](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
