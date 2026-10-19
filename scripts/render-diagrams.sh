#!/usr/bin/env bash
# Render Mermaid diagram sources to committed SVG images.
#
#   scripts/render-diagrams.sh            render every docs/assets/diagrams/*.mmd to .svg
#   scripts/render-diagrams.sh --check    verify every .svg exists and matches its .mmd (no browser needed)
#
# Why: docs show diagrams as SVG images, so they display on GitHub, GitLab mirrors,
# IDEs and offline bundles even when GitHub's live Mermaid renderer is unreachable.
# The .mmd file is the only source; each SVG records the sha256 of the source it came from.
#
# Needs Node.js. Uses $MMDC if set, else `mmdc` on PATH, else `npx @mermaid-js/mermaid-cli@11`.
# Set MERMAID_CHROME to a Chrome/Chromium binary to avoid Puppeteer's browser download
# (auto-detected from google-chrome / chromium / chromium-browser when unset).
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."
DIR=docs/assets/diagrams

sha() { python3 -c 'import hashlib,sys; print(hashlib.sha256(open(sys.argv[1],"rb").read()).hexdigest())' "$1"; }

if [[ "${1:-}" == "--check" ]]; then
  bad=0
  for src in "$DIR"/*.mmd; do
    svg="${src%.mmd}.svg"
    if [[ ! -f "$svg" ]]; then echo "FAIL missing $svg"; bad=1; continue; fi
    grep -q "source-sha256: $(sha "$src")" "$svg" || { echo "FAIL stale $svg (re-run scripts/render-diagrams.sh)"; bad=1; }
  done
  [[ $bad == 0 ]] && echo "diagrams OK ($(ls "$DIR"/*.mmd | wc -l | tr -d ' ') sources)"
  exit $bad
fi

if [[ -n "${MMDC:-}" ]]; then cmd=("$MMDC")
elif command -v mmdc >/dev/null; then cmd=(mmdc)
else cmd=(npx -y @mermaid-js/mermaid-cli@11); fi

chrome="${MERMAID_CHROME:-$(command -v google-chrome || command -v chromium || command -v chromium-browser || true)}"
pp="$(mktemp)"; trap 'rm -f "$pp"' EXIT
if [[ -n "$chrome" ]]; then
  printf '{"executablePath":"%s","args":["--no-sandbox"]}\n' "$chrome" > "$pp"
  export PUPPETEER_SKIP_DOWNLOAD=1
else
  echo '{"args":["--no-sandbox"]}' > "$pp"
fi

for src in "$DIR"/*.mmd; do
  svg="${src%.mmd}.svg"
  "${cmd[@]}" -q -p "$pp" -t neutral -b white -i "$src" -o "$svg"
  if grep -qi 'syntax error' "$svg"; then echo "FAIL $src: Mermaid syntax error"; exit 1; fi
  { echo "<!-- source-sha256: $(sha "$src") — generated from $(basename "$src") by scripts/render-diagrams.sh; do not edit -->"; cat "$svg"; } > "$svg.tmp"
  mv "$svg.tmp" "$svg"
  echo "rendered $svg"
done
