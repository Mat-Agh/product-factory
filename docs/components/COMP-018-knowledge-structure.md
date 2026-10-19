# COMP-018 — Knowledge structure per stack

**Status:** Implemented — templates distilled from the five Connect repositories; `pf knowledge plan` run read-only on all of them; init verified on scratch repos (2026-10-19)
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

The shape is the one already used across Daria Connect. The factory installs
it wherever it is missing, keeps it true through every job, and refreshes the
stack knowledge against upstream changes.

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
| `spring-kotlin-service` | Back-End, Federation | `01` service architecture, `02` Kotlin, `03` clean-architecture layers, `04` persistence, `05` events and messaging, `06` security contracts, `07` testing and quality, `08` build and CI, `10` cryptography; `api/` (openapi, asyncapi, schemas), `data/`, `security/`, `operations/`, `deployment/` |
| `kmp-android-client` | Client | `01` KMP module structure, `02` Kotlin, `03` presentation (MVI), `04` data and realtime, `05` library stack, `07` Android development, `10` testing/performance/security, `11` Gradle build, `12` client cryptography, `ANDROID_ACCEPTANCE_TEST.md`, `DEV_SETUP.md`; `data/`, `integrations/`, UX spec template |
| `typescript-web` | Front-End | `01` app architecture, `02` TypeScript, `03` UI and design system, `04` API contracts and clients, `05` security, `06` testing, `07` build and deployment, `08` library stack |
| `product-docs` | Product | `vision/`, `roadmap/`, `prd/` with a PRD template, `feature-matrix/`, `acceptance/`, `delivery/`, `governance/`, `metrics/`, `risks/`, `design/`, `data/` (it excludes the code handbook from the base) |
| `generic` | — | The base only |

The numbers `00`, `09`, `15` and `16` mean the same thing in every stack. Stack
chapters use the remaining numbers.

## Interface

```bash
pf knowledge stacks
pf knowledge plan  [--root <repo>] [--stack <s>] [--docs <dir>] [--modules]
pf knowledge init  [--root <repo>] [--stack <s>] [--docs <dir>] [--modules]   # creates missing files only
```

The stack and docs root come from the descriptor (`modules[].stack`,
`modules[].docs_root`), and flags override them. `plan` reports four things:
- **present**: the file exists;
- **covered by**: an equivalent exists, either the same chapter number on a
  matching topic or the same file name elsewhere in the docs;
- **missing**: it would be created;
- **numbering conflicts**: a template chapter's number is already used for
  another topic. Nothing is created for these, and the operator decides.

`init` fills `{{MODULE}}`, `{{PRODUCT}}`, `{{STACK}}` and `{{DATE}}`. It is
idempotent and never overwrites a file.

`/factory knowledge init` then runs a `docs` job that **fills** the new files
from the code and the existing docs. That covers the real commands, the real
canonical sample, the real principles, and a first history entry from `git log`.
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

## Connect today (read-only plan, 2026-10-19)

| Repo | Stack | Present | Missing | Conflicts |
|---|---|---|---|---|
| Back-End | spring-kotlin-service | 32 | 6 (history, sessions, OVERVIEW, ADR/feature templates) | 0 |
| Federation | spring-kotlin-service | 9 | 29 (`CURRENT_STATE.md` and most of the handbook and areas) | 0 |
| Client | kmp-android-client | 21 | 12 | 2 (`03` and `09` already used for other topics) |
| Front-End | typescript-web (`doc/`) | 6 | 23 | 0 |
| Product | product-docs (`product/`) | 16 | 11 | 0 |
| workspace root | generic | 1 | 20 | 0 |

## Known limitations

- Equivalence is by chapter number and topic word, or by file name. A doc with a
  different name and number is reported missing, and the operator maps it
  (record the mapping in the handbook README).
- Templates need the factory checkout. `pf` finds it as it does for `install`.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Components](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
