#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
cd "$ROOT_DIR"

echo "[e2e-separator] contract: checklist e2e-selector/stable-wait governance"
cargo test -p ui-layout --test separator_semantics --no-default-features --features component-separator,inject-css separator_check2_documents_e2e_selector_and_stable_wait_rules

echo "[e2e-separator] contract: semantic selectors + wasm-stable waits"
cargo test -p ui-layout --test separator_semantics --no-default-features --features component-separator,inject-css separator_e2e_selector_contract_uses_semantic_markers_and_stable_waits

echo "[e2e-separator] contract: repeatable key flow with semantic breakpoints"
cargo test -p ui-layout --test separator_semantics --no-default-features --features component-separator,inject-css separator_e2e_key_flow_is_repeatable_and_failure_points_are_semantic

echo "[e2e-separator] contract: checklist repeatable-flow governance + high-risk path policy"
cargo test -p ui-layout --test separator_semantics --no-default-features --features component-separator,inject-css separator_check2_documents_e2e_repeatable_key_flow_rules
cargo test -p ui-layout --test separator_semantics --no-default-features --features component-separator,inject-css separator_e2e_high_risk_paths_are_explicitly_na_for_non_interactive_component

echo "[e2e-separator] OK"
