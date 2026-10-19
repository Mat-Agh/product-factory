#!/usr/bin/env bash
# Product Factory installer.
#
#   scripts/install.sh project <path>   install the kit into a product repo (copies; you commit them)
#   scripts/install.sh user             link the factory into ~/.claude for every project on this machine
#   scripts/install.sh all <path>       both
#
# Never commits, never overwrites a project's settings.json (merges with jq),
# never overwrites an existing descriptor or backlog.
set -euo pipefail

FACTORY="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
KIT="$FACTORY/kit"
REV="$(git -C "$FACTORY" rev-parse --short HEAD 2>/dev/null || echo unknown)"

say()  { printf '  %s\n' "$*"; }
warn() { printf '  ! %s\n' "$*" >&2; }

merge_hook() { # <settings.json> <hook command>
  local settings="$1" cmd="$2" tmp
  mkdir -p "$(dirname "$settings")"
  [[ -f "$settings" ]] || echo '{}' > "$settings"
  if ! command -v jq >/dev/null; then
    warn "jq not found: add this PreToolUse hook to $settings yourself:"
    sed "s#\"python3 [^\"]*\"#\"$cmd\"#" "$KIT/.claude/settings.factory.json" >&2
    return
  fi
  if jq -e --arg c "$cmd" '[.hooks.PreToolUse[]?.hooks[]?.command] | index($c)' "$settings" >/dev/null; then
    say "guard hook already wired in $settings"; return
  fi
  tmp="$(mktemp)"
  jq --arg c "$cmd" '.hooks.PreToolUse = ((.hooks.PreToolUse // []) + [{"matcher":"Bash","hooks":[{"type":"command","command":$c}]}])' \
    "$settings" > "$tmp" && mv "$tmp" "$settings"
  say "wired guard hook into $settings"
}

install_project() {
  local root="${1:?usage: install.sh project <path>}"
  root="$(cd "$root" && pwd)"
  git -C "$root" rev-parse --show-toplevel >/dev/null 2>&1 || { warn "$root is not a git repo"; exit 1; }
  echo "Installing Product Factory @ $REV into $root"

  mkdir -p "$root/.claude/skills" "$root/.claude/agents" "$root/.claude/hooks" "$root/.factory"
  rm -rf "$root/.claude/skills/factory"
  cp -RL "$KIT/.claude/skills/factory" "$root/.claude/skills/factory"   # -L: ship the constitution, not a link
  python3 - "$root/.claude/skills/factory/CONSTITUTION.md" <<'PY_STRIP'
import re, sys
p = sys.argv[1]; s = open(p).read()
s = re.sub(r"\n*<!-- documentation-routing:start -->.*?<!-- documentation-routing:end -->\n?", "\n", s, flags=re.S)
open(p, "w").write(s)  # its doc-tree links only resolve inside the factory repo
PY_STRIP
  cp "$KIT"/.claude/agents/*.md "$root/.claude/agents/"
  cp "$KIT/.claude/hooks/factory-guard.py" "$root/.claude/hooks/"
  chmod +x "$root/.claude/hooks/factory-guard.py"
  say "copied skill, $(ls "$KIT"/.claude/agents/*.md | wc -l | tr -d ' ') agents and guard hook"

  merge_hook "$root/.claude/settings.json" 'python3 "$CLAUDE_PROJECT_DIR/.claude/hooks/factory-guard.py"'

  for f in factory.yaml backlog.md; do
    if [[ -e "$root/.factory/$f" ]]; then say "kept existing .factory/$f"
    else cp "$KIT/.factory/$f" "$root/.factory/$f"; say "created .factory/$f (run /factory init to fill it)"; fi
  done
  cp "$KIT/.factory/dashboard.html" "$KIT/.factory/docs-map.py" "$root/.factory/"
  chmod +x "$root/.factory/docs-map.py"

  touch "$root/.gitignore"
  grep -qxF '.factory/jobs/' "$root/.gitignore" || { echo '.factory/jobs/' >> "$root/.gitignore"; say "added .factory/jobs/ to .gitignore"; }

  if [[ -f "$root/AGENTS.md" ]] && grep -q '^## Product Factory' "$root/AGENTS.md"; then
    say "AGENTS.md already has the Product Factory section"
  else
    { [[ -f "$root/AGENTS.md" ]] && echo; sed '1d' "$KIT/AGENTS.factory.md"; } >> "$root/AGENTS.md"
    say "appended the Product Factory section to AGENTS.md"
  fi
  [[ -e "$root/CLAUDE.md" ]] || { echo '@AGENTS.md' > "$root/CLAUDE.md"; say "created CLAUDE.md -> @AGENTS.md"; }

  printf 'source: %s\nrevision: %s\ninstalled: %s\n' "$(git -C "$FACTORY" remote get-url origin 2>/dev/null || echo "$FACTORY")" \
    "$REV" "$(date -u +%Y-%m-%dT%H:%M:%SZ)" > "$root/.factory/installed.lock"

  echo "Done. Nothing was committed. Next: open Claude Code in $root, run /factory init, review, commit."
  echo "Docs map: python3 .factory/docs-map.py (add --fix to generate README links and the map)."
}

link() { # <src> <dst>
  if [[ -e "$2" && ! -L "$2" ]]; then warn "$2 exists and is not a link; left alone"; return; fi
  ln -sfn "$1" "$2"
}

install_user() {
  local home_claude="${CLAUDE_CONFIG_DIR:-$HOME/.claude}"
  echo "Linking Product Factory @ $REV into $home_claude"
  mkdir -p "$home_claude/skills" "$home_claude/agents" "$home_claude/hooks"
  link "$KIT/.claude/skills/factory" "$home_claude/skills/factory"
  for a in "$KIT"/.claude/agents/*.md; do link "$a" "$home_claude/agents/$(basename "$a")"; done
  link "$KIT/.claude/hooks/factory-guard.py" "$home_claude/hooks/factory-guard.py"
  say "linked /factory skill, agents and guard (they follow 'git pull' in $FACTORY)"

  local claude_md="$home_claude/CLAUDE.md" line="@$FACTORY/docs/operator/USER.md"
  touch "$claude_md"
  grep -qxF "$line" "$claude_md" || { printf '\n# Product Factory operator profile\n%s\n' "$line" >> "$claude_md"; say "imported docs/operator/USER.md into $claude_md"; }

  if [[ "${WITH_USER_GUARD:-0}" == 1 ]]; then
    merge_hook "$home_claude/settings.json" "python3 \"$home_claude/hooks/factory-guard.py\""
  else
    say "user-wide guard hook not enabled (set WITH_USER_GUARD=1 to apply it in every project)"
  fi
  echo "Done."
}

case "${1:-}" in
  project) install_project "${2:-}";;
  user)    install_user;;
  all)     install_user; install_project "${2:-}";;
  *) sed -n '2,9p' "$0"; exit 1;;
esac
