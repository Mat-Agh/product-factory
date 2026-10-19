# COMP-001 — `/factory` coordinator skill

**Status:** Implemented — not yet run end to end on a real product
**Last updated:** 2026-10-19
**Source:** [`kit/.claude/skills/factory/SKILL.md`](../../kit/.claude/skills/factory/SKILL.md)
**Decisions:** [ADR-001](../decisions/ADR-001-claude-code-primary-runtime.md), [ADR-003](../decisions/ADR-003-files-not-conversation.md), [ADR-005](../decisions/ADR-005-worktree-per-job.md)

## Purpose

This skill turns the Claude Code session that invokes `/factory` into the
**coordinator** of one job. It triages, picks the lane, creates the worktree,
spawns role agents in order, reads their verdicts, keeps `job.json`, opens the
PR and reports. It never writes specs, designs, plans, code, reviews or QA.

## Interface

| Input | Form |
|---|---|
| Command | `/factory init \| <request> \| idea \| issue \| next \| status \| rework \| release \| retro \| clean` (see [guide/02-commands.md](../guide/02-commands.md)) |
| Project facts | `.factory/factory.yaml` ([COMP-005](COMP-005-project-descriptor.md)) and `AGENTS.md` |
| Queue | `.factory/backlog.md` |

| Output | Form |
|---|---|
| Job state | `.factory/jobs/<id>/job.json` (sole writer), `triage.md`, `gate.md` only when rounds run out, `rework-<k>.md`, `retro.md` |
| Git | `factory/<id>` branch in a worktree, pushed only to `vcs.remote` |
| PR/MR | `gh pr create` / `glab mr create`, as a draft on ESCALATE |
| Chat | one line per transition, plus the final outcome |

## Behaviour

- Lanes, stage rules, verdict handling, review rounds and PR opening are as
  in the source, and are modelled in
  [architecture/JOB_LIFECYCLE.md](../architecture/JOB_LIFECYCLE.md).
- Agents are spawned through Claude Code's subagent mechanism, using the
  `subagent_type` names from [COMP-003](COMP-003-role-agents.md). All
  reviewers of a round are spawned in one message, so they run in parallel.
- The engineer is spawned with the name `engineer-<id>` and resumed with
  `SendMessage`.

## Configuration

`limits.max_resumes`, `limits.max_parallel_jobs`, `vcs.*`,
`surfaces.ui.port_base` and `risk.escalate_paths`, all in the descriptor. The
model is whichever model the session runs. Opus is recommended for
coordination.

## Verification

- Static: the skill is a markdown procedure, with no code.
  `python3 kit/.factory/docs-map.py --strict --allow kit/` checks its links.
- Behaviour: the end-to-end pilot procedure in
  [development/04-testing-and-verification.md](../development/04-testing-and-verification.md#3-end-to-end-pilot).
  **It has not been run yet** (see [IMPLEMENTATION_STATUS.md](../status/IMPLEMENTATION_STATUS.md)).

## Known limitations

- `SendMessage` resume relies on Claude Code's named-subagent continuation.
  If a Claude Code version lacks it, the coordinator must spawn a fresh
  engineer with the round folder (the rework path does this already).
- If the coordinator session is closed, the job pauses at its current stage.
  Resume it with `/factory rework <job> "resume from <stage>"`.
- Multi-repo worktree placement is specified but unexercised.

## Change checklist

Follow [development/03-changing-the-factory.md](../development/03-changing-the-factory.md) → "Coordinator skill".

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Components](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
