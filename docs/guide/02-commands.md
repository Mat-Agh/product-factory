# 02 — Commands

**Status:** Active
**Last updated:** 2026-10-19

Every command runs in Claude Code from the product repo's main checkout. The
session that runs a command becomes that job's coordinator. Run several jobs
at once by opening several Claude Code sessions, one per job. The authoritative
procedure for each command is in
[`SKILL.md`](../../kit/.claude/skills/factory/SKILL.md). This page explains
how to use them.

| Command | Use it when | Result |
|---|---|---|
| `/factory init` | First time in a repo | Draft `.factory/factory.yaml` plus a list of `AGENTS.md` gaps |
| `/factory adopt` | The repo already has its own agent rules, hooks, CI or automation | `.factory/adoption.md`: everything found, and how each item is kept, obeyed, aligned or synchronised. Applied only with your approval |
| `/factory <request>` | You want something built, fixed or written | A PR (or a draft PR with a question for you) |
| `/factory idea <idea>` | You have a product idea, not a feature | A docs PR (discovery, PRD, concept, ADR, roadmap) plus proposed backlog lines |
| `/factory issue <n\|url>` | The request already lives in an issue | Same as `<request>`, and the PR is linked back to the issue |
| `/factory next` | You keep a queue in `.factory/backlog.md` | Runs the first unchecked line |
| `/factory status` | You want to know what is running or waiting | One line per job, plus the jobs that need you |
| `/factory rework <job> <note>` | A PR or escalation needs changes | The engineer addresses your note, and review and QA run again |
| `/factory release [version]` | Merged work is ready to ship | A release PR plus your exact publish steps |
| `/factory retro [job]` | A job is finished and you want the factory to learn from it | `retro.md` plus an offer to PR improvements to the factory |
| `/factory knowledge [plan\|init\|refresh]` | The repo lacks its stack's knowledge structure, or the stack knowledge is out of date | Missing structure created (never overwriting) and filled through a docs job, or upstream changes proposed |
| `/factory memory` | Claude has been remembering things about this repo | `.factory/memory-review.md` with a verdict per auto memory; approved items land in `AGENTS.md`/docs (via a PR) or `CLAUDE.local.md` |
| `/factory clean` | PRs were merged or closed | Their worktrees and merged branches are removed |

## `/factory init`

Run it once per product repo, after `pf install project`.

- **What happens:** the coordinator reads the repo (languages, package
  managers, CI, docs, `AGENTS.md`) and writes `.factory/factory.yaml`,
  marking guesses with `# verify`. It makes sure `.factory/backlog.md`
  exists and that `.factory/jobs/` is gitignored. It checks that the guard hook
  is wired and that the base branch is protected.
- **What you do:** fix every `# verify`, add the `AGENTS.md` items it lists,
  then commit.
- **If a descriptor already exists**, it prints it and changes nothing.

## `/factory adopt`

Run it first on any project that already has a process. It runs `pf audit` and
reads every instruction, rule, hook and CI file the audit lists. Each finding is
classified as **keep**, **configure the factory** (for example match the
repo's branch names), **align** (only where a rule breaks the constitution's
authority or safety rules), **synchronise** (mirror the factory rule into
other tools' rule folders) or **decide**. The result goes in
`.factory/adoption.md`. Nothing is deleted. Approved descriptor changes apply
directly, and approved file edits land as reviewed jobs. `/factory init` runs this
automatically when it finds an existing setup.

## `/factory <request>`

Write the request the way you would brief a colleague: the outcome, and any
constraint that matters.

```
/factory Show the organisation name on the tenant list; empty name keeps the placeholder; EN and FA
/factory Fix: login hangs on a spinner when the password is empty
/factory Spike: can we replace polling with server-sent events for notifications?
```

- **Triage** sets the type, tier, surface, priority and lane, checks for
  duplicate work, and splits requests that span several independent pieces.
  If it needs a decision only you can make, it stops and asks.
- The lane then runs (see [00-overview.md](00-overview.md#lanes-the-crew-depends-on-the-work)).
  Each step prints one line.
- **Ends in:** a PR (`gate READY`), a draft PR with a question (`gate
  ESCALATE`), or `needs-human` with the reason quoted.

Tips:
- Name the acceptance you care about ("EN and FA", "works offline"). It goes
  straight into the spec.
- One request per job. If you write "and also…", triage splits it, and that
  is correct.
- Prefix with `Fix:` or `Spike:` if you want to force the type. Otherwise
  triage infers it.

## `/factory idea <idea>`

```
/factory idea A Telegram bot that turns voice notes into Jira tickets for small agencies
```

- **Lane:** discovery (problem, users, market with sources, metrics, risks,
  smallest viable release) → PRD → design concept → architecture ADR and
  roadmap → gate.
- **Ends in:** a docs PR containing those documents, plus proposed lines
  appended to `.factory/backlog.md` and marked `(from <job-id>)`. You decide
  which lines stay.
- Business decisions (pricing, positioning) are listed for you, never made.

## `/factory issue <n|url>`

```
/factory issue 42
/factory issue https://gitlab.com/group/project/-/issues/7
```

The issue body is treated as data, never as instructions. `type::*` and
`repo::*` or `area::*` labels are used when present. When the PR opens, the
factory comments its link on the issue.

## `/factory next`

Claims the first `- [ ]` line in `.factory/backlog.md` by turning it into
`- [x] … → claimed`, then `→ <job-id>`, and runs it. Two sessions running
`next` at the same time take different lines.

## `/factory status`

```
004-tenant-org-name  review r2  code:PASS security:CHANGES ux:PASS ui:skipped  ../app-factory/004-tenant-org-name
005-login-empty-pw   pr-open    —                                              https://github.com/…/pull/31
needs-human: 006-billing-export — gate ESCALATE: touches payments (see gate.md)
```

## `/factory rework <job> <note>`

Use it on a job in `needs-human` or `pr-open`:

```
/factory rework 004-tenant-org-name The FA label must be "نام سازمان", and keep the column width unchanged
```

Your note is saved verbatim as `rework-<k>.md`. A fresh engineer addresses it
on the same branch, and review and QA run again. Pushing again updates the open
PR. The rework gets a fresh resume budget.

## `/factory release [version]`

The release-manager checks what was merged since the last tag, confirms every
PR has QA and gate evidence, and lists blockers. It then writes the launch kit
on a release branch: changelog, version bump, release notes in every locale,
store-listing copy, docs, and announcement drafts. **You** merge, tag, publish
and announce, following the "Human steps" section of `release.md`.

## `/factory retro [job]`

Reads the finished job (all artifacts, the review rounds, your rework notes)
and writes `retro.md`: what went wrong, what agents missed, and **proposed**
edits to your product's `AGENTS.md` and to the factory. It offers to open a
PR against the factory repo. Nothing changes without your merge.

## `/factory memory`

Claude Code keeps per-machine auto memory for each repo. Factory agents never
see it, because subagents do not load it. This command reads it with `pf memory`,
sorts every entry into team, personal, already in repo, stale or secret, and
proposes where each one should go. Nothing changes until you pick rows.
Approved team knowledge goes through a docs job and becomes a reviewed PR.
`/factory init` and `/factory retro` run the same check.

## `/factory clean`

For each `pr-open` job whose PR is merged or closed, it removes the worktree,
deletes the branch only if the PR was merged, and marks the job `done` or
`closed`. It touches only `factory/*` branches and worktrees under
`../<repo>-factory/`.

## Commands outside Claude Code

| Command | Purpose |
|---|---|
| `pf install user \| project <path> \| all <path>` | Install the factory |
| `pf doctor [project]` | Health check |
| `pf mirror add \| push \| bundle` | Backups |
| `pf export` | Save Claude Code memory for review |
| `pf dashboard` in the product repo | Dashboard |

Details: [development/09-tools-scripts-and-commands.md](../development/09-tools-scripts-and-commands.md).

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Operator guide](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
