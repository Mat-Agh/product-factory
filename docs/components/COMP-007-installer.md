# COMP-007 — Installer

**Status:** Implemented in Rust — fresh install, re-install, legacy-Python migration and user link verified on a scratch repo (2026-10-19)
**Last updated:** 2026-11-10
**Source:** [`cli/src/install.rs`](../../cli/src/install.rs), [`kit/AGENTS.factory.md`](../../kit/AGENTS.factory.md)
**Decisions:** [ADR-006](../decisions/ADR-006-portable-distribution.md)

## Bundled project installer

`pf install project`, `pf install module`, `pf knowledge`, `pf audit` and
`pf doctor` can read the embedded generic kit and knowledge library when a
Factory checkout is unavailable. The CLI unpacks them into a temporary
directory for the command and records the binary version and payload
fingerprint in the product's `installed.lock`. `--factory` selects an
explicit checkout. The user-link commands still use a retained checkout so
Claude Code's symlinks and operator profile keep working across updates.

## Modes

| Command | Effect | Idempotent |
|---|---|---|
| `pf install project <path> [--root-only]` | **Copies** all project runtime files into a Git root. It includes independent checkouts listed in the descriptor by default. Shared-history modules use the root kit | yes |
| `pf install user [--with-guard]` | **Symlinks** the kit into `${CLAUDE_CONFIG_DIR:-~/.claude}`, so it follows `git pull` | yes |
| `pf install all <path>` | Both | yes |
| `pf install module <path> --workspace <root>` | All runtime files into one module repo listed in the workspace descriptor | yes |

## `project` writes

| Path in product | Action |
|---|---|
| `.claude/skills/factory/` | Shipped files are updated from the kit. The constitution is copied as a file, with the factory-repo routing block stripped because its links resolve only in the factory repo |
| `.claude/agents/*.md` | Generated from the ten role files with the descriptor’s Claude model routes |
| `.claude/rules/product-factory.md` | The always-on factory rule, overwritten ([COMP-016](COMP-016-session-integration.md)) |
| `.claude/settings.json` | The `pf guard` hook entry is **merged**. The file is never overwritten, other settings are preserved, and legacy `factory-guard.py` entries are removed |
| `.claude/settings.json` → `attribution` | Set to hide AI attribution when `vcs.attribution: none` or `--no-ai-attribution` |
| `.claude/hooks/factory-guard.py`, `.factory/docs-map.py` | Legacy Python files are removed if present |
| `.codex/agents/factory-*.toml` and `.agents/skills/product-factory/SKILL.md` | Codex dispatch and skill. Existing unmanaged files at these paths cause an error |
| `.factory/factory.yaml`, `.factory/backlog.md` | Created only if missing. The draft descriptor includes detected Git submodules, stack and docs roots |
| `.factory/dashboard.html` | Overwritten |
| `.factory/installed.lock` | Source URL, factory revision, UTC timestamp |
| `.gitignore` | `.factory/jobs/` is appended if absent |
| `AGENTS.md` | Appends the Factory section and cross-runtime entry point once. Upgrades replace only the obsolete Factory guidance that assumed every product uses `docs/`; the owning repository keeps its documentation root |
| `GEMINI.md` and existing `.hermes.md` or `HERMES.md` | Adds a reference to the shared project instructions |
| `.agents/agents/factory-*.md` | Antigravity subagents generated from the canonical roles |
| `.claude/rules/modules/*.md` | Generated from the workspace descriptor. Unmanaged files at the same paths are preserved and reported |
| `.factory/graphify/` | Ignored local graph and interactive 3D file network if Graphify is installed and source is present |
| `CLAUDE.md` | Created as `@AGENTS.md` only if missing |

Run `pf audit <path> --modules` before adopting existing projects. Review
the descriptor through `/factory init` or `$product-factory init`, especially
VCS, hooks and gates. `pf knowledge plan --modules` previews the knowledge
imports for review. Git branches, working trees and remote state stay in
the operator's control. The install target must be a Git root.
It warns when `pf` is not on `PATH`, because the hook would then block Bash.

The installer creates missing stack-pack documentation files and regenerates
the documentation map for each independent checkout. It leaves existing files
untouched. When handbook chapter numbers conflict, it reports the conflict
and waits for a reviewed `pf knowledge plan` before importing. Existing
sections are never appended by installation; use `pf knowledge init` after
reviewing its plan. A workspace root receives cross-repository guidance,
while each child repository owns its stack and SQL migration documentation.
If a project already has model routes and Hermes is installed, installation
also trusts its shared project skill in Hermes.

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

## Optional host setup

Use `--install-graphify` to provision the pinned Graphify CLI via `uv` when
missing. A first interactive project install prompts for model routes, and
`--configure-models` reopens setup. In automation, run `pf providers setup`
with explicit flags after installing. [Model routing](../guide/01-getting-started.md#model-routing)

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Components](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
