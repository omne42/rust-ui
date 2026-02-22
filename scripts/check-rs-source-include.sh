#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

# Enforce: source-file text loading via include_str/include_bytes for *.rs paths
# is only allowed in pure test/check files.
matches="$(
  rg -n \
    'include_(str|bytes)!\s*\(\s*"[^"]+\.rs"' \
    crates apps components \
    --glob '**/src/**/*.rs' \
    --glob '!**/src/test/**' \
    --glob '!**/tests/**' \
    --glob '!**/*check*.rs' \
    --glob '!**/*_check.rs' \
    || true
)"

if [[ -n "${matches}" ]]; then
  echo "[rs-source-include] forbidden include_str/include_bytes on .rs in non-test/check code:" >&2
  printf '%s\n' "$matches" >&2
  echo "[rs-source-include] move to runtime API/constant wiring, or limit usage to pure test/check files." >&2
  exit 1
fi

echo "[rs-source-include] OK"
