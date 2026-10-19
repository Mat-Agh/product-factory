#!/usr/bin/env bash
# Product Factory health check.
#   scripts/doctor.sh              check this machine
#   scripts/doctor.sh <project>    also check a product repo's install
# Read-only. Exit code = number of failures.
set -uo pipefail

FACTORY="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
KIT="$FACTORY/kit"
fails=0
ok()   { printf '  ok    %s\n' "$*"; }
bad()  { printf '  FAIL  %s\n' "$*"; fails=$((fails+1)); }
note() { printf '  note  %s\n' "$*"; }

echo "Machine"
for t in git python3; do command -v $t >/dev/null && ok "$t" || bad "$t missing"; done
for t in gh glab jq claude; do command -v $t >/dev/null && ok "$t" || note "$t not installed (optional)"; done
command -v gh >/dev/null && { gh auth status >/dev/null 2>&1 && ok "gh authenticated" || note "gh not logged in (gh auth login)"; }
python3 "$FACTORY/tests/guard_test.py" >/dev/null 2>&1 && ok "guard hook tests pass" || bad "guard hook tests fail (python3 tests/guard_test.py)"
python3 "$KIT/.factory/docs-map.py" --root "$FACTORY" --strict --allow kit/ >/dev/null 2>&1 && ok "factory docs map current" || bad "factory docs map stale or broken (python3 kit/.factory/docs-map.py --strict --allow kit/)"
"$FACTORY/scripts/render-diagrams.sh" --check >/dev/null 2>&1 && ok "diagrams match their sources" || bad "diagrams stale (scripts/render-diagrams.sh)"

echo "Factory repo"
remotes="$(git -C "$FACTORY" remote | wc -l | tr -d ' ')"
[[ "$remotes" -ge 2 ]] && ok "$remotes remotes (mirrored)" || note "only $remotes remote; add a mirror: scripts/mirror.sh add <name> <url>"
[[ -z "$(git -C "$FACTORY" status --porcelain)" ]] && ok "clean" || note "uncommitted changes in the factory repo"
if git -C "$FACTORY" rev-parse -q --verify @{u} >/dev/null 2>&1; then
  git -C "$FACTORY" fetch -q 2>/dev/null
  behind="$(git -C "$FACTORY" rev-list --count HEAD..@{u})"
  [[ "$behind" == 0 ]] && ok "up to date with upstream" || note "$behind commit(s) behind upstream; git pull"
else note "no upstream branch set"; fi

H="${CLAUDE_CONFIG_DIR:-$HOME/.claude}"
[[ -L "$H/skills/factory" ]] && ok "user-level /factory linked" || note "user-level not linked (scripts/install.sh user)"

if [[ -n "${1:-}" ]]; then
  P="$(cd "$1" && pwd)"
  echo "Project $P"
  for f in .factory/factory.yaml .factory/backlog.md .claude/skills/factory/SKILL.md .claude/hooks/factory-guard.py AGENTS.md; do
    [[ -e "$P/$f" ]] && ok "$f" || bad "$f missing (scripts/install.sh project $P)"
  done
  grep -qxF '.factory/jobs/' "$P/.gitignore" 2>/dev/null && ok ".factory/jobs/ ignored" || bad ".factory/jobs/ not in .gitignore"
  grep -q 'factory-guard.py' "$P/.claude/settings.json" 2>/dev/null && ok "guard hook wired" || bad "guard hook not in .claude/settings.json"
  grep -q '^## Product Factory' "$P/AGENTS.md" 2>/dev/null && ok "AGENTS.md factory section" || bad "AGENTS.md lacks the Product Factory section"
  for s in Setup "Dev server" Gates; do grep -qiE "^#+ .*$s" "$P/AGENTS.md" 2>/dev/null && ok "AGENTS.md has a $s heading" || note "AGENTS.md: no '$s' heading yet (the factory needs it)"; done
  drift=0
  for a in "$KIT"/.claude/agents/*.md "$KIT"/.claude/skills/factory/*.md "$KIT/.claude/hooks/factory-guard.py" "$KIT/.factory/docs-map.py" "$KIT/.factory/dashboard.html"; do
    rel="${a#$KIT/}"; cmp -s "$a" "$P/$rel" || { drift=1; note "drift: $rel differs from factory kit"; }
  done
  [[ $drift == 0 ]] && ok "kit matches factory @ $(git -C "$FACTORY" rev-parse --short HEAD)" || note "re-run scripts/install.sh project $P to update (review the diff first)"
  if [[ -f "$P/.factory/docs-map.py" ]]; then
    dm="$(cd "$P" && python3 .factory/docs-map.py 2>&1 | tail -1)"
    [[ "$dm" == "docs map OK"* ]] && ok "$dm" || note "docs: $dm (python3 .factory/docs-map.py --fix)"
  fi
  if command -v gh >/dev/null && git -C "$P" remote get-url origin 2>/dev/null | grep -q github; then
    base="$(sed -n 's/^ *base_branch: *\([^ #]*\).*/\1/p' "$P/.factory/factory.yaml" | head -1)"
    repo="$(cd "$P" && gh repo view --json nameWithOwner -q .nameWithOwner 2>/dev/null)"
    if [[ -n "$repo" && -n "$base" ]]; then
      gh api "repos/$repo/branches/$base/protection" >/dev/null 2>&1 && ok "$base is protected on GitHub" \
        || note "$base has no branch protection on GitHub (or no permission to read it)"
    fi
  fi
fi

echo; [[ $fails == 0 ]] && echo "Healthy." || echo "$fails failure(s)."
exit $fails
