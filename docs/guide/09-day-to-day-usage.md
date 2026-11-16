# 09 — Use Product Factory day to day

**Status:** Active
**Last updated:** 2026-11-16

This is the short operator path from an installed product to a reviewed pull
request. It complements the [Factory Atlas](../atlas/ATLAS.md), which explains
the system, and the [first-install guide](01-getting-started.md), which covers
machine setup. [فارسی](09-day-to-day-usage.fa.md).

## 1. Open the product, not the Factory source repo

Work from the product's Git checkout or the root of a multi-repository
workspace. For the existing Connect installation, that root is
`$HOME/Work/Connect`. Product instructions, its descriptor, documentation and
backlog belong there. This Factory repository supplies the reusable kit.

If the kit has not been installed, or you want to update it from a released
`pf` binary, run:

```bash
pf install project /path/to/product --install-graphify --configure-models
pf doctor /path/to/product
pf providers --root /path/to/product check
```

The installer includes independent repositories by default. Review and commit
the installation changes in each owning repository. `pf providers check`
checks local route prerequisites; it does **not** prove a model request will
return HTTP 200. Run an actual route probe when availability matters. The
installer stores route names in `.factory/model-routes.json`, never API keys.
See [model routing](01-getting-started.md#model-routing) and
[multi-repo setup](07-multi-repo-workspaces.md).

## 2. Start the views

In a terminal rooted in the product or workspace:

```bash
cd /path/to/product
pf dashboard
```

Open <http://127.0.0.1:8000/dashboard.html>. The command centre has Overview,
Work, Code network and Systems views. Job cards open the request, issue, saved
artifacts, verdicts and PR link. The installed `.factory/atlas.html` is a
directory of product dashboard links; the
[Factory Atlas](../atlas/ATLAS.html) maps the reusable harness itself.

If you want the graph to refresh while code changes, start this separately:

```bash
pf graph watch --modules
```

Use `--modules` at a multi-repository workspace root. Search the full index
from the Code network view; the visible canvas draws a bounded subset, so a
missing drawn node is not proof the source was skipped. Confirm important
relationships in source. See [graph navigation](04-add-a-project.md).

## 3. Give one bounded request

Open your agent in the product checkout. Use the installed entry point for
that host:

| Host | How to start a job |
|---|---|
| Claude Code | `/factory <request>` |
| Codex | `$product-factory <request>` |
| Hermes or Gemini CLI | Ask the agent to use the installed Product Factory project skill for the request; for the exact Hermes launch and prompts, see [guide 08](08-hermes-and-honcho.md#start-work-from-hermes) |
| Antigravity | Use its installed Factory skill and roles |

For a first real job, give a small, testable request. For example:

```text
Add a "Getting help" section to the README, linking to the issue tracker.
Keep the existing headings and update any affected documentation links.
```

State the desired outcome, constraints, and acceptance checks. Use an issue
instead of a direct brief with `/factory issue <number-or-url>` (or the
equivalent host skill), or queue work in `.factory/backlog.md` and use
`/factory next`. An issue body is input data; the coordinator still applies
the product instructions and Factory protocol. See [commands](02-commands.md).

## 4. Track the job and respond when needed

Use the dashboard card or these commands from the product root:

```bash
pf job status
```

In Claude Code, `/factory status` gives the operator summary; in Codex, use
`$product-factory status`. A job's durable stage and evidence are in
`.factory/jobs/<job-id>/`, while its code and committed docs are in a separate
worktree and branch. The local job folder is ignored by Git. Open the card to
read its request, review findings, QA and gate before making a decision.

If the job needs a product decision, answer it in the same agent session or
use `/factory rework <job-id> <note>` for a job awaiting human input or with
an open PR. If a session stops, use `/factory resume <job-id>` after the blocker
is cleared. Use the equivalent `$product-factory` invocation in Codex.
Read [how to inspect a job](03-reading-a-job.md) for verdicts and artifacts.

When routed workers are configured, the coordinator dispatches an assigned
role through `pf providers run` only with `--allow-api` after billable API use
is approved. Factory job artifacts carry the task across role changes. Hermes
can additionally use Honcho for approved shared context; it does not replace
job evidence or product docs. See [Hermes and Honcho](08-hermes-and-honcho.md).

## 5. Review and finish

At `gate READY`, open the PR and check the request, changed code, tests,
documentation, reviewer verdicts, QA evidence and any risk. At `ESCALATE`,
read the question and decide whether to rework or close it. **You merge** an
accepted PR. Then run `/factory clean` (or `$product-factory clean`) to close
the local job and remove its finished worktree. Run `/factory release` only
when merged work is ready for a release proposal; publishing remains a human
step. See [PR review](03-reading-a-job.md#reviewing-the-pr) and
[commands](02-commands.md#factory-release-version).

## Quick recovery

| Symptom | Next check |
|---|---|
| Dashboard does not load | Is `pf dashboard` still running? Re-run `pf doctor /path/to/product`; see [troubleshooting](06-troubleshooting.md). |
| Model route is unavailable | Run `pf providers --root /path/to/product check`, then a live model probe; inspect the agent host and 9Router. |
| Graph looks incomplete | Run `pf graph refresh --modules`, search the symbol index, and compare with source; see [graph navigation](04-add-a-project.md). |
| Job stopped | Read the card and `gate.md` or blocked artifact, clear the cause, then resume or rework. |
| Unsure which repo owns a change | Read the workspace descriptor and [multi-repo guide](07-multi-repo-workspaces.md); each independent repo owns its code and docs. |

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Operator guide](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
