# COMP-008 — Doctor

**Status:** Implemented in Rust — verified on this machine and a scratch product (2026-10-19)
**Last updated:** 2026-10-19
**Source:** [`cli/src/doctor.rs`](../../cli/src/doctor.rs)

## Purpose

A read-only health check. Its exit code is the number of `FAIL` lines. `note`
lines are advice that does not fail the check.

## Checks

| Group | Check | Level |
|---|---|---|
| Machine | `git` present; `pf` on `PATH` (the hook needs it) | FAIL |
| Machine | `gh`, `glab`, `claude`, `codex`, `hermes`, `node` and Graphify present; `gh` authenticated | note |
| Machine | guard policy self-test (80 cases) | FAIL |
| Machine | factory docs map current (`--strict --allow kit/`); Mermaid blocks lint clean | FAIL |
| Factory repo | at least 2 remotes (mirrored) | note |
| Factory repo | clean working tree; upstream set and not behind | note |
| Factory repo | user-level `/factory` symlinked | note |
| Project | descriptor is reviewed. Backlog, skill, constitution and `AGENTS.md` exist | FAIL |
| Project | `.factory/jobs/` ignored; `pf guard` wired; no legacy Python guard; `## Product Factory` section | FAIL |
| Project | `AGENTS.md` has Setup, Dev server and Gates headings | note |
| Project | Claude agent routes match the descriptor. Skill, protocol, constitution and dashboard match the kit | note |
| Project | Shared skill and generated Codex and Antigravity agent definitions match the descriptor at the root and in installed modules | FAIL |
| Project | product docs map status at the inferred `docs/`, `doc/`, or `product/` root | note |
| Project | base branch protected on GitHub | note |

From a standalone binary, doctor checks the machine and the embedded kit
against the product. With a Factory checkout, it also checks that
checkout's docs map, Mermaid blocks and Git state.

## Usage

```bash
pf doctor                  # machine + factory repo
pf doctor ~/Work/MyApp     # + project
```

## Known limitations

The branch-protection check covers GitHub only. Private repos on free plans
report "no protection" because the API returns 403 or 404.

Use `pf providers check` to validate the route configuration and see which
local credentials or tools are missing.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Components](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
