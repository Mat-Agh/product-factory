# 01 — Getting started

**Status:** Active
**Last updated:** 2026-10-19

This page takes you from an empty machine to a first pull request opened by the
factory. Follow it top to bottom. Each step gives the command to run and what you
should see. If you see something else, go to
[troubleshooting](06-troubleshooting.md).

Time: about 30 minutes for setup, then 10–40 minutes for the first job,
depending on its size.

If you already have the matching `pf` binary, put it on `PATH` and use
`pf install project <repo>`. Follow the source-build instructions below
when building the binary or setting up the checkout-linked user profile.

## 1. What you need

| Need | Why | Check |
|---|---|---|
| Claude Code or Codex with an available model | Runs the coordinator and installed roles | `claude auth status` or `codex login status` |
| A GitHub (or GitLab) account | Hosts the factory repo and your product, and receives the PRs | — |
| `git` 2.40+ | Branches and worktrees | `git --version` |
| `gh` (GitHub CLI), or `glab` for GitLab | Opens PRs and reads issues | `gh --version` |
| Rust toolchain (`cargo`) 1.89+ | Builds `pf`, the factory's one tool (guard hook, docs map, installer, doctor, dashboard) | `cargo --version` |
| A C linker (`build-essential` on Ubuntu, Xcode tools on macOS) | Needed by `cargo` to build `pf` | `cc --version` |
| Node.js 18+ (only if you install Claude Code with npm, or your product is a web app) | Claude Code, dev servers | `node --version` |
| A browser-automation CLI (optional, for products with pages) | Lets the UX/UI reviewers and QA look at the running app | see step 2 |

### Install them

**Ubuntu / Debian**

```bash
sudo apt update && sudo apt install -y git gh build-essential curl
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y   # Rust, via rustup
source "$HOME/.cargo/env"
```

On older releases where `apt` has no `gh` package, follow
<https://github.com/cli/cli/blob/trunk/docs/install_linux.md>.

**macOS** (with [Homebrew](https://brew.sh))

```bash
xcode-select --install          # C toolchain, once
brew install git gh rustup
rustup-init -y && source "$HOME/.cargo/env"
```

**Windows:** use WSL2 (`wsl --install` in an administrator PowerShell, then
reboot), and follow the Ubuntu steps inside WSL. The factory's CLI builds and runs there.

**Claude Code** (any OS; see <https://docs.claude.com/en/docs/claude-code/setup>
for the current method):

```bash
npm install -g @anthropic-ai/claude-code
claude          # first run opens a browser to sign in; then type /exit
```

## 2. Set up GitHub access

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
cargo install --path cli     # builds pf and puts it in ~/.cargo/bin (about a minute the first time)
pf --version                 # expect: pf 0.2.0
pf install user
```

You should see:

```
Linking Product Factory @ <commit> into /home/<you>/.claude
  linked /factory skill and agents (they follow 'git pull' in /home/<you>/Work/ProductFactory)
  imported docs/operator/USER.md into /home/<you>/.claude/CLAUDE.md
  user-wide guard hook not enabled (pass --with-guard to apply it in every project)
Done.
```

`pf` must stay on your `PATH` (`~/.cargo/bin`). The guard hook in every factory
project calls it, and it blocks Bash on purpose when `pf` is missing.

`/factory` and the ten role agents are now available in every Claude Code
session on this machine. Your working preferences (`docs/operator/USER.md`) load into
every session.

## 4. Check the machine

```bash
pf doctor
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
pf install project ~/Work/hello-factory --install-graphify
```

The installer adds all project entry points and appends a Product Factory
section to `AGENTS.md`. It drafts `.factory/factory.yaml` from the stack and
docs layout and adds the backlog and dashboard. The installer includes independent
checkouts by default. Use `--root-only` to limit it to the workspace root. `--install-graphify` provisions the pinned
Graphify CLI using `uv` if needed. Run `hermes skills trust` in each checkout
you want Hermes to trust, then reopen the project. Antigravity uses the shared
project skill and its installed Factory subagents.

## Model routing

On the first terminal install, Factory asks for a model route for each tier.
Run `pf providers setup` to revisit the choices, and `pf providers check` to
check the local prerequisites. In automation, pass `--build`, `--deep` and
`--economy` with the tiers you want to set. Each accepts `native` or
`provider:model-id`.

- `deep` covers product, design, tech lead, security and release roles.
  Routed workers request high reasoning effort.
- `build` covers engineering, QA and other review roles. Routed workers
  request medium effort.
- `economy` is optional and covers the code lens for trivial docs-only
  work with no security review. Routed workers request low effort.

Use model IDs or gateway aliases available on your account. Put credentials
in the environment of the IDE or terminal that launches `pf`.

| Route prefix | Local environment |
|---|---|
| `anthropic:` | `ANTHROPIC_API_KEY` |
| `openai-api:` | `OPENAI_API_KEY` |
| `gemini:` | `GEMINI_API_KEY` or `GOOGLE_API_KEY` |
| `router:` | `PF_ROUTER_URL` and `PF_ROUTER_API_KEY` |

For a local router, `pf` also accepts
`~/.config/product-factory/router.json` (or the same path under
`XDG_CONFIG_HOME`). It must be a regular, owner-private file (mode `0600`)
with `url` and `api_key` fields. Environment variables take precedence.
Hermes project registration can write this file from its existing private
9Router credential; the project checkout never stores the key.

The router endpoint can be [9Router](https://github.com/decolua/9router/blob/master/gitbook/content/en/integration/other-tools.md)
or an OpenAI-compatible gateway that supports tool calling. For a local 9Router,
set `PF_ROUTER_URL=http://localhost:20128/v1` and enter the tier aliases from
its dashboard. The endpoint belongs to the machine's environment so a project
checkout cannot redirect a key. Native sign-in is handled by each host.
Project configuration stores model routes in `.factory/model-routes.json`.

For configured routes, `pf providers run <job-id> <role> --allow-api` launches
an independent Hermes worker in the job's module worktree. The coordinator
supplies `--allow-api` after the operator approves billable use. The worker
uses the installed role instructions and writes the normal stage artifact.
`pf job advance` remains the gate. Configured routes require Hermes on PATH,
and a missing dependency or credential blocks the role. `pf providers check`
checks local configuration. Model entitlement, tool support and the actual
cost depend on the provider. The worker has a turn and time limit.

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

In Codex, use `$product-factory init` in a new session. The coordinator verifies
the descriptor against the repo, flags assumptions and lists gaps in
`AGENTS.md`. Use `pf knowledge plan` to review the handbook and templates
before importing them. Fix those items, then
quit Claude and run:

```bash
pf doctor ~/Work/hello-factory
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
cd ~/Work/hello-factory && pf dashboard
```

Then open <http://127.0.0.1:8000/dashboard.html>.

## 8. Next steps

- Read [03-reading-a-job.md](03-reading-a-job.md) before your first larger job.
- Put a few requests in `.factory/backlog.md` and use `/factory next`.
- Run `/factory retro 001-readme-getting-help` to see how the factory proposes
  improvements to itself.
- Set up a mirror now, before you need one:
  [05-portability-and-recovery.md](05-portability-and-recovery.md).

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Operator guide](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
