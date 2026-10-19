# ADR-009 — All factory tooling is one Rust CLI

**Status:** Accepted
**Last updated:** 2026-10-19
**Date:** 2026-10-19

## Context

The first versions of the factory used a Python guard hook, a Python docs tool and
Python tests, plus bash for install, doctor, mirror, export and diagram
rendering. The installer also needed `jq`. The operator prefers Rust and asked
that kept tooling be written in Rust by default (`docs/operator/USER.md`).

## Decision

- All factory tooling is one Rust binary, `pf`, in `cli/`, with the subcommands
  `guard`, `docs-map`, `diagrams`, `install`, `doctor`, `mirror`, `export` and
  `dashboard` ([COMP-015](../components/COMP-015-pf-cli.md)).
- The port keeps behaviour identical. Equivalence was checked on the guard's 73
  policy cases plus 44 edge cases, on byte-identical docs-map output, and with
  end-to-end runs of every subcommand. The guard cases live in `cli/src/guard.rs`
  and run as `cargo test` and `pf guard --self-test`.
- The guard hook calls `pf guard` and **fails closed** when `pf` is not on the
  PATH: Bash is blocked, with an install hint.
- `pf install` migrates products and user configs that carry the legacy Python
  guard or `docs-map.py`.
- New tools are added as `pf` subcommands, not as scripts.

## Alternatives considered

- **Keep Python.** It is available almost everywhere, but it goes against the
  operator's language rule and keeps `jq` and bash as extra moving parts.
- **Ship prebuilt binaries through GitHub Releases.** Easier on machines without
  Rust, but it adds a release pipeline and a dependency on one account's
  release assets. This can be added later. `cargo install` from the repo or a
  mirror stays the account-independent path.

## Consequences

- Every machine that runs factory projects needs a Rust toolchain once, to
  `cargo install` `pf`. The beginner guide covers rustup.
- The hook runs as a native binary, which is faster than starting Python on
  every Bash call.
- `pf diagrams render` still calls the Mermaid CLI (JavaScript), because Mermaid
  has no Rust implementation. `pf diagrams check` needs nothing extra.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Architecture decisions](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
