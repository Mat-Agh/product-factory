# COMP-005 — Project descriptor

**Status:** Implemented — schema version 1
**Last updated:** 2026-10-19
**Source:** [`kit/.factory/factory.yaml`](../../kit/.factory/factory.yaml) (template)
**Decisions:** [ADR-007](../decisions/ADR-007-truth-in-product-repo.md)

## Purpose

`.factory/factory.yaml` is the only project-specific file the factory reads
besides `AGENTS.md`. The descriptor says **where** things are and **what the
policy is**. `AGENTS.md` says **how**: commands, conventions, docs map.
Duplicating either one in the other is a defect.

## Schema (version 1)

| Key | Type | Default | Used by | Meaning |
|---|---|---|---|---|
| `factory.version` | int | 1 | coordinator | Schema version. Bump it on breaking changes |
| `factory.source` | URL | factory repo | `/factory retro` | Where improvement PRs go |
| `product.name`, `product.one_liner` | string | — | product-manager, release-manager | Identity |
| `product.locales` | list | `[en]` | product-manager, designer, reviewer-ux, tester, release-manager | Copy, RTL and release-note pairs |
| `product.themes` | list | `[light]` | designer, reviewer-ui | Theme pairs to check |
| `layout` | `single-repo \| multi-repo` | single-repo | coordinator, tech-lead | Module model |
| `workspace_root` | path | `.` | coordinator | Multi-repo: the folder holding module clones |
| `modules[].area` | string | `app` | triage | Matches `area::`/`repo::` labels |
| `modules[].path` | path | `.` | coordinator | Module clone, relative to `workspace_root` |
| `modules[].owner_role` | role | engineer | tech-lead | Who implements there |
| `modules[].agents_md` | path | `AGENTS.md` | every agent | The module's live instructions |
| `modules[].gates` | `see AGENTS.md` or list | see AGENTS.md | engineer, reviewer-code, tester | Quality gates |
| `modules[].tests` | `yes \| no` | yes | product-manager, engineer, tester | `no` means the module forbids unit tests, so acceptance is manual and documented |
| `modules[].vcs` | map | — | coordinator | Per-module override of `vcs` (multi-repo) |
| `vcs.host` | `github \| gitlab` | github | coordinator | Which CLI opens the PR or MR |
| `vcs.remote` | string | origin | coordinator | Push target |
| `vcs.base_branch` | string | main | coordinator, tech-lead | PR target. Never committed to |
| `vcs.branch_prefix` | string | `factory/` | coordinator, guard | Job branches. The guard allows pushes only to `factory/*` |
| `vcs.open_pr` | bool | true | coordinator | `false` means stop at a local branch |
| `vcs.human_merge` | bool | true | — | Documentation of the constitutional rule. Not a switch |
| `docs.prd_dir`, `docs.adr_dir`, `docs.design_dir` | path | `docs/product`, `docs/adr`, `docs/design` | product-manager, tech-lead, designer | Where durable outputs land in the branch |
| `docs.changelog` | path | `CHANGELOG.md` | engineer, release-manager | Changelog |
| `docs.index` | path | `docs/README.md` | every agent | Link map instead of dump-reading |
| `design.source` | `repo \| figma:<key>` | repo | designer, reviewer-ui | Design source of truth |
| `design.require_source` | bool | false | designer | `true` means stop when a needed frame is missing |
| `surfaces.ui.dev_command` | string | see AGENTS.md | engineer, reviewers, tester | Must accept a port |
| `surfaces.ui.port_base` | int | 5100 | coordinator | Port scheme ([JOB_LIFECYCLE](../architecture/JOB_LIFECYCLE.md#isolation-per-job)) |
| `surfaces.ui.browser_tool` | `agent-browser \| playwright` | agent-browser | engineer, reviewers, tester | Browser CLI |
| `surfaces.api.contract` | path | none | tech-lead, reviewer-code | OpenAPI or other contract file |
| `acceptance.ledger` | path | none | tester | Where acceptance lines are recorded |
| `acceptance.gates` | list | `[]` | tester, gate | Extra product-level gates |
| `risk.escalate_paths` | globs | CI, migrations, auth, lockfiles, Docker, infra | tech-lead (gate), reviewer-security | Changes here always escalate |
| `release.drafts_dir` | path | `docs/launch` | release-manager | Announcement drafts. Never sent |
| `release.store_listing` | path | none | release-manager | Store metadata |
| `limits.max_resumes` | int | 2 | coordinator | Engineer resumes after round 1 |
| `limits.max_parallel_jobs` | int | 3 | coordinator | Concurrency cap |
| `models.deep`, `models.build` | string | opus, sonnet | — | Documentation of intent. Agent frontmatter is authoritative |

## Verification

`/factory init` writes it and marks guesses with `# verify`. `scripts/doctor.sh
<project>` checks that it exists and reads `vcs.base_branch` to check branch
protection. There is no schema validator yet (see
[IMPLEMENTATION_STATUS.md](../status/IMPLEMENTATION_STATUS.md)).

## Compatibility

Adding a key with a default is non-breaking. Renaming or removing a key, or
changing its meaning, is breaking: bump `factory.version`, describe the
migration in the evolution log, and update every agent that reads the key.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Components](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
