#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[e2e-command] contract: checklist e2e-selector/stable-wait governance"
cargo test -p ui-command --lib command_check2_documents_e2e_selector_and_stable_wait_rules

echo "[e2e-command] contract: semantic selectors + settled waits"
cargo test -p ui-command --lib command_e2e_selector_contract_uses_semantic_markers_and_settled_waits

echo "[e2e-command] contract: script guard wiring"
cargo test -p ui-command --lib command_e2e_check_script_covers_selector_contract

echo "[e2e-command] contract: checklist repeatable key-flow governance"
cargo test -p ui-command --lib command_check2_documents_e2e_repeatable_key_flow_rules

echo "[e2e-command] contract: repeatable key flow + semantic breakpoints"
cargo test -p ui-command --lib command_e2e_key_flow_is_repeatable_and_failure_points_are_semantic

echo "[e2e-command] contract: high-risk focus/keyboard path coverage"
cargo test -p ui-command --lib command_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints

echo "[e2e-command] contract: repeatable key-flow script wiring"
cargo test -p ui-command --lib command_e2e_check_script_covers_repeatable_key_flow_contract

echo "[e2e-command] contract: interactive playground repeatable e2e flow"
cargo test -p ui-command --lib command_interactive_playground_reuses_repeatable_semantic_e2e_flow

echo "[e2e-command] contract: interactive playground script wiring"
cargo test -p ui-command --lib command_e2e_check_script_covers_interactive_playground_contract

echo "[e2e-command] OK"
