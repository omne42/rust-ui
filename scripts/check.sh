#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[check] fmt"
cargo fmt --all -- --check

echo "[check] clippy"
cargo clippy --workspace --all-targets -- -D warnings

echo "[check] test"
cargo test --workspace

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
cargo check -p ui-components --target wasm32-unknown-unknown
cargo check -p web-demo --target wasm32-unknown-unknown
