#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
cd "$ROOT_DIR"

export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$ROOT_DIR/target/codex-date-input-group-e2e-target}"

echo "[e2e-date-input-group] contract: checklist e2e-selector/stable-wait governance"
cargo test -p ui-date-input-group date_input_group_check2_documents_e2e_selector_and_stable_wait_rules

echo "[e2e-date-input-group] contract: semantic selectors + stable waits"
cargo test -p ui-date-input-group date_input_group_e2e_selector_contract_uses_semantic_markers_and_stable_waits

echo "[e2e-date-input-group] contract: ready/settled semantic breakpoints"
cargo test -p ui-date-input-group date_input_group_e2e_flow_covers_ready_and_settled_semantic_breakpoints

echo "[e2e-date-input-group] contract: repeatable key flow with semantic failure breakpoints"
cargo test -p ui-date-input-group date_input_group_check2_documents_e2e_repeatable_key_flow_rules
cargo test -p ui-date-input-group date_input_group_e2e_key_flow_is_repeatable_and_failure_points_are_semantic

echo "[e2e-date-input-group] contract: e2e script coverage is complete"
cargo test -p ui-date-input-group date_input_group_e2e_check_script_covers_selector_and_settled_wait_contracts

echo "[e2e-date-input-group] OK"
