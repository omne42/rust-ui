#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
cd "$ROOT_DIR"

export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-/tmp/codex-field-e2e-target}"

echo "[e2e-field] contract: checklist e2e-selector/stable-wait governance"
cargo test -p ui-field field_check2_documents_e2e_selector_and_stable_wait_rules

echo "[e2e-field] contract: semantic selectors + stable waits"
cargo test -p ui-field field_e2e_selector_contract_uses_semantic_markers_and_stable_waits

echo "[e2e-field] contract: motion path uses semantic ready/settled breakpoints"
cargo test -p ui-field field_e2e_flow_covers_ready_and_settled_semantic_breakpoints

echo "[e2e-field] contract: repeatable flow with semantic failure breakpoints"
cargo test -p ui-field field_e2e_flow_is_repeatable_and_failure_points_are_semantic

echo "[e2e-field] contract: high-risk path coverage (focus/keyboard + settled semantic breakpoints)"
cargo test -p ui-field field_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints

echo "[e2e-field] contract: interactive playground flow is repeatable with semantic breakpoints"
cargo test -p ui-field field_interactive_playground_reuses_repeatable_semantic_e2e_flow

echo "[e2e-field] contract: e2e script coverage includes interactive playground contract"
cargo test -p ui-field field_e2e_check_script_covers_interactive_playground_contract

echo "[e2e-field] OK"
