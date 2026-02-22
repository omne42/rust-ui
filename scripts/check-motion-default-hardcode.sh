#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

BASELINE_FILE="$ROOT_DIR/scripts/baseline/motion_default_hardcode_allowlist.txt"

collect_hits() {
  find components crates apps -type f -name '*.rs' -path '*/src/*' | sort | while read -r file; do
    awk -v file="$file" '
      {
        line = $0

        if (!in_impl) {
          if (line ~ /impl[[:space:]]+Default[[:space:]]+for[[:space:]]+[A-Za-z0-9_]*Motion/) {
            in_impl = 1
            impl_name = line
            sub(/^.*for[[:space:]]+/, "", impl_name)
            sub(/[[:space:]]*\{.*$/, "", impl_name)
            open_line = line
            close_line = line
            depth = gsub(/\{/, "{", open_line) - gsub(/\}/, "}", close_line)
            if (depth <= 0) {
              in_impl = 0
              impl_name = ""
              depth = 0
            }
          }
          next
        }

        code = line
        sub(/\/\/.*/, "", code)
        gsub(/[[:space:]]+/, " ", code)
        gsub(/^[[:space:]]+|[[:space:]]+$/, "", code)

        if (code != "" && code ~ /(^|[^A-Za-z_])[-]?[0-9]+([.][0-9]+)?([eE][-+]?[0-9]+)?([^A-Za-z_]|$)/) {
          print file "|" impl_name "|" code
        }

        open_line = line
        close_line = line
        depth += gsub(/\{/, "{", open_line) - gsub(/\}/, "}", close_line)
        if (depth <= 0) {
          in_impl = 0
          impl_name = ""
          depth = 0
        }
      }
    ' "$file"
  done | sort -u
}

if [[ "${1:-}" == "--dump-current" ]]; then
  collect_hits
  exit 0
fi

if [[ ! -f "$BASELINE_FILE" ]]; then
  echo "[motion-hardcode] missing baseline: $BASELINE_FILE" >&2
  echo "[motion-hardcode] run: ./scripts/check-motion-default-hardcode.sh --dump-current > $BASELINE_FILE" >&2
  exit 1
fi

tmp_current="$(mktemp)"
tmp_baseline="$(mktemp)"
tmp_new="$(mktemp)"
trap 'rm -f "$tmp_current" "$tmp_baseline" "$tmp_new"' EXIT

collect_hits >"$tmp_current"
sort -u "$BASELINE_FILE" >"$tmp_baseline"
comm -13 "$tmp_baseline" "$tmp_current" >"$tmp_new"

if [[ -s "$tmp_new" ]]; then
  echo "[motion-hardcode] found new hardcoded numeric defaults in impl Default for *Motion:" >&2
  cat "$tmp_new" >&2
  echo "[motion-hardcode] migrate these defaults to token/preset/shared source, then update baseline only for intentional legacy compatibility." >&2
  exit 1
fi

echo "[motion-hardcode] OK"
