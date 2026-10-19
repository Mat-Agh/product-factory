#!/usr/bin/env python3
"""Product Factory guard: a Claude Code PreToolUse hook for Bash.

It blocks the actions CONSTITUTION.md reserves for humans or forbids outright:
merging, tagging, releasing, deploying through git or gh, force-pushing,
pushing to a protected branch, skipping hooks, and discarding work.

Exit code 2 blocks the call and shows the reason to the model. The hook is
deliberately conservative. Remote branch protection is the real enforcement,
and this hook is a second line in front of it.

Override for a human-driven session only: export FACTORY_GUARD=off before
starting Claude. An inline `FACTORY_GUARD=off git ...` inside a command has no
effect, because the hook reads its own environment.
"""

import json
import os
import re
import shlex
import sys

PROTECTED = {"main", "master", "develop", "trunk", "production", "prod", "release"}
PROTECTED_PREFIXES = ("release/", "hotfix/")

GIT_GLOBAL_OPTS_WITH_ARG = {"-C", "-c", "--git-dir", "--work-tree", "--namespace", "--exec-path"}
WRAPPERS = {"env", "command", "builtin", "exec", "nice", "nohup", "time", "sudo", "xargs"}


def is_protected(ref):
    ref = ref.split(":")[-1].lstrip("+")
    ref = re.sub(r"^refs/heads/", "", ref)
    return ref in PROTECTED or ref.startswith(PROTECTED_PREFIXES)


def split_segments(command):
    """Split a shell command into simple commands on ; && || | and newlines.

    Heredoc bodies are dropped so text inside them is never parsed as commands.
    """
    lines, out, skip_until = command.split("\n"), [], None
    for line in lines:
        if skip_until is not None:
            if line.strip() == skip_until:
                skip_until = None
            continue
        m = re.search(r"<<-?\s*['\"]?([A-Za-z_][A-Za-z0-9_]*)['\"]?", line)
        if m:
            skip_until = m.group(1)
            line = line[: m.start()]
        out.append(line)
    text = "\n".join(out)
    return [s for s in re.split(r"\|\||&&|[;|\n]|\$\(|`", text) if s.strip()]


def tokens_of(segment):
    try:
        toks = shlex.split(segment, posix=True)
    except ValueError:
        toks = segment.split()
    # Drop leading VAR=value assignments and wrapper commands.
    while toks and (re.match(r"^[A-Za-z_][A-Za-z0-9_]*=", toks[0]) or toks[0] in WRAPPERS):
        head = toks.pop(0)
        if head in ("sudo", "nice", "xargs"):
            while toks and toks[0].startswith("-"):
                toks.pop(0)
    if toks:
        toks[0] = os.path.basename(toks[0])
    return toks


def git_sub(toks):
    """Return (subcommand, args) for a git invocation, skipping global options."""
    i = 1
    while i < len(toks):
        t = toks[i]
        if t in GIT_GLOBAL_OPTS_WITH_ARG:
            i += 2
            continue
        if t.startswith("-"):
            i += 1
            continue
        return t, toks[i + 1:]
    return None, []


def short_flags(args):
    flags = set()
    for a in args:
        if a.startswith("-") and not a.startswith("--"):
            flags.update(a[1:])
    return flags


def check_git(toks):
    sub, args = git_sub(toks)
    if sub is None:
        return None
    if "--no-verify" in args or ("n" in short_flags(args) and sub == "commit"):
        return "Skipping hooks (--no-verify) is forbidden."
    if sub == "merge" and not any(a in ("--abort", "--quit") for a in args):
        return "Merging is human-only. The factory opens a PR and stops."
    if sub == "merge-tree":
        return None
    listing = any(a in ("-l", "--list", "--contains", "--points-at") or a.startswith("-n") for a in args)
    if sub == "tag" and args and not listing:
        return "Tagging is human-only."
    if sub == "push":
        if "--force" in args or "--force-with-lease" in args or "f" in short_flags(args) \
                or any(a.startswith("--force") or a.startswith("+") or ":+" in a for a in args):
            return "Force-push is forbidden."
        if "--tags" in args or "--follow-tags" in args or "--mirror" in args or "--all" in args:
            return "Pushing tags, --all or --mirror is human-only."
        if "--delete" in args or "d" in short_flags(args) or any(a.startswith(":") for a in args):
            return "Deleting remote refs is human-only."
        refs = [a for a in args if not a.startswith("-")][1:]
        if not refs:
            return "Name the branch explicitly: git push <remote> factory/<job-id>."
        for r in refs:
            if is_protected(r) or "refs/tags/" in r:
                return f"Pushing to protected ref '{r}' is human-only."
            if not r.split(":")[-1].startswith("factory/"):
                return f"The factory pushes only factory/* branches, not '{r}'."
        return None
    if sub == "reset" and "--hard" in args:
        return "git reset --hard discards work. Ask the human."
    if sub == "clean" and ("f" in short_flags(args) or "--force" in args):
        return "git clean -f discards work. Ask the human."
    if sub == "stash" and (not args or args[0] in ("push", "save", "drop", "clear", "-u", "--include-untracked")):
        return "Stashing or dropping someone's work is forbidden. Ask the human."
    if sub in ("checkout", "restore") and ("--" in args and "." in args):
        return "Discarding working-tree changes wholesale is forbidden. Ask the human."
    if sub == "branch" and ("D" in short_flags(args) or "--delete" in args or "d" in short_flags(args)):
        targets = [a for a in args if not a.startswith("-")]
        if any(not t.startswith("factory/") for t in targets):
            return "Only factory/* branches may be deleted by the factory."
    if sub == "commit":
        return None
    if sub == "update-ref" or sub == "filter-branch" or sub == "filter-repo":
        return f"git {sub} rewrites refs. Human-only."
    return None


def check_gh(toks):
    if len(toks) < 3:
        return None
    area, verb = toks[1], toks[2]
    if area == "pr" and verb == "merge":
        return "Merging is human-only."
    if area == "pr" and verb == "review" and ("--approve" in toks or "-a" in toks):
        return "Approving PRs is human-only."
    if area == "release" and verb in ("create", "edit", "delete", "upload"):
        return "Releases are human-only."
    if area == "repo" and verb in ("delete", "archive", "rename", "edit"):
        return f"gh repo {verb} is human-only."
    if area == "secret" or area == "variable":
        return "Secrets and variables are operator-only."
    if area == "workflow" and verb in ("run", "enable", "disable"):
        return "Running or toggling workflows is human-only."
    if area == "api":
        joined = " ".join(toks)
        if re.search(r"-X\s*(PUT|PATCH|DELETE)|--method\s+(PUT|PATCH|DELETE)", joined) and \
                re.search(r"/(merge|protection|releases|git/refs|actions/secrets)", joined):
            return "This API call merges, releases or changes protection. Human-only."
    return None


def check_glab(toks):
    if len(toks) >= 3 and toks[1] == "mr" and toks[2] in ("merge", "approve"):
        return "Merging and approving MRs is human-only."
    if len(toks) >= 3 and toks[1] == "release" and toks[2] == "create":
        return "Releases are human-only."
    return None


def check(command):
    for seg in split_segments(command):
        toks = tokens_of(seg)
        if not toks:
            continue
        name = toks[0]
        reason = None
        if name == "git":
            reason = check_git(toks)
        elif name == "gh":
            reason = check_gh(toks)
        elif name == "glab":
            reason = check_glab(toks)
        elif name in ("rm",) and re.search(r"-\w*r\w*f|-\w*f\w*r", seg) and re.search(r"(^|\s)(/|~|\$HOME|\.\.?)(\s|/?$)", seg):
            reason = "Recursive delete of a root, home or parent directory is forbidden."
        if reason:
            return reason
    return None


def main():
    if os.environ.get("FACTORY_GUARD", "").lower() == "off":
        return 0
    try:
        payload = json.load(sys.stdin)
    except Exception:
        print("factory-guard: unreadable hook input; blocking to fail closed.", file=sys.stderr)
        return 2
    if payload.get("tool_name") != "Bash":
        return 0
    command = (payload.get("tool_input") or {}).get("command", "")
    reason = check(command)
    if reason:
        print(f"factory-guard: {reason} (see CONSTITUTION.md §5)", file=sys.stderr)
        return 2
    return 0


if __name__ == "__main__":
    sys.exit(main())
