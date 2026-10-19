# Current repository state

**Last updated:** 2026-10-19

This repository owns the Product Factory: an agentic pipeline that takes a product
idea or request through triage, product, design, planning, build, parallel
review, QA and a gate to a pull request that a human merges. It also owns the
release lane, which produces a launch kit.

It contains:

- **Rules:** [`CONSTITUTION.md`](CONSTITUTION.md) (v1).
- **The kit** installed into products: the `/factory` coordinator skill, the
  shared agent protocol, ten role agents, a guard hook that blocks merges,
  tags, releases, force-pushes and discarding work, a project descriptor
  template, a backlog and a local dashboard.
- **The operator profile** (`operator/USER.md`), which keeps preferences out of
  vendor memory.
- **Adapters** for running the same roles in Hermes, Codex, Cursor and other
  agents.
- **Tooling:** installer (project copy or user symlinks), doctor, mirror and
  bundle backups, memory export, and the docs gate.
- **Documentation:** operator guide, architecture, component contracts with
  implementation status, a development handbook, ADRs, roadmap and evolution log.

**Verification state:** the guard hook unit tests (73/73), the installer and
doctor on a scratch repo, and the docs gate all pass. **No job has run end to end
on a real product yet.** That is the Phase 1 exit criterion. Details:
[docs/components/IMPLEMENTATION_STATUS.md](docs/components/IMPLEMENTATION_STATUS.md).

The repository is hosted at `Mat-Agh/product-factory` (private) with no mirror
yet. Product code, product docs, job runtime state, credentials and Claude Code
memory exports are not stored here.
