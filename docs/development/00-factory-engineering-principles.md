# 00 — Factory engineering principles

**Status:** Active
**Last updated:** 2026-10-19

These are the rules for changing the factory. The rules the factory applies to
products are in [`CONSTITUTION.md`](../governance/CONSTITUTION.md). This page covers
how the factory's own source stays correct.

## 1. One source per concept

| Concept | The only place it is defined |
|---|---|
| Rules | `CONSTITUTION.md` |
| Orchestration procedure, lanes, commands | `kit/.claude/skills/factory/SKILL.md` |
| Rules shared by all agents | `kit/.claude/skills/factory/protocol.md` |
| One role's behaviour and artifact format | `kit/.claude/agents/<role>.md` |
| Descriptor schema | `kit/.factory/factory.yaml` (template), explained in [COMP-005](../components/COMP-005-project-descriptor.md) |
| Operator preferences | `docs/operator/USER.md` |

Docs **explain and link**. They never restate a rule, a lane table or an
artifact format in a way that could drift. Where a summary table is
unavoidable, as in [JOB_LIFECYCLE](../architecture/JOB_LIFECYCLE.md), it is
updated in the same commit as its source.

## 2. The kit is project-agnostic

Nothing in `kit/` names a product, stack, command, path or person. Everything
project-specific is read at run time from the product's `AGENTS.md` and
`.factory/factory.yaml`. Write product-specific instructions in the owning product’s `AGENTS.md`.

## 3. Contracts are stable

These are the factory's public interface. Changing one breaks running jobs or
installed projects:

- artifact file names and **first-line verdict formats**
- `job.json` keys and stage names
- descriptor keys and their meaning
- agent `name` values (the coordinator calls them as `subagent_type`)
- command names (`/factory <subcommand>`)

You may add to them. Renaming, removing or changing the meaning of one is a
**breaking change** ([03-changing-the-factory.md](03-changing-the-factory.md#breaking-changes)).

## 4. Fail closed, at every layer

The coordinator stops on a missing or malformed verdict. The guard blocks on
unreadable input. The gate escalates when evidence is missing. A new feature
must define what happens when its input is absent, and that answer is "stop and
tell the human", never "assume".

## 5. No invented knowledge

Agents and docs never invent commands, file names, credentials or product
behaviour. Docs about commands show commands that have been run. Docs about
behaviour say whether it is verified (see
[IMPLEMENTATION_STATUS](../status/IMPLEMENTATION_STATUS.md)).

## 6. Smallest effective mechanism

Prefer a sentence in a role file over a new role, a new role over a new stage,
and a stage over a new tool. Every agent, stage and script must justify its
cost in tokens, latency and maintenance.

## 7. Evidence for every change

A factory change is done when its gates pass (`guard_test`, `bash -n`,
`docs-map`, `doctor`), its docs and implementation status are updated, the
evolution log has an entry, and **behavioural changes have been observed in a
real or scratch job**. A change that only "reads correctly" is not done.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Factory development handbook](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
