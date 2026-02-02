#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"

if [[ ! -f "$repo_root/Cargo.toml" ]]; then
  echo "gate: no Cargo.toml found; skipping." >&2
  exit 0
fi

if [[ -z "${SKIP_WASM:-}" ]]; then
  export SKIP_WASM=1
fi

echo "gate: rust (scripts/check.sh)" >&2
(
  cd "$repo_root"
  ./scripts/check.sh
)

