# Agent instructions — Product Factory repo

This repo is the factory itself, not a product. It holds rules
(`CONSTITUTION.md`), the installable kit (`kit/`), the operator profile
(`operator/`), adapters, scripts and docs. Read `README.md` first, then route
through [`docs/development/16-documentation-link-map.md`](docs/development/16-documentation-link-map.md).
Current state: [`CURRENT_STATE.md`](docs/status/CURRENT_STATE.md). How to change things:
[`CONTRIBUTING.md`](docs/development/CONTRIBUTING.md).

## Working on the factory

- `CONSTITUTION.md` wins over every other file. A rule change starts there.
- Roles live in **one place**: `kit/.claude/agents/*.md`. The orchestration
  procedure lives in `kit/.claude/skills/factory/SKILL.md`, and the shared agent
  rules live in `kit/.claude/skills/factory/protocol.md`. Do not restate them in
  docs. Link to them.
- **All documentation lives under `docs/`**, in a folder per area, each with a
  README. Only `README.md`, `AGENTS.md` and `CLAUDE.md` stay at the root. After
  adding or moving a doc, run `python3 kit/.factory/docs-map.py --fix --strict --allow kit/`,
  which regenerates every README's links and the map in `docs/README.md`.
  Diagrams are `docs/assets/diagrams/*.mmd` rendered to SVG, never inline Mermaid.
- Keep the kit project-agnostic. Anything about a specific product belongs in
  that product's `AGENTS.md` or `.factory/factory.yaml`, never here.
- Every factory-level change updates, **in the same commit**, the docs listed for
  it in `docs/development/03-changing-the-factory.md`, the affected rows of
  `docs/status/IMPLEMENTATION_STATUS.md`, and adds a dated entry to
  `docs/status/EVOLUTION_LOG.md`.
- After changing the guard hook, run `python3 tests/guard_test.py`, and add a
  case for the new behaviour.
- After changing scripts, run `bash -n scripts/*.sh` and `scripts/doctor.sh`.

## Gates

```bash
python3 tests/guard_test.py
bash -n scripts/*.sh
scripts/render-diagrams.sh --check   # after editing a .mmd: scripts/render-diagrams.sh
python3 kit/.factory/docs-map.py --strict --allow kit/   # --fix after adding/moving a doc
scripts/doctor.sh
```

## Do not

- Commit secrets, `.env`, `.claude-export/` (review it first and fold it
  into `USER.md`), or any product's job folders.
- Commit unless asked. No `--no-verify`. No force-push to `main`.
- Copy product docs into this repo.
