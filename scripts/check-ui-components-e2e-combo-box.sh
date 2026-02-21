#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[e2e-combo-box] contract: semantic selectors + settled waits"
CARGO_TARGET_DIR=target-combo-box-check2 cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_e2e_selector_contract_uses_semantic_markers_and_settled_waits

echo "[e2e-combo-box] contract: repeatable flow with semantic ready/settled breakpoints"
CARGO_TARGET_DIR=target-combo-box-check2 cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_e2e_flow_covers_ready_and_settled_semantic_breakpoints

echo "[e2e-combo-box] contract: repeatable regression set covers overlay/focus/keyboard high-risk path"
CARGO_TARGET_DIR=target-combo-box-check2 cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_e2e_regression_set_covers_repeatable_overlay_focus_keyboard_paths

echo "[e2e-combo-box] OK"
