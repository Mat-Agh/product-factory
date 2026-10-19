# COMP-013 — Constitution

**Status:** Active — v1.1
**Last updated:** 2026-10-19
**Source:** [`CONSTITUTION.md`](../governance/CONSTITUTION.md)

## Purpose

The constitution holds the rules every agent, adapter and human in the factory
follows. When any other file disagrees with it, that file is a defect.

## Sections

| § | Rule family |
|---|---|
| 1 | Truth lives in the product repository. Nothing durable exists only in an account. Documentation lives under `docs/` as one connected tree |
| 2 | Separate roles and bounded scope; triage first |
| 3 | Context from files; one writer per file; verdict first lines; untrusted input |
| 4 | Evidence over confidence; independent multi-lens review; bounded rework |
| 5 | Humans own the irreversible: the authority table and the always-escalate list |
| 6 | Fail closed; preserve work; no secrets; never commit to the base branch |
| 7 | The factory improves from its own record (retro, evolution log, metrics) |
| 8 | Voice |

## Where each section is enforced

| § | Enforcement |
|---|---|
| 1 | `docs-sync` check in reviewer-code; installer layout; [ADR-007](../decisions/ADR-007-truth-in-product-repo.md); docs tree and map by `pf docs-map` ([COMP-012](COMP-012-docs-gate.md)) |
| 2 | Lanes in [COMP-001](COMP-001-coordinator-skill.md); role tools and instructions in [COMP-003](COMP-003-role-agents.md) |
| 3 | Protocol ([COMP-002](COMP-002-agent-protocol.md)); the coordinator stops on a bad verdict |
| 4 | Gate rules in tech-lead; `max_resumes` |
| 5 | Guard hook ([COMP-004](COMP-004-guard-hook.md)); branch protection; gate escalation |
| 6 | Guard hook (discard and `--no-verify` rules); protocol |
| 7 | `/factory retro`; [EVOLUTION_LOG](../status/EVOLUTION_LOG.md) |

## Change checklist

A constitutional change starts here, then flows to protocol, agents, skill, docs
and the Hermes invariants (`docs/06-invariants.md` in `Mat-Agh/Hermes`), all in
one change, with an evolution-log entry and an ADR when it reverses an earlier
decision.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Components](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
