# Agent instructions — Product Factory repo

This repo is the factory itself, not a product. It holds rules
(`CONSTITUTION.md`), the installable kit (`kit/`), the operator profile
(`operator/`), adapters, scripts and docs. Read `README.md` first.

## Working on the factory

- `CONSTITUTION.md` wins over every other file. A rule change starts there.
- Roles live in **one place**: `kit/.claude/agents/*.md`. The orchestration
  procedure lives in `kit/.claude/skills/factory/SKILL.md`, and the shared agent
  rules live in `kit/.claude/skills/factory/protocol.md`. Do not restate them in
  docs. Link to them.
- Keep the kit project-agnostic. Anything about a specific product belongs in
  that product's `AGENTS.md` or `.factory/factory.yaml`, never here.
- Every factory-level change updates, **in the same commit**, the docs it
  affects and adds a dated entry to `docs/evolution-log.md`.
- After changing the guard hook, run `python3 tests/guard_test.py`, and add a
  case for the new behaviour.
- After changing scripts, run `bash -n scripts/*.sh` and `scripts/doctor.sh`.

## Gates

```bash
python3 tests/guard_test.py
bash -n scripts/*.sh
scripts/doctor.sh
```

## Do not

- Commit secrets, `.env`, `operator/claude-export/` (review it first and fold it
  into `USER.md`), or any product's job folders.
- Commit unless asked. No `--no-verify`. No force-push to `main`.
- Copy product docs into this repo.
