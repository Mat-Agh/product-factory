# 04 — Testing and verification

**Status:** Active
**Last updated:** 2026-10-19

The factory has three levels of verification. Record every result in
[IMPLEMENTATION_STATUS](../status/IMPLEMENTATION_STATUS.md) with a date.

## 1. Static and unit (every commit)

```bash
(cd cli && cargo test --release && cargo clippy --release && cargo fmt --check)   # unit tests incl. 73 guard cases
pf guard --self-test                 # the installed pf agrees with the source
pf docs-map --strict --allow kit/    # docs tree, links, map
pf diagrams check                    # diagrams match their sources
pf doctor                            # machine + factory repo health
```

All five must pass before a commit. After changing `cli/`, reinstall with
`cargo install --path cli` so the installed `pf` matches the source.

## 2. Installer smoke test

Run it whenever `cli/src/install.rs`, `cli/src/doctor.rs` or the kit layout changes.

```bash
T=$(mktemp -d); mkdir "$T/app" && cd "$T/app" && git init -q -b main \
  && echo "# App" > AGENTS.md && git add -A && git -c user.email=t@t -c user.name=t commit -qm init && cd -
pf install project "$T/app"            # first install
pf install project "$T/app"            # re-install: must keep descriptor/backlog, not duplicate hook or AGENTS section
CLAUDE_CONFIG_DIR="$T/home" pf install user
CLAUDE_CONFIG_DIR="$T/home" pf doctor "$T/app"   # expect 0 failures
grep -c '^## Product Factory' "$T/app/AGENTS.md"          # expect 1
grep -c 'pf guard' "$T/app/.claude/settings.json"         # expect 1 (one hook entry)
rm -rf "$T"
```

## 3. End-to-end pilot

This is the only level that verifies the coordinator and agents. Run it in the
sandbox (or on a real product, with your approval) after any change to the
skill, protocol or agents. Phase 1 of the roadmap requires all of the following
on a real product.

| # | Job | Proves |
|---|---|---|
| P1 | `/factory init` | descriptor drafting, AGENTS gap report, hook and protection checks |
| P2 | trivial docs change | trivial lane, code lens only, gate, PR opens |
| P3 | standard feature with `surface: ui` | spec → design → plan → build → 4 lenses in parallel → QA → gate; ports and browser sessions |
| P4 | a seeded review failure (for example "do not update docs") | CHANGES → `SendMessage` resume → round 2 → PASS |
| P5 | a bug | `repro.md` before any fix; QA confirms repro steps now pass |
| P6 | a change under `risk.escalate_paths` | gate ESCALATE → draft PR with "For the human" |
| P7 | the guard | ask the session to `git merge` or `gh pr merge` and confirm it is blocked |
| P8 | two sessions with `/factory next` | distinct backlog lines and job ids |
| P9 | `/factory clean` after merging P2 | worktree removed, branch deleted, `done` |
| P10 | `/factory retro` on P3 | proposals, plus an offered PR to the factory |

For each one, record the job id, PR URL, verdict first lines
(`head -n1 .factory/jobs/<id>/*.md .factory/jobs/<id>/round-*/*.md`) and any
deviation from the documented behaviour. A deviation becomes either a factory
fix or a documented limitation.

## Recording evidence

In IMPLEMENTATION_STATUS, change the component's "Verified" cell and add an
"Evidence" note: the date, the command or job id, and the result. Add an
evolution-log line when a pilot completes.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Factory development handbook](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
