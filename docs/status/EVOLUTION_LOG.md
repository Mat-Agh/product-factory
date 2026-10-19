# Evolution log

**Status:** Active — append-only
**Last updated:** 2026-10-19

Append-only. One dated entry for each factory-level change: what changed, why,
and its evidence.

## 2026-10-19 — All docs under `docs/`, live docs map, rendered diagrams

- **Rule (CONSTITUTION §1, v1.1):** documentation lives under `docs/` in one
  folder per area, each with a README. READMEs link to their parent, documents and
  subfolders. Every document links back to its folder README. `docs/README.md`
  carries a map of the whole tree. This applies to the factory repo and to every
  product.
- Moved `CONSTITUTION.md` → `docs/governance/`, `CONTRIBUTING.md` →
  `docs/development/`, `CURRENT_STATE.md`, `IMPLEMENTATION_STATUS.md`,
  `ROADMAP.md` and `EVOLUTION_LOG.md` → `docs/status/`, `adapters/` →
  `docs/adapters/`, and `operator/` → `docs/operator/`. Rewrote every relative
  link. The kit carries the constitution as a link, and the installer copies it
  into products with the repo-only routing block stripped.
- Replaced `scripts/check-docs.py` and `DOCUMENT_INVENTORY.md` with
  `kit/.factory/docs-map.py`, which generates and checks routing blocks, the map,
  missing READMEs, broken links and orphans. It is installed into products as
  `.factory/docs-map.py`, used by agents (protocol), reviewer-code, `/factory init`
  and `doctor.sh`.
- GitHub showed "Unable to render rich display". All 7 Mermaid diagrams were
  valid (Mermaid 10.9.8 and 11.17.2), so the failure was GitHub's live renderer
  (`viewscreen.githubusercontent.com`) not loading. Diagrams are now
  `docs/assets/diagrams/*.mmd` rendered to committed SVGs by
  `scripts/render-diagrams.sh`, with a `--check` freshness gate.
- Evidence: `docs-map.py --strict` OK (62 docs); `render-diagrams.sh --check` OK;
  `guard_test.py` 73/73; installer smoke test, including `docs-map.py` inside the
  installed product.

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

- Parent: [Status](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
