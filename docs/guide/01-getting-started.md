# 01 — Getting started

**Status:** Active
**Last updated:** 2026-10-19

This page takes you from an empty machine to a first pull request opened by the
factory. Follow it top to bottom. Each step gives the command to run and what you
should see. If you see something else, go to
[troubleshooting](06-troubleshooting.md).

Time: about 30 minutes for setup, then 10–40 minutes for the first job,
depending on its size.

## 1. What you need

| Need | Why | Check |
|---|---|---|
| A Claude account with Claude Code access (Pro, Max, Team or Enterprise, or an API key) | Runs the agents | `claude --version` |
| A GitHub (or GitLab) account | Hosts the factory repo and your product, and receives the PRs | — |
| `git` 2.40+ | Branches and worktrees | `git --version` |
| `gh` (GitHub CLI), or `glab` for GitLab | Opens PRs and reads issues | `gh --version` |
| `python3` 3.9+ | Guard hook, docs check, dashboard server | `python3 --version` |
| `jq` | The installer merges hook settings safely | `jq --version` |
| Node.js 18+ (only if you install Claude Code with npm, or your product is a web app) | Claude Code, dev servers | `node --version` |
| A browser-automation CLI (optional, for products with pages) | Lets the UX/UI reviewers and QA look at the running app | see step 2 |

### Install them

**Ubuntu / Debian**

```bash
sudo apt update && sudo apt install -y git python3 jq gh
```

On older releases where `apt` has no `gh` package, follow
<https://github.com/cli/cli/blob/trunk/docs/install_linux.md>.

**macOS** (with [Homebrew](https://brew.sh))

```bash
brew install git python jq gh
```

**Windows:** use WSL2 (`wsl --install` in an administrator PowerShell, then
reboot), and follow the Ubuntu steps inside WSL. The factory's scripts are bash.

**Claude Code** (any OS; see <https://docs.claude.com/en/docs/claude-code/setup>
for the current method):

```bash
npm install -g @anthropic-ai/claude-code
claude          # first run opens a browser to sign in; then type /exit
```

## 2. Connect GitHub

```bash
gh auth login        # choose GitHub.com → SSH → generate or pick a key → log in with browser
gh auth status       # expect: "Logged in to github.com account <you>"
```

Choosing SSH in `gh auth login` also uploads your SSH key, which `git clone
git@github.com:…` needs.

Optional, for products with pages: install the browser CLI named in your
product's `AGENTS.md`. The default is
[`agent-browser`](https://github.com/vercel-labs/agent-browser), installed by
following its README. Playwright works too (`npx playwright install`).

## 3. Get the factory

```bash
git clone git@github.com:Mat-Agh/product-factory.git "$HOME/Work/ProductFactory"
cd "$HOME/Work/ProductFactory"
scripts/install.sh user
```

You should see:

```
Linking Product Factory @ <commit> into /home/<you>/.claude
  linked /factory skill, agents and guard (they follow 'git pull' in /home/<you>/Work/ProductFactory)
  imported operator/USER.md into /home/<you>/.claude/CLAUDE.md
  user-wide guard hook not enabled (set WITH_USER_GUARD=1 to apply it in every project)
Done.
```

`/factory` and the ten role agents are now available in every Claude Code
session on this machine. Your working preferences (`operator/USER.md`) load into
every session.

## 4. Check the machine

```bash
scripts/doctor.sh
```

Every line should say `ok` or `note`. A `FAIL` line names what is missing. Fix
it and run the check again. `note` lines are suggestions, such as adding a
mirror remote.

## 5. Prepare a product repo

Use a real product, or start a tiny one for practice:

```bash
mkdir -p ~/Work/hello-factory && cd ~/Work/hello-factory
git init -b main && echo "# Hello Factory" > README.md
git add README.md && git commit -m "Initial commit"
gh repo create hello-factory --private --source . --remote origin --push
```

Install the kit into it:

```bash
~/Work/ProductFactory/scripts/install.sh project ~/Work/hello-factory
```

The installer copies `.claude/` (skill, agents, guard hook) and `.factory/`
(descriptor, backlog, dashboard), appends a Product Factory section to
`AGENTS.md`, and commits nothing.

## 6. Describe the product to the factory

The agents know nothing about your product beyond two files:

- **`AGENTS.md`**: how to set up, run, check and write code in this repo. Copy
  the structure of [examples/AGENTS.example.md](examples/AGENTS.example.md) and
  replace the commands with your real ones. The factory works only as well as
  this file is written.
- **`.factory/factory.yaml`**: where things are and what the policy is. Let
  Claude draft it:

```bash
cd ~/Work/hello-factory && claude
```
```
/factory init
```

Claude inspects the repo, fills `.factory/factory.yaml`, marks guesses with
`# verify`, and lists anything `AGENTS.md` is missing. Fix those items, then
quit Claude and run:

```bash
~/Work/ProductFactory/scripts/doctor.sh ~/Work/hello-factory
git add -A && git commit -m "Install Product Factory" && git push
```

**Protect the base branch** so nothing can merge without you. On GitHub, open
Settings → Branches → Add rule for `main`, and turn on "Require a pull request
before merging". Free private repos may not offer this, in which case the guard
hook is the only mechanical protection.

## 7. Run the first job

Start small. In Claude Code inside the product repo:

```
/factory Add a "Getting help" section to README.md that points to the issues page
```

You will see one line per step, for example:

```
001-readme-getting-help: triage → docs, trivial, surface none
001-readme-getting-help: build → STATUS: READY
001-readme-getting-help: round 1 — code PASS → gate
001-readme-getting-help: gate READY → PR https://github.com/<you>/hello-factory/pull/1
```

Open the PR, read the summary and evidence, and merge it yourself if you agree.

To watch jobs as they run, open a second terminal:

```bash
cd ~/Work/hello-factory/.factory && python3 -m http.server
```

Then open <http://localhost:8000/dashboard.html>.

## 8. Next steps

- Read [03-reading-a-job.md](03-reading-a-job.md) before your first larger job.
- Put a few requests in `.factory/backlog.md` and use `/factory next`.
- Run `/factory retro 001-readme-getting-help` to see how the factory proposes
  improvements to itself.
- Set up a mirror now, before you need one:
  [05-portability-and-recovery.md](05-portability-and-recovery.md).

<!-- documentation-routing:start -->
## Documentation routing

- [Parent README](README.md)
- [Governing agent guide](../../AGENTS.md)
<!-- documentation-routing:end -->
