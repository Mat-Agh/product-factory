# Current repository state

**Status:** Active
**Last updated:** 2026-11-17

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
- **Distribution:** the standalone binary carries the generic project kit
  and stack knowledge. A Factory checkout supplies editable assets and the
  personal operator profile for user-level links.
- **Runtimes:** the installer generates Claude Code, Codex and Antigravity
  roles and provides a shared project skill for Hermes and Gemini CLI.
  Workspaces can route independent workers through Hermes to Claude, GPT,
  Gemini or a gateway such as 9Router by task effort.
- **Code navigation:** Graphify builds and refreshes a local graph for each
  Git checkout and job worktree. `pf graph watch --modules` keeps every
  independent checkout and the combined workspace network current after code
  changes. The protocol uses it for bounded navigation; installation also
  writes an interactive local 3D file network.
- **Discovery:** audit and knowledge planning can discover checked-out Git
  submodules and stack directories without a descriptor. Project install
  drafts a descriptor and can install both kits into independent modules.
- **Tooling:** one Rust CLI, `pf` (`cli/`). It covers the guard hook, docs map,
  diagrams, installer (project copy or user symlinks), doctor, mirror and bundle
  backups, memory export, and the dashboard server. There is no Python and no
  bash script.
- **Job control:** `pf job` serializes intake, creates an isolated worktree per
  job, verifies module scope and advances only on the required artifacts. The
  coordinator uses the installed native role dispatcher. The local board
  shows queued requests, module filters, review rounds and evidence in a
  responsive English/Persian dashboard with persistent light and dark themes.
- **Session integration:** an always-on rule makes every Claude session in a
  factory repo, or in a module of a factory workspace, route change requests
  through `/factory`. Module kits and path-scoped module rules cover multi-repo
  workspaces, and `/factory memory` promotes auto memory
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
  Installation creates missing stack pages per independent checkout and
  regenerates documentation maps, while preserving existing content and
  reporting handbook numbering conflicts for review.
- **Documentation:** everything is under `docs/`, one connected tree generated
  and checked by `pf docs-map`. It covers governance, the operator guide,
  architecture, component contracts, the development handbook, ADRs, status
  (this page, implementation status, roadmap, evolution log), adapters and the
  operator profile. Diagrams are inline Mermaid blocks, linted and parsed by `pf diagrams`.

**Verification state:** CLI tests include scratch worktree runs, Graphify
updates and provider dispatch with a synthetic credential. The Connect
installation resolves three configured private 9Router routes. On 2026-11-17,
the five live Hermes combos contained only GPT and DeepSeek members, and all
five answered through GPT fallback. DeepSeek accepted the API key but returned
insufficient balance for both Pro and Flash chat requests. A complete Factory
job through review and delivery has
not yet run on a real product. That remains the Phase 1 exit criterion. Details:
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
