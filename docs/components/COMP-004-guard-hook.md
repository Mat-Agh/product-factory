# COMP-004 — Guard hook

**Status:** Implemented in Rust — 80/80 policy cases pass (the 73 original cases behave identically to the Python original); project branch template and hook-bypass blocking added; not yet observed inside a live Claude Code session
**Last updated:** 2026-10-19
**Source:** [`cli/src/guard.rs`](../../cli/src/guard.rs), wiring in [`kit/.claude/settings.factory.json`](../../kit/.claude/settings.factory.json)
**Decisions:** [ADR-002](../decisions/ADR-002-human-only-merge.md)

## Purpose

This is the mechanical second line behind CONSTITUTION §5. `pf guard` is a
Claude Code `PreToolUse` hook on the `Bash` tool that rejects commands reserved
for humans or forbidden outright. Branch protection on the remote is the first
line.

## Interface

- **Input:** the hook JSON on stdin, `{"tool_name": "Bash", "tool_input": {"command": "…"}}`.
- **Output:** exit `0` allows the command. Exit `2` blocks it, and the reason on
  stderr is shown to the model.
- **Wiring** (merged into `.claude/settings.json` by `pf install`; the file is
  never overwritten):

  ```text
  command -v pf >/dev/null 2>&1 || { echo '… pf CLI is not installed, so Bash is blocked (fail closed) …' >&2; exit 2; }; pf guard
  ```

  Without `pf` on `PATH`, every Bash call is blocked with an install hint. The
  hook fails closed, never open.
- **Override:** start the session with `FACTORY_GUARD=off`. It is read from the
  hook's own environment, so an inline `FACTORY_GUARD=off git …` inside a
  command has no effect.

## Policy

| Blocked | Examples |
|---|---|
| Merging | `git merge <x>` (except `--abort`, `--quit` and `merge-tree`), `gh pr merge`, `glab mr merge` |
| Approving | `gh pr review --approve`, `glab mr approve` |
| Tags and releases | `git tag <name>`, `git push --tags`, `gh release create\|edit\|delete\|upload`, `glab release create` |
| Force and ref rewriting | `git push -f/--force/--force-with-lease/+ref`, `update-ref`, `filter-branch`, `filter-repo` |
| Pushing anything but `factory/*` | `git push origin main`, `HEAD:main`, `develop`, `release/*`, bare `git push`, `git push origin HEAD` |
| Deleting remote refs | `--delete`, `-d`, `:ref` |
| Skipping hooks | `--no-verify`, `git commit -n` |
| Discarding work | `reset --hard`, `clean -f`, `stash` (push, save, drop, clear), `checkout -- .`, deleting non-`factory/*` branches |
| Repo, secrets and workflows | `gh repo delete\|archive\|rename\|edit`, `gh secret\|variable`, `gh workflow run\|enable\|disable`, `gh api` PUT/PATCH/DELETE on merge, protection, releases, refs or secrets |
| Destructive rm | `rm -rf` of `/`, `~`, `$HOME`, `.` or `..` |

The parser:
- splits on `;`, `&&`, `||`, `|`, newlines, `$(` and backticks;
- strips leading `VAR=` assignments and wrappers (`env`, `command`, `sudo`,
  `xargs`, …);
- skips git global options (`-C`, `-c`, …);
- ignores heredoc bodies.

Malformed hook input is blocked, so the hook fails closed.

## Verification

```bash
pf guard --self-test                       # 80/80
cd cli && cargo test --release guard       # the same cases as a unit test
```

`pf doctor` runs the same self-test. The cases are the `BLOCK` and `ALLOW`
lists in `cli/src/guard.rs`.

## Known limitations

- It is not a sandbox. A process with a shell can still reach git or the host
  API in ways the parser does not recognise. Branch protection is the control
  that holds.
- It applies to every session in a project that has the kit, including your
  own. Use `FACTORY_GUARD=off` for deliberate human sessions.
- It does not see commands run by tools other than `Bash`.

## Change checklist

1. Add the case to `BLOCK` or `ALLOW` in `cli/src/guard.rs`.
2. Change the policy, then run `cargo test`.
3. Update the policy table above and the evolution log.
4. Reinstall `pf` (`cargo install --path cli`).

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Components](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
