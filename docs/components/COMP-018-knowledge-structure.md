# COMP-018 — Knowledge structure per stack

**Status:** Implemented — real stack knowledge packs distilled from the five Connect repositories; plan and import (files and section merge) verified on copies of all five, never on the originals (2026-10-19)
**Last updated:** 2026-10-19
**Source:** [`kit/knowledge/`](../../kit/knowledge/), [`cli/src/knowledge.rs`](../../cli/src/knowledge.rs), `/factory knowledge` in [`SKILL.md`](../../kit/.claude/skills/factory/SKILL.md), knowledge upkeep in [`protocol.md`](../../kit/.claude/skills/factory/protocol.md)
**Rule:** [CONSTITUTION §1](../governance/CONSTITUTION.md)
**Decisions:** [ADR-012](../decisions/ADR-012-knowledge-structure-per-stack.md)

## Purpose

Every repo the factory runs keeps the same kind of knowledge, in the same shape:
- what it is now;
- how it got there;
- how to develop on its stack, including the tools, commands and living code
  samples;
- what was decided, and what is built.

The shape is the one already used across Daria Connect. The factory imports it
wherever it is missing, adds what an existing doc lacks, keeps it true through
every job, and refreshes the stack knowledge against upstream changes.

## Two kinds of template

| Kind | Marked by | Examples | On import |
|---|---|---|---|
| **Knowledge pack** | A `**Source:** Product Factory knowledge pack (<stack>)` line | Handbook chapters (`00`–`16`), `ANDROID_ACCEPTANCE_TEST.md`, `REVIEW_CHECKLIST.md`, `api/README.md` | Created with the full content when missing. When an equivalent file exists, the sections it lacks are appended |
| **Scaffold** | No `Source` line | `CURRENT_STATE.md`, `DEVELOPMENT_LOG.md`, `IMPLEMENTATION_STATUS.md`, area READMEs, routers, `CICD.md`, `CONTRIBUTING.md`, ADR/feature/PRD templates | Created when missing, never merged. They describe one project, so the docs job fills them from the code |

The knowledge packs hold real content: the engineering principles, layer rules,
naming, persistence, outbox/inbox, security, cryptography, testing policy, build
and CI practice, library stacks and code samples of each stack, written
generically with Connect-specific names removed.

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

**Stack overlays** (`pf knowledge stacks`):

| Stack | From | Adds |
|---|---|---|
| `spring-kotlin-service` | Back-End, Federation | Its own `00`, `09`, `15`, `16` and handbook README; `01` service architecture, `02` Kotlin, `03` clean-architecture layers, `04` persistence, `05` events and messaging, `06` security contracts, `07` testing and quality, `08` build and CI, `10` cryptography; `api/` (openapi, asyncapi, schemas), `data/`, `security/`, `operations/`, `deployment/` |
| `kmp-android-client` | Client | Its own `00`, `09`, `15`, `16` and handbook README; `01` KMP module structure, `02` Kotlin, `03` presentation (MVI), `04` data and realtime, `05` library stack, `07` Android development, `10` testing/performance/security, `11` Gradle build, `12` client cryptography, `ANDROID_ACCEPTANCE_TEST.md`, `DEV_SETUP.md`; `data/`, `integrations/`, UX spec template |
| `typescript-web` | Front-End | `01` app architecture, `02` TypeScript, `03` UI and design system, `04` API contracts and clients, `05` security, `06` testing, `07` build and deployment, `08` library stack; its own `00`, `09`, `15`, `16`, handbook README and `REVIEW_CHECKLIST.md` |
| `product-docs` | Product | `vision/`, `roadmap/`, `prd/` with a PRD template, `feature-matrix/`, `acceptance/`, `delivery/`, `governance/`, `metrics/`, `risks/`, `design/`, `data/`; a product router, `development/01-product-documentation.md` (ownership, writing rules, the evidence model) and its own `16` (it excludes the code handbook from the base) |
| `generic` | — | The base only: generic `00` principles, `16` link map, `CICD.md` and `CONTRIBUTING.md` with real guidance; `09` and `15` stay scaffolds |

The numbers `00`, `09`, `15` and `16` mean the same thing in every stack. Stack
chapters use the remaining numbers.

## Interface

```bash
pf knowledge stacks
pf knowledge plan  [--root <repo>] [--stack <s>] [--docs <dir>] [--modules]
pf knowledge init  [--root <repo>] [--stack <s>] [--docs <dir>] [--modules] [--files-only]
```

The stack and docs root come from the descriptor (`modules[].stack`,
`modules[].docs_root`), and flags override them. `plan` changes nothing and
reports:
- `+` **missing**: no equivalent exists, so the file will be imported;
- `=` **plays**: an equivalent exists under another name and covers the pack;
- `~` **missing sections**: an existing (or equivalent) knowledge file lacks some
  of the pack's `##` sections, listed by heading;
- `!` **numbering conflicts**: the chapter number is used for another topic.
  Nothing is imported for these; the operator renumbers or records the mapping.

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
3. fixes links in what it wrote: a link that resolves stays, a handbook chapter
   is pointed at the repo's own chapter with the same number, and any other dead
   link becomes plain text.

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

## Connect today (plan and import on copies, 2026-10-19)

Run against copies of the repositories' docs in a temporary directory; the
originals were not touched.

| Repo | Stack | Present | Missing (imported) | Files missing sections (merged) | Conflicts |
|---|---|---|---|---|---|
| Back-End | spring-kotlin-service | 32 | 8 (history, sessions, OVERVIEW, ADR/feature templates) | 7 (`api/README`, `03`, `04`, `05`, `07`, `08`, `09`) | 0 |
| Federation | spring-kotlin-service | 9 | 31 (`CURRENT_STATE.md`, most of the handbook and areas) | 4 (`00`, `09`, `15`, `16`) | 0 |
| Client | kmp-android-client | 23 | 12 | 5 (`07`, `15`, `16`, `17`, acceptance) | 2 (`03`, `09` used for other topics) |
| Front-End | typescript-web (`doc/`, flat) | 9 | 19 | 4 (`01`, `04`, `07`, `16`) | 4 (`02`, `03`, `08`, `09`) |
| Product | product-docs (`product/`) | 16 | 15 | 0 | 0 |

Every link added by the import resolved, and a second run changed nothing.

## Known limitations

- Equivalence and coverage are heuristics on file names and headings, not on
  meaning. A section that covers a topic under an unrelated heading is merged
  again; the review note asks the docs job to delete it. A doc with an unrelated
  name and number is reported missing, and the operator maps it.
- Merged sections are generic. They are a starting point for the docs job, not
  verified facts about the repo.
- Templates need the factory checkout. `pf` finds it as it does for `install`.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Components](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
