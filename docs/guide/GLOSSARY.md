# Glossary

**Status:** Active
**Last updated:** 2026-10-19

| Term | Meaning |
|---|---|
| **Acceptance criteria** | A numbered list in `spec.md` (copied into `plan.md`) of observable behaviours, each with a way to check it. "Done" means every one has evidence. |
| **Adapter** | Instructions for running the factory's roles in a tool other than Claude Code (Hermes, Codex, Cursor). See `adapters/`. |
| **ADR** | Architecture Decision Record: a short document recording one technical decision, its context and its consequences. Products keep theirs under `docs.adr_dir`, and the factory keeps its own in `docs/decisions/`. |
| **Agent / role agent** | A Claude Code subagent defined by one markdown file in `.claude/agents/`, with its own instructions, tools and model. The factory has ten. |
| **`AGENTS.md`** | The file at a repo root that tells any AI agent how to work in that repo: setup, commands, conventions. `CLAUDE.md` usually just imports it (`@AGENTS.md`). |
| **Artifact** | A file an agent writes in the job folder (`spec.md`, `build.md`, `review-code.md`, …). Each has one owner, and its first line is a verdict. |
| **Base branch** | The branch PRs target, for example `main` or `develop` (`vcs.base_branch`). The factory never commits to it. |
| **Branch protection** | A Git host setting that stops direct pushes or merges without review on a branch. It is the strongest guarantee that only humans merge. |
| **Coordinator** | The Claude Code session that runs `/factory`. It triages, spawns agents, and keeps `job.json`. It never writes specs, code or reviews. |
| **Cross-cutting** | The widest tier: the change touches product, design, an API or schema, auth, tenancy, or several modules. It always escalates. |
| **Descriptor** | `.factory/factory.yaml` in a product repo. It says where things are and what the policy is for that product. See [COMP-005](../components/COMP-005-project-descriptor.md). |
| **Draft PR** | A PR marked "not ready". The factory opens one when the gate escalates, so that you look before anything merges. |
| **Escalate** | The gate's verdict that a human must decide. It happens for risky areas, product decisions, exhausted review rounds, or missing evidence. |
| **Evidence** | Proof attached to an acceptance criterion: the command run and its output, or what a page or device showed. A compile is not evidence. |
| **Gate** | The final check by the tech-lead before a PR opens. Its verdict is READY or ESCALATE. |
| **Guard hook** | `factory-guard.py`, a Claude Code `PreToolUse` hook that blocks forbidden git, gh and glab commands. See [COMP-004](../components/COMP-004-guard-hook.md). |
| **Hook** | A command Claude Code runs automatically around tool calls. Configured in `.claude/settings.json`. |
| **Idea lane** | The path for a product idea: discovery → PRD → design concept → architecture → roadmap, ending in a docs PR. |
| **Job / job id** | One request moving through the pipeline. Its id is a number plus a slug: `004-tenant-org-name`. |
| **`job.json`** | The machine-readable state of a job. Only the coordinator writes it, and the dashboard reads it. |
| **Lane** | The ordered list of stages a job goes through. Triage picks it from type, tier and surface. |
| **Lens** | One reviewer's angle: code, security, UX or UI. Lenses run in parallel. |
| **Living sample** | Existing code in the repo that new code should copy in structure, instead of inventing a new pattern. |
| **Module** | One repository or area of a product. A single-repo product has one, and multi-repo products (like Daria Connect) have several. |
| **`needs-human`** | The stage a job enters when it cannot continue without you. The coordinator quotes the reason. |
| **Operator** | You: the person who runs the factory and makes the decisions it escalates. Described in `operator/USER.md`. |
| **PR / MR** | Pull request (GitHub) or merge request (GitLab): a proposed set of changes on a branch, which a human reviews and merges. |
| **PRD** | Product Requirements Document: what a product or feature must do, and why. |
| **Protocol** | `.claude/skills/factory/protocol.md`, the rules every agent reads first. |
| **Resume** | Sending the same engineer back to fix review or QA findings. The number of resumes is capped by `limits.max_resumes` (default 2). |
| **Retro** | `/factory retro`, which looks back at finished jobs and proposes improvements to the product docs and to the factory. |
| **Round** | One pass of all required review lenses over the current build. Its results are in `round-<N>/`. |
| **Skill** | A Claude Code capability defined by a `SKILL.md`, invoked as a slash command. The factory is the `factory` skill. |
| **Spike** | A time-boxed investigation that produces an ADR, not shipped code. |
| **Stage** | One step of a lane (triage, spec, plan, build, review, …). |
| **Subagent** | A separate Claude instance that a session starts for a task, with its own context. The factory's role agents are subagents. |
| **Surface** | Where a change shows: `ui` (pages or screens), `api` (contracts) or `none`. UX and UI lenses run only on `ui`. |
| **Tier** | How wide a change is: trivial, standard or cross-cutting. The lower tier is preferred whenever it honestly fits. |
| **Triage** | The first stage: understand, dedupe, classify, size, prioritise, split and route. |
| **Verdict** | The machine-readable first line of every artifact (`STATUS: READY`, `VERDICT: PASS`, `ESCALATE`, …). |
| **Worktree** | A second working folder of the same git repo, checked out on a different branch. Each job has one, so jobs never disturb your checkout or each other. |

<!-- documentation-routing:start -->
## Documentation routing

- [Parent README](README.md)
- [Governing agent guide](../../AGENTS.md)
<!-- documentation-routing:end -->
