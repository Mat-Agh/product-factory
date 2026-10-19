#!/usr/bin/env python3
"""Tests for kit/.claude/hooks/factory-guard.py. Run: python3 tests/guard_test.py"""

import json
import os
import subprocess
import sys

HOOK = os.path.join(os.path.dirname(__file__), "..", "kit", ".claude", "hooks", "factory-guard.py")

BLOCK = [
    "git merge factory/001-x",
    "git -C /repo merge factory/001-x",
    "cd /w && git merge --no-ff factory/001-x -m 'm'",
    "GIT_DIR=x git merge foo",
    "env git merge foo",
    "command git merge foo",
    "/usr/bin/git merge foo",
    "git push origin main",
    "git push origin HEAD:main",
    "git push origin factory/001-x:main",
    "git push origin develop",
    "git push origin release/1.2",
    "git push -f origin factory/001-x",
    "git push -fu origin factory/001-x",
    "git push --force-with-lease origin factory/001-x",
    "git push origin +factory/001-x",
    "git push --tags",
    "git push origin --delete factory/001-x",
    "git push origin :factory/001-x",
    "git push",
    "git push origin",
    "git push origin HEAD",
    "git push origin feature/other",
    "git tag v1.0.0",
    "git tag -a v1 -m x",
    "git commit --no-verify -m x",
    "git commit -nm x",
    "git reset --hard HEAD~1",
    "git clean -fd",
    "git stash",
    "git stash drop",
    "git checkout -- .",
    "git branch -D main",
    "gh pr merge 12 --squash",
    "gh pr review 12 --approve",
    "gh release create v1",
    "gh repo delete Mat-Agh/x --yes",
    "gh secret set TOKEN",
    "gh workflow run deploy.yml",
    "gh api -X PUT repos/o/r/pulls/1/merge",
    "glab mr merge 3",
    "glab mr approve 3",
    "rm -rf ~",
    "rm -rf /",
    "true && git merge x",
    "echo $(git merge x)",
]

ALLOW = [
    "git status --porcelain",
    "git log --oneline main..HEAD",
    "git diff main...HEAD",
    "git merge-tree --write-tree origin/main HEAD",
    "git merge --abort",
    "git worktree add -b factory/001-x ../r-factory/001-x main",
    "cd /w && git commit -m 'Add reading time'",
    "git push -u origin factory/001-x",
    "git push origin factory/001-x",
    "git branch -d factory/001-x",
    "git branch --show-current",
    "git tag --list",
    "git tag -l 'v*'",
    "git stash list",
    "git fetch origin main",
    "gh pr create --base main --head factory/001-x --title t --body-file pr.md",
    "gh pr create --draft --base main --head factory/001-x --title t --body-file pr.md",
    "gh pr view 1 --json state",
    "gh issue view 3 --json title,body",
    "gh issue comment 3 --body-file -",
    "glab mr create --target-branch main --source-branch factory/001-x",
    "rm -rf node_modules",
    "npm test",
    "cat <<'FACTORY_COMMENT'\ngit merge main\ngit push origin main\nFACTORY_COMMENT",
    "python3 -m http.server",
]


def run(cmd, env=None):
    payload = json.dumps({"tool_name": "Bash", "tool_input": {"command": cmd}})
    p = subprocess.run([sys.executable, HOOK], input=payload, capture_output=True, text=True,
                       env={**os.environ, **(env or {})})
    return p.returncode


def main():
    failures = []
    for c in BLOCK:
        if run(c) != 2:
            failures.append(f"should BLOCK: {c!r}")
    for c in ALLOW:
        if run(c) != 0:
            failures.append(f"should ALLOW: {c!r}")
    if run("git merge x", {"FACTORY_GUARD": "off"}) != 0:
        failures.append("FACTORY_GUARD=off should allow")
    p = subprocess.run([sys.executable, HOOK], input="not json", capture_output=True, text=True)
    if p.returncode != 2:
        failures.append("bad input should fail closed")
    total = len(BLOCK) + len(ALLOW) + 2
    for f in failures:
        print("FAIL", f)
    print(f"{total - len(failures)}/{total} passed")
    sys.exit(1 if failures else 0)


if __name__ == "__main__":
    main()
