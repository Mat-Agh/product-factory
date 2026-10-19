# Evolution log

Append-only. One dated entry for each factory-level change: what changed, why,
and its evidence.

## 2026-10-19 — v1: Product Factory founded

- Merged the Hermes factory philosophy (`Mat-Agh/Hermes` @ `d458700`) with the
  Claude Code software-factory pattern (ZazenCodes, `ctoaIC4LHmI`). The design
  record is in `docs/02-sources-and-decisions.md`.
- Added the `/factory` coordinator skill with lanes per type and tier, 10 role
  agents, the agent protocol, the descriptor template, the backlog and the
  dashboard.
- Added the guard hook, which blocks merge, tag, release, force-push, non-factory
  pushes, `--no-verify` and discarding work. Evidence: `tests/guard_test.py`, 73/73
  passing.
- Added the operator profile (`operator/USER.md`), installers (project copy,
  user symlinks), `doctor.sh`, `mirror.sh` and `export-claude.sh`.
- The pipeline has not run on a real product yet. That is the Phase 1 exit
  criterion.
