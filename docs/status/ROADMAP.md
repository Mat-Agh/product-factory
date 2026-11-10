# Roadmap

**Status:** Active
**Last updated:** 2026-11-10

Grow autonomy only as fast as evidence allows. Each phase has an exit criterion,
and the factory does not move to the next phase until that criterion is met.

## v1 installation and delivery criteria

- **Install and adopt.** Start from a standalone repo, a mixed-stack Git repo,
  or a workspace of independent checkouts. Detect module boundaries and
  install Claude Code and Codex routes. Audit existing instructions and verify
  VCS, stack documentation, gate commands and the descriptor before delivery.
- **Distribution.** Verify the standalone binary can install and check a
  project after its build checkout is removed. Publish binaries only for
  platforms verified from the reviewed release commit.
- **Intake and visibility.** Accept a direct brief, a GitHub/GitLab issue, or
  the first unchecked backlog line. Record ownership and progress on the
  local board. Concurrent claims must reserve distinct jobs and obey the
  workspace's active-job limit.
- **Isolation.** Prepare concurrent jobs in disjoint modules on separate
  branches and worktrees. Prove that a commit in one job cannot change the
  other checkout or the operator's checkout. Reject an out-of-module diff
  before review. Sequence jobs that share contracts or root-level files.
- **Evidence and recovery.** Reject a missing verdict or a changed review
  commit, wait for every required lens, bound automated rework, preserve the
  evidence from every round, and recover from a stopped coordinator.
- **Runtime and delivery.** On an authorized test repo, run a real job in each
  of Claude Code and Codex. Confirm the actual role and model selected, the
  setup and gates run in the job worktree, the reviews reach a decision, and
  the job can reach a local delivery branch or a PR under the configured VCS
  policy. The mechanical tests supply fixture artifacts and cover a different
  part of this criterion.

The evidence and remaining actions are in
[IMPLEMENTATION_STATUS](IMPLEMENTATION_STATUS.md).

## Phase 1 — Prove the loop on real products

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
- Apply the workspace WIP limit to automated issue-triggered sessions.

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
