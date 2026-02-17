#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[wasm-debug] compile-only: button wasm debug feature path"
cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features inject-css,button-wasm-debug

echo "[wasm-debug] contract: button wasm debug feature/replay markers"
cargo test -p ui-components --test button_semantics button_wasm_debug_contract_is_feature_gated_and_dev_only

echo "[wasm-debug] OK"
