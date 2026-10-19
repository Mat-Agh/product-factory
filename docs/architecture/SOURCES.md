# Research sources and design synthesis

**Status:** Active — research record
**Last updated:** 2026-10-19

The factory combines two designs. This page records what came from each source,
where they conflicted, and why each conflict was resolved the way it was.

## Source A — the Hermes factory (`Mat-Agh/Hermes`, 2026-09)

This is your own factory, built on Hermes Agent with 9Router and GitLab. What it
contributes:

- **Philosophy.** Truth stays in the repository. Do not duplicate docs, link
  them. Make small bounded changes. Separate roles. Evidence over confidence
  ("compile is not done"). Fail closed. Humans merge. Issue text is untrusted.
- **Crew.** A coordinator that runs scrum triage (understand, dedupe, size,
  prioritise, split), plus product manager, designer, technical lead, software
  engineer, reviewer and tester.
- **Flow.** Issue → triage → first role → Tech Lead bounded plan with an
  acceptance list → engineer → independent reviewer → tester → Tech Lead opens
  the MR → human merges. Bugs are reproduced before anyone fixes them. Designs
  stop when the Figma frame is missing.
- **Docs-first tiers** (trivial, standard, cross-cutting) and the **docs-sync
  gate**: a behaviour change with no doc change fails review.
- **Project descriptor.** One file per project makes the crew project-agnostic.
- **Role-class model routing.** Deep models for thinking and review, build
  models for execution, and an independent reviewer.
- **Operations.** Portable git home, install and sync knowledge scripts, an
  evolution log, health checks, and bounded retries with an explicit `@user`.

## Source B — "Build Your Own AI Software Factory with Claude Code"

ZazenCodes, YouTube `ctoaIC4LHmI`, October 2026. The companion source is
`zazencodes/zazencodes-season-3/src/software-factory-claude-code`, together with
a community gap review (`mkelley33/zazencodes-season-3#7`). What it contributes:

- **One orchestrator skill** (`/factory`). The session that runs it becomes the
  orchestrator and is the only writer of job state.
- **Subagents defined as markdown files** with their own model, tools and a
  single output file.
- **Files, not conversation.** Every agent reads its context from the job folder
  and writes one file whose **first line is the verdict**.
- **Parallel review lenses** (security, UX, UI, code) with **WAIT FOR ALL**. The
  same builder is resumed by name (`SendMessage`), with a cap on resumes.
- **An approver with explicit escalation rules**: money, auth, deploy/CI,
  migrations, email, dependencies, product decisions.
- **One git worktree per job** with per-job ports and browser sessions, so
  several sessions run jobs in parallel. Jobs are claimed atomically with
  `mkdir`.
- `backlog.md` with `/factory next`, `approve`, `rework`, and a polling
  dashboard.
- Suggested next steps: issue triggers, observability.
- From the community review: a `PreToolUse` guard hook for "never merge",
  branch protection, a `Surface` line that skips UI/UX lenses when there is no
  page, a `clean` command, and least-privilege tools for read-only agents.

## Conflicts and resolutions

| Question | Hermes | Video | Product Factory | Why |
|---|---|---|---|---|
| Who merges? | human always | approver auto-merges to `main` | **human always.** The gate opens a PR, and escalations open a draft. | Your non-negotiable boundary. The video's own community fork moved to PRs and a merge guard too. |
| Who writes the spec? | PM for packets, Tech Lead for the plan | spec-writer | **PM writes the spec, Tech Lead writes the plan** | Keeps Hermes' "what" and "how" separate. The video's single spec mixes them. |
| Reviewer | one independent reviewer | four parallel lenses | **four lenses, conditional on surface**, plus a separate QA stage | Lenses catch more, and QA keeps "review is not acceptance". |
| Final gate | Tech Lead sanity gate | approver agent | **tech-lead in gate mode**, using the approver's escalation list | No eighth role. The Tech Lead already owns the MR in Hermes. |
| Triage | coordinator scrum triage | none | **coordinator triage first, always** | Dedupe and splitting prevent wasted runs. |
| Bugs | QA reproduces first | none | **repro stage** | Fixing before reproducing is fake-done. |
| Ideas to market | out of scope (engineering gateway) | out of scope | **idea lane** (discovery → PRD → concept → ADR → roadmap) and **release lane** (launch kit, drafts only) | You want ideas to market-ready products. The irreversible launch steps stay human. |
| Runtime | Hermes profiles, Multica board, cron | Claude Code subagents | **Claude Code first**, with role files that any agent can follow (adapters) | Officially supported runtime, and nothing locked to it. |
| State | Hermes Kanban or Multica | `job.json` per job | **`job.json` per job, PRs and issues as durable record** | Survives account loss. The git host is the ledger. |
| Model routing | 9Router combos per profile | opus/sonnet per agent | **opus for deep roles, sonnet for build roles**, in frontmatter | The same role classes, expressed natively. |
| Docs | docs-sync map | "update docs" in the spec | **docs-sync gate in reviewer-code**, with docs paths in the descriptor | A missing doc update fails review. |
| Self-improvement | evolution log, curator review | Basili "reuse of experience" | **`/factory retro` → PR to this repo**, plus a dated evolution log | Improvements are proposed, never silent. |

## Deliberately left out (for now)

- Cron and issue polling (Hermes Door B). This waits until the manual loop has
  proven itself. See [ROADMAP.md](../ROADMAP.md).
- Mixture-of-agents and cross-vendor routing. These are on the roadmap as an
  optional extra review lens.
- Auto-merge in any form.

<!-- documentation-routing:start -->
## Documentation routing

- [Parent README](README.md)
- [Governing agent guide](../../AGENTS.md)
<!-- documentation-routing:end -->
