#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[e2e-button] contract: semantic selectors + settled waits"
# cargo test -p ui-components --test button_semantics button_e2e_selector_contract_uses_semantic_markers_and_settled_waits
CARGO_TARGET_DIR=target-button-check2 cargo test -p ui-components --test button_semantics --no-default-features --features component-button,inject-css button_e2e_selector_contract_uses_semantic_markers_and_settled_waits

echo "[e2e-button] contract: keyboard flow + code snapshot sync"
# cargo test -p ui-components --test button_semantics button_e2e_key_flow_covers_keyboard_and_code_sync_path
CARGO_TARGET_DIR=target-button-check2 cargo test -p ui-components --test button_semantics --no-default-features --features component-button,inject-css button_e2e_key_flow_covers_keyboard_and_code_sync_path

echo "[e2e-button] contract: checklist-backed semantic selector stability"
# cargo test -p ui-components --test button_semantics button_check2_documents_e2e_selector_stability_rules
CARGO_TARGET_DIR=target-button-check2 cargo test -p ui-components --test button_semantics --no-default-features --features component-button,inject-css button_check2_documents_e2e_selector_stability_rules

echo "[e2e-button] contract: repeatable key flow regression set"
# cargo test -p ui-components --test button_semantics button_e2e_flow_is_in_repeatable_regression_set
CARGO_TARGET_DIR=target-button-check2 cargo test -p ui-components --test button_semantics --no-default-features --features component-button,inject-css button_e2e_flow_is_in_repeatable_regression_set
# cargo test -p ui-components --test button_semantics button_check2_documents_repeatable_e2e_regression_rules
CARGO_TARGET_DIR=target-button-check2 cargo test -p ui-components --test button_semantics --no-default-features --features component-button,inject-css button_check2_documents_repeatable_e2e_regression_rules

echo "[e2e-button] contract: button-copy repeatable key flow"
# cargo test -p ui-components --test button_copy_semantics button_copy_e2e_flow_is_in_repeatable_regression_set
CARGO_TARGET_DIR=target-button-check2 cargo test -p ui-components --test button_copy_semantics --no-default-features --features component-button_copy,inject-css button_copy_e2e_flow_is_in_repeatable_regression_set

echo "[e2e-button] OK"
