# Factory overview

**Status:** Active
**Last updated:** 2026-10-19

## Layers

```mermaid
flowchart TB
  subgraph Factory["Product Factory repo: portable and mirrored"]
    Constitution["CONSTITUTION.md: rules"]
    Operator["docs/operator/USER.md: who you are"]
    Kit["kit/: skill, agents, guard, templates"]
  end
  subgraph Machine["Any machine, any account"]
    UserCfg["~/.claude: links to the kit, imports USER.md"]
    Runtime["Agent runtime: Claude Code, Codex, others"]
  end
  subgraph Product["Each product repo: the source of truth"]
    Code["AGENTS.md, docs/ and code"]
    Descriptor[".factory/factory.yaml"]
    Backlog[".factory/backlog.md"]
    KitCopy[".claude/: installed kit copy"]
    Jobs[".factory/jobs/: runtime state, gitignored"]
  end
  subgraph Host["Git host"]
    Branches["Job branches"]
    Reviews["Pull requests, issues, branch protection"]
  end
  Kit -- "pf install project" --> KitCopy
  Kit -- "pf install user" --> UserCfg
  Operator --> UserCfg
  UserCfg --> Runtime
  KitCopy --> Runtime
  Runtime -- "reads" --> Code
  Runtime -- "reads" --> Descriptor
  Runtime -- "reads" --> Backlog
  Runtime -- "writes" --> Jobs
  Runtime -- "pushes job branch, opens PR" --> Branches
  Branches --> Reviews
  Reviews -- "human merges" --> Code
```

| Layer | Holds | Lifetime | Lost if… |
|---|---|---|---|
| Factory repo | rules, roles, procedures, operator profile | permanent, versioned | every mirror and bundle is gone |
| Product repo | product truth: PRDs, ADRs, contracts, code, descriptor, backlog | permanent, versioned | the product's remotes are gone |
| Git host | work in flight (branches, PRs), intake (issues), protection | per job | the host account is gone, but branches can be re-pushed from any clone |
| Job folder | per-job scratch: triage, spec, plan, reviews, QA, gate | until `/factory clean` | the machine is lost, which is acceptable because the PR body carries the summary and the durable docs are in the branch |
| Agent runtime | nothing durable | per session | the account is lost, which costs nothing because no durable state lives there |

## Runtime: one job

```mermaid
sequenceDiagram
  participant H as Human
  participant C as Coordinator session
  participant PM as Product manager
  participant TL as Tech lead
  participant E as Engineer
  participant RV as Four reviewers
  participant QA as Tester
  participant G as Git host
  H->>C: /factory request
  C->>C: claim job, write triage.md, create worktree and branch
  C->>PM: spec
  PM-->>C: spec.md READY
  C->>TL: plan
  TL-->>C: plan.md READY
  C->>E: build round 1
  E-->>C: build.md READY and commits
  loop up to 1 + max_resumes rounds
    C->>RV: review round N, in parallel
    RV-->>C: review files with verdicts
    alt any CHANGES
      C->>E: address round N
    end
  end
  C->>QA: acceptance
  QA-->>C: qa.md PASS
  C->>TL: gate
  TL-->>C: gate.md READY or ESCALATE, and pr.md
  C->>G: push branch, open PR (draft on ESCALATE)
  G-->>H: PR to review
  H->>G: merge, human only
```

## Contracts between components

- **Descriptor** (`.factory/factory.yaml`) says *where* things are and *what the
  policy is*: modules, VCS, docs paths, surfaces, risk paths, limits. It contains
  no conventions or commands. Those stay in `AGENTS.md`.
- **AGENTS.md** says *how*: setup, dev server, gates, conventions, docs map.
  Every agent reads it live, and the factory never copies it.
- **Artifacts** each have exactly one writer, and their first line is a verdict
  (table in `kit/.claude/skills/factory/SKILL.md`). The coordinator acts only on
  verdicts. A missing or malformed verdict stops the job.
- **job.json** has one writer, the coordinator. The dashboard and
  `/factory status` read it.

## Roles and models

| Role | Model | Writes | Never |
|---|---|---|---|
| coordinator (the `/factory` session) | session model (use Opus) | `triage.md`, `job.json`, exhausted-rounds `gate.md` | specs, plans, code, reviews |
| product-manager | opus | `discovery.md`, `spec.md`, PRD in branch | runtime code |
| designer | opus | `design.md`, design docs in branch | invented product behaviour, invented pixels |
| tech-lead | opus | `plan.md`, ADR in branch, `gate.md`, `pr.md` | implementation, merge |
| engineer | sonnet | code + docs commits, `build.md` | widening scope, pushing |
| reviewer-code | sonnet | `review-code.md` | fixing |
| reviewer-security | opus | `review-security.md` | fixing |
| reviewer-ux / reviewer-ui | sonnet | `review-ux.md` / `review-ui.md` | fixing |
| tester | sonnet | `repro.md`, `qa.md` | fixing, self sign-off |
| release-manager | opus | `release.md`, release files in branch | tagging, publishing, sending |

These follow Hermes' role classes. Thinking and gating roles use the deep model,
and execution roles use the build model. Security is deep because its misses
are the most expensive. To change a model, edit the agent's frontmatter in
`kit/` and re-install. A reviewer from a different model family (for example a
Codex review lens) is on the roadmap.

## Enforcement

| Rule | Prose | Mechanical |
|---|---|---|
| Humans merge, tag, release | CONSTITUTION §5, every role | guard hook blocks `git merge`, `gh pr merge`, `glab mr merge`, tags, releases; **branch protection** on the remote |
| Factory pushes only `factory/*` | SKILL.md | guard hook rejects any other push target and any force-push |
| No skipped hooks, no discarded work | CONSTITUTION §6 | guard blocks `--no-verify`, `reset --hard`, `clean -f`, `stash`, `checkout -- .` |
| Bounded rework | SKILL.md `max_resumes` | coordinator counter in `job.json` |
| One writer per file | protocol.md | review: reviewers have no Edit tool; the coordinator checks expected files exist |
| Done = evidence | engineer, tester, gate | the gate refuses READY without evidence in `build.md` and `qa.md` |

The guard is a second line of defence, not a sandbox. A determined process with a
shell can still write files. Branch protection on the base branch is the control
that holds, and `pf doctor` checks for it.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Architecture](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
