# Knowledge packs

Project scaffolds plus, per stack, a `pack.map` into the factory's development
knowledge library (`docs/knowledge/`). `pf knowledge plan` shows what a repository
lacks; `pf knowledge init` imports it.

- `base/` — scaffolds every repository gets (current state, history, status, area
  READMEs, handbook index, link map, CI/CD, contributing, ADR and feature templates).
- `stacks/<name>/` — the stack's own scaffolds, overriding base files with the same
  path, plus:
  - `pack.map` — `<library page> <destination>` per line; a line ending in `/`
    imports a whole library folder, skipping pages mapped explicitly;
  - `.exclude` — base paths the stack does not need.

Paths under `docs/` map to the module's `docs_root`. Placeholders in scaffolds:
`{{PRODUCT}}`, `{{MODULE}}`, `{{AREA}}`, `{{STACK}}`, `{{DATE}}`.

Library pages are knowledge: created when missing and merged section by section
into an existing equivalent. Scaffolds are created when missing and never merged.
Write knowledge in `docs/knowledge/`, never here.
