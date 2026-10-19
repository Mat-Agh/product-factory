# COMP-008 — Doctor

**Status:** Implemented — verified on this machine and a scratch project on 2026-10-19
**Last updated:** 2026-10-19
**Source:** [`scripts/doctor.sh`](../../scripts/doctor.sh)

## Purpose

A read-only health check. Its exit code is the number of `FAIL` lines. `note`
lines are advice that does not fail the check.

## Checks

| Group | Check | Level |
|---|---|---|
| Machine | `git`, `python3` present | FAIL |
| Machine | `gh`, `glab`, `jq`, `claude` present; `gh` authenticated | note |
| Machine | guard tests pass (`tests/guard_test.py`) | FAIL |
| Factory repo | at least 2 remotes (mirrored) | note |
| Factory repo | clean working tree; upstream set and not behind | note |
| Factory repo | user-level `/factory` symlinked | note |
| Project | descriptor, backlog, skill, guard, `AGENTS.md` exist | FAIL |
| Project | `.factory/jobs/` ignored; guard wired in `.claude/settings.json`; `## Product Factory` section | FAIL |
| Project | `AGENTS.md` has Setup, Dev server and Gates headings | note |
| Project | agent, skill, protocol and guard copies identical to the kit (drift) | note |
| Project | base branch protected on GitHub | note |

## Usage

```bash
scripts/doctor.sh                 # machine + factory repo
scripts/doctor.sh ~/Work/MyApp    # + project
```

## Known limitations

The branch-protection check covers GitHub only. Private repos on free plans
report "no protection" because the API returns 403 or 404.

<!-- documentation-routing:start -->
## Documentation routing

- [Parent README](README.md)
- [Governing agent guide](../../AGENTS.md)
<!-- documentation-routing:end -->
