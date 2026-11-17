# Evolution log

**Status:** Active — append-only
**Last updated:** 2026-11-17

Append-only. One dated entry for each factory-level change: what changed, why,
and its evidence.

## 2026-11-17 — CLI 1.5.1 production routing and Connect installation

- Updated the Hermes operator guide and current-state record for five GPT/DeepSeek-only 9Router combos. Connect keeps its three non-secret `router:hermes-*` route aliases.
- Verified all five live combos answered through GPT fallback. DeepSeek accepted its API key but returned insufficient balance for Pro and Flash; the account needs credit before DeepSeek is usable.
- Released CLI 1.5.1 as a signed production build and installed the tagged version on Connect.
- Release gates passed: 39 Rust tests, Clippy, formatting, 80 guard cases, 157-document strict docs map, 31 Mermaid blocks parsed, and Factory doctor. The local CLI installation and Connect workspace doctor were healthy; Connect's three route aliases resolved through its private router config.

## 2026-11-16 — Version 1.5.0 release: Hermes coordinator and model fallback verification

- Verified provider fallback failover across 9Router and LiteLLM routers: when primary endpoints return 503 or hit quota limits, requests gracefully cascade to secondary verified models.
- Standardized Connect workspace configuration with Setup, Dev server, and Gates contracts in AGENTS.md, passing all `pf doctor` invariants.
- Enabled global user linking with `pf install user` to provide seamless `/factory` orchestration across Hermes, Claude Code, Codex, and Antigravity.
- Bumped CLI version to 1.5.0 with full test, clippy, formatting, and guard validation.

## 2026-11-16 — Hermes coordinator usage

- Documented the verified `hermes-deep` default, the Connect project chat
  command, the installed project-skill prompt, a read-only first conversation,
  and the path from bounded request to dashboard card and human PR review.
- Clarified that a configured route check is not a live model response and
  linked the Hermes live route probe from the operator guide.

## 2026-11-16 — Day-to-day usage guide

- Added English and Persian operator paths from installed product to dashboard,
  bounded job, evidence, PR review and cleanup. Linked them from the Atlas,
  handbook, repository entry point and documentation router. Corrected the
  getting-started version example to the current v1 CLI.
- The guide distinguishes a local provider prerequisite check from a live
  model request and keeps product credentials outside the repo.

## 2026-11-16 — Factory Atlas

- Added a sourced English/Persian [Factory Atlas](../atlas/README.md) and a
  self-contained bilingual, themeable browser view modelled on Connect's
  product Atlas. It maps the factory contract, work loop, information homes,
  operator surfaces and verified delivery limits without copying product facts
  into the installable kit.
- Linked it from the documentation router, repository entry point and dashboard
  component. The Markdown pages retain the detailed, source-linked facts.

## 2026-11-16 — Keep the dashboard responsive with large code graphs

- Deferred the 30 MB Connect workspace network until the Code network tab is
  opened. Served dashboard requests concurrently so a graph transfer cannot
  block status refreshes. The graph remains enabled and available on demand.
  Prepared CLI 1.3.5 for the installable kit.

## 2026-11-16 — Unified bilingual command centre

- Refactored the shipped board into Overview, Work, Code network and Systems
  sections with shared cards, metrics, responsive layout, browser-persisted
  dark/light theme and English/Persian controls.
- Clarified the graph canvas's 400-file draw limit, displayed drawn versus
  indexed counts and made symbol search cover the full index.
- Prepared CLI 1.3.4 so the installer embeds these assets.

## 2026-11-16 — Product Factory command centre and live graph watch

- Replaced the ambiguous board-only view with a Product Factory command centre:
  factory-state status, labelled review chips, request details, source-issue and
  pull-request links, artifacts, live workspace network and project-configured
  dashboard links are available from one local UI.
- Added a shipped Atlas page and an owner-maintained `atlas.json` link registry.
  The static server exposes only those deliberate read-only surfaces.
- Added `pf graph watch --modules`, which detects Git/source changes across all
  independent checkouts and refreshes every graph plus the combined workspace
  network. Prepared CLI 1.3.3.

## 2026-11-16 — Hermes and Honcho production operator guide

- Added a production guide for installing and operating a Factory project
  through Hermes with 9Router routes and optional guarded Honcho context.
- The guide makes the boundary explicit: `pf` collects only non-secret route
  aliases; machine credentials and shared-memory services stay in Hermes. It
  includes the Connect reinstall, verification and first-session procedure.

## 2026-11-16 — Preserve Product atlas hashes

- Product-docs installations no longer run the Factory docs-map writer over Product's authoritative pages. This keeps English/Persian Atlas content hashes and Product's own documentation gate intact while still importing missing generic knowledge. Prepared CLI 1.3.2.

## 2026-11-15 — Clean generated role files

- Runtime generation trims the extra trailing blank line from role files so a new child-repository install passes `git diff --check`. Prepared CLI 1.3.1 for the final `v1` rebuild.

## 2026-11-15 — Code network context and controls

- Local graph views now show bounded source previews and directed extracted relationships for files and symbols. Workspace graph data retains symbol links and relation names while keeping repository membership distinct.
- Added search, focus, pan, zoom slider and buttons, reset and keyboard controls to both brain-cloud and thought-sphere layouts. Verified against Connect's multi-repository graph.
- Prepared CLI `1.3.0` for a fresh `v1` production release.
- Rebuilt signed `v1` from the reviewed main commit with CLI 1.3.0 and a checksum-verified Linux binary.

## 2026-11-15 — Respect adopted documentation layouts

- Project entrypoints no longer tell repositories with `doc/` or `product/` to move documentation under `docs/`. Installation replaces only that obsolete Factory paragraph; doctor checks the inferred documentation root.

## 2026-11-15 — Single v1 production release

- Consolidated the published production line under signed `v1`, retaining the tested CLI 1.2.0 binary and its checksum. Historical versioned release records and tags were retired at the operator's request; Git commit history remains available.

## 2026-11-15 — Combined workspace code network

- `pf graph view --modules` joins independent repository indexes in a local 3D view, with repository and workspace hubs. The browser preserves hub links when limiting other visible edges.
- Source symbols and locations remain searchable; hub links represent topology rather than inferred cross-repository calls.
- Release candidate version: `1.2.0`.
- Published signed `v1.2.0` with a checksum-verified Linux binary; installed in Connect and generated the combined workspace view from the released CLI.

## 2026-11-15 — Topology-aware installation and code graph view

- Project installation now prepares missing stack-pack pages for each
  independent repository, regenerates its docs map, and stops automatic import
  when chapter numbers conflict. Existing pages and SQL migrations are not
  overwritten.
- When an installed project has model routes and Hermes on PATH, the installer
  trusts its project skill. Graphify now produces a local interactive 3D file
  network with symbol and source-location search; bounded CLI queries remain
  the agent navigation path.
- The shared protocol asks agents to document public code contracts and
  non-obvious state at definitions, then link lasting relationships from the
  owning repository's docs. Source locations support line-level navigation.
- Release candidate version: `1.1.0`.

## 2026-11-15 — Private local router configuration

- `pf providers` now reads a regular owner-private local router config when
  the launching shell does not export router credentials. Environment values
  still take precedence; the project descriptor and route file contain no key.
- Hermes project registration writes this local file from its existing private
  9Router credential. The Connect routes passed `pf providers check` without
  exported variables. Rust unit and integration tests and Clippy passed.
- Prepared CLI version `1.0.1` for the next patch release. Connect received
  the reviewed source revision as a local installation; published release
  status depends on the human merge, signed tag and release procedure.

## 2026-11-10 — Consolidated Product Factory v1 release

- Consolidated the installer, knowledge capture and CLI usage updates into
  a single v1.0.0 release. Existing job claims retain their evidence format.

## 2026-11-10 — Knowledge capture operator guide

- Aligned the CLI reference with the released command help and documented the
  kit upgrade, job report and evidence failure recovery.

## 2026-11-10 — Knowledge capture during delivery

- New jobs detect committed dependency signals and check that required lessons
  are captured in changed module docs. The code lens records its independent
  check before a PASS, and the gate rechecks capture on the pinned branch.
- Update `pf` and rerun `pf install project .` to enroll new jobs. Existing
  job claims retain their evidence format.
- Integration fixtures cover missing evidence, a script-only manifest edit,
  an undocumented dependency change, a spike with a flat docs root, and
  legacy job compatibility.

## 2026-11-10 — First standalone installer

- Aligned the CLI package with the first release tag. The release includes a
  Linux x86-64 binary with the project installer and stack knowledge embedded.
- The release notes record the tested installation paths and the remaining
  live runtime and hosted delivery checks.

## 2026-11-10 — Cross-runtime installation, model routes and Graphify

- Project installs generate Antigravity agents alongside Codex and Claude
  roles. Independent modules receive their own entry points. Hermes and Gemini
  CLI load the shared coordinator skill and project context.
- Project routes select direct Claude, GPT and Gemini models or 9Router-compatible
  aliases by role and effort. Setup saves model identifiers. Runtime credentials
  come from the host environment. The fixture verified routing, missing-key
  and authorization checks, stage ownership and artifact handoff.
- Graphify indexes existing code and refreshes in job worktrees as source is
  added, renamed or removed. A local fixture verified submodule separation,
  uncommitted queries and removal of a stale graph from an empty checkout.

## 2026-11-10 — Product-independent installation materials

- Replaced project-specific guides, descriptor examples and test fixtures
  with generic workspace examples. Removed project identifiers, local paths
  and service references from the documentation and operator profile.

## 2026-11-10 — Standalone project installer

- The Rust build embeds the generic project kit, constitution and stack
  library. Project installs, knowledge, audit and doctor use the bundled kit
  when the source checkout is absent.
- The project lock records a stable bundled revision. Personal operator
  profiles and user-level symlinks remain checkout-based.
- Release checks now include installing with only the binary in a clean
  environment and attaching checksums for the platforms actually verified.

## 2026-11-10 — Durable job execution and the operator board

- `pf job` serializes claims from chat, issues and the backlog. It enforces
  workspace capacity, prepares branches in the selected Git history, checks
  module scope, and advances from artifact verdicts on a pinned commit.
- The coordinator dispatches the native roles and uses the persisted stage and
  round to continue a job. Review and QA rework retain prior evidence and stop
  at the configured bound. The board adds queued items, module filtering,
  partial reviewer progress and blocking reasons.
- Integration fixtures verified parallel claims, separate worktrees, submodule
  base refs, out-of-scope changes, a blocked review, rework, QA, and a gated
  local delivery. Live runtime and hosted PR verification remain release checks.

## 2026-11-10 — Workspace discovery and Codex installation

- `pf audit --modules` and `pf knowledge plan --modules` discover submodules
  and supported stack directories before adoption. Project install drafts
  topology and docs settings, and `--modules` copies the kits to independent
  checkouts. The coordinator and doctor flag an unreviewed descriptor.
- The installer generates Codex custom agents and configures Claude agents
  from the canonical role files. Deep roles use high effort and build roles
  use medium. The descriptor can select separate models in each runtime.
- The read-only workspace plan detected all five modules. A docs-only copy
  verified install, local Codex skill discovery, high/medium TOML routes and
  an idempotent knowledge import. Front-End and Product kept their handbook
  layouts. Live jobs in both runtimes and five handbook collisions remain
  open for the product pilot.

## 2026-10-20 — Inline Mermaid diagrams

- **Why.** Rendered SVGs were static, not resizable, partly dropped by IntelliJ, and
  unreadable to agents. The operator asked for inline Mermaid ([ADR-014](../decisions/ADR-014-inline-mermaid-diagrams.md)).
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
  agent instructions, memory, safety, workspaces). Sources: existing stack handbooks,
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
  Plan and import on copies of five module repos: every added link resolved,
  reruns changed nothing; originals untouched.

## 2026-10-19 — Real stack knowledge, imported and merged; gap audit

- **Why.** The first knowledge templates were headings and comments. The operator
  asked for the stack knowledge itself to be imported where missing, and for
  existing docs to receive what they lack.
- **Knowledge packs.** Rewrote every stack overlay with real content distilled
  from generic stack practices: Spring/Kotlin (principles,
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
  Plan and import ran on copies of five module repos in a temporary
  directory: every added link resolved and a second run changed nothing. The
  originals were not touched.

## 2026-10-19 — Knowledge structure per stack

- Documented a shared handbook structure: `CURRENT_STATE.md`, a docs router, a numbered handbook (`00`
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
  `CONTRIBUTING.md` and `INSTALL.md` at the root.
- Evidence: 20 cargo tests; the read-only workspace plan is recorded in
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
- Workspace audit (read-only): the branch conventions in all five repos reject
  `factory/<job-id>`, a BLOCKER, which the adopted descriptor resolves with
  `{kind}/{job-id}`. Also found: Conventional Commits, no AI attribution,
  approval-triggered auto-merge (compatible), inactive `.githooks`, bypass
  switches, three rule mirrors, `CLAUDE.md` files without `.md`, Hermes
  references, and a gitignored plaintext token. The reconciliation is in
  `docs/guide/07-multi-repo-workspaces.md`.
- Evidence: 18 cargo tests and clippy clean; a scratch-repo scenario; `pf audit`
  on a product workspace, with no change to them (`git status` clean).

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
  writes Claude Code's `attribution` setting, as required by the adopted workspace’s attribution policy.
- Added `doctor` checks for the rule, attribution, module installs, module rules,
  a `CLAUDE.md` that does not import `AGENTS.md`, and auto memory awaiting review.
- Added the workspace walkthrough (`docs/guide/07-multi-repo-workspaces.md`) and the
  draft descriptor (`docs/guide/examples/workspace.factory.yaml`).
- Evidence: 15 cargo tests and clippy clean; an end-to-end scratch-workspace run;
  `pf memory` on an existing auto-memory directory (read-only).

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

- Restructured `docs/` to use a consistent repository handbook. Added
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
