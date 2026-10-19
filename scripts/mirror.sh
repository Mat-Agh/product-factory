#!/usr/bin/env bash
# Keep the factory alive if any one account disappears.
#
#   scripts/mirror.sh add <name> <git-url>   add a mirror remote (e.g. gitlab, codeberg, a USB bare repo)
#   scripts/mirror.sh push                   push every branch and tag to every remote
#   scripts/mirror.sh bundle [dir]           write an offline git bundle (default ~/Backups/product-factory)
#
# A bundle restores with: git clone product-factory-<date>.bundle product-factory
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."

case "${1:-}" in
  add)
    git remote add "${2:?name}" "${3:?url}"
    git push "$2" --all && git push "$2" --tags
    echo "Mirror '$2' added and pushed." ;;
  push)
    for r in $(git remote); do
      echo "-> $r"; git push "$r" --all && git push "$r" --tags || echo "  push to $r failed"
    done ;;
  bundle)
    dir="${2:-$HOME/Backups/product-factory}"; mkdir -p "$dir"
    f="$dir/product-factory-$(date +%Y%m%d).bundle"
    git bundle create "$f" --all && git bundle verify "$f" >/dev/null && echo "Wrote $f" ;;
  *) sed -n '2,9p' "$0"; exit 1 ;;
esac
