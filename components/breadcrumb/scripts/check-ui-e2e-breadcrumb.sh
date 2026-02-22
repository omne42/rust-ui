#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
cd "$ROOT_DIR"

echo "[e2e-breadcrumb] contract: checklist e2e-selector/stable-wait governance"
cargo test -p ui --test breadcrumb_semantics --no-default-features --features component-breadcrumb,inject-css breadcrumb_check2_documents_e2e_selector_and_stable_wait_rules

echo "[e2e-breadcrumb] contract: semantic selectors + wasm-stable waits"
cargo test -p ui --test breadcrumb_semantics --no-default-features --features component-breadcrumb,inject-css breadcrumb_e2e_selector_contract_uses_semantic_markers_and_stable_waits

echo "[e2e-breadcrumb] contract: async/animation N/A is explicit with semantic settled markers"
cargo test -p ui --test breadcrumb_semantics --no-default-features --features component-breadcrumb,inject-css breadcrumb_e2e_async_and_animation_axes_are_explicitly_not_applicable_and_semantically_settled

echo "[e2e-breadcrumb] contract: checklist repeatable-key-flow governance"
cargo test -p ui --test breadcrumb_semantics --no-default-features --features component-breadcrumb,inject-css breadcrumb_check2_documents_e2e_repeatable_key_flow_rules

echo "[e2e-breadcrumb] contract: repeatable key flow with semantic breakpoints"
cargo test -p ui --test breadcrumb_semantics --no-default-features --features component-breadcrumb,inject-css breadcrumb_e2e_key_flow_is_repeatable_and_failure_points_are_semantic

echo "[e2e-breadcrumb] contract: high-risk path covers focus/keyboard/async semantic breakpoints"
cargo test -p ui --test breadcrumb_semantics --no-default-features --features component-breadcrumb,inject-css breadcrumb_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints

echo "[e2e-breadcrumb] OK"
