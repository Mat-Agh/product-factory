# 08 — Run Product Factory through Hermes and Honcho

**Status:** Active
**Last updated:** 2026-11-16

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

For Hermes + 9Router, provide these values:

| Prompt | Value | Purpose |
|---|---|---|
| `deep` | `router:hermes-deep` | GPT-first, Gemini-second decisions, planning and review |
| `build` | `router:hermes-build` | Gemini-first, GPT-second, Qwen worker route |
| `economy` | `router:hermes-economy` | Gemini-first, GPT-second, Qwen low-cost worker route |

These aliases are not credentials. The installer writes them to
`.factory/model-routes.json`, trusts the shared project skill in Hermes when
available, and leaves router secrets in the local Hermes scope. In automation:

```bash
pf providers --root /path/to/product setup \
  --deep router:hermes-deep \
  --build router:hermes-build \
  --economy router:hermes-economy
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

Open Connect in Hermes, or start a chat rooted in the workspace. Hermes reads
the installed `AGENTS.md` and routes work through the Factory coordinator.

```bash
cd "$HOME/Work/Connect"
hermes skills trust .             # normally performed when routes already exist
pf dashboard                      # open http://127.0.0.1:8000/dashboard.html
```

Give the coordinator a product request, then use `pf job status` to see its
durable stage. Use `pf providers run <job> <role> --allow-api` only after you
approve billable routed work. A role switch preserves Factory job state in
`.factory/jobs/<id>/`; Honcho adds guarded cross-session operator context, but
does not replace job artifacts or repository documentation.

## Verify before the first task

```bash
pf doctor "$HOME/Work/Connect"
pf providers --root "$HOME/Work/Connect" check
cd "$HOME/Work/Hermes" && ./bin/agentctl test-routes
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
