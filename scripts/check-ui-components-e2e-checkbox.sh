#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[e2e-checkbox] contract: checklist e2e-selector/stable-wait governance"
cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_check2_documents_e2e_selector_and_stable_wait_rules

echo "[e2e-checkbox] contract: semantic selectors + settled waits"
cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_e2e_selector_contract_uses_semantic_markers_and_settled_waits

echo "[e2e-checkbox] contract: ready/settled semantic breakpoints"
cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_e2e_contract_covers_ready_and_settled_conditions_for_checkbox_paths

echo "[e2e-checkbox] contract: repeatable e2e key flow governance"
cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_check2_documents_repeatable_e2e_regression_collection

echo "[e2e-checkbox] contract: repeatable key flow with semantic breakpoints"
cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_e2e_key_flow_is_repeatable_and_failure_points_are_semantic

echo "[e2e-checkbox] contract: high-risk paths keep focus/keyboard/disabled semantic breakpoints explicit"
cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints

echo "[e2e-checkbox] OK"
