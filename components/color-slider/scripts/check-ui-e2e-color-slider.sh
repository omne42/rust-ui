#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
cd "$ROOT_DIR"

echo "[e2e-color-slider] contract: checklist e2e-selector/stable-wait governance"
cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_check2_documents_e2e_selector_and_stable_wait_rules

echo "[e2e-color-slider] contract: semantic selectors + wasm-stable waits"
cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_e2e_selector_contract_uses_semantic_markers_and_stable_waits

echo "[e2e-color-slider] contract: interaction path ready/settled semantic breakpoints"
cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_e2e_animation_path_covers_ready_and_settled_semantic_breakpoints

echo "[e2e-color-slider] contract: repeatable key flow + high-risk semantic breakpoints"
cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_check2_documents_e2e_repeatable_key_flow_rules
cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_e2e_key_flow_is_repeatable_and_failure_points_are_semantic
cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints

echo "[e2e-color-slider] OK"
