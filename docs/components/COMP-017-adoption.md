# COMP-017 — Adoption of an existing project

**Status:** Implemented — `pf audit` run read-only on the real Connect root, Back-End and Front-End; reconciliation flow verified on a scratch repo (2026-10-19)
**Last updated:** 2026-10-19
**Source:** [`cli/src/audit.rs`](../../cli/src/audit.rs), `/factory adopt` in [`SKILL.md`](../../kit/.claude/skills/factory/SKILL.md), project branch policy in [`cli/src/guard.rs`](../../cli/src/guard.rs)
**Rule:** [CONSTITUTION §8](../governance/CONSTITUTION.md)
**Decisions:** [ADR-011](../decisions/ADR-011-adopt-existing-process.md)

## Purpose

The factory takes over projects that already run their own agentic process
without breaking it. It inventories that process first, keeps all of it,
configures itself to obey the product's rules, and proposes edits only where a
rule contradicts the constitution's authority or safety rules, or where the
factory's rule has to be mirrored into the product's other agent surfaces.

## `pf audit [path] [--modules] [--json]`

Read-only. The exit code is 1 when any BLOCKER is found.

| Area | Detects | Typical class |
|---|---|---|
| instructions | `AGENTS.md`, `CLAUDE.md`, `CONTRIBUTING.md`, `GEMINI.md`, Copilot/Cursor/Windsurf/Cline files, `.cursor/.codex/.agents/.junie` folders; a `CLAUDE.md` without `@AGENTS.md`; rule mirrors kept in sync | synchronise |
| claude | existing `.claude/agents/<factory name>.md` or `skills/factory` that are not the factory's (the installer keeps them as `*.pre-factory*`); existing hooks; broad pre-approved commands | keep or decide |
| hooks | `.githooks`, `.husky`, `lefthook.yml`, `.pre-commit-config.yaml`; whether `core.hooksPath` activates them; bypass switches such as `*SKIP*`, `HUSKY` or `LEFTHOOK` | keep; bypass is blocked |
| ci | GitLab/GitHub CI files; approval-triggered auto-merge | keep |
| conventions | branch regexes in CI, hooks and build logic (Kotlin strings, Python raw strings, JS literals), tested against the factory's job branch; commit-message rules | configure the factory (a BLOCKER if the branch is rejected) |
| attribution | instructions that forbid AI attribution, and whether Claude Code's `attribution` setting is configured | configure the factory |
| automation | Hermes and other agent tooling | decide |
| secrets | credential-like values in agent config folders, tracked or not (values are never printed) | BLOCKER if tracked, otherwise WARN |
| docs | documentation-routing markers in a foreign format; docs in `doc/` | decide |

## Configuration it drives

| Descriptor key | Effect |
|---|---|
| `vcs.branch_template` (`factory/{job-id}` or `{kind}/{job-id}`) and `vcs.branch_kinds` | Job branch names, and the **guard's** allowed push and delete targets. The guard reads the descriptor through `$CLAUDE_PROJECT_DIR` or the hook's `cwd`, and accepts only `<kind>/NNN-slug` shapes, so agents cannot push other people's branches |
| `vcs.commit_convention` (`none` or `conventional`) | Agents' commit subjects |
| `vcs.attribution` (`none`) | Claude Code `attribution` setting written by `pf install` |
| `risk.escalate_paths` | The product's risky paths (CI, installer, deploy, migrations, secrets) always escalate |

## Guard additions

- Blocks setting hook-bypass variables (`*SKIP*`, `HUSKY`, `LEFTHOOK`, `SKIP`)
  around git writes. For example `DARIA_HOOK_SKIP_GRADLE=1 git push` and
  `export HUSKY=0; git commit`.
- Blocks `git -c core.hooksPath=…` and changing `core.hooksPath` with `git config`.
- Pushes and branch deletes are limited to the project's job-branch shape.

These cases are part of the 80 guard policy cases.

## `/factory adopt`

The procedure in `SKILL.md`, which is step 0 of `/factory init`:
1. Run the audit, and read every listed file.
2. Classify every rule as keep, configure the factory, align, synchronise or
   decide. Nothing is ever deleted.
3. Write `.factory/adoption.md`.
4. Apply only the rows the operator approves: descriptor rows directly, repo
   edits as reviewed `chore` or `docs` jobs.
5. Re-run the audit until no BLOCKER remains.

## Verification

- Unit tests: branch-regex extraction for Kotlin, Python and JS, the sample
  branch from the template, the guard's template policy and bypass cases.
- Real data (read-only): Connect root (0 blockers, 3 conflicts); Back-End and
  Front-End (1 blocker each: branch names), matching a manual review of their
  hooks, CI and build logic.
- Scratch repo: a foreign agent was kept as `engineer.pre-factory.md`; the
  branch template cleared the blocker; the guard allowed `feature/004-…` and
  refused `factory/…`, other branches, `develop` and `DARIA_HOOK_SKIP_GRADLE`.

## Known limitations

- Branch-rule detection is heuristic: string or regex literals on lines that
  mention "branch", in CI, hooks, build logic and tooling. Rules defined in
  unusual places can be missed, so `/factory adopt` still reads the files.
- The audit does not run the product's hooks. The first pilot job proves them.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Components](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
