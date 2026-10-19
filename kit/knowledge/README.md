# Knowledge packs

Imported by `pf knowledge init`; `pf knowledge plan` shows what it would do.
`base/` applies to every repo. `stacks/<name>/` adds the stack's handbook
chapters and areas and overrides base files with the same path. A stack's
`.exclude` lists base paths it does not need. Paths under `docs/` map to the
module's `docs_root`. Placeholders: `{{PRODUCT}}`, `{{MODULE}}`, `{{AREA}}`,
`{{STACK}}`, `{{DATE}}`.

Two kinds of file:
- **Knowledge** files carry a `**Source:** Product Factory knowledge pack (<stack>)`
  line. They hold real stack knowledge. When missing they are created; when an
  equivalent exists, the `##` sections it lacks are appended in a marked block for
  review.
- **Scaffolds** have no `Source` line (current state, history, status, area
  READMEs, routers, CI/CD, contributing, ADR/feature/PRD templates). They are
  created when missing and never merged.

Write pack content generically: no product, company, host or person names.
Commands that depend on the repo are written as `<…>` for the docs job to fill.

The knowledge is distilled from the Daria Connect repositories:
- Back-End and Federation: `spring-kotlin-service`
- Client: `kmp-android-client`
- Front-End: `typescript-web`
- Product: `product-docs`
