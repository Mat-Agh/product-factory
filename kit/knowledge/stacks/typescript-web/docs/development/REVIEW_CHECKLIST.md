# Review checklist

**Status:** Active
**Last updated:** {{DATE}}
**Source:** Product Factory knowledge pack (typescript-web)

Use before approving a material UI change.

## Product and architecture

- The governing product behaviour and design contract are identified and linked.
- The backend OpenAPI is the API truth; no front-end-owned policy was invented.
- The change fits the `apps/`, `packages/`, `tooling/` and feature boundaries.
- Static hosting and same-origin assumptions still hold.

## Experience

- Loading, empty, success, error, unauthorized, forbidden, retry and destructive
  states are deliberate.
- Keyboard navigation, labels, focus, live feedback, RTL, responsive layouts and
  reduced motion are reviewed.
- Every panel performs its mutation and refreshes authoritative data.
- Secret inputs are redacted and cleared; errors expose no sensitive detail.

## Engineering

- No `any`, unbounded assertions, duplicated calls, stale state or avoidable
  request waterfalls.
- Direct dependencies are exact and justified.
- The gate in [00](00-engineering-principles.md) passes.
- Production output has SRI and a complete integrity manifest.
- Docs reflect new commands, constraints, contracts and decisions.

## Release

- Container headers and non-root serving are verified.
- Release CI produced scan results, SBOM, provenance, a digest pin and a signature.
- Remaining risk is written down. Obfuscation is never presented as protection.
