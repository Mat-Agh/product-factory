# Current repository state

**Status:** Active
**Last updated:** 2026-10-19

This repository owns the Product Factory: an agentic pipeline that takes a product
idea or request through triage, product, design, planning, build, parallel
review, QA and a gate to a pull request that a human merges. It also owns the
release lane, which produces a launch kit.

It contains:

- **Rules:** [`CONSTITUTION.md`](../governance/CONSTITUTION.md) (v1).
- **The kit** installed into products: the `/factory` coordinator skill, the
  shared agent protocol, ten role agents, wiring for the `pf guard` hook that blocks merges,
  tags, releases, force-pushes and discarding work, a project descriptor
  template, a backlog and a local dashboard.
- **The operator profile** (`docs/operator/USER.md`), which keeps preferences out of
  vendor memory.
- **Adapters** for running the same roles in Hermes, Codex, Cursor and other
  agents.
- **Tooling:** one Rust CLI, `pf` (`cli/`). It covers the guard hook, docs map,
  diagrams, installer (project copy or user symlinks), doctor, mirror and bundle
  backups, memory export, and the dashboard server. There is no Python and no
  bash script.
- **Session integration:** an always-on rule makes every Claude session in a
  factory repo, or in a module of a factory workspace, route change requests
  through `/factory`. Module kits and path-scoped module rules cover multi-repo
  workspaces such as Daria Connect, and `/factory memory` promotes auto memory
  into git.
- **Adoption:** `pf audit` and `/factory adopt` take over a project that already
  has its own process. The factory keeps that process, configures itself to obey
  it (branch template, commit convention, attribution, risk paths), and proposes
  only the necessary edits ([COMP-017](../components/COMP-017-adoption.md)).
- **Knowledge structure:** every repo gets its stack's knowledge structure:
  current state, development history, numbered handbook (`00` principles,
  `09` tools, `15` code samples, `16` link map, stack chapters), architecture,
  decisions, features and status. General engineering knowledge for every
  language and stack used so far lives in one **development knowledge library**
  ([docs/knowledge/](../knowledge/README.md)): principles, Kotlin (client: KMP,
  Android, iOS; backend: Spring), TypeScript web, Rust, API, security, product,
  CI/CD and agentic workflows. Each stack's pack map says which pages a repo
  imports; `pf knowledge init` imports missing pages and appends the sections an
  existing doc lacks, never overwriting, and every job keeps it true
  ([COMP-018](../components/COMP-018-knowledge-structure.md)).
- **Documentation:** everything is under `docs/`, one connected tree generated
  and checked by `pf docs-map`. It covers governance, the operator guide,
  architecture, component contracts, the development handbook, ADRs, status
  (this page, implementation status, roadmap, evolution log), adapters and the
  operator profile. Diagrams are committed SVGs rendered from Mermaid sources.

**Verification state:** `pf` passes its 27 cargo tests (including the 80 guard
policy cases) and matches the Python tools it replaced. Every subcommand has
run end to end on scratch setups. The docs map and diagram checks pass. **No job has run end to end
on a real product yet.** That is the Phase 1 exit criterion. Details:
[IMPLEMENTATION_STATUS.md](IMPLEMENTATION_STATUS.md).

The repository is hosted at `Mat-Agh/product-factory` (private) with no mirror
yet. Product code, product docs, job runtime state, credentials and Claude Code
memory exports are not stored here.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Status](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
