#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[dx] contract: playground css hot-reload path"
cargo test -p ui-components --test button_semantics button_dx_playground_supports_css_hot_reload_without_wasm_rebuild

echo "[dx] contract: button workbench optional state persistence"
cargo test -p ui-components --test button_semantics button_dx_workbench_supports_optional_state_persistence_and_isolated_canvas

echo "[dx] contract: button-copy workbench optional state persistence"
cargo test -p ui-components --test button_copy_semantics button_copy_dx_workbench_supports_optional_state_persistence_and_isolated_canvas

echo "[dx] contract: action-button playground css hot-reload path"
cargo test -p ui-components --test action_button_semantics action_button_dx_playground_supports_css_hot_reload_without_wasm_rebuild

echo "[dx] contract: action-button workbench optional state persistence"
cargo test -p ui-components --test action_button_semantics action_button_dx_workbench_supports_optional_state_persistence_and_isolated_canvas

echo "[dx] OK"
