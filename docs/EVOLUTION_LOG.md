# Evolution log

**Status:** Active — append-only
**Last updated:** 2026-10-19

Append-only. One dated entry for each factory-level change: what changed, why,
and its evidence.

## 2026-10-19 — Documentation set aligned with the operator's repo conventions

- Restructured `docs/` to match the Daria Connect repositories. Added
  `docs/README.md` as a router; `guide/` (getting started, commands, reading a
  job, troubleshooting, glossary, example `AGENTS.md`); `architecture/` (plus
  `JOB_LIFECYCLE.md`); `components/` (COMP-001…013 and `IMPLEMENTATION_STATUS.md`);
  `development/` (00–05, 09, 16 link map); and `decisions/` (ADR-001…008).
  Added root `CURRENT_STATE.md` and `CONTRIBUTING.md`, and a generated
  `DOCUMENT_INVENTORY.md`.
- Added `scripts/check-docs.py` (links, Status headers, inventory) as a gate.
- Fixed the coordinator lane table: a trivial bug no longer skips reproduction.
  Bugs always run `repro` first (Hermes rule).
- Evidence: `check-docs.py` passes; `guard_test.py` 73/73; `bash -n` passes.

## 2026-10-19 — v1: Product Factory founded

- Merged the Hermes factory philosophy (`Mat-Agh/Hermes` @ `d458700`) with the
  Claude Code software-factory pattern (ZazenCodes, `ctoaIC4LHmI`). The design
  record is in `docs/architecture/SOURCES.md` (then `docs/02-sources-and-decisions.md`).
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

<!-- documentation-routing:start -->
## Documentation routing

- [Parent README](README.md)
- [Governing agent guide](../AGENTS.md)
<!-- documentation-routing:end -->
