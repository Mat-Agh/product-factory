# COMP-012 — Documentation gate

**Status:** Implemented
**Last updated:** 2026-10-19
**Source:** [`scripts/check-docs.py`](../../scripts/check-docs.py)

## Purpose

This gate keeps the documentation trustworthy. It is the factory repo's
equivalent of the docs-sync gate that reviewers apply to products.

## Checks

1. Every relative Markdown link in the repo (outside code spans and blocks)
   resolves to an existing file or folder.
2. Every Markdown file under `docs/` has a `**Status:**` line in its first 8
   lines.
3. `docs/DOCUMENT_INVENTORY.md` lists exactly the Markdown files under `docs/`.

## Usage

```bash
python3 scripts/check-docs.py               # check; exit 1 on failure
python3 scripts/check-docs.py --inventory   # regenerate the inventory, then check
```

## Known limitations

It does not validate `#anchor` fragments or external URLs.

<!-- documentation-routing:start -->
## Documentation routing

- [Parent README](README.md)
- [Governing agent guide](../../AGENTS.md)
<!-- documentation-routing:end -->
