#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[wasm-debug] compile-only: button wasm debug feature path"
cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features inject-css,button-wasm-debug

echo "[wasm-debug] contract: button wasm debug feature/replay markers"
# cargo test -p ui-components --test button_semantics button_wasm_debug_contract_is_feature_gated_and_dev_only
cargo test -p ui-components --test button_semantics --no-default-features --features component-button,inject-css button_wasm_debug_contract_is_feature_gated_and_dev_only

echo "[wasm-debug] contract: tag reuses shared wasm debug contract"
cargo test -p ui-components --test tag_semantics --no-default-features --features component-tag,inject-css tag_wasm_debug_contract_reuses_global_debug_trace_and_keeps_feature_isolated

echo "[wasm-debug] contract: tag-group reuses shared wasm debug contract"
cargo test -p ui-components --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_wasm_debug_contract_reuses_global_debug_trace_and_keeps_feature_isolated

echo "[wasm-debug] contract: well keeps wasm debug isolation and reuses global trace overlay"
# cargo test -p ui-components --test well_semantics --no-default-features --features component-well,inject-css well_wasm_debug_capability_stays_feature_isolated_and_non_polluting
# cargo test -p ui-components --test well_semantics --no-default-features --features component-well,inject-css well_wasm_debug_observability_reuses_global_trace_overlay_with_timestamped_events
cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_wasm_debug_capability_stays_feature_isolated_and_non_polluting
cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_wasm_debug_observability_reuses_global_trace_overlay_with_timestamped_events

echo "[wasm-debug] contract: swatch reuses shared wasm debug contract"
cargo test -p ui-components --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_wasm_debug_contract_reuses_global_debug_trace_and_keeps_feature_isolated

echo "[wasm-debug] contract: textarea reuses shared wasm debug contract"
cargo test -p ui-components --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_wasm_debug_contract_reuses_global_debug_trace_and_keeps_feature_isolated

echo "[wasm-debug] contract: circular-progress stays wasm debug feature-isolated and reuses global trace entry"
cargo test -p ui-components --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_wasm_debug_contract_is_explicitly_na_and_feature_isolated

echo "[wasm-debug] contract: time-field reuses shared wasm debug contract"
cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_wasm_debug_contract_reuses_global_debug_trace_and_keeps_feature_isolated

echo "[wasm-debug] contract: form-field keeps wasm debug N/A boundary and feature isolation"
cargo test -p ui-components --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_wasm_debug_contract_is_na_and_feature_isolated

echo "[wasm-debug] contract: slider reuses shared wasm debug contract"
cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated

echo "[wasm-debug] contract: bottom-sheet reuses shared wasm debug contract"
cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated

echo "[wasm-debug] contract: coachmark reuses global wasm debug trace contract"
cargo test -p ui-components --lib --no-default-features --features component-coachmark,inject-css coachmark_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated

echo "[wasm-debug] contract: checkbox-field stays wasm debug feature-isolated and reuses global trace entry"
cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_wasm_debug_contract_is_explicitly_na_and_feature_isolated

echo "[wasm-debug] contract: checkbox stays wasm debug feature-isolated and reuses global trace entry"
cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_wasm_debug_contract_is_explicitly_na_and_feature_isolated_locally

echo "[wasm-debug] contract: checkbox-group stays wasm debug feature-isolated and reuses global debug overlay entry"
cargo test -p ui-components --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_wasm_debug_contract_is_explicitly_na_and_feature_isolated

echo "[wasm-debug] contract: fieldset reuses shared wasm debug contract"
cargo test -p ui-components --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_wasm_debug_contract_reuses_global_trace_overlay_and_stays_feature_isolated

echo "[wasm-debug] contract: scroll-area reuses shared wasm debug contract"
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated

echo "[wasm-debug] contract: alert-dialog reuses shared wasm debug contract"
cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_wasm_debug_contract_reuses_global_debug_trace_and_keeps_feature_isolated

echo "[wasm-debug] contract: dialog reuses shared wasm debug contract"
cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated

echo "[wasm-debug] contract: chart reuses shared wasm debug contract"
cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated

echo "[wasm-debug] contract: carousel reuses shared wasm debug contract"
cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated

echo "[wasm-debug] contract: collapsible reuses shared wasm debug contract"
cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated

echo "[wasm-debug] contract: command-dialog reuses shared wasm debug contract"
cargo test -p ui-components --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_wasm_debug_contract_reuses_global_trace_and_keeps_feature_isolated

echo "[wasm-debug] contract: modal reuses shared wasm debug contract"
cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated

echo "[wasm-debug] contract: overlays reuses shared wasm debug contract"
cargo test -p ui-overlays overlays_wasm_debug_contract_reuses_global_trace_overlay_and_stays_feature_isolated

echo "[wasm-debug] contract: drawer reuses shared wasm debug contract"
cargo test -p ui-components --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated

echo "[wasm-debug] contract: hover-card reuses shared wasm debug contract"
cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated

echo "[wasm-debug] contract: list keeps wasm debug N/A boundary and feature isolation"
cargo test -p ui-components --test list_module_semantics --no-default-features --features component-list,inject-css list_wasm_debug_contract_is_explicitly_na_and_feature_isolated

echo "[wasm-debug] contract: menu reuses shared wasm debug trace and keeps feature isolation"
cargo test -p ui-menu menu_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated

echo "[wasm-debug] contract: date-input-group keeps wasm debug N/A boundary and feature isolation"
cargo test -p ui-date-input-group date_input_group_wasm_debug_contract_is_na_and_feature_isolated

echo "[wasm-debug] contract: combo-box reuses shared wasm debug contract"
cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_wasm_debug_contract_reuses_global_debug_trace_and_keeps_feature_isolated

echo "[wasm-debug] contract: autocomplete reuses shared wasm debug trace and keeps feature isolation"
cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated

echo "[wasm-debug] contract: drop-zone reuses shared wasm debug contract"
cargo test -p ui-components --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated

echo "[wasm-debug] contract: flip-card keeps wasm debug N/A boundary and feature isolation"
cargo test -p ui-flip-card flip_card_wasm_debug_contract_is_explicitly_na_and_feature_isolated

echo "[wasm-debug] contract: error-view reuses global wasm debug trace contract"
cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated

echo "[wasm-debug] contract: color-editor stays wasm debug feature-isolated and reuses global trace entry"
cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_wasm_debug_contract_is_explicitly_na_and_feature_isolated

echo "[wasm-debug] contract: color-swatch stays wasm debug feature-isolated and reuses global trace entry"
cargo test -p ui-components --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_wasm_debug_contract_is_explicitly_na_and_feature_isolated

echo "[wasm-debug] contract: color-thumb reuses shared wasm debug trace and keeps feature isolated"
cargo test -p ui-components --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_wasm_debug_contract_reuses_shared_trace_and_stays_feature_isolated

echo "[wasm-debug] contract: color-swatch-picker reuses global wasm debug trace contract"
cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated

echo "[wasm-debug] contract: color-picker stays wasm debug feature-isolated and reuses global trace entry"
cargo test -p ui-color-picker color_picker_wasm_debug_contract_is_explicitly_na_and_feature_isolated

echo "[wasm-debug] contract: color-slider stays wasm debug feature-isolated and reuses global trace entry"
cargo test -p ui-color-slider color_slider_wasm_debug_contract_is_explicitly_na_and_feature_isolated

echo "[wasm-debug] OK"
