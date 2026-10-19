# Roadmap

**Status:** Active
**Last updated:** 2026-10-19

Grow autonomy only as fast as evidence allows. Each phase has an exit criterion,
and the factory does not move to the next phase until that criterion is met.

## Phase 1 — Prove the loop (now)

- Install on one real product. Fill in `AGENTS.md`. Protect the base branch.
- Run 3 trivial, 3 standard and 1 bug job. Run `/factory retro` on each, and
  merge the agreed improvements into this repo.
- Set up mirror remotes and the first offline bundle.

**Exit:** 7 jobs reach a PR with no manual state repair. Every PR has acceptance
evidence. `pf doctor` is clean on two machines.

## Phase 2 — Product front end

- Run `/factory idea` on one new product idea, through to a merged docs PR and a
  seeded backlog.
- Run `/factory release` once, through to a published release.
- Tailor the reviewer checklists to the stack, covering its security hot spots
  and its design system.

**Exit:** one idea reaches a first public release using only factory PRs and
human merges.

## Phase 3 — Intake automation

- Trigger from issues. Labels such as `factory:queued` start `/factory issue` in
  a headless Claude Code run (CI job or scheduled routine), and the PR links
  back to the issue. Issue text remains untrusted data.
- Add `factory-fix-ci`: when a factory PR's CI fails, resume the engineer on the
  failure, bounded by `max_resumes`.
- Enforce WIP limits across sessions, using `limits.max_parallel_jobs`.

**Exit:** 10 issue-triggered jobs with no human intervention caused by agent
failure. Human interventions for real decisions are expected.

## Phase 4 — Observability and cost

- Each job records tokens, wall-clock time per stage, the number of rounds and
  the verdicts, stored in `job.json` and summarised by `/factory status --metrics`.
- Track the metrics: lead time from request to PR, first-pass review rate,
  escaped defects (bugs traced to factory PRs), human interventions per job, and
  cost per merged PR.

**Exit:** a monthly retro made from metrics, not from anecdotes.

## Phase 5 — Independence and diversity

- Add an optional **cross-vendor review lens**, a reviewer on a different model
  family through the Codex CLI or the Hermes `reviewer` profile, for
  cross-cutting jobs.
- Generate the Hermes adapter: create Hermes profiles from `kit/.claude/agents/*.md`,
  so both runtimes share one source of roles.

## Never

Auto-merge, auto-deploy, auto-publish, auto-send, or factory-managed secrets.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Status](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
