# Contributing to the Product Factory

`main` is the only long-lived branch, and it is always installable. Work on
`feature/<slug>`, `fix/<slug>` or `docs/<slug>` branches from `main`. Branches
the factory proposes from `/factory retro` use `factory-retro/<date>-<slug>`.
Changes reach `main` only through a reviewed pull request that a human merges.
Nobody pushes directly to `main`, and nobody force-pushes it.

Commit messages are short, imperative, one-line summaries (for example "Add
reviewer-a11y lens"), followed by a body when the reason is not obvious.
AI-authored commits carry a `Co-Authored-By` trailer. Never use `--no-verify`.

## Before opening a PR

1. Route through [`docs/development/16-documentation-link-map.md`](docs/development/16-documentation-link-map.md)
   and follow [`docs/development/03-changing-the-factory.md`](docs/development/03-changing-the-factory.md).
   The docs the change affects are updated in the same commit.
2. Run the gates:
   ```bash
   python3 tests/guard_test.py
   bash -n scripts/*.sh
   python3 scripts/check-docs.py
   scripts/doctor.sh
   ```
3. Behavioural changes to the skill, protocol or agents need evidence from a
   sandbox or pilot job ([04-testing-and-verification](docs/development/04-testing-and-verification.md)).
4. Add a dated entry to [`docs/EVOLUTION_LOG.md`](docs/EVOLUTION_LOG.md) and
   update [`IMPLEMENTATION_STATUS`](docs/components/IMPLEMENTATION_STATUS.md).
5. Breaking changes also need an ADR and migration steps
   ([03 → Breaking changes](docs/development/03-changing-the-factory.md#breaking-changes)).

## PR description

State what changed and why. List the evidence (gate output, sandbox job ids,
PR links), whether the change is breaking, and the migration steps when it is.
Review checks the change against `CONSTITUTION.md`, the single-source principle
([00-factory-engineering-principles](docs/development/00-factory-engineering-principles.md)),
and docs completeness.

## Releases

Tags (`vX.Y.Z`) are human-only. See [05-releases-and-versioning](docs/development/05-releases-and-versioning.md).
After tagging, run `scripts/mirror.sh push` and `scripts/mirror.sh bundle`.
