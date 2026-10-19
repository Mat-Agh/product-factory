# {{MODULE}} development handbook

**Status:** Active
**Last updated:** {{DATE}}

The engineering handbook for {{MODULE}}, a TypeScript web workspace (pnpm
monorepo of browser apps and shared packages). Agents route through
[16-documentation-link-map.md](16-documentation-link-map.md).

## Start here

| Step | Document | Purpose |
|---|---|---|
| 1 | `AGENTS.md` (repo root) | Scope, workflow, gates |
| 2 | [16-documentation-link-map.md](16-documentation-link-map.md) | The smallest doc set for the task |
| 3 | [00-engineering-principles.md](00-engineering-principles.md) | Invariants |
| 4 | [09-tools-scripts-and-commands.md](09-tools-scripts-and-commands.md) | Commands |
| 5 | [15-code-samples-index.md](15-code-samples-index.md) | Living samples |

## Full index

| # | Document | Scope |
|---|---|---|
| 00 | [engineering-principles](00-engineering-principles.md) | Boundaries, engineering rules, quality gate |
| 01 | [app-architecture](01-app-architecture.md) | Workspace layout, rendering model, runtime flow, source layout |
| 02 | [typescript-language](02-typescript-language.md) | Strictness, types at boundaries, naming |
| 03 | [ui-design-system](03-ui-design-system.md) | Tokens, components, states, accessibility, RTL |
| 04 | [api-contracts-clients](04-api-contracts-clients.md) | Same-origin client, contract change order |
| 05 | [security](05-security.md) | Threat model, CSRF, CSP, secrets, supply chain |
| 06 | [testing](06-testing.md) | Layers, completion gate |
| 07 | [build-deployment](07-build-deployment.md) | Static artifact, integrity manifest, container, signing |
| 08 | [library-stack](08-library-stack.md) | Each choice, why, and what we never do |
| 09 | [tools-scripts-and-commands](09-tools-scripts-and-commands.md) | Commands and setup |
| 15 | [code-samples-index](15-code-samples-index.md) | Living samples |
| 16 | [documentation-link-map](16-documentation-link-map.md) | Agent routing |
| — | [REVIEW_CHECKLIST](REVIEW_CHECKLIST.md) | Before approving a material UI change |
| — | [CICD](CICD.md) | Branching, conventions, pipeline |
