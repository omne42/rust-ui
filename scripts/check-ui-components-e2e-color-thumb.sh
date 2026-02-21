#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[e2e-color-thumb] contract: checklist e2e-selector/stable-wait governance"
cargo test -p ui-components --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_check2_documents_e2e_selector_and_stable_wait_rules

echo "[e2e-color-thumb] contract: semantic selectors + wasm-stable waits"
cargo test -p ui-components --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_e2e_selector_contract_uses_semantic_markers_and_stable_waits

echo "[e2e-color-thumb] contract: interaction path ready/settled semantic breakpoints"
cargo test -p ui-components --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_e2e_animation_path_covers_ready_and_settled_semantic_breakpoints

echo "[e2e-color-thumb] contract: checklist repeatable e2e regression collection"
cargo test -p ui-components --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_check2_documents_repeatable_e2e_regression_collection

echo "[e2e-color-thumb] contract: repeatable key flow + high-risk semantic breakpoints"
cargo test -p ui-components --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_e2e_key_flow_is_repeatable_and_failure_points_are_semantic
cargo test -p ui-components --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints

echo "[e2e-color-thumb] contract: interactive playground semantic flow is repeatable"
cargo test -p ui-components --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_interactive_playground_reuses_repeatable_semantic_e2e_flow
cargo test -p ui-components --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_e2e_check_script_covers_interactive_playground_contract

echo "[e2e-color-thumb] OK"
