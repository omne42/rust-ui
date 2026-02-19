#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

if command -v sccache >/dev/null 2>&1; then
  export RUSTC_WRAPPER="${RUSTC_WRAPPER:-sccache}"
  echo "[check] sccache enabled (RUSTC_WRAPPER=${RUSTC_WRAPPER})"
fi

echo "[check] fmt"
cargo fmt --all -- --check

echo "[check] rust-hygiene"
./scripts/check-rust-hygiene.sh

echo "[check] protocol"
./scripts/check-component-protocol.sh

echo "[check] clippy"
cargo clippy --workspace --all-targets -- -D warnings

echo "[check] test"
cargo test --workspace

echo "[check] full feature matrix (native, dev)"
cargo check -p ui-components --no-default-features --features inject-css,dev-all-components
cargo check -p ui-layout --no-default-features --features inject-css,dev-all-components

echo "[check] minimal feature matrix (native)"
cargo check -p ui-components --no-default-features --features component-button,inject-css
cargo check -p ui-components --no-default-features --features component-calendar,inject-css
cargo check -p ui-components --no-default-features --features component-date_picker,inject-css
cargo check -p ui-components --no-default-features --features component-date_range_picker,inject-css
cargo check -p ui-components --no-default-features --features component-time_field,inject-css

echo "[check] minimal feature dependency isolation (native)"
BUTTON_TREE="$(cargo tree -e features -p ui-components --no-default-features --features component-button,inject-css)"
if rg -n "ui-logic-calendar|logic-calendar" <<<"$BUTTON_TREE" >/dev/null; then
  echo "[check] unexpected calendar satellite dependency in button-only feature set" >&2
  exit 1
fi

DATE_PICKER_TREE="$(cargo tree -e features -p ui-components --no-default-features --features component-date_picker,inject-css)"
if ! rg -n "ui-logic-calendar|logic-calendar" <<<"$DATE_PICKER_TREE" >/dev/null; then
  echo "[check] missing calendar satellite dependency in date-picker feature set" >&2
  exit 1
fi

echo "[check] ssr (compile-only)"
cargo check -p ui-headless --no-default-features --features ssr

if [[ "${SKIP_WASM:-}" == "1" ]]; then
  echo "[check] wasm (skipped: SKIP_WASM=1)"
  exit 0
fi

SYSROOT="$(rustc --print sysroot)"
if [[ ! -d "$SYSROOT/lib/rustlib/wasm32-unknown-unknown" ]]; then
  echo "[check] wasm32-unknown-unknown target not installed"
  if command -v rustup >/dev/null 2>&1; then
    echo "hint: run 'rustup target add wasm32-unknown-unknown'"
  else
    echo "hint: install rustup, then run 'rustup target add wasm32-unknown-unknown'"
    echo "hint: or temporarily run 'SKIP_WASM=1 ./scripts/check.sh'"
  fi
  exit 1
fi

echo "[check] wasm"
cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web
cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features inject-css,dev-all-components
cargo check -p ui-layout --target wasm32-unknown-unknown --no-default-features --features inject-css,dev-all-components
cargo check -p web-demo --target wasm32-unknown-unknown
cargo check -p docs-app --target wasm32-unknown-unknown

echo "[check] minimal feature matrix (wasm)"
cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features component-button,inject-css
cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features component-time_field,inject-css
