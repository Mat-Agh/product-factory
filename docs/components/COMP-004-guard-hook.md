# COMP-004 — Guard hook

**Status:** Implemented — unit-tested (73/73); not yet observed inside a live Claude Code session
**Last updated:** 2026-10-19
**Source:** [`kit/.claude/hooks/factory-guard.py`](../../kit/.claude/hooks/factory-guard.py), [`kit/.claude/settings.factory.json`](../../kit/.claude/settings.factory.json)
**Tests:** [`tests/guard_test.py`](../../tests/guard_test.py)
**Decisions:** [ADR-002](../decisions/ADR-002-human-only-merge.md)

## Purpose

This is the mechanical second line behind CONSTITUTION §5. It is a Claude Code
`PreToolUse` hook on the `Bash` tool that rejects commands reserved for humans
or forbidden outright. Branch protection on the remote is the first line.

## Interface

- **Input:** the hook JSON on stdin, `{"tool_name": "Bash", "tool_input": {"command": "…"}}`.
- **Output:** exit `0` allows the command. Exit `2` blocks it, and the reason on
  stderr is shown to the model.
- **Wiring:** `.claude/settings.json` →
  `hooks.PreToolUse[{matcher: "Bash", hooks: [{type: "command", command: "python3 \"$CLAUDE_PROJECT_DIR/.claude/hooks/factory-guard.py\""}]}]`.
  The installer merges this entry and never overwrites the file.
- **Override:** start the session with `FACTORY_GUARD=off`. It is read from the
  hook's own environment, so an inline `FACTORY_GUARD=off git …` inside a
  command has no effect.

## Policy

| Blocked | Examples |
|---|---|
| Merging | `git merge <x>` (except `--abort`, `--quit`, and `merge-tree`), `gh pr merge`, `glab mr merge` |
| Approving | `gh pr review --approve`, `glab mr approve` |
| Tags and releases | `git tag <name>`, `git push --tags`, `gh release create\|edit\|delete\|upload`, `glab release create` |
| Force and ref rewriting | `git push -f/--force/--force-with-lease/+ref`, `update-ref`, `filter-branch`, `filter-repo` |
| Pushing anything but `factory/*` | `git push origin main`, `HEAD:main`, `develop`, `release/*`, bare `git push`, `git push origin HEAD` |
| Deleting remote refs | `--delete`, `-d`, `:ref` |
| Skipping hooks | `--no-verify`, `git commit -n` |
| Discarding work | `reset --hard`, `clean -f`, `stash` (push, save, drop, clear), `checkout -- .`, deleting non-`factory/*` branches |
| Repo, secrets and workflows | `gh repo delete\|archive\|rename\|edit`, `gh secret\|variable`, `gh workflow run\|enable\|disable`, `gh api` PUT/PATCH/DELETE on merge, protection, releases, refs or secrets |
| Destructive rm | `rm -rf` of `/`, `~`, `$HOME`, `.` or `..` |

The parser splits on `;`, `&&`, `||`, `|`, newlines, `$(` and backticks. It strips
leading `VAR=` assignments and wrappers (`env`, `command`, `sudo`, `xargs`, …),
skips git global options (`-C`, `-c`, …), and ignores heredoc bodies.

Malformed hook input is blocked, so the hook fails closed.

## Verification

```bash
python3 tests/guard_test.py      # 73/73 on 2026-10-19
```

`scripts/doctor.sh` runs the same suite.

## Known limitations

- It is not a sandbox. Any process with a shell can write files or call the git
  host API in ways the parser does not recognise (for example a Python script
  that calls `git merge`). Branch protection is the control that holds.
- It applies to every session in a project that has the kit, including your
  own. Use `FACTORY_GUARD=off` for deliberate human sessions.
- It does not see commands run by tools other than `Bash`.

## Change checklist

Add a BLOCK or ALLOW case to `tests/guard_test.py` first, then change the hook,
then update the policy table above and the evolution log.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Components](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
