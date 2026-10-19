# COMP-006 — Backlog and dashboard

**Status:** Implemented — dashboard not yet viewed against live jobs
**Last updated:** 2026-10-19
**Source:** [`kit/.factory/backlog.md`](../../kit/.factory/backlog.md), [`kit/.factory/dashboard.html`](../../kit/.factory/dashboard.html)

## Backlog

- **File:** `.factory/backlog.md`, committed in the product repo.
- **Format:** one request per `- [ ] …` line, highest priority first.
- **Claiming:** `/factory next` turns the first unchecked line into
  `- [x] <item> → claimed` with a single exact-line edit, then `→ <job-id>`. A
  failed edit means another session claimed the line.
- **Idea output:** `/factory idea` appends roadmap items ending in
  `(from <job-id>)`. You reorder or delete them.

## Dashboard

- **File:** `.factory/dashboard.html`, a single self-contained page. It is
  overwritten on every install, so do not edit the project copy.
- **Run:** `cd .factory && python3 -m http.server`, then open
  `http://localhost:8000/dashboard.html`.
- **How it reads state:** it parses the `jobs/` directory listing that
  `http.server` serves, fetches each `jobs/<id>/job.json`, and lists each job's
  files from the job folder's own listing. There is no build step, no
  dependencies and no writes. It refreshes every 3 seconds.
- **View:** six columns (Product, Plan, Build, Review & QA, Gate/PR, Human & done),
  review chips by verdict, `needs-human` cards highlighted, and a click to read
  any artifact. It supports light and dark colour schemes and works at phone
  width.

## Known limitations

- It depends on directory listings, so it works with `python3 -m http.server`
  but not with a server that disables indexes.
- It is read-only. Commands run in Claude Code, not from the page.
- It is local only. A remote dashboard waits for phase 4 (observability).

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Components](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
