# COMP-006 — Backlog and dashboard

**Status:** Implemented — unified local command centre and serialized backlog claims
**Last updated:** 2026-11-16
**Source:** [`kit/.factory/backlog.md`](../../kit/.factory/backlog.md), [`kit/.factory/dashboard.html`](../../kit/.factory/dashboard.html), [`cli/src/dashboard.rs`](../../cli/src/dashboard.rs)

## Backlog

- **File:** `.factory/backlog.md`, committed in the product repo.
- **Format:** one request per `- [ ] …` line, highest priority first.
- **Claiming:** the coordinator calls `pf job claim --backlog`. Under the
  workspace lock it claims the first unchecked line and marks it with the
  allocated job ID.
- **Idea output:** `/factory idea` appends roadmap items ending in
  `(from <job-id>)`. You reorder or delete them.

## Dashboard

- **Files:** `.factory/dashboard.html` is the live command centre and
  `.factory/atlas.html` is the product explanation and dashboard directory.
  Both are overwritten on every install. `.factory/atlas.json` is
  project-owned and supplies its additional local or remote dashboard links.
- **Run:** `pf dashboard` from the product repo root, then open
  `http://127.0.0.1:8000/`. From a module, the server finds the workspace
  descriptor. It serves the board, backlog and job artifacts over loopback.
- **How it reads state:** it parses the `jobs/` directory listing that
  `pf dashboard` serves, fetches each `jobs/<id>/job.json`, and lists each job's
  files from the job folder's own listing. There is no build step, no
  dependencies and no writes. It refreshes every 3 seconds.
- **View:** queued requests, a labelled factory-state chip, module filter,
  dashboard/graph links, embedded live workspace code network and six job
  columns. Overview, Work, Code network and Systems share a consistent UI with
  persistent dark/light theme and English/Persian language controls. Cards show
  the request description, source label, explicit review
  verdict labels and blockers. Opening a card exposes its source issue, pull
  request and every saved artifact. It supports light and dark colour schemes
  and works at phone width.
- **Graph freshness:** `pf graph watch --modules` polls every independent Git
  checkout and rebuilds its graph plus the combined workspace network after a
  code or Git-state change. `pf job` also refreshes a prepared worktree at the
  stages where implementation changes occur.
- **Load behavior:** the 3D workspace network is requested only after opening
  the Code network tab. The loopback server handles requests concurrently, so
  serving a large graph does not hold up job status and dashboard requests.

## Verification

The local board rendered a queued item, a module-filterable job, a partial
review and a blocking reason in Chrome. The server returned 404 for the
descriptor and a symlink out of the job artifacts.

## Known limitations

- It depends on directory listings, which `pf dashboard` provides. A static
  server without indexes will not work.
- It is read-only. Intake and rework run through the Claude Code or Codex skill.
- It is local only. A remote dashboard waits for phase 4 (observability).

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Components](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
