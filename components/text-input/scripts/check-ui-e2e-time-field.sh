#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
cd "$ROOT_DIR"

echo "[e2e-time-field] contract: checklist e2e-selector/stable-wait governance"
cargo test -p ui --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_check2_documents_e2e_selector_and_stable_wait_rules

echo "[e2e-time-field] contract: semantic selectors + settled waits"
cargo test -p ui --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_e2e_selector_contract_uses_semantic_markers_and_settled_waits

echo "[e2e-time-field] contract: repeatable key flow with semantic breakpoints"
cargo test -p ui --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_e2e_key_flow_is_repeatable_and_failure_points_are_semantic

echo "[e2e-time-field] contract: checklist repeatable-flow governance + high-risk path coverage"
cargo test -p ui --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_check2_documents_e2e_repeatable_key_flow_rules
cargo test -p ui --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints

echo "[e2e-time-field] OK"
