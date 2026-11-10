# COMP-014 — Diagrams

**Status:** Implemented — inline Mermaid blocks, linted and parsed by the Mermaid CLI (2026-10-20)
**Last updated:** 2026-10-20
**Source:** [`cli/src/diagrams.rs`](../../cli/src/diagrams.rs) (`pf diagrams`)
**Decisions:** [ADR-014](../decisions/ADR-014-inline-mermaid-diagrams.md)

## Purpose

Diagrams are inline ` ```mermaid ` blocks inside the documents. GitHub, GitLab and IDE previews render them live, so they resize,
follow the reader's theme and stay editable text that agents can read and change.
The style rules are in
[principles/documentation](../knowledge/principles/documentation.md#diagrams).

## Interface

```bash
pf diagrams check      # lint every mermaid block in tracked Markdown (no tools needed)
pf diagrams validate   # lint, then parse every block with the Mermaid CLI (needs Node.js)
```

`check` fails on the constructs that break older renderers such as GitHub's:
- a diagram type outside the supported list (`flowchart`, `graph`,
  `sequenceDiagram`, `stateDiagram-v2`, `erDiagram`, `classDiagram`, `gantt`,
  `journey`, `gitGraph`, `timeline`, `pie`);
- `%%{init}` directives (themes are the renderer's);
- HTML-like text in labels (`<job>`) other than `<br/>`;
- `A --> B & C` edge chaining (one edge per line);
- unbalanced quotes, unclosed blocks;
- diagrams over 60 lines (split them).

`validate` runs each block through `@mermaid-js/mermaid-cli` 11 (`$MMDC`, `mmdc`
on the PATH, or `npx`, with system Chrome when present) and reports
`file:line: parse error`.

## Rules

- No diagram images and no separate diagram files in docs; the block lives where
  it is explained.
- `pf diagrams check` is a gate (`AGENTS.md`, `pf doctor`); run
  `pf diagrams validate` after adding or editing a diagram.
- Viewers that do not render Mermaid (terminals, plain-text tools) show the source,
  which is written to be readable as text.

## Known limitations

- GitHub's Mermaid version lags the CLI's; the lint covers the constructs known to
  break it, but a brand-new diagram type can still pass `validate` and fail on
  GitHub. Prefer the types listed above.
- IntelliJ needs its Mermaid Markdown extension enabled (Settings → Languages &
  Frameworks → Markdown → Markdown Extensions → Mermaid).

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Components](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
