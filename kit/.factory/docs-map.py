#!/usr/bin/env python3
"""Product Factory docs map: keep a repo's documentation in one connected tree.

    python3 .factory/docs-map.py            check (exit 1 on failure)
    python3 .factory/docs-map.py --fix      regenerate routing blocks + map, create missing READMEs, then check
    python3 .factory/docs-map.py --strict   also require a **Status:** header in every doc and no docs outside docs/

Options: --root <repo> (default: git top level), --docs <dir> (default: docs),
         --allow <prefix> (repeatable; Markdown outside docs/ that is not documentation, e.g. kit/).

Rules (CONSTITUTION §1):
  1. Documentation lives under <docs>/. At the repo root only README.md, AGENTS.md, CLAUDE.md and
     tool-required files (LICENSE, CHANGELOG.md, SECURITY.md, CODE_OF_CONDUCT.md) may stay.
  2. Every folder under <docs>/ that holds documents has a README.md.
  3. Every README carries a generated routing block: parent README, agent guide, the documents in its
     folder and its subfolder READMEs. Every other document links back to its folder README.
  4. <docs>/README.md carries a generated map of the whole tree.
  5. Every relative link in every tracked Markdown file resolves, and every document is reachable
     from <docs>/README.md.
Python 3.8+ standard library only. Only git-tracked and untracked-but-not-ignored files are considered.
"""

import argparse
import os
import re
import subprocess
import sys
from collections import deque

ROUTE_START, ROUTE_END = "<!-- documentation-routing:start -->", "<!-- documentation-routing:end -->"
MAP_START, MAP_END = "<!-- documentation-map:start -->", "<!-- documentation-map:end -->"
ROOT_ALLOWED = {"README.md", "AGENTS.md", "CLAUDE.md", "CHANGELOG.md", "LICENSE.md", "SECURITY.md", "CODE_OF_CONDUCT.md"}
LINK = re.compile(r"(?<!!)\[[^\]]*\]\(([^)\s]+)(?:\s+\"[^\"]*\")?\)")


def git_root(start):
    try:
        return subprocess.check_output(["git", "-C", start, "rev-parse", "--show-toplevel"], text=True,
                                       stderr=subprocess.DEVNULL).strip()
    except Exception:
        return os.path.abspath(start)


def markdown_files(root):
    try:
        out = subprocess.check_output(["git", "-C", root, "ls-files", "-co", "--exclude-standard", "--", "*.md"],
                                      text=True, stderr=subprocess.DEVNULL)
        files = [f for f in out.splitlines() if f and os.path.exists(os.path.join(root, f))]
    except Exception:
        files = []
        for d, dirs, fs in os.walk(root):
            dirs[:] = [x for x in dirs if not x.startswith(".") and x != "node_modules"]
            files += [os.path.relpath(os.path.join(d, f), root) for f in fs if f.endswith(".md")]
    files = [f for f in files if not os.path.islink(os.path.join(root, f))]  # links are copies, not docs
    return sorted(set(f.replace(os.sep, "/") for f in files))


def read(path):
    with open(path, encoding="utf-8") as fh:
        return fh.read()


def write(path, text):
    with open(path, "w", encoding="utf-8") as fh:
        fh.write(text)


def title_of(path, fallback):
    try:
        for line in read(path).splitlines()[:15]:
            if line.startswith("# "):
                return line[2:].strip()
    except OSError:
        pass
    return fallback


def status_of(path):
    for line in read(path).splitlines()[:10]:
        m = re.match(r"\*\*Status:\*\*\s*(.+)", line.strip())
        if m:
            return m.group(1).strip()
    return None


def strip_code(text):
    text = re.sub(r"```.*?```", "", text, flags=re.S)
    return re.sub(r"`[^`\n]*`", "", text)


def replace_block(text, start, end, block):
    if start in text and end in text:
        return re.sub(re.escape(start) + r".*?" + re.escape(end), lambda _: block, text, count=1, flags=re.S)
    return text.rstrip() + "\n\n" + block + "\n"


def rel(from_dir, target):
    return os.path.relpath(target, from_dir).replace(os.sep, "/")


class Tree:
    def __init__(self, root, docs, files):
        self.root, self.docs = root, docs
        self.docs_abs = os.path.join(root, docs)
        self.docs_files = [f for f in files if f.startswith(docs + "/")]
        self.dirs = set()
        for f in self.docs_files:
            d = os.path.dirname(f)
            while d and d.startswith(docs):
                self.dirs.add(d)
                if d == docs:
                    break
                d = os.path.dirname(d)
        self.dirs.add(docs)

    def children(self, d):
        files = sorted(f for f in self.docs_files if os.path.dirname(f) == d and os.path.basename(f) != "README.md")
        subdirs = sorted(x for x in self.dirs if os.path.dirname(x) == d and x != d)
        return files, subdirs

    def readme(self, d):
        return os.path.join(self.root, d, "README.md")


def routing_block(tree, d, is_readme, path_abs):
    here = os.path.dirname(path_abs)
    root_readme = os.path.join(tree.root, "README.md")
    agents = os.path.join(tree.root, "AGENTS.md")
    lines = [ROUTE_START, "## Documentation routing", ""]
    if is_readme:
        parent = root_readme if d == tree.docs else tree.readme(os.path.dirname(d))
        if os.path.exists(parent):
            lines.append(f"- Parent: [{title_of(parent, 'README')}]({rel(here, parent)})")
    else:
        lines.append(f"- Parent: [{title_of(tree.readme(d), d)}]({rel(here, tree.readme(d))})")
    parent_is_home = (is_readme and os.path.dirname(d) == tree.docs) or (not is_readme and d == tree.docs)
    if d != tree.docs and not parent_is_home:
        docs_readme = tree.readme(tree.docs)
        lines.append(f"- Documentation home: [{title_of(docs_readme, 'Documentation')}]({rel(here, docs_readme)})")
    if os.path.exists(agents):
        lines.append(f"- Agent guide: [AGENTS.md]({rel(here, agents)})")
    if is_readme:
        files, subdirs = tree.children(d)
        if files:
            lines.append("- In this folder:")
            for f in files:
                p = os.path.join(tree.root, f)
                lines.append(f"  - [{title_of(p, os.path.basename(f))}]({rel(here, p)})")
        if subdirs:
            lines.append("- Subfolders:")
            for s in subdirs:
                p = tree.readme(s)
                lines.append(f"  - [{title_of(p, os.path.basename(s))}]({rel(here, p)})")
    lines.append(ROUTE_END)
    return "\n".join(lines)


def map_block(tree, tool_cmd):
    home = tree.readme(tree.docs)
    here = os.path.dirname(home)
    out = [MAP_START, "## Documentation map", "",
           f"Generated by `{tool_cmd} --fix`. Do not edit by hand.", ""]

    def entry(path_abs, depth):
        st = status_of(path_abs) if os.path.exists(path_abs) else None
        suffix = f" — {st}" if st else ""
        out.append(f"{'  ' * depth}- [{title_of(path_abs, os.path.basename(path_abs))}]({rel(here, path_abs)}){suffix}")

    def walk(d, depth):
        files, subdirs = tree.children(d)
        for f in files:
            entry(os.path.join(tree.root, f), depth)
        for s in subdirs:
            entry(tree.readme(s), depth)
            walk(s, depth + 1)

    walk(tree.docs, 0)
    out.append(MAP_END)
    return "\n".join(out)


def fix(tree, tool_cmd):
    created = []
    for d in sorted(tree.dirs):
        p = tree.readme(d)
        if not os.path.exists(p):
            name = os.path.basename(d).replace("-", " ").replace("_", " ").capitalize()
            write(p, f"# {name}\n\n**Status:** Draft\n**Last updated:** —\n\nDescribe what this folder holds.\n")
            created.append(os.path.relpath(p, tree.root))
            tree.docs_files.append(os.path.relpath(p, tree.root).replace(os.sep, "/"))
    for f in tree.docs_files:
        p = os.path.join(tree.root, f)
        d = os.path.dirname(f)
        is_readme = os.path.basename(f) == "README.md"
        text = replace_block(read(p), ROUTE_START, ROUTE_END, routing_block(tree, d, is_readme, p))
        if f == tree.docs + "/README.md":
            text = replace_block(text, MAP_START, MAP_END, map_block(tree, tool_cmd))
            # keep the map above the routing block
            if text.index(MAP_START) > text.index(ROUTE_START):
                m = re.search(re.escape(MAP_START) + r".*?" + re.escape(MAP_END), text, flags=re.S).group(0)
                text = text.replace(m, "").rstrip() + "\n"
                text = text.replace(ROUTE_START, m + "\n\n" + ROUTE_START)
        write(p, re.sub(r"\n{3,}", "\n\n", text))
    return created


def check(tree, files, strict, allow, tool_cmd):
    errors, warnings = [], []
    # 1. docs outside docs/
    for f in files:
        if f.startswith(tree.docs + "/") or any(f.startswith(a) for a in allow):
            continue
        if any(part.startswith(".") for part in f.split("/")[:-1]):
            continue  # tool folders (.claude/, .factory/, .github/) hold payload, not docs
        if "/" not in f and f in ROOT_ALLOWED:
            continue
        (errors if strict else warnings).append(f"documentation outside {tree.docs}/: {f}")
    # 2. READMEs
    for d in sorted(tree.dirs):
        if not os.path.exists(tree.readme(d)):
            errors.append(f"missing {d}/README.md")
    # 3/4. generated blocks current
    for f in tree.docs_files:
        p = os.path.join(tree.root, f)
        if not os.path.exists(p):
            continue
        text = read(p)
        want = routing_block(tree, os.path.dirname(f), os.path.basename(f) == "README.md", p)
        if want not in text:
            errors.append(f"{f}: routing block missing or stale")
        if strict and status_of(p) is None:
            errors.append(f"{f}: missing **Status:** header")
    home = tree.readme(tree.docs)
    if os.path.exists(home) and map_block(tree, tool_cmd) not in read(home):
        errors.append(f"{tree.docs}/README.md: documentation map missing or stale")
    # 5. links resolve
    graph = {}
    for f in files:
        p = os.path.join(tree.root, f)
        targets = []
        for t in LINK.findall(strip_code(read(p))):
            if re.match(r"^[a-z][a-z0-9+.-]*:", t) or t.startswith("#") or t.startswith("<"):
                continue
            t = t.split("#")[0]
            if not t:
                continue
            resolved = os.path.normpath(os.path.join(os.path.dirname(p), t))
            if not os.path.exists(resolved):
                errors.append(f"{f}: broken link -> {t}")
            elif resolved.endswith(".md"):
                targets.append(os.path.relpath(resolved, tree.root).replace(os.sep, "/"))
        graph[f] = targets
    # reachability
    start = tree.docs + "/README.md"
    seen, q = {start}, deque([start])
    while q:
        for n in graph.get(q.popleft(), []):
            if n not in seen:
                seen.add(n)
                q.append(n)
    for f in tree.docs_files:
        if f not in seen:
            errors.append(f"{f}: not reachable from {start}")
    root_readme = os.path.join(tree.root, "README.md")
    if os.path.exists(root_readme) and start not in graph.get("README.md", []):
        warnings.append(f"README.md does not link {start}")
    return errors, warnings


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--root")
    ap.add_argument("--docs", default="docs")
    ap.add_argument("--fix", action="store_true")
    ap.add_argument("--strict", action="store_true")
    ap.add_argument("--allow", action="append", default=[])
    a = ap.parse_args()
    root = git_root(a.root or os.getcwd())
    tool_cmd = "python3 " + os.path.relpath(os.path.abspath(__file__), root).replace(os.sep, "/")
    if not os.path.isdir(os.path.join(root, a.docs)):
        if not a.fix:
            print(f"FAIL no {a.docs}/ folder in {root} (--fix creates it)")
            return 1
        os.makedirs(os.path.join(root, a.docs))
    files = markdown_files(root)
    tree = Tree(root, a.docs, files)
    if a.fix:
        for c in fix(tree, tool_cmd):
            print(f"created {c} (stub: fill it in)")
        files = markdown_files(root)
        tree = Tree(root, a.docs, files)
    errors, warnings = check(tree, files, a.strict, a.allow, tool_cmd)
    for w in warnings:
        print("WARN", w)
    for e in errors:
        print("FAIL", e)
    n = len(tree.docs_files)
    print(f"docs map OK ({n} documents)" if not errors else f"{len(errors)} docs problem(s); try --fix")
    return 1 if errors else 0


if __name__ == "__main__":
    sys.exit(main())
