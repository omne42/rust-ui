#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

echo "[e2e-autocomplete] contract: checklist e2e-selector/stable-wait governance"
cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_check2_documents_e2e_selector_and_stable_wait_rules

echo "[e2e-autocomplete] contract: semantic selectors + wasm-stable waits"
cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_e2e_selectors_are_semantic_and_wasm_wait_strategy_is_stable

echo "[e2e-autocomplete] contract: ready/settled semantic breakpoints"
cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_e2e_contract_covers_ready_and_settled_semantic_breakpoints

echo "[e2e-autocomplete] contract: repeatable key flow with semantic breakpoints"
cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_e2e_key_flow_is_repeatable_with_semantic_breakpoints

echo "[e2e-autocomplete] contract: checklist repeatable-flow governance"
cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_check2_documents_e2e_repeatable_key_flow_rules

echo "[e2e-autocomplete] contract: high-risk overlay/focus/keyboard path uses settled semantic breakpoints"
cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints

echo "[e2e-autocomplete] OK"
