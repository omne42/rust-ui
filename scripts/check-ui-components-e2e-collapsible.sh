#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[e2e-collapsible] contract: checklist e2e-selector/stable-wait governance"
cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_check2_documents_e2e_selector_and_stable_wait_rules

echo "[e2e-collapsible] contract: semantic selectors + settled waits"
cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_e2e_selector_contract_uses_semantic_markers_and_settled_waits

echo "[e2e-collapsible] contract: ready/settled semantic breakpoints for disclosure paths"
cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_e2e_contract_covers_ready_and_settled_conditions_for_disclosure_paths

echo "[e2e-collapsible] contract: checklist repeatable keyflow governance"
cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_check2_documents_repeatable_keyflow_regression_rules

echo "[e2e-collapsible] contract: repeatable open-interact-close keyflow in e2e suite"
cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_e2e_regression_suite_contains_repeatable_disclosure_keyflow

echo "[e2e-collapsible] contract: semantic breakpoints localize e2e failures"
cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_e2e_regression_failures_map_to_semantic_contract_breakpoints

echo "[e2e-collapsible] contract: high-risk focus/keyboard paths prioritized"
cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_e2e_regression_prioritizes_focus_and_keyboard_risk_paths

echo "[e2e-collapsible] OK"
