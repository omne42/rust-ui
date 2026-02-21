#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-/tmp/codex-color-swatch-picker-target}"

echo "[e2e-color-swatch-picker] contract: checklist e2e-selector/stable-wait governance"
cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_check2_documents_e2e_selector_and_stable_wait_rules

echo "[e2e-color-swatch-picker] contract: semantic selectors + wasm-stable waits"
cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_e2e_selector_contract_uses_semantic_markers_and_stable_waits

echo "[e2e-color-swatch-picker] contract: interaction path ready/settled semantic breakpoints"
cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_e2e_animation_path_covers_ready_and_settled_semantic_breakpoints

echo "[e2e-color-swatch-picker] contract: checklist repeatable-flow governance"
cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_check2_documents_repeatable_e2e_regression_collection

echo "[e2e-color-swatch-picker] contract: repeatable key flow + high-risk semantic breakpoints"
cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_e2e_key_flow_is_repeatable_and_failure_points_are_semantic
cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints

echo "[e2e-color-swatch-picker] OK"
