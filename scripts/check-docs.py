#!/usr/bin/env python3
"""Documentation gate for the Product Factory repo.

  python3 scripts/check-docs.py              check links, headers and inventory
  python3 scripts/check-docs.py --inventory  regenerate docs/DOCUMENT_INVENTORY.md

Checks:
  1. Every relative Markdown link in the repo points at a file or folder that exists.
  2. Every Markdown file under docs/ declares a **Status:** line in its first 8 lines.
  3. docs/DOCUMENT_INVENTORY.md lists exactly the Markdown files under docs/.
Exit code 1 on any failure.
"""

import datetime
import os
import re
import sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
DOCS = os.path.join(ROOT, "docs")
INVENTORY = os.path.join(DOCS, "DOCUMENT_INVENTORY.md")
SKIP_DIRS = {".git", "node_modules", "claude-export"}
LINK = re.compile(r"(?<!!)\[[^\]]*\]\(([^)\s]+)(?:\s+\"[^\"]*\")?\)")


def markdown_files(base):
    for dirpath, dirnames, filenames in os.walk(base):
        dirnames[:] = sorted(d for d in dirnames if d not in SKIP_DIRS)
        for f in sorted(filenames):
            if f.endswith(".md"):
                yield os.path.join(dirpath, f)


def strip_code(text):
    text = re.sub(r"```.*?```", "", text, flags=re.S)
    return re.sub(r"`[^`\n]*`", "", text)


def check_links():
    errors = []
    for path in markdown_files(ROOT):
        text = strip_code(open(path, encoding="utf-8").read())
        for target in LINK.findall(text):
            if re.match(r"^[a-z]+:", target) or target.startswith("#") or target.startswith("<"):
                continue
            target = target.split("#")[0]
            if not target:
                continue
            resolved = os.path.normpath(os.path.join(os.path.dirname(path), target))
            if not os.path.exists(resolved):
                errors.append(f"{os.path.relpath(path, ROOT)}: broken link -> {target}")
    return errors


def title_and_status(path):
    title, status = os.path.basename(path), None
    with open(path, encoding="utf-8") as fh:
        head = [next(fh, "") for _ in range(8)]
    for line in head:
        if line.startswith("# ") and title == os.path.basename(path):
            title = line[2:].strip()
        m = re.match(r"\*\*Status:\*\*\s*(.+)", line.strip())
        if m:
            status = m.group(1).strip()
    return title, status


def check_headers():
    errors = []
    for path in markdown_files(DOCS):
        if path == INVENTORY:
            continue
        if title_and_status(path)[1] is None:
            errors.append(f"{os.path.relpath(path, ROOT)}: missing **Status:** header")
    return errors


def inventory_text():
    rows = []
    for path in markdown_files(DOCS):
        if path == INVENTORY:
            continue
        rel = os.path.relpath(path, DOCS)
        title, status = title_and_status(path)
        rows.append(f"| [{rel}]({rel}) | {title} | {status or '—'} |")
    today = datetime.date.today().isoformat()
    return (
        "# Documentation inventory\n\n"
        "**Status:** Generated completeness map\n"
        f"**Last regenerated:** {today}\n\n"
        f"All {len(rows)} Markdown documents under `docs/`. Regenerate with "
        "`python3 scripts/check-docs.py --inventory`; do not edit by hand.\n\n"
        "| Document | Title | Status |\n|---|---|---|\n" + "\n".join(rows) + "\n"
    )


def check_inventory():
    if not os.path.exists(INVENTORY):
        return ["docs/DOCUMENT_INVENTORY.md missing (run with --inventory)"]
    listed = set(re.findall(r"^\| \[([^\]]+)\]", open(INVENTORY, encoding="utf-8").read(), flags=re.M))
    actual = {os.path.relpath(p, DOCS) for p in markdown_files(DOCS) if p != INVENTORY}
    errors = [f"inventory missing: {p}" for p in sorted(actual - listed)]
    errors += [f"inventory lists deleted file: {p}" for p in sorted(listed - actual)]
    return errors


def main():
    if "--inventory" in sys.argv:
        with open(INVENTORY, "w", encoding="utf-8") as fh:
            fh.write(inventory_text())
        print("Wrote docs/DOCUMENT_INVENTORY.md")
    errors = check_links() + check_headers() + check_inventory()
    for e in errors:
        print("FAIL", e)
    print("docs OK" if not errors else f"{len(errors)} docs problem(s)")
    sys.exit(1 if errors else 0)


if __name__ == "__main__":
    main()
