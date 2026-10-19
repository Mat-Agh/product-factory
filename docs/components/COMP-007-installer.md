# COMP-007 — Installer

**Status:** Implemented — smoke-tested on a scratch repo (install, re-install, user link) on 2026-10-19
**Last updated:** 2026-10-19
**Source:** [`scripts/install.sh`](../../scripts/install.sh), [`kit/AGENTS.factory.md`](../../kit/AGENTS.factory.md)
**Decisions:** [ADR-006](../decisions/ADR-006-portable-distribution.md)

## Modes

| Command | Effect | Idempotent |
|---|---|---|
| `install.sh project <path>` | **Copies** the kit into a product repo for you to commit there | yes |
| `install.sh user` | **Symlinks** the kit into `${CLAUDE_CONFIG_DIR:-~/.claude}`, so it follows `git pull` | yes |
| `install.sh all <path>` | both | yes |

## `project` writes

| Path in product | Action |
|---|---|
| `.claude/skills/factory/` | replaced from the kit |
| `.claude/agents/*.md` | the ten agents overwritten. Other agents are untouched |
| `.claude/hooks/factory-guard.py` | overwritten and made executable |
| `.claude/settings.json` | guard entry **merged** with `jq`. The file is never overwritten. Without `jq`, the snippet is printed |
| `.factory/factory.yaml`, `.factory/backlog.md` | created only if missing |
| `.factory/dashboard.html` | overwritten |
| `.factory/installed.lock` | source URL, factory revision, timestamp |
| `.gitignore` | `.factory/jobs/` appended if absent |
| `AGENTS.md` | Product Factory section appended once (detected by its `## Product Factory` heading) |
| `CLAUDE.md` | created as `@AGENTS.md` only if missing |

It never commits, never pushes, and refuses a target that is not a git repo.

## `user` writes

Symlinks for `skills/factory`, each agent and the guard script. It appends an
`@<factory>/docs/operator/USER.md` import to `CLAUDE.md` once. It leaves alone any
existing non-symlink file of the same name, and warns. A user-wide guard hook
is opt-in (`WITH_USER_GUARD=1`).

## Verification

Run on 2026-10-19 against a scratch git repo: a first install, then a re-install
(which kept the descriptor and backlog, did not duplicate the hook, and did not
duplicate the AGENTS section), then a user install into a temporary
`CLAUDE_CONFIG_DIR`, then `doctor.sh` with 0 failures. The procedure is in
[development/04-testing-and-verification.md](../development/04-testing-and-verification.md#2-installer-smoke-test).

## Known limitations

- Bash only (Linux, macOS, WSL). There is no native Windows PowerShell port.
- Project and user installs can both exist. Project agents take precedence in
  that project, which is intended.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Components](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
