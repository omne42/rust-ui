#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[e2e-button] contract: semantic selectors + settled waits"
cargo test -p ui-layout --test button_semantics button_e2e_selector_contract_uses_semantic_markers_and_settled_waits

echo "[e2e-button] contract: keyboard flow + code snapshot sync"
cargo test -p ui-layout --test button_semantics button_e2e_key_flow_covers_keyboard_and_code_sync_path

echo "[e2e-button] contract: button-copy repeatable key flow"
cargo test -p ui-layout --test button_copy_semantics button_copy_e2e_flow_is_in_repeatable_regression_set

echo "[e2e-button] OK"
