#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[dx] contract: playground css hot-reload path"
cargo test -p ui-layout --test button_semantics button_dx_playground_supports_css_hot_reload_without_wasm_rebuild

echo "[dx] contract: button workbench optional state persistence"
cargo test -p ui-layout --test button_semantics button_dx_workbench_supports_optional_state_persistence_and_isolated_canvas

echo "[dx] contract: button-copy workbench optional state persistence"
cargo test -p ui-layout --test button_copy_semantics button_copy_dx_workbench_supports_optional_state_persistence_and_isolated_canvas

echo "[dx] contract: action-button playground css hot-reload path"
cargo test -p ui-layout --test action_button_semantics action_button_dx_playground_supports_css_hot_reload_without_wasm_rebuild

echo "[dx] contract: action-button workbench optional state persistence"
cargo test -p ui-layout --test action_button_semantics action_button_dx_workbench_supports_optional_state_persistence_and_isolated_canvas

echo "[dx] contract: well reuses playground css hot-reload and isolated canvas"
cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_dx_playground_supports_css_hot_reload_without_wasm_rebuild
cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_dx_non_interactive_scope_keeps_isolated_canvas_and_marks_persist_state_na

echo "[dx] contract: tabs playground css hot-reload + workbench persistence"
cargo test -p ui-layout --test tabs_semantics tabs_dx_playground_supports_css_hot_reload_without_wasm_rebuild
cargo test -p ui-layout --test tabs_semantics tabs_dx_workbench_supports_optional_state_persistence_and_isolated_canvas

echo "[dx] contract: swatch playground css hot-reload + isolated canvas"
cargo test -p ui-layout --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_dx_playground_supports_css_hot_reload_without_wasm_rebuild
cargo test -p ui-layout --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_dx_interactive_scope_keeps_isolated_canvas_and_context_visible_with_optional_persist_na

echo "[dx] contract: textarea playground css hot-reload + isolated canvas"
cargo test -p ui-layout --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_dx_playground_supports_css_hot_reload_without_wasm_rebuild
cargo test -p ui-layout --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_dx_interactive_scope_keeps_isolated_canvas_and_context_visible_with_optional_persist_na
cargo test -p ui-layout --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_docs_app_provides_interactive_playground_for_props_state_and_preview
cargo test -p ui-layout --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_interactive_playground_reuses_repeatable_semantic_e2e_flow

echo "[dx] contract: time-field playground css hot-reload + isolated canvas"
cargo test -p ui-layout --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_dx_playground_supports_css_hot_reload_without_wasm_rebuild
cargo test -p ui-layout --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_dx_interactive_scope_keeps_isolated_canvas_and_context_visible_with_optional_persist_na
cargo test -p ui-layout --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_docs_app_provides_interactive_playground_for_props_state_and_preview
cargo test -p ui-layout --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_interactive_playground_reuses_repeatable_semantic_e2e_flow

echo "[dx] contract: slider playground css hot-reload + isolated canvas"
cargo test -p ui-layout --test slider_semantics --no-default-features --features component-slider,inject-css slider_dx_playground_supports_css_hot_reload_without_wasm_rebuild
cargo test -p ui-layout --test slider_semantics --no-default-features --features component-slider,inject-css slider_dx_interactive_scope_keeps_isolated_canvas_and_context_visible_with_optional_persist_na

echo "[dx] contract: scroll-area playground css hot-reload + isolated canvas"
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_dx_playground_supports_css_hot_reload_without_wasm_rebuild
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_dx_interactive_scope_keeps_isolated_canvas_and_context_visible_with_optional_persist_na
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_docs_app_provides_interactive_playground_for_props_state_and_preview
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_interactive_playground_reuses_repeatable_semantic_e2e_flow

echo "[dx] OK"
