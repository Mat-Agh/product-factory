# Hermes adapter

`Mat-Agh/Hermes` (`$HOME/Work/Hermes`) is the earlier implementation of the same
philosophy on Hermes Agent, 9Router and GitLab. Use it when you want cron
intake, Telegram control or multi-provider routing. Use Claude Code `/factory`
when you want the officially supported runtime and parallel worktrees.

## Role mapping

| Product Factory agent | Hermes profile | Hermes skill |
|---|---|---|
| coordinator (`/factory` session) | `default` (Coordinator) | `factory-delivery`, `factory-decompose` |
| product-manager | `product_manager` | `<slug>-pm` |
| designer | `designer` | `<slug>-design` |
| tech-lead (plan + gate) | `technical_lead` | `factory-tech-lead`, `factory-vcs-mr` |
| engineer | `software_engineer <area>` | `factory-engineer-execute` |
| reviewer-code + reviewer-security | `reviewer` | `factory-code-review` (+ built-in `security-review`) |
| reviewer-ux + reviewer-ui | `tester` (flow/UX walk) | `factory-qa` |
| tester | `tester` | `factory-qa` |
| release-manager | `technical_lead` | `factory-release-readiness` |

## Running a Product Factory role inside Hermes

In a project where the kit is installed, start the matching profile and tell it:

> Act as the Product Factory `<role>`. Read `.claude/skills/factory/protocol.md`
> and `.claude/agents/<role>.md`, then follow them for job `<job-id>`.

Both factories use the same verdict lines and job folder, so a Hermes reviewer
can review a Claude Code build (the cross-vendor lens on the roadmap).

## Converging

The goal is one source of roles. The planned direction (roadmap, phase 5) is to
generate Hermes profile `SOUL.md` files and skills from `kit/.claude/agents/*.md`,
and to retire the duplicated Hermes role text. Until then, a rule change goes
into `CONSTITUTION.md` first and is mirrored into Hermes `docs/06-invariants.md`
in the same session.
