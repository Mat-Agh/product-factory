# ADR-014 — Inline Mermaid diagrams instead of rendered images

**Status:** Accepted
**Date:** 2026-10-20
**Last updated:** 2026-10-20

## Context

The factory first showed diagrams as SVGs rendered from separate `.mmd` files,
after GitHub displayed "Unable to render rich display" for inline Mermaid. Rendered
images are static: they do not resize or follow the reader's theme, IDE previews
dropped some of them, and agents cannot read a picture. The operator wants proper
inline Mermaid for live rendering.

The earlier GitHub failure came from constructs its older Mermaid version does not
accept: HTML-like text in labels (`engineer-<job>`, `/factory <request>`), `<br/>`
tags and `&` edge chaining.

## Decision

- Diagrams are inline ` ```mermaid ` blocks in the document they illustrate, in the
  house style (quoted labels, one idea per diagram, no theme directives).
- `pf diagrams check` lints every block for constructs that break renderers;
  `pf diagrams validate` parses every block with the Mermaid CLI.
- `docs/assets/diagrams/` and `pf diagrams render` are removed.
- ASCII diagrams in the knowledge library become Mermaid, and the knowledge pages
  gain diagrams where a flow is clearer drawn.

## Alternatives considered

- **Keep rendered SVGs.** Rejected: static, and dropped by some IDE previews.
- **Both: inline blocks plus rendered fallbacks.** Rejected: two copies of every
  diagram drift.

## Consequences

- Live, resizable diagrams on GitHub, GitLab and in IDEs (IntelliJ needs its Mermaid
  extension enabled); terminals show readable source.
- Supersedes the diagram part of the documentation rules recorded in COMP-014 on
  2026-10-19.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Architecture decisions](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
