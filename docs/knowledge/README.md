# Development knowledge

**Status:** Active
**Last updated:** 2026-10-20

General engineering knowledge that applies to any project on these stacks,
collected from established engineering practices and written without product-specific names.
It is the single source: `pf knowledge init` imports the parts a repository's stack
needs ([how](#how-projects-get-it)).

## By language and stack

| Area | Covers |
|---|---|
| [principles/](principles/README.md) | Engineering principles, design discipline, evidence and "done", documentation |
| [kotlin/](kotlin/README.md) | Kotlin language, Gradle build, the **client** (KMP, Android, iOS, Compose MVI) and the **backend** (Spring Boot services) |
| [typescript/](typescript/README.md) | TypeScript, and static **web** apps (Next.js, React, pnpm, Playwright) |
| [rust/](rust/README.md) | Rust CLI tooling (the default for scripts and tools) |
| [api/](api/README.md) | HTTP conventions, errors, events: contracts shared by every stack |
| [security/](security/README.md) | Data classification, cryptography policy, transport and identity, threat modeling |
| [product/](product/README.md) | Product documentation, requirements and delivery, multi-repo product model |
| [cicd/](cicd/README.md) | GitFlow, merge-request approval, pipeline design, supply chain, runners, deployment |
| [agentic/](agentic/README.md) | The agent factory workflow, agent instructions, knowledge and memory, safety, workspaces |

Not yet gathered: Ktor **server** (Ktor is used only as the KMP HTTP client), iOS
release and background behaviour, and stacks not used so far. Add a page when a
product uses them; do not write knowledge that has not been earned.

## How projects get it

Each stack pack in `kit/knowledge/stacks/<stack>/pack.map` lists the library pages
a repository of that stack imports and where they land: stack chapters become the
numbered handbook (`docs/development/00-…`, `03-…`), and the cross-cutting areas
land under `docs/development/knowledge/`. `pf knowledge plan` shows what a repo
lacks; `pf knowledge init` creates missing pages and appends missing sections to
existing ones, never overwriting. Contract: COMP-018 in the components docs.

## Changing it

Edit the page here, in a factory change. Keep it generic: no product, company, host
or person names; write repo-specific commands as `<…>`. Projects pick up changes
with `pf knowledge plan` / `init` after `git pull` in the factory checkout.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
- Subfolders:
  - [Agentic workflows](agentic/README.md)
  - [API contracts](api/README.md)
  - [CI/CD](cicd/README.md)
  - [Kotlin](kotlin/README.md)
  - [Principles](principles/README.md)
  - [Product](product/README.md)
  - [Rust](rust/README.md)
  - [Security](security/README.md)
  - [TypeScript](typescript/README.md)
<!-- documentation-routing:end -->
