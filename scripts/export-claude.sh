#!/usr/bin/env bash
# Copy what Claude Code learned on this machine into the factory repo for review,
# so it survives losing the machine or the account.
#
#   scripts/export-claude.sh
#
# Writes .claude-export/<date>/ (gitignored) with: the global CLAUDE.md, the per-project
# memory files (~/.claude/projects/*/memory/*.md), and the names of any user-level
# skills and agents that are not part of the factory. It copies no credentials,
# sessions or transcripts.
#
# REVIEW BEFORE COMMITTING. Memories can contain private details. Fold anything
# durable into docs/operator/USER.md or a product repo's AGENTS.md, then delete the
# export instead of committing it raw.
set -euo pipefail
FACTORY="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
H="${CLAUDE_CONFIG_DIR:-$HOME/.claude}"
OUT="$FACTORY/.claude-export/$(date +%Y-%m-%d)"
mkdir -p "$OUT/memory"

[[ -f "$H/CLAUDE.md" ]] && cp "$H/CLAUDE.md" "$OUT/CLAUDE.global.md"
n=0
for d in "$H"/projects/*/memory; do
  [[ -d "$d" ]] || continue
  proj="$(basename "$(dirname "$d")")"
  mkdir -p "$OUT/memory/$proj"
  find "$d" -maxdepth 1 -name '*.md' -exec cp {} "$OUT/memory/$proj/" \; && n=$((n+1))
done
{
  echo "# User-level skills and agents not managed by the factory"
  for x in "$H"/skills/* "$H"/agents/*; do [[ -e "$x" && ! -L "$x" ]] && echo "- ${x#$H/}"; done
} > "$OUT/inventory.md"

echo "Exported to ${OUT#$FACTORY/} ($n project memory dirs)."
echo "Review it, move durable facts into docs/operator/USER.md, then delete the raw export."
