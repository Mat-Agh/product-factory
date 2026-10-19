# Product Factory

An agentic product factory. You give it an idea or a request. It produces a
reviewed, tested, documented, launch-ready product increment as a pull
request. **You merge.**

![The Product Factory pipeline: idea to human merge, release and retro](docs/assets/diagrams/pipeline.svg)

<sub>Diagram source: [`pipeline.mmd`](docs/assets/diagrams/pipeline.mmd) · re-render with `pf diagrams render`</sub>

It merges two designs. The first is the Hermes factory (`Mat-Agh/Hermes`): roles,
triage, bounded plans, evidence-based done, human merge, and truth that lives in
the repo. The second is the Claude Code software-factory pattern from the
ZazenCodes video: one orchestrator skill, file-driven subagents, parallel review
lenses, worktree-per-job and machine-readable verdicts. See
[docs/architecture/SOURCES.md](docs/architecture/SOURCES.md) and the
[decision records](docs/decisions/README.md).

## New here?

Read [docs/guide/00-overview.md](docs/guide/00-overview.md), then follow
[docs/guide/01-getting-started.md](docs/guide/01-getting-started.md). It goes
from an empty machine to your first factory PR, with every command and its
expected output. Terms are explained in the
[glossary](docs/guide/GLOSSARY.md). What works today is recorded in
[CURRENT_STATE.md](docs/status/CURRENT_STATE.md).

## Start on any machine or account

```bash
git clone git@github.com:Mat-Agh/product-factory.git "$HOME/Work/ProductFactory"
cd "$HOME/Work/ProductFactory"
cargo install --path cli         # builds pf, the factory's Rust CLI (needs a Rust toolchain)
pf install user                  # /factory + agents in every project, your operator profile imported
pf install project ~/Work/MyApp  # put the kit inside a product repo (commit it there)
pf doctor ~/Work/MyApp           # verify
```

Then, in Claude Code inside the product repo:

```
/factory init                       draft .factory/factory.yaml from the repo
/factory idea <idea>                discovery → PRD → design concept → architecture → roadmap (docs PR)
/factory <request>                  triage and run the right lane to a PR
/factory issue <n|url>              same, from a GitHub/GitLab issue
/factory next                       first unchecked line of .factory/backlog.md
/factory status                     what is running, what needs you
/factory rework <job> <note>        send a job back with your feedback
/factory release [version]          readiness brief + launch kit (release PR)
/factory retro [job]                propose factory/AGENTS.md improvements
/factory memory                     promote what Claude remembered into the repo (with approval)
/factory clean                      remove worktrees of merged/closed jobs
```

The dashboard is `pf dashboard`, then open
<http://127.0.0.1:8000/dashboard.html>.

Codex, Cursor, Gemini or Hermes can run the same roles. See
[docs/adapters/](docs/adapters/README.md).

## What is where

| Path | What |
|---|---|
| [docs/](docs/README.md) | **All documentation**, one connected tree with a generated map. Start at [docs/README.md](docs/README.md) |
| [docs/governance/CONSTITUTION.md](docs/governance/CONSTITUTION.md) | The rules. Everything else applies them |
| [docs/guide/](docs/guide/README.md) | Operator handbook, from zero to the first PR |
| [docs/status/](docs/status/README.md) | Current state, implementation status, roadmap, evolution log |
| [docs/development/](docs/development/README.md) | How to change the factory, including [CONTRIBUTING](docs/development/CONTRIBUTING.md) |
| [kit/](kit/) | Payload installed into a product repo: the `/factory` skill, 10 role agents, the `pf guard` hook wiring, the descriptor template, backlog, dashboard, and the `AGENTS.md` section |
| [cli/](cli/) | `pf`, the factory's single Rust CLI: guard, docs map, diagrams, install, doctor, mirror, export, dashboard ([COMP-015](docs/components/COMP-015-pf-cli.md)) |

## Survives losing this account

Nothing the factory needs lives only in one account. The rules, roles, operator
profile and procedures are in this repo, which can be mirrored to several remotes
and an offline bundle. Each product's descriptor, PRDs, ADRs and backlog are in
the product's own repo. Work in flight is in branches and PRs. See
[docs/guide/05-portability-and-recovery.md](docs/guide/05-portability-and-recovery.md).
