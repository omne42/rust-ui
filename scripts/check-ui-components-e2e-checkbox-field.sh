#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[e2e-checkbox-field] contract: checklist e2e-selector/stable-wait governance"
cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_check2_documents_e2e_selector_and_stable_wait_rules

echo "[e2e-checkbox-field] contract: semantic selectors + settled waits"
cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_e2e_selector_contract_uses_semantic_markers_and_settled_waits

echo "[e2e-checkbox-field] contract: ready/settled semantic breakpoints"
cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_e2e_contract_covers_ready_and_settled_conditions_for_checkbox_field_paths

echo "[e2e-checkbox-field] contract: checklist repeatable-key-flow governance"
cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_check2_documents_e2e_repeatable_key_flow_rules

echo "[e2e-checkbox-field] contract: repeatable key flow with semantic ready/settled breakpoints"
cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_e2e_key_flow_is_repeatable_and_failure_points_are_semantic

echo "[e2e-checkbox-field] OK"
