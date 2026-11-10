# COMP-001 — `/factory` coordinator skill

**Status:** Implemented — not yet run end to end on a real product
**Last updated:** 2026-10-19
**Source:** [`kit/.claude/skills/factory/SKILL.md`](../../kit/.claude/skills/factory/SKILL.md)
**Decisions:** [ADR-001](../decisions/ADR-001-claude-code-primary-runtime.md), [ADR-003](../decisions/ADR-003-files-not-conversation.md), [ADR-005](../decisions/ADR-005-worktree-per-job.md)

## Codex entry point

The installed `$product-factory` skill reads the shared coordinator and
dispatches generated `factory-<role>` custom agents. See
[the runtime adapter](../adapters/other-agents.md).

## Purpose

The invoking Claude Code or Codex session coordinates one job. It triages,
dispatches native roles, and hands their evidence to `pf job` for validated
state transitions. It handles the VCS handoff after a verified gate. It never writes specs, designs, plans, code, reviews or QA.

## Interface

| Input | Form |
|---|---|
| Command | `/factory init \| <request> \| idea \| issue \| next \| status \| resume \| rework \| release \| retro \| clean` (see [guide/02-commands.md](../guide/02-commands.md)) |
| Project facts | `.factory/factory.yaml` ([COMP-005](COMP-005-project-descriptor.md)) and `AGENTS.md` |
| Queue | `.factory/backlog.md` |

| Output | Form |
|---|---|
| Job state | `.factory/jobs/<id>/job.json` (persisted by the CLI), `triage.md`, `gate.md` only when rounds run out, `rework-<k>.md`, `retro.md` |
| Git | `factory/<id>` branch in a worktree, pushed only to `vcs.remote` |
| PR/MR | `gh pr create` / `glab mr create`, as a draft on ESCALATE |
| Chat | one line per transition, plus the final outcome |

## Behaviour

- Lanes, stage rules, verdict handling, review rounds and PR opening are as
  in the source, and are modelled in
  [architecture/JOB_LIFECYCLE.md](../architecture/JOB_LIFECYCLE.md).
- The coordinator selects a role from [COMP-003](COMP-003-role-agents.md)
  and uses the active runtime's subagent mechanism. It starts all applicable
  review lenses and keeps the engineer's handle for rework.

## Configuration

`limits.max_resumes`, `limits.max_parallel_jobs`, `vcs.*`,
`surfaces.ui.port_base` and `risk.escalate_paths`, all in the descriptor. The
model is whichever model the session runs. Opus is recommended for
coordination.

## Verification

- The Rust integration fixtures exercise parallel jobs, evidence transitions
  and recovery through the CLI.
- The live runtime acceptance procedure is in
  [testing and verification](../development/04-testing-and-verification.md#3-end-to-end-pilot).

## Known limitations

- `SendMessage` resume relies on Claude Code's named-subagent continuation.
  If a Claude Code version lacks it, the coordinator must spawn a fresh
  engineer with the round folder (the rework path does this already).
- A closed coordinator leaves the recorded stage and round in place. Resume
  in a new session after verifying the original coordinator stopped.

## Change checklist

Follow [development/03-changing-the-factory.md](../development/03-changing-the-factory.md) → "Coordinator skill".

The shared entry point exposes the same coordinator to Hermes, Antigravity
and Gemini CLI. Dispatch follows the project's API route or the host's
installed native role. A configured worker writes the canonical artifact,
then `pf job advance` validates it. [Routing](../guide/01-getting-started.md#model-routing)

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Components](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
