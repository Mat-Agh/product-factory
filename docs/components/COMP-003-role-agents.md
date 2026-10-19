# COMP-003 — Role agents

**Status:** Implemented — not yet run end to end on a real product
**Last updated:** 2026-10-19
**Source:** [`kit/.claude/agents/`](../../kit/.claude/agents/)
**Decisions:** [ADR-002](../decisions/ADR-002-human-only-merge.md), [ADR-004](../decisions/ADR-004-review-lenses-and-qa.md), [ADR-008](../decisions/ADR-008-idea-and-release-lanes.md)

## Purpose

Ten Claude Code subagents. Each is one markdown file with frontmatter (`name`,
`description`, `tools`, `model`) and a body that says what it reads, what it
writes, and the exact format and verdict of its artifact. They carry nothing
project-specific. Everything project-specific comes from the descriptor and
`AGENTS.md`.

## Roster

| Agent | Model | Tools | Stages | Writes | Verdict line |
|---|---|---|---|---|---|
| [product-manager](../../kit/.claude/agents/product-manager.md) | opus | Read, Grep, Glob, Bash, Write, Edit, WebSearch, WebFetch | discovery, spec | `discovery.md`, `spec.md`, PRD in branch | `STATUS: READY \| BLOCKED` |
| [designer](../../kit/.claude/agents/designer.md) | opus | Read, Grep, Glob, Bash, Write, Edit | design | `design.md`, design docs in branch | `STATUS: READY \| BLOCKED` |
| [tech-lead](../../kit/.claude/agents/tech-lead.md) | opus | Read, Grep, Glob, Bash, Write, Edit | plan, gate | `plan.md`, ADR in branch, `gate.md`, `pr.md` | `STATUS: …` / `READY \| ESCALATE` |
| [engineer](../../kit/.claude/agents/engineer.md) | sonnet | Read, Grep, Glob, Bash, Edit, Write | build | code and doc commits, `build.md` | `STATUS: READY \| BLOCKED` |
| [reviewer-code](../../kit/.claude/agents/reviewer-code.md) | sonnet | Read, Grep, Glob, Bash, Write | review | `round-N/review-code.md` | `VERDICT: PASS \| CHANGES` |
| [reviewer-security](../../kit/.claude/agents/reviewer-security.md) | opus | Read, Grep, Glob, Bash, Write | review | `round-N/review-security.md` | `VERDICT: PASS \| CHANGES` |
| [reviewer-ux](../../kit/.claude/agents/reviewer-ux.md) | sonnet | Read, Grep, Glob, Bash, Write | review (ui) | `round-N/review-ux.md` | `VERDICT: PASS \| CHANGES` |
| [reviewer-ui](../../kit/.claude/agents/reviewer-ui.md) | sonnet | Read, Grep, Glob, Bash, Write | review (ui) | `round-N/review-ui.md` | `VERDICT: PASS \| CHANGES` |
| [tester](../../kit/.claude/agents/tester.md) | sonnet | Read, Grep, Glob, Bash, Write | repro, qa | `repro.md`, `qa.md` | `REPRODUCED…` / `VERDICT: PASS \| FAIL` |
| [release-manager](../../kit/.claude/agents/release-manager.md) | opus | Read, Grep, Glob, Bash, Write, Edit, WebSearch, WebFetch | release | `release.md`, release files in branch | `STATUS: READY \| BLOCKED` |

Model classes follow the Hermes routing: deep for thinking, planning, security
and gating; build for execution and the routine lenses.

## Separation of duties

| Never | Who |
|---|---|
| Edits code | product-manager, designer, tech-lead, every reviewer, tester |
| Decides product scope | designer, tech-lead, engineer, reviewers, tester |
| Pushes, merges, tags | every agent (the coordinator pushes `factory/*` only, and humans merge) |
| Signs off its own work | the engineer is reviewed by the lenses, and the tester never fixes |

Reviewers and the tester have no `Edit` tool. They can still write with Bash,
so this is least privilege in practice, not isolation.

## Verification

Role behaviour is verified by the pilot
([development/04-testing-and-verification.md](../development/04-testing-and-verification.md)).
Frontmatter validity: Claude Code lists the agents in `/agents` after
installation.

## Known limitations

- Every agent runs on Claude, so there is no cross-vendor review yet. A Codex
  lens is planned ([ROADMAP](../ROADMAP.md) phase 5).
- UX and UI lenses depend on a browser CLI and a working dev-server section in
  `AGENTS.md`. Without them, they review by reading code only.

## Change checklist

See [development/03-changing-the-factory.md](../development/03-changing-the-factory.md) → "Role agents".

<!-- documentation-routing:start -->
## Documentation routing

- [Parent README](README.md)
- [Governing agent guide](../../AGENTS.md)
<!-- documentation-routing:end -->
