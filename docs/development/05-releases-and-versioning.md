# 05 — Releases and versioning

**Status:** Active
**Last updated:** 2026-10-19

## Versions

The factory uses semantic versioning on its git tags: `vMAJOR.MINOR.PATCH`.

| Bump | When |
|---|---|
| MAJOR | Any breaking change ([03](03-changing-the-factory.md#breaking-changes)): artifact or verdict format, `job.json`, descriptor schema, agent names, commands |
| MINOR | New lane, agent, lens, command, descriptor key with a default, script |
| PATCH | Wording, checklists, bug fixes, docs |

Current: **unreleased v1** (no tag yet). The first tag, `v1.0.0`, is planned
after the Phase 1 pilot passes.

## Releasing the factory

Tagging is human-only, here as everywhere.

1. All gates pass ([04](04-testing-and-verification.md)).
2. IMPLEMENTATION_STATUS is current. The evolution log has an entry headed with
   the version.
3. A human tags and pushes: `git tag -s vX.Y.Z -m "Product Factory vX.Y.Z" && git push origin vX.Y.Z`.
4. Run `scripts/mirror.sh push` so the tag reaches every mirror, then
   `scripts/mirror.sh bundle`.

## Updating products

Each product records the factory revision it runs in `.factory/installed.lock`.

```bash
cd ~/Work/ProductFactory && git pull
scripts/doctor.sh ~/Work/MyApp            # shows drift
scripts/install.sh project ~/Work/MyApp   # keeps factory.yaml and backlog.md
cd ~/Work/MyApp && git diff               # review, then commit
```

For a MAJOR update, follow the migration steps in that version's evolution-log
entry, and finish in-flight jobs first.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Factory development handbook](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
