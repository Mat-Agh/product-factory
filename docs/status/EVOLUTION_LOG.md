# Evolution log

**Status:** Active — append-only
**Last updated:** 2026-10-19

Append-only. One dated entry for each factory-level change: what changed, why,
and its evidence.

## 2026-10-20 — Inline Mermaid diagrams, Connect style

- **Why.** Rendered SVGs were static, not resizable, partly dropped by IntelliJ, and
  unreadable to agents. The operator asked for proper inline Mermaid as in the
  Connect docs ([ADR-014](../decisions/ADR-014-inline-mermaid-diagrams.md)).
- **Change.** The seven factory diagrams are inline blocks again, rewritten without
  the constructs that broke GitHub's renderer (HTML-like labels, `<br/>`, `&`
  chaining); the pipeline is now top-down. `docs/assets/diagrams/` and
  `pf diagrams render` are removed. `pf diagrams check` lints every block (type,
  init directives, HTML, chaining, quotes, size) and `pf diagrams validate` parses
  every block with the Mermaid CLI. The knowledge library's ASCII sketches became
  Mermaid, and 15 diagrams were added (layers, service topology, outbox and inbox,
  key hierarchy, client architecture and MVI, auth and realtime sequences, web
  runtime, GitFlow, approval flow, pipeline, deployment channels, authorization,
  events, contract direction, agent workflow). Diagram style rules are in
  `knowledge/principles/documentation.md`.
- **Evidence.** 29 blocks lint clean and parse with Mermaid CLI 11; `validate`
  fails a deliberately broken block with `file:line`; 28 cargo tests; docs map OK.
  Not yet seen on GitHub or in IntelliJ.

## 2026-10-20 — Diagrams render in IDE Markdown previews

- **Why.** In IntelliJ IDEA's Markdown preview, six of the seven diagrams did not
  load. They carried labels as HTML inside `<foreignObject>`, `width="100%"` with a
  CSS `max-width`, and a comment before the `<svg>` element; browsers accept that,
  IDE SVG renderers do not. The one diagram without HTML labels displayed.
- **Change.** `pf diagrams render` passes Mermaid `htmlLabels: false`, writes the
  explicit pixel size from the `viewBox`, and puts the hash note inside `<svg>`.
  All seven diagrams were re-rendered: no `<foreignObject>` remains.
- **Evidence.** `pf diagrams check` OK; headless Chrome screenshots show text
  labels and intact layout; a unit test covers the SVG rewrite. Not yet confirmed
  in IntelliJ itself.

## 2026-10-20 — One development knowledge library for every stack

- **Why.** The operator wants every piece of general knowledge gathered so far, for
  every language and stack, in one place under the development docs, and imported
  into projects from there. The per-stack template folders were hard to browse,
  duplicated shared pages, and had no home for CI/CD, API, security, product,
  Rust or agentic knowledge.
- **Library.** Added `docs/knowledge/` ([ADR-013](../decisions/ADR-013-development-knowledge-library.md)):
  `principles/` (engineering, evidence and done, documentation), `kotlin/` (shared
  language and Gradle; `client/` with architecture, KMP source sets, Android, iOS,
  MVI, data and realtime, library stack, cryptography, testing, acceptance, setup,
  tools, samples; `backend/spring/` with the full service handbook), `typescript/`
  (`web/`), `rust/` (CLI tooling), `api/` (HTTP conventions, errors, events),
  `security/` (classification, cryptography policy, transport and identity,
  threat modeling), `product/`, `cicd/` (GitFlow, approval-gated merge, pipeline
  design, supply chain, runners, deployment) and `agentic/` (factory workflow,
  agent instructions, memory, safety, workspaces). Sources: the Connect handbooks,
  CI/CD, security, API, deployment and governance docs, the workspace CI and
  runner docs, and the Hermes factory docs. Ktor server and iOS release are named
  as not yet gathered.
- **Packs.** The stack chapters moved out of `kit/knowledge/stacks/` into the
  library. Each stack now has a `pack.map` (library page → project path) and keeps
  only scaffolds; the handbook indexes and link maps route to the imported
  `knowledge/` areas.
- **Engine.** `pf knowledge` reads pack maps, imports library pages without the
  factory routing block, and rewrites links between library pages to wherever the
  targets landed (created files or existing equivalents). `pf docs-map` resolves
  kit scaffold links through the base overlay and the pack maps.
- **Evidence.** 26 cargo tests (new: library import with mapped links, every
  library page mapped by a pack), clippy and fmt clean, docs map strict OK.
  Plan and import on copies of the five Connect repos: every added link resolved,
  reruns changed nothing; originals untouched.

## 2026-10-19 — Real stack knowledge, imported and merged; gap audit

- **Why.** The first knowledge templates were headings and comments. The operator
  asked for the stack knowledge itself to be imported where missing, and for
  existing docs to receive what they lack.
- **Knowledge packs.** Rewrote every stack overlay with real content distilled
  from the Connect handbooks, with Connect names removed: Spring/Kotlin (principles,
  service ownership, Kotlin rules, layers, persistence, outbox/inbox, security,
  testing policy, build/CI, cryptography by layer, samples, link map, API
  conventions), KMP/Android (principles, source sets, MVI, data and realtime,
  library stack with owning facades, Android shell, Gradle, client cryptography,
  acceptance checklist, setup), TypeScript web (boundaries, architecture,
  TypeScript, design system, API client, security, testing, build, library stack,
  review checklist) and product docs (router, documentation handbook with the
  evidence model, area guides, PRD template). The generic base got real
  principles, link map, CI/CD and contributing guidance. Project-state files stay
  scaffolds. A `**Source:** Product Factory knowledge pack` line marks the files
  that may be merged.
- **Merge.** `pf knowledge plan` now lists existing files missing pack sections;
  `pf knowledge init` creates missing files and appends missing `##` sections in a
  marked `pf-knowledge` block before the routing block, fixing links to the repo's
  own chapters. It never overwrites or edits existing text (`--files-only` skips
  merging). Equivalence now also finds flat chapters in the docs root and the same
  topic under another number; heading coverage matches stems and section numbers.
- **Workflow.** `/factory knowledge init` runs the import inside a `docs` job
  worktree, never on the base branch, and the engineer adapts or deletes each
  merged section. reviewer-code treats leftover markers or placeholders as
  CHANGES. `pf audit` and `pf doctor` count files missing sections.
- **Gap audit fixes.** The product-docs pack lacked its own router, handbook README
  and architecture README (links broke after import); the SKILL ran the import on
  the checkout; `pf … | head` panicked on a closed pipe (SIGPIPE now default on
  Unix); COMP-012 still said it needed Python; test counts in the status pages
  were stale.
- **Evidence.** 24 cargo tests, clippy clean, fmt clean. Knowledge tests cover
  equivalence, section matching (stems, numbering, non-ASCII), merge placement,
  link fixing, idempotence, and a link check of a fresh import for every stack.
  Plan and import ran on copies of the five Connect repos in a temporary
  directory: every added link resolved and a second run changed nothing. The
  originals were not touched.

## 2026-10-19 — Knowledge structure per stack, distilled from Connect

- Surveyed the docs of all five Connect repos and the workspace root. They
  share a shape: `CURRENT_STATE.md`, a docs router, a numbered handbook (`00`
  principles, `09` tools, `15` samples, `16` link map, plus stack chapters),
  architecture with a conformance ledger, ADRs, features with
  `IMPLEMENTATION_STATUS.md`, cross-project routers and sessions. Front-End
  (`doc/`) and Product (`product/`) use their own roots, and no repo had a
  running development history.
- Added `kit/knowledge/`: a base for every repo (including
  `history/DEVELOPMENT_LOG.md`), plus overlays `spring-kotlin-service`,
  `kmp-android-client`, `typescript-web` and `product-docs`, and `generic`.
- Added `pf knowledge stacks|plan|init`. Plan reports present, covered by an
  equivalent, missing, and numbering conflicts. Init creates only missing files
  and never overwrites.
- Added descriptor keys `modules[].stack` and `docs_root`; `/factory knowledge
  plan|init|refresh`; `init` step 8a and the adopt class **initialize**; and a
  knowledge-upkeep table in the protocol, enforced by reviewer-code, with
  history entries by the engineer and release notes built from history.
- `pf docs-map` now ignores HTML comments, and allows `CURRENT_STATE.md`,
  `CONTRIBUTING.md` and `INSTALL.md` at the root (the Connect convention).
- Evidence: 20 cargo tests; the read-only plan on Connect is recorded in
  COMP-018; scratch init passes `docs-map --strict`.

## 2026-10-19 — Adopting projects that already have a process (constitution v1.2)

- Added CONSTITUTION §8: adoption means inventory first, preserve everything,
  follow a precedence order (constitution authority and safety, then the
  product's own rules, which the factory obeys, then factory defaults),
  synchronise into the product's other agent surfaces, and record the result in
  `.factory/adoption.md`.
- Added `pf audit`, which inventories instructions, rule mirrors, Claude config,
  git hooks and bypass switches, CI and auto-merge, branch and commit conventions
  (regexes extracted and tested against the job branch), attribution rules, other
  automations, secrets (values never printed) and foreign docs markers. Also
  added `/factory adopt` as step 0 of `/factory init`.
- Added `vcs.branch_template`, `vcs.branch_kinds` and `vcs.commit_convention`. The
  guard now reads the project's job-branch shape from the descriptor, and blocks
  hook-bypass variables and `core.hooksPath` overrides. There are now 80 policy
  cases.
- The installer keeps same-named non-factory agents and skills as
  `*.pre-factory*` instead of overwriting them. `pf doctor` fails on audit
  blockers.
- Connect audit (read-only): the branch conventions in all five repos reject
  `factory/<job-id>`, a BLOCKER, which the Connect descriptor resolves with
  `{kind}/{job-id}`. Also found: Conventional Commits, no AI attribution,
  approval-triggered auto-merge (compatible), inactive `.githooks`, bypass
  switches, three rule mirrors, `CLAUDE.md` files without `.md`, Hermes
  references, and a gitignored plaintext token. The reconciliation is in
  `docs/guide/07-workspace-connect.md`.
- Evidence: 18 cargo tests and clippy clean; a scratch-repo scenario; `pf audit`
  on the real Connect repos, with no change to them (`git status` clean).

## 2026-10-19 — Sessions default to the factory; workspaces and memory

- Added the always-on rule `kit/.claude/rules/product-factory.md`, installed in
  every factory repo and module. It routes change requests through `/factory` and
  keeps knowledge in git. Gave the skill a `when_to_use` trigger.
- Multi-repo workspaces: the skill finds the descriptor in a parent folder.
  Worktrees live outside the workspace (`<ws>-factory/<area>/<job-id>`). Added
  `pf install module` for child repos (each nested repo is its own Claude Code
  project) and `pf rules sync` for path-scoped module rules.
- Memory: added `pf memory` and `/factory memory`, which promote auto memory
  into `AGENTS.md`, docs or rules, or `CLAUDE.local.md`, with approval. Auto
  memory is not loaded into subagents, so knowledge left there was invisible to
  factory roles ([ADR-010](../decisions/ADR-010-session-behaviour-and-memory.md)).
- Attribution: added `vcs.attribution: none`, and `pf install --no-ai-attribution`
  writes Claude Code's `attribution` setting, as required by Connect's
  no-AI-attribution rule.
- Added `doctor` checks for the rule, attribution, module installs, module rules,
  a `CLAUDE.md` that does not import `AGENTS.md`, and auto memory awaiting review.
- Added the Connect walkthrough (`docs/guide/07-workspace-connect.md`) and the
  draft descriptor (`docs/guide/examples/connect.factory.yaml`).
- Evidence: 15 cargo tests and clippy clean; an end-to-end scratch-workspace run;
  `pf memory` on the real Connect auto memory (read-only).

## 2026-10-19 — All tooling ported to one Rust CLI, `pf`

- Following the operator's Rust-by-default rule ([ADR-009](../decisions/ADR-009-rust-tooling.md)),
  replaced the Python guard hook, `docs-map.py`, `guard_test.py`, and the bash
  `install.sh`, `doctor.sh`, `mirror.sh`, `export-claude.sh` and
  `render-diagrams.sh` with one Rust binary, `pf` (`cli/`). Added
  `pf dashboard`, so the dashboard no longer needs `python3 -m http.server`.
  `jq` is no longer needed.
- The hook command is now `pf guard`, and it **fails closed** when `pf` is not
  installed. `pf install project` migrates products that have the legacy Python
  guard and `docs-map.py`.
- Equivalence evidence: guard decisions and messages identical to the Python
  original on 117 inputs (73 policy cases plus 44 edge cases). Docs-map output
  byte-identical except the "Generated by" line. Failure reports identical on a
  broken tree.
- Evidence: `cargo test` 10/10, `cargo clippy` 0 warnings; end-to-end runs of
  every subcommand on scratch setups, including bundle restore and the exact
  hook command with and without `pf` on PATH.
- **Migration for existing installs:** `cargo install --path cli`, then
  `pf install user` and `pf install project <path>` for each product, and commit
  the product diff.

## 2026-10-19 — All docs under `docs/`, live docs map, rendered diagrams

- **Rule (CONSTITUTION §1, v1.1):** documentation lives under `docs/` in one
  folder per area, each with a README. READMEs link to their parent, documents and
  subfolders. Every document links back to its folder README. `docs/README.md`
  carries a map of the whole tree. This applies to the factory repo and to every
  product.
- Moved `CONSTITUTION.md` → `docs/governance/`, `CONTRIBUTING.md` →
  `docs/development/`, `CURRENT_STATE.md`, `IMPLEMENTATION_STATUS.md`,
  `ROADMAP.md` and `EVOLUTION_LOG.md` → `docs/status/`, `adapters/` →
  `docs/adapters/`, and `operator/` → `docs/operator/`. Rewrote every relative
  link. The kit carries the constitution as a link, and the installer copies it
  into products with the repo-only routing block stripped.
- Replaced `scripts/check-docs.py` and `DOCUMENT_INVENTORY.md` with
  `kit/.factory/docs-map.py`, which generates and checks routing blocks, the map,
  missing READMEs, broken links and orphans. It is installed into products as
  `.factory/docs-map.py`, used by agents (protocol), reviewer-code, `/factory init`
  and `doctor.sh`.
- GitHub showed "Unable to render rich display". All 7 Mermaid diagrams were
  valid (Mermaid 10.9.8 and 11.17.2), so the failure was GitHub's live renderer
  (`viewscreen.githubusercontent.com`) not loading. Diagrams are now
  `docs/assets/diagrams/*.mmd` rendered to committed SVGs by
  `scripts/render-diagrams.sh`, with a `--check` freshness gate.
- Evidence: `docs-map.py --strict` OK (62 docs); `render-diagrams.sh --check` OK;
  `guard_test.py` 73/73; installer smoke test, including `docs-map.py` inside the
  installed product.

## 2026-10-19 — Documentation set aligned with the operator's repo conventions

- Restructured `docs/` to match the Daria Connect repositories. Added
  `docs/README.md` as a router; `guide/` (getting started, commands, reading a
  job, troubleshooting, glossary, example `AGENTS.md`); `architecture/` (plus
  `JOB_LIFECYCLE.md`); `components/` (COMP-001…013 and `IMPLEMENTATION_STATUS.md`);
  `development/` (00–05, 09, 16 link map); and `decisions/` (ADR-001…008).
  Added root `CURRENT_STATE.md` and `CONTRIBUTING.md`, and a generated
  `DOCUMENT_INVENTORY.md`.
- Added `scripts/check-docs.py` (links, Status headers, inventory) as a gate.
- Fixed the coordinator lane table: a trivial bug no longer skips reproduction.
  Bugs always run `repro` first (Hermes rule).
- Evidence: `check-docs.py` passes; `guard_test.py` 73/73; `bash -n` passes.

## 2026-10-19 — v1: Product Factory founded

- Merged the Hermes factory philosophy (`Mat-Agh/Hermes` @ `d458700`) with the
  Claude Code software-factory pattern (ZazenCodes, `ctoaIC4LHmI`). The design
  record is in `docs/architecture/SOURCES.md` (then `docs/02-sources-and-decisions.md`).
- Added the `/factory` coordinator skill with lanes per type and tier, 10 role
  agents, the agent protocol, the descriptor template, the backlog and the
  dashboard.
- Added the guard hook, which blocks merge, tag, release, force-push, non-factory
  pushes, `--no-verify` and discarding work. Evidence: `tests/guard_test.py`, 73/73
  passing.
- Added the operator profile (`operator/USER.md`), installers (project copy,
  user symlinks), `doctor.sh`, `mirror.sh` and `export-claude.sh`.
- The pipeline has not run on a real product yet. That is the Phase 1 exit
  criterion.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Status](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
