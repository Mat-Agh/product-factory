# COMP-007 — Installer

**Status:** Implemented in Rust — fresh install, re-install, legacy-Python migration and user link verified on a scratch repo (2026-10-19)
**Last updated:** 2026-10-19
**Source:** [`cli/src/install.rs`](../../cli/src/install.rs), [`kit/AGENTS.factory.md`](../../kit/AGENTS.factory.md)
**Decisions:** [ADR-006](../decisions/ADR-006-portable-distribution.md)

## Modes

| Command | Effect | Idempotent |
|---|---|---|
| `pf install project <path>` | **Copies** the kit into a product repo, for you to commit there | yes |
| `pf install user [--with-guard]` | **Symlinks** the kit into `${CLAUDE_CONFIG_DIR:-~/.claude}`, so it follows `git pull` | yes |
| `pf install all <path>` | Both | yes |
| `pf install module <path> --workspace <root>` | The Claude Code kit only (skill, agents, rule, guard, attribution) into one module repo listed in the workspace descriptor | yes |

## `project` writes

| Path in product | Action |
|---|---|
| `.claude/skills/factory/` | Replaced from the kit. The constitution is copied as a file, with the factory-repo routing block stripped because its links resolve only in the factory repo |
| `.claude/agents/*.md` | The ten agents are overwritten. Other agents are untouched |
| `.claude/rules/product-factory.md` | The always-on factory rule, overwritten ([COMP-016](COMP-016-session-integration.md)) |
| `.claude/settings.json` | The `pf guard` hook entry is **merged**. The file is never overwritten, other settings are preserved, and legacy `factory-guard.py` entries are removed |
| `.claude/settings.json` → `attribution` | Set to hide AI attribution when `vcs.attribution: none` or `--no-ai-attribution` |
| `.claude/hooks/factory-guard.py`, `.factory/docs-map.py` | Legacy Python files are removed if present |
| `.factory/factory.yaml`, `.factory/backlog.md` | Created only if missing |
| `.factory/dashboard.html` | Overwritten |
| `.factory/installed.lock` | Source URL, factory revision, UTC timestamp |
| `.gitignore` | `.factory/jobs/` is appended if absent |
| `AGENTS.md` | The Product Factory section is appended once, detected by its `## Product Factory` heading |
| `CLAUDE.md` | Created as `@AGENTS.md` only if missing |

It never commits, never pushes, and refuses a target that is not a git repo.
It warns when `pf` is not on `PATH`, because the hook would then block Bash.

## `user` writes

- Symlinks for `skills/factory` and for each agent.
- An `@<factory>/docs/operator/USER.md` import in `CLAUDE.md`, added once.
- With `--with-guard`, or when a legacy Python guard is already wired, the `pf
  guard` hook is merged into the user `settings.json`.

An existing file that is not a symlink is left alone, with a warning.

## Verification

The installer smoke test is in
[04-testing-and-verification](../development/04-testing-and-verification.md#2-installer-smoke-test).
It was last run on 2026-10-19, and the settings merge has a unit test
(`merge_hook_is_idempotent_and_replaces_legacy`).

## Known limitations

- The user install needs symlinks (Linux, macOS or WSL).
- Project and user installs can both exist. Project agents take precedence in
  that project, which is intended.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Components](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
