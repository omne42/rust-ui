#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[e2e-checkbox-group] contract: checklist e2e-selector/stable-wait governance"
cargo test -p ui-components --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_check2_documents_e2e_selector_and_stable_wait_rules

echo "[e2e-checkbox-group] contract: semantic selectors + settled waits"
cargo test -p ui-components --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_e2e_selector_contract_uses_semantic_markers_and_settled_waits

echo "[e2e-checkbox-group] contract: ready/settled semantic breakpoints"
cargo test -p ui-components --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_e2e_contract_covers_ready_and_settled_conditions_for_checkbox_group_paths

echo "[e2e-checkbox-group] contract: checklist repeatable e2e governance"
cargo test -p ui-components --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_check2_documents_e2e_repeatable_key_flow_rules

echo "[e2e-checkbox-group] contract: repeatable key flow with semantic breakpoints"
cargo test -p ui-components --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_e2e_key_flow_is_repeatable_and_failure_points_are_semantic

echo "[e2e-checkbox-group] contract: high-risk focus/keyboard/disabled semantic breakpoints"
cargo test -p ui-components --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints

echo "[e2e-checkbox-group] OK"
