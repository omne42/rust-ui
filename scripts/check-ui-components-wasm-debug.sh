#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[wasm-debug] compile-only: button wasm debug feature path"
cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features inject-css,button-wasm-debug

echo "[wasm-debug] contract: button wasm debug feature/replay markers"
cargo test -p ui-components --test button_semantics button_wasm_debug_contract_is_feature_gated_and_dev_only

echo "[wasm-debug] contract: tag reuses shared wasm debug contract"
cargo test -p ui-components --test tag_semantics --no-default-features --features component-tag,inject-css tag_wasm_debug_contract_reuses_global_debug_trace_and_keeps_feature_isolated

echo "[wasm-debug] contract: tag-group reuses shared wasm debug contract"
cargo test -p ui-components --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_wasm_debug_contract_reuses_global_debug_trace_and_keeps_feature_isolated

echo "[wasm-debug] contract: well keeps wasm debug isolation and reuses global trace overlay"
cargo test -p ui-components --test well_semantics --no-default-features --features component-well,inject-css well_wasm_debug_capability_stays_feature_isolated_and_non_polluting
cargo test -p ui-components --test well_semantics --no-default-features --features component-well,inject-css well_wasm_debug_observability_reuses_global_trace_overlay_with_timestamped_events

echo "[wasm-debug] contract: swatch reuses shared wasm debug contract"
cargo test -p ui-components --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_wasm_debug_contract_reuses_global_debug_trace_and_keeps_feature_isolated

echo "[wasm-debug] contract: textarea reuses shared wasm debug contract"
cargo test -p ui-components --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_wasm_debug_contract_reuses_global_debug_trace_and_keeps_feature_isolated

echo "[wasm-debug] contract: time-field reuses shared wasm debug contract"
cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_wasm_debug_contract_reuses_global_debug_trace_and_keeps_feature_isolated

echo "[wasm-debug] contract: slider reuses shared wasm debug contract"
cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated

echo "[wasm-debug] contract: scroll-area reuses shared wasm debug contract"
cargo test -p ui-components --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated

echo "[wasm-debug] OK"
