# 06 — Troubleshooting

**Status:** Active
**Last updated:** 2026-10-19

Start with `scripts/doctor.sh <product>`. It catches most setup problems. Then
find your symptom below.

## Setup

| Symptom | Cause | Fix |
|---|---|---|
| `/factory` is not recognised, or Claude answers without running the pipeline | The skill is not installed where this session looks, or the session started before installation | Run `scripts/install.sh user` or `project <path>`, then **start a new Claude Code session**. Skills and agents load at session start. |
| "Agent type 'engineer' not found" | Agents are missing from `.claude/agents/` and `~/.claude/agents/` | Re-run the installer, and start a new session |
| `git clone git@github.com:…` fails with `Permission denied (publickey)` | No SSH key on GitHub | Run `gh auth login` and choose SSH, or `gh ssh-key add ~/.ssh/id_ed25519.pub` |
| The installer prints "jq not found" | `jq` missing | Install it (see getting started), then re-run. Or paste the printed hook block into `.claude/settings.json` yourself |
| The installer says "<path> is not a git repo" | Target folder is not a repository | Run `git init` there first, or point at the repository root |
| `doctor.sh`: "drift: … differs from factory kit" | The project copy is older (or newer, if edited) than the factory | Review the diff (`git diff --no-index`), then re-run `install.sh project`. Edit the factory, not the copy |

## Guard hook

| Symptom | Cause | Fix |
|---|---|---|
| `factory-guard: Merging is human-only…` when **you** asked Claude to merge | The guard applies to every session in a project that has the kit | Merge in the GitHub/GitLab UI or your own terminal. For a deliberate human session, start Claude with `FACTORY_GUARD=off claude`. Never set it for factory runs |
| `factory-guard: The factory pushes only factory/* branches` | Something tried to push another branch | Intended. If a job did this, it is a bug, so report it via `/factory retro` |
| `factory-guard: unreadable hook input; blocking to fail closed` | Claude Code sent malformed hook input, or the hook ran with the wrong interpreter | Check that `python3 --version` works in the shell Claude uses. Run `python3 tests/guard_test.py` in the factory repo |
| The hook never fires (merge commands go through) | Not wired in `.claude/settings.json`, or settings changed mid-session | Run `doctor.sh <product>`, check `/hooks` in an interactive `claude` session, and restart the session |

## Jobs

| Symptom | Cause | Fix |
|---|---|---|
| "Worktree setup failed" | The branch or worktree already exists from an earlier run, or setup commands in `AGENTS.md` fail | `git worktree list`. Remove the stale one with `git worktree remove <path>` if its job is finished, and fix the setup commands in `AGENTS.md` |
| The job stops: "file missing" or "first line is not a verdict" | An agent ran out of context, errored, or wrote the wrong format | Read the agent's file (if any). Re-run with `/factory rework <job> "re-run <stage>"`. If it recurs, run `/factory retro` |
| The dev server fails in the engineer, UX or UI stage: "address in use" | A port is still taken by an earlier job or your own server | Stop the old process (`lsof -i :<port>`), or change `surfaces.ui.port_base` |
| Reviewers keep asking for changes the engineer "already did" | The engineer committed to the wrong branch, or did not commit | Check `git -C <worktree> log --oneline <base>..HEAD`. The engineer must commit on `factory/<job-id>` |
| UX/UI reviews say PASS without looking | No browser CLI installed, or `AGENTS.md` has no dev-server command | Install the browser tool and add a "Dev server" section to `AGENTS.md` |
| Every bug job ends `NOT REPRODUCED` | The tester cannot run the app the way users do | Add run instructions, test accounts or fixtures to `AGENTS.md` |
| A job sits in one stage with no activity | The coordinator session was closed | Open Claude Code and run `/factory status`. Then `/factory rework <job> "resume from <stage>"`. Job state is on disk, not in the closed session |
| Two sessions claimed the same backlog line | Someone edited the backlog by hand during a claim | Close one session. Claims use exact-line edits, so give each line unique text |

## PRs

| Symptom | Cause | Fix |
|---|---|---|
| `gh pr create` fails with "must first push" | The push was blocked or failed | Read the push error. Check `vcs.remote` in the descriptor and `gh auth status` |
| The PR cannot be created ("no commits between") | The engineer made no commits | Read `build.md`. The job may be BLOCKED, or the change already existed |
| `glab: command not found` on a GitLab project | GitLab CLI missing | Install `glab` and run `glab auth login` |
| The PR shows merge conflicts | The base branch moved while the job ran | `/factory rework <job> "rebase on <base> and resolve conflicts"`. The engineer resolves it on the job branch |

## Dashboard

| Symptom | Cause | Fix |
|---|---|---|
| The dashboard is empty | The server was started in the wrong folder | Run `python3 -m http.server` **inside `.factory/`**, not the repo root |
| Opening `dashboard.html` as a file shows nothing | Browsers block `fetch` from `file://` | Use the HTTP server |

<!-- documentation-routing:start -->
## Documentation routing

- [Parent README](README.md)
- [Governing agent guide](../../AGENTS.md)
<!-- documentation-routing:end -->
