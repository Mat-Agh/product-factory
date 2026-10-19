# 03 — Changing the factory

**Status:** Active
**Last updated:** 2026-10-19

Every change updates, in the same commit, everything listed for it below, plus
a dated entry in [EVOLUTION_LOG](../status/EVOLUTION_LOG.md) and the affected rows in
[IMPLEMENTATION_STATUS](../status/IMPLEMENTATION_STATUS.md).

## Docs-sync map

| When you change… | Also update |
|---|---|
| `CONSTITUTION.md` | protocol and agents that apply the rule; [COMP-013](../components/COMP-013-constitution.md); an ADR if it reverses a decision; Hermes `docs/06-invariants.md` |
| Coordinator skill: a command | [guide/02-commands.md](../guide/02-commands.md), [COMP-001](../components/COMP-001-coordinator-skill.md), README command list |
| Coordinator skill: a lane or stage | [JOB_LIFECYCLE](../architecture/JOB_LIFECYCLE.md) (state machine and lane table), [guide/00](../guide/00-overview.md), [guide/03](../guide/03-reading-a-job.md), dashboard `COLS` if it is a new stage |
| Protocol | [COMP-002](../components/COMP-002-agent-protocol.md); re-read all 10 agents for contradictions |
| A role agent's behaviour | [COMP-003](../components/COMP-003-role-agents.md) roster row if tools, model or outputs change |
| An artifact's name or verdict line | **breaking**: SKILL verdict table, JOB_LIFECYCLE artifact table, [guide/03](../guide/03-reading-a-job.md), dashboard `KNOWN` |
| Add or remove an agent | SKILL roles table, COMP-003, [GLOSSARY](../guide/GLOSSARY.md) if it introduces a term, `kit/AGENTS.factory.md` role list, [adapters/hermes.md](../adapters/hermes.md) mapping |
| A descriptor key | template `kit/.factory/factory.yaml`, [COMP-005](../components/COMP-005-project-descriptor.md) schema table, every agent that reads it |
| The guard hook | `tests/guard_test.py` first, then [COMP-004](../components/COMP-004-guard-hook.md) policy table |
| `install.sh` | [COMP-007](../components/COMP-007-installer.md), [guide/01](../guide/01-getting-started.md), [guide/04](../guide/04-add-a-project.md) |
| `doctor.sh` | [COMP-008](../components/COMP-008-doctor.md) check table, [guide/06](../guide/06-troubleshooting.md) if a new failure mode appears |
| Any script's flags | [09-tools-scripts-and-commands](09-tools-scripts-and-commands.md) |
| A new, moved or renamed document | put it under `docs/<area>/`; run `python3 kit/.factory/docs-map.py --fix --strict --allow kit/`; describe it in the area README's own text; add a route in the [link map](16-documentation-link-map.md) if agents need it |
| A diagram | edit `docs/assets/diagrams/<name>.mmd`, run `scripts/render-diagrams.sh`, commit both files ([COMP-014](../components/COMP-014-diagrams.md)) |
| A design decision | a new `decisions/ADR-NNN-*.md` and the [decisions index](../decisions/README.md) |

## Procedures

### Coordinator skill

1. Edit `kit/.claude/skills/factory/SKILL.md`.
2. Update the docs per the map.
3. Observe the change in the sandbox: `install.sh project ~/Work/factory-sandbox`,
   start a **new** Claude Code session there, and run a job through the changed
   path.
4. Record the evidence in IMPLEMENTATION_STATUS.

### Role agents

1. Edit `kit/.claude/agents/<role>.md`. Keep the frontmatter `name` unchanged
   unless you mean a breaking change.
2. Keep the role's artifact format and first line exactly as the coordinator
   expects.
3. Run a sandbox job that exercises the role, and attach the artifact path as
   evidence.

### Adding a review lens

1. Add `kit/.claude/agents/reviewer-<lens>.md`, copying an existing reviewer's
   shape (one file, `VERDICT:` first line, no Edit tool).
2. SKILL: add it to the roles table and the lens rules (when it applies).
3. `job.json.reviews` gains a key. The dashboard picks up chips automatically.
4. Update COMP-003, JOB_LIFECYCLE and guide/03.

### Adding a lane or type

1. SKILL: add the row to the lane table and any new stage rule.
2. JOB_LIFECYCLE: state machine and lane table.
3. guide/00 and guide/02.
4. The dashboard `COLS`, if there is a new stage.

## Breaking changes

These are breaking: renaming or removing an artifact, a verdict word, a
`job.json` key or stage, a descriptor key, an agent name, or a command. For
each one:

1. Bump `factory.version` in the descriptor template, if the descriptor is
   affected.
2. Add an ADR explaining the change and the migration.
3. Add an evolution-log entry with **migration steps** for installed products,
   such as "re-run `install.sh project`; rename key X to Y in factory.yaml".
4. Finish or abandon in-flight jobs before updating a product.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Factory development handbook](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
