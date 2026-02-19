#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

MIN_FEATURES="component-accordion,inject-css"
BUDGET_FILE="$ROOT_DIR/scripts/tree_shaking_budget.env"

echo "[tree-shaking] minimal feature tree"
MIN_TREE_OUTPUT="$(cargo tree -e features -i ui-layout -p ui-layout --no-default-features --features "$MIN_FEATURES")"
echo "$MIN_TREE_OUTPUT"

if ! grep -q 'feature "component-accordion" (command-line)' <<<"$MIN_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: component-accordion" >&2
  exit 1
fi

if ! grep -q 'feature "inject-css" (command-line)' <<<"$MIN_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: inject-css" >&2
  exit 1
fi

if grep -q 'all-components' <<<"$MIN_TREE_OUTPUT"; then
  echo "[tree-shaking] unexpected all-components in minimal feature tree" >&2
  exit 1
fi

echo "[tree-shaking] reverse dependency tree (web-demo)"
WEB_DEMO_TREE_OUTPUT="$(cargo tree -e features -i ui-layout -p web-demo)"
echo "$WEB_DEMO_TREE_OUTPUT"

if grep -q 'all-components' <<<"$WEB_DEMO_TREE_OUTPUT"; then
  echo "[tree-shaking] web-demo should not pull all-components" >&2
  exit 1
fi

if ! grep -q 'web-demo-components' <<<"$WEB_DEMO_TREE_OUTPUT"; then
  echo "[tree-shaking] web-demo should pull web-demo-components feature bundle" >&2
  exit 1
fi

echo "[tree-shaking] minimal wasm check"
cargo check -p ui-layout --target wasm32-unknown-unknown --no-default-features --features "$MIN_FEATURES"

echo "[tree-shaking] minimal wasm release build for budget"
cargo build -p ui-layout --target wasm32-unknown-unknown --release --no-default-features --features "$MIN_FEATURES"

if [[ ! -f "$BUDGET_FILE" ]]; then
  echo "[tree-shaking] missing budget file: $BUDGET_FILE" >&2
  exit 1
fi

# shellcheck source=/dev/null
source "$BUDGET_FILE"

if [[ -z "${TREE_SHAKING_BASELINE_RLIB_BYTES:-}" || -z "${TREE_SHAKING_MAX_RATIO_PERCENT:-}" ]]; then
  echo "[tree-shaking] budget file must define TREE_SHAKING_BASELINE_RLIB_BYTES and TREE_SHAKING_MAX_RATIO_PERCENT" >&2
  exit 1
fi

LATEST_RLIB="$(ls -1t target/wasm32-unknown-unknown/release/deps/libui_layout-*.rlib | head -n 1)"
CURRENT_BYTES="$(stat -c '%s' "$LATEST_RLIB")"
MAX_BYTES=$((TREE_SHAKING_BASELINE_RLIB_BYTES * TREE_SHAKING_MAX_RATIO_PERCENT / 100))

echo "[tree-shaking] budget check"
echo "  latest rlib: $LATEST_RLIB"
echo "  current bytes: $CURRENT_BYTES"
echo "  baseline bytes: $TREE_SHAKING_BASELINE_RLIB_BYTES"
echo "  max ratio: ${TREE_SHAKING_MAX_RATIO_PERCENT}%"
echo "  max bytes: $MAX_BYTES"

if (( CURRENT_BYTES > MAX_BYTES )); then
  echo "[tree-shaking] size regression: $CURRENT_BYTES > $MAX_BYTES" >&2
  exit 1
fi

echo "[tree-shaking] OK"
