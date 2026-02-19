#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

failures=0

check_root() {
  local root="$1"

  while IFS= read -r mod_path; do
    local dir
    dir="$(dirname "$mod_path")"

    # A component-like directory is any module with at least one layer file.
    local has_layer=0
    for layer in logic.rs view.rs styles.rs motion.rs; do
      if [[ -f "$dir/$layer" ]]; then
        has_layer=1
      fi
    done
    [[ "$has_layer" -eq 1 ]] || continue

    local protocol="$dir/protocol.rs"
    if [[ ! -f "$protocol" ]]; then
      echo "[protocol-check] missing: $protocol" >&2
      failures=1
      continue
    fi

    # Hard contract: protocol must be serde-serializable schema types.
    if ! rg -n "Serialize|Deserialize" "$protocol" >/dev/null; then
      echo "[protocol-check] invalid protocol schema (serde markers missing): $protocol" >&2
      failures=1
    fi
  done < <(find "$root" -type f -name mod.rs | sort)
}

echo "[protocol-check] enforcing protocol.rs in component directories"
check_root "crates/ui-components/src"
check_root "crates/ui-layout/src"

if [[ "$failures" -ne 0 ]]; then
  echo "[protocol-check] failed" >&2
  exit 1
fi

echo "[protocol-check] OK"
