# COMP-006 — Backlog and dashboard

**Status:** Implemented — local board and serialized backlog claims
**Last updated:** 2026-10-19
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

- **File:** `.factory/dashboard.html`, a single self-contained page. It is
  overwritten on every install, so do not edit the project copy.
- **Run:** `pf dashboard` from the product repo root, then open
  `http://127.0.0.1:8000/`. From a module, the server finds the workspace
  descriptor. It serves the board, backlog and job artifacts over loopback.
- **How it reads state:** it parses the `jobs/` directory listing that
  `pf dashboard` serves, fetches each `jobs/<id>/job.json`, and lists each job's
  files from the job folder's own listing. There is no build step, no
  dependencies and no writes. It refreshes every 3 seconds.
- **View:** queued requests, a module filter and six columns (Product, Plan, Build, Review & QA, Gate/PR, Human & done),
  review chips as lens artifacts arrive, blocking reasons, and job details
  with artifacts and the pull request link. It supports light and dark colour schemes and works at phone
  width.

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
