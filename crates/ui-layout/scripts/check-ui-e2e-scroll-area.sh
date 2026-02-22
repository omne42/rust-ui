#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
cd "$ROOT_DIR"

echo "[e2e-scroll-area] contract: checklist e2e-selector/stable-wait governance"
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_check2_documents_e2e_selector_and_stable_wait_rules

echo "[e2e-scroll-area] contract: semantic selectors + wasm-stable waits"
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_e2e_selector_contract_uses_semantic_markers_and_stable_waits

echo "[e2e-scroll-area] contract: motion/disabled path ready+settled semantic breakpoints"
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_e2e_ready_and_settled_contract_covers_motion_and_disabled_semantic_breakpoints

echo "[e2e-scroll-area] contract: checklist repeatable key-flow governance + semantic failure breakpoints"
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_check2_documents_e2e_repeatable_key_flow_rules
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_e2e_key_flow_is_repeatable_and_failure_points_are_semantic

echo "[e2e-scroll-area] contract: high-risk focus/keyboard path uses settled semantic markers"
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints

echo "[e2e-scroll-area] OK"
