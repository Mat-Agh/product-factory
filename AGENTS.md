# Agent instructions — Product Factory repo

This repo is the factory itself, not a product. It holds rules
(`CONSTITUTION.md`), the installable kit (`kit/`), the operator profile
(`operator/`), adapters, scripts and docs. Read `README.md` first, then route
through [`docs/development/16-documentation-link-map.md`](docs/development/16-documentation-link-map.md).
Current state: [`CURRENT_STATE.md`](docs/status/CURRENT_STATE.md). How to change things:
[`CONTRIBUTING.md`](docs/development/CONTRIBUTING.md).

## Working on the factory

- `CONSTITUTION.md` wins over every other file. A rule change starts there.
- Roles live in **one place**: `kit/.claude/agents/*.md`. The orchestration
  procedure lives in `kit/.claude/skills/factory/SKILL.md`, and the shared agent
  rules live in `kit/.claude/skills/factory/protocol.md`. Do not restate them in
  docs. Link to them.
- **All documentation lives under `docs/`**, in a folder per area, each with a
  README. Only `README.md`, `AGENTS.md` and `CLAUDE.md` stay at the root. After
  adding or moving a doc, run `pf docs-map --fix --strict --allow kit/`,
  which regenerates every README's links and the map in `docs/README.md`.
  Diagrams are inline ` ```mermaid ` blocks in the Connect style
  (`docs/knowledge/principles/documentation.md`, Diagrams), never images.
- General engineering knowledge (any stack, any project) lives in `docs/knowledge/`,
  by language and stack. It is the single source: projects import it through
  `kit/knowledge/stacks/<stack>/pack.map`. Keep it generic (no product, company,
  host or person names). A new page must be mapped by at least one pack.
- Keep the kit project-agnostic. Anything about a specific product belongs in
  that product's `AGENTS.md` or `.factory/factory.yaml`, never here.
- Every factory-level change updates, **in the same commit**, the docs listed for
  it in `docs/development/03-changing-the-factory.md`, the affected rows of
  `docs/status/IMPLEMENTATION_STATUS.md`, and adds a dated entry to
  `docs/status/EVOLUTION_LOG.md`.
- **Tooling is Rust.** All factory tooling is the `pf` CLI in `cli/`. Add new tools
  as `pf` subcommands, not scripts (operator rule, `docs/operator/USER.md`).
- After changing the guard, add the case to `BLOCK`/`ALLOW` in `cli/src/guard.rs`.
  After changing `cli/`, run `cargo install --path cli` so the installed `pf` matches.

## Gates

```bash
(cd cli && cargo test --release && cargo clippy --release && cargo fmt --check)
pf guard --self-test
pf diagrams check   # after adding or editing a diagram: pf diagrams validate
pf docs-map --strict --allow kit/   # --fix after adding/moving a doc
pf doctor
```

## Do not

- Commit secrets, `.env`, `.claude-export/` (review it first and fold it
  into `USER.md`), or any product's job folders.
- Commit unless asked. No `--no-verify`. No force-push to `main`.
- Copy product docs into this repo.
