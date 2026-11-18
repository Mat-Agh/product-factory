# 08 — Run Product Factory through Hermes and Honcho

**Status:** Active
**Last updated:** 2026-11-17

Use this guide when Hermes is the agent host and its optional **Honcho** service
provides durable, guarded context between role changes. There is no Factory
component named “Hunter”; this guide uses the intended Honcho name.

## Boundary of responsibility

Product Factory installs the project kit, project instructions, role prompts,
durable job artifacts, model-route choices and local code graph. Hermes owns
the machine-level agent service, 9Router credentials, model sessions and
Honcho. This keeps private credentials and shared operator context out of a
product repository.

The installer never asks for or writes an API token into the project. Its only
interactive input is the three non-secret route names: deep decision-making,
build workers and economy workers. Hermes supplies their private credentials.

## One-time machine preparation

On the Hermes machine, bootstrap and verify the harness:

```bash
cd "$HOME/Work/Hermes"
./bin/provision-router-key     # validates 9Router and scopes its key to every Hermes role
./scripts/apply-routing.sh     # applies deep/build/economy role ordering
./bin/agentctl verify          # requires Honcho and write approval when Honcho is enabled
```

`provision-router-key` writes the credential only to owner-private Hermes
runtime files. Honcho is optional for Factory job execution, but when enabled
it provides approved operator decisions and preferences across Hermes role
switches. Product truth, task state, review evidence and code documentation
remain in the product repository.

## Install a product interactively

Use the released `pf` binary and ask the installer to collect route names:

```bash
pf install project /path/to/product --install-graphify --configure-models
```

For Hermes, provide direct provider routes or router aliases:

| Prompt | Value | Purpose |
|---|---|---|
| `deep` | `fableai:claude-fable-5-2` | Claude Fable 5.2 (Fable API), with DeepSeek fallback for reasoning, planning, and review |
| `build` | `deepseek:deepseek-v4-pro` | DeepSeek V4 Pro, with DeepSeek Flash and Claude fallbacks for build work |
| `economy` | `deepseek:deepseek-flash` | DeepSeek Flash, with Claude Haiku fallback for routine work |

These aliases and provider IDs are not credentials. Private API keys stay in the
Hermes environment (`HERMES_CUSTOM_FABLEAI_API_KEY`, `DEEPSEEK_API_KEY`). The
installer writes the routes to
`.factory/model-routes.json`, trusts the shared project skill in Hermes when
available, and leaves credentials in the local Hermes scope. In automation:

```bash
pf providers --root /path/to/product setup \
  --deep fableai:claude-fable-5-2 \
  --build deepseek:deepseek-v4-pro \
  --economy deepseek:deepseek-flash
pf providers --root /path/to/product check
```

## Connect production install

Connect is a multi-repository workspace. Reinstall it from the released `pf`
binary at its workspace root:

```bash
pf install project "$HOME/Work/Connect" --install-graphify --configure-models
pf doctor "$HOME/Work/Connect"
pf providers --root "$HOME/Work/Connect" check
```

The installer retains Connect’s descriptor, backlog, project documentation and
each child repository’s authority. It refreshes the installed kit, generated
role entry points, dashboard and graph; review the resulting Git changes before
committing them.

## Start work from Hermes

Open Connect in Hermes, or start a chat rooted in the workspace. The current
Hermes `default` profile uses `hermes-deep` through 9Router. That is the
coordinator route; you do not need to switch the model before asking it to
coordinate. The `software_engineer` and `tester` profiles use `hermes-build`,
and the Factory's installed model routes assign `deep`, `build` and `economy`
roles. A plain chat does not, by itself, create a Factory job.

```bash
cd "$HOME/Work/Hermes"
./bin/agentctl chat Connect       # starts a new Hermes chat rooted in Connect
```

The Connect checkout has `.agents/skills/product-factory/SKILL.md`. Project
skill trust was set during installation when routes were available. If Hermes
does not list or load that skill, run
`$HOME/Work/Hermes/bin/hermes skills trust "$HOME/Work/Connect"`, then start a
new chat. In the Hermes desktop app, open a chat for the Connect workspace;
the terminal command above is the unambiguous way to set its working directory.

For a safe first conversation, paste this into the new chat:

> Use the `product-factory` project skill in this Connect workspace. Read the
> installed coordinator and `AGENTS.md`. Show the Factory status and available
> routes. Do not claim an issue or start a job yet.

When you are ready to assign work, give one bounded request:

> Use the `product-factory` skill to run this as a Factory job: [outcome].
> Keep these constraints: [constraints]. Acceptance: [observable checks].
> Work in the owning Connect child repository and show me the job ID and
> dashboard card.

The coordinator should triage and prepare the job, dispatch the assigned roles,
verify each artifact and eventually report a PR/MR or a specific blocker. It
should not implement the entire request in the coordinator chat. To inspect
progress in another terminal:

```bash
cd "$HOME/Work/Connect"
pf job status
pf dashboard                      # open http://127.0.0.1:8000/dashboard.html
```

Ask Hermes to “show Factory status” to inspect jobs in chat. For a stopped
job, ask it to “resume Factory job `<job-id>`” after clearing the blocker. For
an open PR needing changes, ask it to “rework Factory job `<job-id>`: [specific
feedback]”. Open the dashboard card to review the request, stage artifacts,
review verdicts, QA evidence and PR/MR. You decide whether to merge.

`pf providers check` confirms local route prerequisites, not a live HTTP 200
from each model. Use `./bin/agentctl test-routes` from the Hermes checkout for
a live model probe. A routed role uses `pf providers run <job> <role>
--allow-api` only after you authorize billable API use for that job. A role
switch preserves Factory task state in `.factory/jobs/<id>/`; Honcho adds
guarded cross-session operator context but does not replace job artifacts or
repository documentation.

## Verify before the first task

```bash
pf doctor "$HOME/Work/Connect"
pf providers --root "$HOME/Work/Connect" check
cd "$HOME/Work/Hermes" && ./bin/agentctl test-routes
pf graph --root "$HOME/Work/Connect" watch --modules
```

If a worker cannot see a previous decision, check `./bin/hermes honcho status`
and the Hermes memory guide first; do not put a token, private conversation or
transient task result into a project file.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Operator guide](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
