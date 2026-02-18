#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[e2e-color-area] contract: checklist e2e-selector/stable-wait governance"
CARGO_TARGET_DIR=target-colorarea-check2 cargo test -p ui-components --test color_area_semantics --no-default-features --features component-color_area,inject-css color_area_check2_documents_e2e_selector_and_stable_wait_rules

echo "[e2e-color-area] contract: semantic selectors + wasm-stable waits"
CARGO_TARGET_DIR=target-colorarea-check2 cargo test -p ui-components --test color_area_semantics --no-default-features --features component-color_area,inject-css color_area_e2e_selector_contract_uses_semantic_markers_and_stable_waits

echo "[e2e-color-area] contract: repeatable key flow and semantic breakpoints"
CARGO_TARGET_DIR=target-colorarea-check2 cargo test -p ui-components --test color_area_semantics --no-default-features --features component-color_area,inject-css color_area_check2_documents_e2e_repeatable_key_flow_rules
CARGO_TARGET_DIR=target-colorarea-check2 cargo test -p ui-components --test color_area_semantics --no-default-features --features component-color_area,inject-css color_area_e2e_key_flow_is_repeatable_and_failure_points_are_semantic
CARGO_TARGET_DIR=target-colorarea-check2 cargo test -p ui-components --test color_area_semantics --no-default-features --features component-color_area,inject-css color_area_e2e_high_risk_paths_cover_keyboard_and_disabled_branches

echo "[e2e-color-area] OK"
