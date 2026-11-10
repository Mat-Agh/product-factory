# COMP-018 — Knowledge structure per stack

**Status:** Implemented — stack pack imports verified in scratch workspaces
**Last updated:** 2026-10-19
**Source:** [`docs/knowledge/`](../knowledge/README.md) (the library), [`kit/knowledge/`](../../kit/knowledge/) (scaffolds and `pack.map`), [`cli/src/knowledge.rs`](../../cli/src/knowledge.rs), `/factory knowledge` in [`SKILL.md`](../../kit/.claude/skills/factory/SKILL.md), knowledge upkeep in [`protocol.md`](../../kit/.claude/skills/factory/protocol.md)
**Rule:** [CONSTITUTION §1](../governance/CONSTITUTION.md)
**Decisions:** [ADR-012](../decisions/ADR-012-knowledge-structure-per-stack.md), [ADR-013](../decisions/ADR-013-development-knowledge-library.md)

## Purpose

Every repo the factory runs keeps the same kind of knowledge, in the same shape:
- what it is now;
- how it got there;
- how to develop on its stack, including the tools, commands and living code
  samples;
- what was decided, and what is built.

The factory imports this structure
wherever it is missing, adds what an existing doc lacks, keeps it true through
every job, and refreshes the stack knowledge against upstream changes.

## Where the knowledge lives

| Part | Location | Holds |
|---|---|---|
| **Library** | [`docs/knowledge/`](../knowledge/README.md) | All general engineering knowledge, by language and stack: principles, Kotlin (language, Gradle, client: KMP/Android/iOS, backend: Spring), TypeScript web, Rust, API, security, product, CI/CD, agentic workflows |
| **Pack map** | `kit/knowledge/stacks/<stack>/pack.map` | Which library pages a repo of that stack imports, and where they land |
| **Scaffolds** | `kit/knowledge/base/`, `kit/knowledge/stacks/<stack>/` | Project-state files: current state, history, status, area READMEs, the handbook index, the link map, CI/CD, contributing, ADR/feature/PRD templates |

A pack map line is `<library page> <destination>`; a line ending in `/` imports a
whole library folder, skipping pages mapped explicitly. Stack chapters map onto the
numbered handbook (for example `kotlin/backend/spring/layers.md` →
`docs/development/03-clean-architecture-layers.md`); cross-cutting areas map to
`docs/development/knowledge/<area>/`.

Library pages (and any template with a `**Source:** Product Factory knowledge` line)
are **knowledge**: created when missing, and merged section by section into an
existing equivalent. Scaffolds are created when missing and never merged.

## The structure

**Base (every repo)**, with paths under the module's `docs_root`:

| Path | Holds | Kept true by |
|---|---|---|
| `CURRENT_STATE.md` (repo root) | What the repo owns, contains and depends on now | engineer, in the branch of any change that alters it |
| `CONTRIBUTING.md` (repo root) | Branches, commits, review, merge | when conventions change |
| `docs/README.md` | Router: start here, areas | docs changes |
| `docs/development/00-engineering-principles.md` | Invariants | convention changes |
| `docs/development/09-tools-scripts-and-commands.md` | Every command, with working samples | tool, script or version changes |
| `docs/development/15-code-samples-index.md` | The canonical living sample, plus task → sample | new or changed patterns |
| `docs/development/16-documentation-link-map.md` | Agent routing | doc or module changes |
| `docs/development/CICD.md` | Pipelines, branch and commit rules, approval, release | CI changes |
| `docs/architecture/` | `OVERVIEW.md`, `CONFORMANCE_LEDGER.md` | tech-lead on cross-cutting work and deviations |
| `docs/decisions/` | `ADR-NNN` from `_templates/ADR_TEMPLATE.md` | tech-lead |
| `docs/features/` | One contract per feature, `IMPLEMENTATION_STATUS.md`, `_templates/` | engineer and tester evidence |
| `docs/cross-project/` | Routers into sibling repos (link, never copy) | when contracts move |
| `docs/history/DEVELOPMENT_LOG.md` | Dated, newest-first, one entry per merged change | engineer, every job; release-manager compiles releases from it |
| `docs/sessions/` | Temporary dated handoffs, then promoted | anyone; promote or delete |

**Stack packs** (`pf knowledge stacks`; the full list is each `pack.map`):

| Stack | From | Imports from the library |
|---|---|---|
| `spring-kotlin-service` | Back-End, Federation | Spring principles (`00`), service architecture (`01`), Kotlin language (`02`), layers (`03`), persistence (`04`), outbox/inbox (`05`), security (`06`), testing (`07`), build and CI (`08`), tools (`09`), cryptography by layer (`10`), Gradle (`11`), samples (`15`); plus `principles/`, `api/`, `security/`, `cicd/`, `agentic/`, `rust/` |
| `kmp-android-client` | Client | Client principles (`00`), KMP source sets (`01`), Kotlin (`02`), MVI (`03`), data and realtime (`04`), library stack (`05`), iOS (`06`), Android (`07`), tools (`09`), testing (`10`), Gradle (`11`), client cryptography (`12`), samples (`15`), acceptance, setup, `architecture/APP_ARCHITECTURE.md`; plus the cross-cutting areas |
| `typescript-web` | Front-End | Web principles (`00`), architecture (`01`), TypeScript (`02`), UI (`03`), API client (`04`), security (`05`), testing (`06`), build (`07`), library stack (`08`), tools (`09`), samples (`15`), review checklist; plus the cross-cutting areas |
| `product-docs` | Product | Product documentation (`01`), requirements and delivery (`02`), repository model (`03`); plus `principles/`, `cicd/`, `agentic/`, `rust/` (it excludes the code handbook scaffolds) |
| `generic` | — | General engineering principles (`00`) and the cross-cutting areas |

The numbers `00`, `09`, `15` and `16` mean the same thing in every stack. Stack
chapters use the remaining numbers.

## Interface

```bash
pf knowledge stacks
pf knowledge plan  [--root <repo>] [--stack <s>] [--docs <dir>] [--modules]
pf knowledge init  [--root <repo>] [--stack <s>] [--docs <dir>] [--modules] [--files-only]
```

Flags override descriptor values. Without either, the planner uses local
Gradle, TypeScript and product handbook evidence, falling back to `generic`.
With `--modules`, it uses the workspace descriptor or discovers checked-out
Git submodules and stack directories with supported manifests. Same-stack
packages inherit the nearest handbook unless they carry their own
`AGENTS.md`. An unavailable checkout is reported before any workspace-wide
import. Pass an explicit `--root` to target a stack directory in a shared Git
history.

New handbooks use `<docs_root>/development`. An existing flat handbook with a
root link map stays flat. A product-docs module with its own top-level
`development/README.md` keeps that handbook in `development/`, independently
of the product area. `plan` reports:
- `+` **missing**: no equivalent exists, so the file will be imported;
- `=` **plays**: an equivalent exists under another name and covers the pack;
- `~` **missing sections**: an existing (or equivalent) knowledge file lacks some
  of the pack's `##` sections, listed by heading;
- `!` **numbering conflicts**: the chapter number is used for another topic.
  Resolve the filenames and update inbound links as part of adoption.

An existing file is **equivalent** when it has the same path, the same chapter
number on a shared topic word (in the handbook folder or flat in the docs root),
the same topic under another number (two or more shared topic words covering
half of them), or the same file name elsewhere in the docs.

A pack section is **covered** when the file has a `##`/`###` heading with the same
words, a subset or superset of them, half of them in common, or the same section
number and a shared word. Words match on a six-letter stem, so "Classify before
implementing" matches "Classification before implementation". Code fences are
ignored. `Related` and `Documentation routing` are never merged.

`init` does three things, idempotently:
1. creates every missing file with its full content, filling `{{MODULE}}`,
   `{{PRODUCT}}`, `{{STACK}}` and `{{DATE}}`;
2. appends the missing sections to existing knowledge files, inside
   `<!-- pf-knowledge:start … -->` / `<!-- pf-knowledge:end -->` with a note asking
   for review, placed before a generated routing block when there is one;
3. fixes links in what it wrote: a link between library pages follows the pack
   map to wherever its target landed (a created file or an existing equivalent);
   a link that resolves stays; a handbook chapter is pointed at the repo's own
   chapter with the same number; any other dead link becomes plain text.

Library pages are imported without the factory's generated routing block.

It never overwrites a file and never edits existing text. `--files-only` skips
step 2.

`/factory knowledge init` then runs a `docs` job that **adapts** the import to
the repo: real commands, the real canonical sample, scaffolds filled from the
code, merged sections kept, adjusted or deleted, and a first history entry from
`git log`.
`/factory knowledge refresh` checks upstream release notes for every library and
tool the handbook lists, and proposes updates. Version upgrades go as separate,
escalated jobs.

## Where it runs

| Who | How |
|---|---|
| `/factory init` step 8a and `/factory adopt` (class **initialize**) | plan, then init with approval |
| every role, per the protocol's knowledge-upkeep table | keeps state, history, tools, samples, library stack and status true in the same branch |
| reviewer-code | missing upkeep is **CHANGES** |
| release-manager | builds release notes from `DEVELOPMENT_LOG.md` |
| `pf audit`, `pf doctor` | report knowledge gaps per repo |

## Known limitations

- Equivalence and coverage are heuristics on file names and headings, not on
  meaning. A section that covers a topic under an unrelated heading is merged
  again; the review note asks the docs job to delete it. A doc with an unrelated
  name and number is reported missing, and the operator maps it.
- Merged sections are generic. They are a starting point for the docs job, not
  verified facts about the repo.
- The standalone binary carries the stack packs and knowledge library. A
  Factory checkout supplies the current editable versions when available.

## Local code graph

Graphify indexes supported files locally using code-only extraction. The
Factory skips its own runtime directories and indexes Git submodules
separately. It builds during install and job preparation, refreshes after
plan, build and release, and refreshes before a query to include uncommitted
changes. Empty projects are indexed when supported files appear. Graphs
live in the checkout's ignored `.factory/graphify/` directory. Source files
and the stack handbook remain authoritative. [Graph commands](../guide/02-commands.md)

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Components](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
