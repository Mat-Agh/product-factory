# 02 — Repository layout

**Status:** Active
**Last updated:** 2026-10-19

```
product-factory/
├── README.md                     entry point; links docs/README.md
├── AGENTS.md / CLAUDE.md         instructions for agents working on this repo
├── kit/                          ── payload installed into products (not docs) ──
│   ├── AGENTS.factory.md         section appended to a product's AGENTS.md (COMP-007)
│   ├── .claude/
│   │   ├── skills/factory/
│   │   │   ├── SKILL.md          coordinator procedure (COMP-001)
│   │   │   ├── protocol.md       shared agent rules (COMP-002)
│   │   │   └── CONSTITUTION.md   → link to docs/governance/CONSTITUTION.md (copied as a file on install)
│   │   ├── agents/*.md           10 role agents (COMP-003)
│   │   ├── rules/product-factory.md  always-on session rule (COMP-016)
│   │   └── settings.factory.json `pf guard` hook wiring (COMP-004)
│   └── .factory/
│       ├── factory.yaml          descriptor template (COMP-005)
│       ├── backlog.md            backlog template (COMP-006)
│       └── dashboard.html        dashboard (COMP-006)
│   └── knowledge/                project scaffolds (base/ + stacks/<name>/) and each stack's pack.map into docs/knowledge/ (COMP-018)
├── cli/                          ── the `pf` Rust CLI (COMP-015) ──
│   ├── Cargo.toml, Cargo.lock
│   └── src/                      main, guard, docs_map, diagrams, install, doctor, backup, dashboard, descriptor, rules, memory, util
└── docs/                         ── all documentation ──
    ├── README.md                 home: start here, areas, rules, generated map
    ├── governance/               CONSTITUTION (COMP-013)
    ├── guide/                    operator handbook (+ examples/)
    ├── architecture/             overview, job lifecycle, sources
    ├── components/               COMP-NNN contracts
    ├── knowledge/                development knowledge library by language and stack (COMP-018)
    ├── development/              this handbook, CONTRIBUTING
    ├── decisions/                ADR-NNN
    ├── status/                   CURRENT_STATE, IMPLEMENTATION_STATUS, ROADMAP, EVOLUTION_LOG
    ├── adapters/                 Hermes, Codex/Cursor/other (COMP-011)
    └── operator/                 USER.md operator profile (COMP-010)
```

Every folder under `docs/` has a README, and `pf docs-map` generates the links
between them. See [COMP-012](../components/COMP-012-docs-gate.md).

## In a product after installation

```
<product>/
├── AGENTS.md                     + "## Product Factory" section
├── docs/                         all product docs (PRDs, ADRs, design, launch…), README per folder
├── CLAUDE.md                     @AGENTS.md
├── .claude/{skills/factory, agents, hooks, settings.json}   copied kit (commit it)
└── .factory/
    ├── factory.yaml, backlog.md, dashboard.html, installed.lock   (commit them)
    └── jobs/                     runtime state (gitignored)
<product>/../<product>-factory/<job-id>/   worktrees on factory/<job-id>
```

## Ignored on purpose

`.env`, `*.local`, `.claude-export/` (review it, then delete it),
`__pycache__/`, `.factory/jobs/`.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Factory development handbook](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
