#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[platform] compile-only: default native path"
cargo check -p ui-components

echo "[platform] compile-only: minimal native path"
cargo check -p ui-components --no-default-features --features component-button,inject-css

echo "[platform] compile-only: ui-motion native path"
cargo check -p ui-motion

echo "[platform] compile-only: ssr native path"
cargo check -p ui-headless --no-default-features --features ssr

echo "[platform] compile-only: web wasm path (ui-headless)"
cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web

echo "[platform] compile-only: ui-motion wasm path"
cargo check -p ui-motion --target wasm32-unknown-unknown

echo "[platform] compile-only: web wasm path"
cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features component-button,inject-css

echo "[platform] compile guard: ui-headless web+ssr must fail"
MUTEX_LOG="$(mktemp)"
if cargo check -p ui-headless --no-default-features --features web,ssr >"$MUTEX_LOG" 2>&1; then
  echo "[platform] expected ui-headless web+ssr to fail, but command succeeded" >&2
  cat "$MUTEX_LOG" >&2
  rm -f "$MUTEX_LOG"
  exit 1
fi
if ! rg -n "mutually exclusive" "$MUTEX_LOG" >/dev/null; then
  echo "[platform] ui-headless web+ssr failed for an unexpected reason" >&2
  cat "$MUTEX_LOG" >&2
  rm -f "$MUTEX_LOG"
  exit 1
fi
rm -f "$MUTEX_LOG"

echo "[platform] ui-motion non-wasm stub tests"
cargo test -p ui-motion --test non_wasm_stub

echo "[platform] source guard: non-wasm button files must not reference web_sys"
for file in \
  crates/ui-components/src/button/mod.rs \
  crates/ui-components/src/button/logic.rs \
  crates/ui-components/src/button/spec.rs \
  crates/ui-components/src/button/styles.rs \
  crates/ui-components/src/button/view.rs
do
  if rg -n "web_sys" "$file" >/dev/null; then
    echo "[platform] forbidden web_sys reference in non-wasm path file: $file" >&2
    exit 1
  fi
done

echo "[platform] source guard: button motion must keep explicit wasm/non-wasm branches"
if ! rg -n -F '#[cfg(target_arch = "wasm32")]' crates/ui-components/src/button/motion.rs >/dev/null; then
  echo "[platform] missing wasm cfg branch in button motion" >&2
  exit 1
fi

if ! rg -n -F '#[cfg(not(target_arch = "wasm32"))]' crates/ui-components/src/button/motion.rs >/dev/null; then
  echo "[platform] missing non-wasm cfg branch in button motion" >&2
  exit 1
fi

echo "[platform] OK"
