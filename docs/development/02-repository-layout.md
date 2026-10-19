# 02 — Repository layout

**Status:** Active
**Last updated:** 2026-10-19

```
product-factory/
├── README.md                     entry point, quick start
├── CONSTITUTION.md               the rules (COMP-013)
├── CURRENT_STATE.md              one-page state of the repository
├── CONTRIBUTING.md               branches, commits, PRs, review
├── AGENTS.md / CLAUDE.md         instructions for agents working on this repo
├── kit/                          ── installed into products ──
│   ├── AGENTS.factory.md         section appended to a product's AGENTS.md (COMP-007)
│   ├── .claude/
│   │   ├── skills/factory/
│   │   │   ├── SKILL.md          coordinator procedure (COMP-001)
│   │   │   └── protocol.md       shared agent rules (COMP-002)
│   │   ├── agents/*.md           10 role agents (COMP-003)
│   │   ├── hooks/factory-guard.py  guard hook (COMP-004)
│   │   └── settings.factory.json hook wiring snippet
│   └── .factory/
│       ├── factory.yaml          descriptor template (COMP-005)
│       ├── backlog.md            backlog template (COMP-006)
│       └── dashboard.html        dashboard (COMP-006)
├── operator/USER.md              operator profile (COMP-010)
├── adapters/                     Hermes, Codex/Cursor/other (COMP-011)
├── scripts/
│   ├── install.sh                COMP-007
│   ├── doctor.sh                 COMP-008
│   ├── mirror.sh, export-claude.sh  COMP-009
│   └── check-docs.py             COMP-012
├── tests/guard_test.py           guard hook tests
└── docs/
    ├── README.md                 router
    ├── guide/                    operator handbook
    ├── architecture/             overview, job lifecycle, sources
    ├── components/               COMP-NNN contracts + IMPLEMENTATION_STATUS
    ├── development/              this handbook
    ├── decisions/                ADR-NNN
    ├── ROADMAP.md, EVOLUTION_LOG.md
    └── DOCUMENT_INVENTORY.md     generated
```

## In a product after installation

```
<product>/
├── AGENTS.md                     + "## Product Factory" section
├── CLAUDE.md                     @AGENTS.md
├── .claude/{skills/factory, agents, hooks, settings.json}   copied kit (commit it)
└── .factory/
    ├── factory.yaml, backlog.md, dashboard.html, installed.lock   (commit them)
    └── jobs/                     runtime state (gitignored)
<product>/../<product>-factory/<job-id>/   worktrees on factory/<job-id>
```

## Ignored on purpose

`.env`, `*.local`, `operator/claude-export/` (review it, then delete it),
`__pycache__/`, `.factory/jobs/`.

<!-- documentation-routing:start -->
## Documentation routing

- [Parent README](README.md)
- [Governing agent guide](../../AGENTS.md)
<!-- documentation-routing:end -->
