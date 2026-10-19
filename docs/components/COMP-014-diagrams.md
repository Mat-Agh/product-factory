# COMP-014 — Diagrams

**Status:** Implemented — all 7 diagrams validated with Mermaid 10.9 and 11.17 and rendered (2026-10-19)
**Last updated:** 2026-10-19
**Source:** [`scripts/render-diagrams.sh`](../../scripts/render-diagrams.sh), [`docs/assets/diagrams/`](../assets/diagrams/)

## Purpose

These diagrams display everywhere the docs are read: GitHub, a GitLab mirror,
an IDE preview, or an offline bundle. Inline ` ```mermaid ` blocks depend on the
host's live renderer. On GitHub that renderer is an iframe from
`viewscreen.githubusercontent.com`, and when it cannot load, the page shows
"Unable to render rich display". The docs therefore show **committed SVG
images**, rendered from Mermaid sources.

## Layout

| File | Role |
|---|---|
| `docs/assets/diagrams/<name>.mmd` | The only source of a diagram. Edit this |
| `docs/assets/diagrams/<name>.svg` | Generated image. Its first line records the `source-sha256` of the `.mmd` it came from. Never edit it |
| The document that shows it | `![alt](…/assets/diagrams/<name>.svg)` plus a small "Diagram source" link to the `.mmd` |

| Diagram | Shown in |
|---|---|
| `pipeline` | [README](../../README.md) |
| `idea-to-market` | [guide/00-overview](../guide/00-overview.md) |
| `job-stages` | [guide/03-reading-a-job](../guide/03-reading-a-job.md) |
| `layers`, `job-sequence` | [architecture/FACTORY_OVERVIEW](../architecture/FACTORY_OVERVIEW.md) |
| `job-state-machine` | [architecture/JOB_LIFECYCLE](../architecture/JOB_LIFECYCLE.md) |
| `doc-routing` | [development/16-documentation-link-map](../development/16-documentation-link-map.md) |

## Interface

```bash
scripts/render-diagrams.sh            # render every .mmd (needs Node; uses system Chrome when present)
scripts/render-diagrams.sh --check    # every .svg exists and matches its .mmd hash (no browser needed)
```

Rendering uses `@mermaid-js/mermaid-cli` 11 (`$MMDC`, `mmdc` on the PATH, or
`npx`), with the neutral theme on a white background so diagrams stay readable
in dark mode. A source with a syntax error fails the render.

## Rules

- No inline ` ```mermaid ` blocks in docs. Add a `.mmd`, render it, and embed
  the SVG.
- Commit the `.mmd` and the `.svg` together. `--check` is a gate (`AGENTS.md`,
  `scripts/doctor.sh`).

## Known limitations

Rendering needs Node.js and a Chrome or Chromium binary (or Puppeteer's
download). Checking needs only Python.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Components](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
