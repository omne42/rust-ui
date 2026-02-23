#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

MIN_FEATURES="component-accordion,inject-css"
AUTOCOMPLETE_MIN_FEATURES="component-autocomplete,inject-css"
BUTTON_MIN_FEATURES="component-button,inject-css"
FIELDSET_MIN_FEATURES="component-fieldset,inject-css"
COMMAND_MIN_FEATURES="component-command,inject-css"
COMMAND_DIALOG_MIN_FEATURES="component-command_dialog,inject-css"
DIALOG_MIN_FEATURES="component-dialog,inject-css"
ALERT_DIALOG_MIN_FEATURES="component-alert_dialog,inject-css"
HOVER_CARD_MIN_FEATURES="component-hover_card,inject-css"
COLOR_EDITOR_MIN_FEATURES="component-color_editor,inject-css"
COLOR_SLIDER_MIN_FEATURES="component-color_slider,inject-css"
COLOR_WHEEL_MIN_FEATURES="component-color_wheel,inject-css"
COLOR_SWATCH_PICKER_MIN_FEATURES="component-color_swatch_picker,inject-css"
COLOR_PICKER_MIN_FEATURES="component-color_picker,inject-css"
ERROR_VIEW_MIN_FEATURES="component-error_view,inject-css"
FLIP_CARD_MIN_FEATURES="component-flip_card,inject-css"
DATE_INPUT_GROUP_MIN_FEATURES="component-date_input_group,inject-css"
CHECKBOX_FIELD_MIN_FEATURES="component-checkbox_field,inject-css"
CHECKBOX_MIN_FEATURES="component-checkbox,inject-css"
CHECKBOX_GROUP_MIN_FEATURES="component-checkbox_group,inject-css"
MODAL_MIN_FEATURES="component-modal,inject-css"
OVERLAYS_MIN_FEATURES="component-overlays,inject-css"
DRAWER_MIN_FEATURES="component-drawer,inject-css"
BOTTOM_SHEET_MIN_FEATURES="component-bottom_sheet,inject-css"
LIST_MIN_FEATURES="component-list,inject-css"
MENU_MIN_FEATURES="component-menu,inject-css"
CIRCULAR_PROGRESS_MIN_FEATURES="component-circular_progress,inject-css"
COACHMARK_MIN_FEATURES="component-coachmark,inject-css"
CHART_MIN_FEATURES="component-chart,inject-css"
CAROUSEL_MIN_FEATURES="component-carousel,inject-css"
COLLAPSIBLE_MIN_FEATURES="component-collapsible,inject-css"
PREVIEW_LINK_CARD_MIN_FEATURES="component-preview_link_card,inject-css"
SIDEBAR_MIN_FEATURES="component-sidebar,inject-css"
BUDGET_FILE="$ROOT_DIR/scripts/tree_shaking_budget.env"

echo "[tree-shaking] button feature registration + gated aggregation contract"
cargo test -p ui --test button_semantics --no-default-features --features component-button,inject-css button_tree_shaking_keeps_component_feature_and_css_boundaries
cargo test -p ui --test button_semantics --no-default-features --features component-button,inject-css button_tree_shaking_check_script_covers_feature_tree_wasm_and_budget
cargo test -p ui --test button_semantics --no-default-features --features component-button,inject-css button_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget
cargo test -p ui --test button_semantics --no-default-features --features component-button,inject-css button_check2_marks_tree_shaking_feature_pruning_contract_complete

echo "[tree-shaking] autocomplete feature registration + gated aggregation contract"
cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_tree_shaking_feature_gates_are_explicit
cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget
cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_check2_marks_tree_shaking_feature_pruning_contract_complete

echo "[tree-shaking] autocomplete minimal feature tree"
AUTOCOMPLETE_TREE_OUTPUT="$(cargo tree -e features -i ui -p ui --no-default-features --features "$AUTOCOMPLETE_MIN_FEATURES")"
echo "$AUTOCOMPLETE_TREE_OUTPUT"

if ! grep -q 'feature "component-autocomplete" (command-line)' <<<"$AUTOCOMPLETE_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: component-autocomplete" >&2
  exit 1
fi

if ! grep -q 'feature "inject-css" (command-line)' <<<"$AUTOCOMPLETE_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: inject-css for autocomplete minimal tree" >&2
  exit 1
fi

if grep -q 'all-components' <<<"$AUTOCOMPLETE_TREE_OUTPUT"; then
  echo "[tree-shaking] autocomplete minimal feature tree should not pull all-components" >&2
  exit 1
fi

echo "[tree-shaking] autocomplete minimal wasm check"
cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features "$AUTOCOMPLETE_MIN_FEATURES"

echo "[tree-shaking] button minimal feature tree"
BUTTON_TREE_OUTPUT="$(cargo tree -e features -i ui -p ui --no-default-features --features "$BUTTON_MIN_FEATURES")"
echo "$BUTTON_TREE_OUTPUT"

if ! grep -q 'feature "component-button" (command-line)' <<<"$BUTTON_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: component-button" >&2
  exit 1
fi

if ! grep -q 'feature "inject-css" (command-line)' <<<"$BUTTON_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: inject-css for button minimal tree" >&2
  exit 1
fi

if grep -q 'all-components' <<<"$BUTTON_TREE_OUTPUT"; then
  echo "[tree-shaking] button minimal feature tree should not pull all-components" >&2
  exit 1
fi

echo "[tree-shaking] button minimal wasm check"
cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features "$BUTTON_MIN_FEATURES"

echo "[tree-shaking] coachmark feature registration + gated aggregation contract"
cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_tree_shaking_contract_is_component_feature_gated_and_budgeted_in_ci
cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget
cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_check2_marks_tree_shaking_feature_pruning_contract_complete

echo "[tree-shaking] coachmark minimal feature tree"
COACHMARK_TREE_OUTPUT="$(cargo tree -e features -i ui -p ui --no-default-features --features "$COACHMARK_MIN_FEATURES")"
echo "$COACHMARK_TREE_OUTPUT"

if ! grep -q 'feature "component-coachmark" (command-line)' <<<"$COACHMARK_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: component-coachmark" >&2
  exit 1
fi

if ! grep -q 'feature "inject-css" (command-line)' <<<"$COACHMARK_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: inject-css for coachmark minimal tree" >&2
  exit 1
fi

if grep -q 'all-components' <<<"$COACHMARK_TREE_OUTPUT"; then
  echo "[tree-shaking] coachmark minimal feature tree should not pull all-components" >&2
  exit 1
fi

echo "[tree-shaking] coachmark minimal wasm check"
cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features "$COACHMARK_MIN_FEATURES"

echo "[tree-shaking] color-editor feature registration + gated aggregation contract"
cargo test -p ui --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_tree_shaking_feature_pruning_is_gated_in_lib_and_css
cargo test -p ui --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_check2_marks_tree_shaking_feature_pruning_contract_complete
cargo test -p ui --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget

echo "[tree-shaking] color-editor minimal feature tree"
COLOR_EDITOR_TREE_OUTPUT="$(cargo tree -e features -i ui -p ui --no-default-features --features "$COLOR_EDITOR_MIN_FEATURES")"
echo "$COLOR_EDITOR_TREE_OUTPUT"

if ! grep -q 'feature "component-color_editor" (command-line)' <<<"$COLOR_EDITOR_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: component-color_editor" >&2
  exit 1
fi

if ! grep -q 'feature "inject-css" (command-line)' <<<"$COLOR_EDITOR_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: inject-css for color-editor minimal tree" >&2
  exit 1
fi

if grep -q 'all-components' <<<"$COLOR_EDITOR_TREE_OUTPUT"; then
  echo "[tree-shaking] color-editor minimal feature tree should not pull all-components" >&2
  exit 1
fi

echo "[tree-shaking] color-editor minimal wasm check"
cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features "$COLOR_EDITOR_MIN_FEATURES"

echo "[tree-shaking] color-slider feature registration + gated aggregation contract"
cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_tree_shaking_contract_is_feature_gated_and_budget_guarded
cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_check2_marks_tree_shaking_feature_pruning_contract_complete

echo "[tree-shaking] color-slider minimal feature tree"
COLOR_SLIDER_TREE_OUTPUT="$(cargo tree -e features -i ui -p ui --no-default-features --features "$COLOR_SLIDER_MIN_FEATURES")"
echo "$COLOR_SLIDER_TREE_OUTPUT"

if ! grep -q 'feature "component-color_slider" (command-line)' <<<"$COLOR_SLIDER_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: component-color_slider" >&2
  exit 1
fi

if ! grep -q 'feature "inject-css" (command-line)' <<<"$COLOR_SLIDER_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: inject-css for color-slider minimal tree" >&2
  exit 1
fi

if grep -q 'all-components' <<<"$COLOR_SLIDER_TREE_OUTPUT"; then
  echo "[tree-shaking] color-slider minimal feature tree should not pull all-components" >&2
  exit 1
fi

echo "[tree-shaking] color-slider minimal wasm check"
cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features "$COLOR_SLIDER_MIN_FEATURES"

echo "[tree-shaking] color-wheel feature registration + gated aggregation contract"
cargo test -p ui --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_tree_shaking_contract_is_feature_gated_and_budget_guarded
cargo test -p ui --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_check2_marks_tree_shaking_feature_pruning_contract_complete
cargo test -p ui --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget

echo "[tree-shaking] color-wheel minimal feature tree"
COLOR_WHEEL_TREE_OUTPUT="$(cargo tree -e features -i ui -p ui --no-default-features --features "$COLOR_WHEEL_MIN_FEATURES")"
echo "$COLOR_WHEEL_TREE_OUTPUT"

if ! grep -q 'feature "component-color_wheel" (command-line)' <<<"$COLOR_WHEEL_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: component-color_wheel" >&2
  exit 1
fi

if ! grep -q 'feature "inject-css" (command-line)' <<<"$COLOR_WHEEL_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: inject-css for color-wheel minimal tree" >&2
  exit 1
fi

if grep -q 'all-components' <<<"$COLOR_WHEEL_TREE_OUTPUT"; then
  echo "[tree-shaking] color-wheel minimal feature tree should not pull all-components" >&2
  exit 1
fi

echo "[tree-shaking] color-wheel minimal wasm check"
cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features "$COLOR_WHEEL_MIN_FEATURES"

echo "[tree-shaking] color-swatch feature registration + gated aggregation contract"
cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_tree_shaking_contract_enforces_component_feature_gates_and_budgeted_ci
cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_check2_marks_tree_shaking_feature_pruning_contract_complete

echo "[tree-shaking] color-swatch-picker feature registration + gated aggregation contract"
cargo test -p ui --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_tree_shaking_keeps_component_feature_and_css_boundaries
cargo test -p ui --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_tree_shaking_check_script_covers_feature_tree_wasm_and_budget
cargo test -p ui --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_check2_marks_tree_shaking_contract_complete

echo "[tree-shaking] color-swatch-picker minimal feature tree"
COLOR_SWATCH_PICKER_TREE_OUTPUT="$(cargo tree -e features -i ui -p ui --no-default-features --features "$COLOR_SWATCH_PICKER_MIN_FEATURES")"
echo "$COLOR_SWATCH_PICKER_TREE_OUTPUT"

if ! grep -q 'feature "component-color_swatch_picker" (command-line)' <<<"$COLOR_SWATCH_PICKER_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: component-color_swatch_picker" >&2
  exit 1
fi

if ! grep -q 'feature "inject-css" (command-line)' <<<"$COLOR_SWATCH_PICKER_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: inject-css for color-swatch-picker minimal tree" >&2
  exit 1
fi

if grep -q 'all-components' <<<"$COLOR_SWATCH_PICKER_TREE_OUTPUT"; then
  echo "[tree-shaking] color-swatch-picker minimal feature tree should not pull all-components" >&2
  exit 1
fi

echo "[tree-shaking] color-swatch-picker minimal wasm check"
cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features "$COLOR_SWATCH_PICKER_MIN_FEATURES"

echo "[tree-shaking] color-picker feature registration + gated aggregation contract"
cargo test -p ui-color-picker color_picker_tree_shaking_contract_is_feature_gated_for_module_and_css_paths
cargo test -p ui-color-picker color_picker_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget
cargo test -p ui-color-picker color_picker_check2_marks_tree_shaking_feature_pruning_contract_complete

echo "[tree-shaking] color-picker minimal feature tree"
COLOR_PICKER_TREE_OUTPUT="$(cargo tree -e features -i ui -p ui --no-default-features --features "$COLOR_PICKER_MIN_FEATURES")"
echo "$COLOR_PICKER_TREE_OUTPUT"

if ! grep -q 'feature "component-color_picker" (command-line)' <<<"$COLOR_PICKER_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: component-color_picker" >&2
  exit 1
fi

if ! grep -q 'feature "inject-css" (command-line)' <<<"$COLOR_PICKER_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: inject-css for color-picker minimal tree" >&2
  exit 1
fi

if grep -q 'all-components' <<<"$COLOR_PICKER_TREE_OUTPUT"; then
  echo "[tree-shaking] color-picker minimal feature tree should not pull all-components" >&2
  exit 1
fi

echo "[tree-shaking] color-picker minimal wasm check"
cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features "$COLOR_PICKER_MIN_FEATURES"

echo "[tree-shaking] date-input-group feature registration + gated aggregation contract"
cargo test -p ui-date-input-group date_input_group_tree_shaking_is_feature_gated_in_ui_components
cargo test -p ui-date-input-group date_input_group_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget
cargo test -p ui-date-input-group date_input_group_check2_marks_tree_shaking_feature_pruning_contract_complete

echo "[tree-shaking] date-input-group minimal feature tree"
DATE_INPUT_GROUP_TREE_OUTPUT="$(cargo tree -e features -i ui -p ui --no-default-features --features "$DATE_INPUT_GROUP_MIN_FEATURES")"
echo "$DATE_INPUT_GROUP_TREE_OUTPUT"

if ! grep -q 'feature "component-date_input_group" (command-line)' <<<"$DATE_INPUT_GROUP_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: component-date_input_group" >&2
  exit 1
fi

if ! grep -q 'feature "inject-css" (command-line)' <<<"$DATE_INPUT_GROUP_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: inject-css for date-input-group minimal tree" >&2
  exit 1
fi

if grep -q 'all-components' <<<"$DATE_INPUT_GROUP_TREE_OUTPUT"; then
  echo "[tree-shaking] date-input-group minimal feature tree should not pull all-components" >&2
  exit 1
fi

echo "[tree-shaking] date-input-group minimal wasm check"
cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features "$DATE_INPUT_GROUP_MIN_FEATURES"

echo "[tree-shaking] checkbox feature registration + gated aggregation contract"
cargo test -p ui --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_tree_shaking_contract_is_feature_gated_and_ci_enforced
cargo test -p ui --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_check2_marks_tree_shaking_feature_pruning_contract_complete

echo "[tree-shaking] checkbox minimal feature tree"
CHECKBOX_TREE_OUTPUT="$(cargo tree -e features -i ui -p ui --no-default-features --features "$CHECKBOX_MIN_FEATURES")"
echo "$CHECKBOX_TREE_OUTPUT"

if ! grep -q 'feature "component-checkbox" (command-line)' <<<"$CHECKBOX_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: component-checkbox" >&2
  exit 1
fi

if ! grep -q 'feature "inject-css" (command-line)' <<<"$CHECKBOX_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: inject-css for checkbox minimal tree" >&2
  exit 1
fi

if grep -q 'all-components' <<<"$CHECKBOX_TREE_OUTPUT"; then
  echo "[tree-shaking] checkbox minimal feature tree should not pull all-components" >&2
  exit 1
fi

echo "[tree-shaking] checkbox minimal wasm check"
cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features "$CHECKBOX_MIN_FEATURES"

echo "[tree-shaking] checkbox-field feature registration + gated aggregation contract"
cargo test -p ui --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_tree_shaking_contract_is_feature_gated_in_ui_components_lib_and_css
cargo test -p ui --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_tree_shaking_script_enforces_component_minimal_feature_tree_and_web_demo_reverse_dependency
cargo test -p ui --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_check2_marks_tree_shaking_feature_pruning_contract_complete

echo "[tree-shaking] checkbox-field minimal feature tree"
CHECKBOX_FIELD_TREE_OUTPUT="$(cargo tree -e features -i ui -p ui --no-default-features --features "$CHECKBOX_FIELD_MIN_FEATURES")"
echo "$CHECKBOX_FIELD_TREE_OUTPUT"

if ! grep -q 'feature "component-checkbox_field" (command-line)' <<<"$CHECKBOX_FIELD_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: component-checkbox_field" >&2
  exit 1
fi

if ! grep -q 'feature "inject-css" (command-line)' <<<"$CHECKBOX_FIELD_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: inject-css for checkbox-field minimal tree" >&2
  exit 1
fi

if grep -q 'all-components' <<<"$CHECKBOX_FIELD_TREE_OUTPUT"; then
  echo "[tree-shaking] checkbox-field minimal feature tree should not pull all-components" >&2
  exit 1
fi

echo "[tree-shaking] checkbox-field minimal wasm check"
cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features "$CHECKBOX_FIELD_MIN_FEATURES"

echo "[tree-shaking] checkbox-group feature registration + gated aggregation contract"
cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_tree_shaking_contract_is_feature_gated_and_budgeted
cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_check2_marks_tree_shaking_feature_pruning_contract_complete

echo "[tree-shaking] checkbox-group minimal feature tree"
CHECKBOX_GROUP_TREE_OUTPUT="$(cargo tree -e features -i ui -p ui --no-default-features --features "$CHECKBOX_GROUP_MIN_FEATURES")"
echo "$CHECKBOX_GROUP_TREE_OUTPUT"

if ! grep -q 'feature "component-checkbox_group" (command-line)' <<<"$CHECKBOX_GROUP_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: component-checkbox_group" >&2
  exit 1
fi

if ! grep -q 'feature "inject-css" (command-line)' <<<"$CHECKBOX_GROUP_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: inject-css for checkbox-group minimal tree" >&2
  exit 1
fi

if grep -q 'all-components' <<<"$CHECKBOX_GROUP_TREE_OUTPUT"; then
  echo "[tree-shaking] checkbox-group minimal feature tree should not pull all-components" >&2
  exit 1
fi

echo "[tree-shaking] checkbox-group minimal wasm check"
cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features "$CHECKBOX_GROUP_MIN_FEATURES"

echo "[tree-shaking] error-view feature registration + gated aggregation contract"
cargo test -p ui --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_tree_shaking_contract_is_feature_gated_and_budgeted
cargo test -p ui --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_check2_marks_tree_shaking_feature_pruning_contract_complete
cargo test -p ui --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget

echo "[tree-shaking] error-view minimal feature tree"
ERROR_VIEW_TREE_OUTPUT="$(cargo tree -e features -i ui -p ui --no-default-features --features "$ERROR_VIEW_MIN_FEATURES")"
echo "$ERROR_VIEW_TREE_OUTPUT"

if ! grep -q 'feature "component-error_view" (command-line)' <<<"$ERROR_VIEW_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: component-error_view" >&2
  exit 1
fi

if ! grep -q 'feature "inject-css" (command-line)' <<<"$ERROR_VIEW_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: inject-css for error-view minimal tree" >&2
  exit 1
fi

if grep -q 'all-components' <<<"$ERROR_VIEW_TREE_OUTPUT"; then
  echo "[tree-shaking] error-view minimal feature tree should not pull all-components" >&2
  exit 1
fi

echo "[tree-shaking] error-view minimal wasm check"
cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features "$ERROR_VIEW_MIN_FEATURES"

echo "[tree-shaking] flip-card feature registration + gated aggregation contract"
cargo test -p ui-flip-card flip_card_tree_shaking_keeps_component_feature_and_css_boundaries
cargo test -p ui-flip-card flip_card_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget
cargo test -p ui-flip-card flip_card_check2_marks_tree_shaking_feature_pruning_contract_complete

echo "[tree-shaking] flip-card minimal feature tree"
FLIP_CARD_TREE_OUTPUT="$(cargo tree -e features -i ui -p ui --no-default-features --features "$FLIP_CARD_MIN_FEATURES")"
echo "$FLIP_CARD_TREE_OUTPUT"

if ! grep -q 'feature "component-flip_card" (command-line)' <<<"$FLIP_CARD_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: component-flip_card" >&2
  exit 1
fi

if ! grep -q 'feature "inject-css" (command-line)' <<<"$FLIP_CARD_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: inject-css for flip-card minimal tree" >&2
  exit 1
fi

if grep -q 'all-components' <<<"$FLIP_CARD_TREE_OUTPUT"; then
  echo "[tree-shaking] flip-card minimal feature tree should not pull all-components" >&2
  exit 1
fi

echo "[tree-shaking] flip-card minimal wasm check"
cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features "$FLIP_CARD_MIN_FEATURES"

echo "[tree-shaking] command-dialog feature registration + gated aggregation contract"
cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_tree_shaking_feature_registration_and_gated_aggregates

echo "[tree-shaking] dialog feature registration + gated aggregation contract"
cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_tree_shaking_contract_registers_component_feature_and_gates_lib_css_aggregation
cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget
cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_check2_marks_tree_shaking_feature_pruning_contract_complete

echo "[tree-shaking] alert-dialog feature registration + gated aggregation contract"
cargo test -p ui --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_tree_shaking_keeps_component_feature_and_css_boundaries
cargo test -p ui --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_tree_shaking_check_script_covers_feature_tree_wasm_and_budget
cargo test -p ui --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget
cargo test -p ui --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_check2_marks_tree_shaking_feature_pruning_contract_complete

echo "[tree-shaking] alert-dialog minimal feature tree"
ALERT_DIALOG_TREE_OUTPUT="$(cargo tree -e features -i ui -p ui --no-default-features --features "$ALERT_DIALOG_MIN_FEATURES")"
echo "$ALERT_DIALOG_TREE_OUTPUT"

if ! grep -q 'feature "component-alert_dialog" (command-line)' <<<"$ALERT_DIALOG_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: component-alert_dialog" >&2
  exit 1
fi

if ! grep -q 'feature "inject-css" (command-line)' <<<"$ALERT_DIALOG_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: inject-css for alert-dialog minimal tree" >&2
  exit 1
fi

if grep -q 'all-components' <<<"$ALERT_DIALOG_TREE_OUTPUT"; then
  echo "[tree-shaking] alert-dialog minimal feature tree should not pull all-components" >&2
  exit 1
fi

echo "[tree-shaking] alert-dialog minimal wasm check"
cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features "$ALERT_DIALOG_MIN_FEATURES"

echo "[tree-shaking] drop-zone feature registration + gated aggregation contract"
cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_tree_shaking_contract_uses_feature_gated_exports_and_ci_budget_checks

echo "[tree-shaking] modal feature registration + gated aggregation contract"
cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_tree_shaking_contract_is_feature_gated
cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_tree_shaking_script_covers_feature_tree_wasm_and_budget
cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_check2_marks_tree_shaking_feature_pruning_contract_complete

echo "[tree-shaking] modal minimal feature tree"
MODAL_TREE_OUTPUT="$(cargo tree -e features -i ui -p ui --no-default-features --features "$MODAL_MIN_FEATURES")"
echo "$MODAL_TREE_OUTPUT"

if ! grep -q 'feature "component-modal" (command-line)' <<<"$MODAL_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: component-modal" >&2
  exit 1
fi

if ! grep -q 'feature "inject-css" (command-line)' <<<"$MODAL_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: inject-css for modal minimal tree" >&2
  exit 1
fi

if grep -q 'all-components' <<<"$MODAL_TREE_OUTPUT"; then
  echo "[tree-shaking] modal minimal feature tree should not pull all-components" >&2
  exit 1
fi

echo "[tree-shaking] modal minimal wasm check"
cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features "$MODAL_MIN_FEATURES"

echo "[tree-shaking] sidebar feature registration + gated aggregation contract"
cargo test -p ui --test sidebar_semantics --no-default-features --features component-sidebar,inject-css sidebar_tree_shaking_keeps_component_feature_and_css_boundaries
cargo test -p ui --test sidebar_semantics --no-default-features --features component-sidebar,inject-css sidebar_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget

echo "[tree-shaking] sidebar minimal feature tree"
SIDEBAR_TREE_OUTPUT="$(cargo tree -e features -i ui -p ui --no-default-features --features "$SIDEBAR_MIN_FEATURES")"
echo "$SIDEBAR_TREE_OUTPUT"

if ! grep -q 'feature "component-sidebar" (command-line)' <<<"$SIDEBAR_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: component-sidebar" >&2
  exit 1
fi

if ! grep -q 'feature "inject-css" (command-line)' <<<"$SIDEBAR_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: inject-css for sidebar minimal tree" >&2
  exit 1
fi

if grep -q 'all-components' <<<"$SIDEBAR_TREE_OUTPUT"; then
  echo "[tree-shaking] sidebar minimal feature tree should not pull all-components" >&2
  exit 1
fi

echo "[tree-shaking] sidebar minimal wasm check"
cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features "$SIDEBAR_MIN_FEATURES"

echo "[tree-shaking] overlays feature registration + gated aggregation contract"
cargo test -p ui-overlays overlays_tree_shaking_contract_is_feature_gated_and_budget_guarded
cargo test -p ui-overlays overlays_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget
cargo test -p ui-overlays overlays_check2_marks_tree_shaking_feature_pruning_contract_complete

echo "[tree-shaking] overlays minimal feature tree"
OVERLAYS_TREE_OUTPUT="$(cargo tree -e features -i ui -p ui --no-default-features --features "$OVERLAYS_MIN_FEATURES")"
echo "$OVERLAYS_TREE_OUTPUT"

if ! grep -q 'feature "component-overlays" (command-line)' <<<"$OVERLAYS_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: component-overlays" >&2
  exit 1
fi

if ! grep -q 'feature "inject-css" (command-line)' <<<"$OVERLAYS_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: inject-css for overlays minimal tree" >&2
  exit 1
fi

if grep -q 'all-components' <<<"$OVERLAYS_TREE_OUTPUT"; then
  echo "[tree-shaking] overlays minimal feature tree should not pull all-components" >&2
  exit 1
fi

echo "[tree-shaking] overlays minimal wasm check"
cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features "$OVERLAYS_MIN_FEATURES"

echo "[tree-shaking] drawer feature registration + gated aggregation contract"
cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_tree_shaking_contract_is_feature_gated_and_budget_guarded
cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget
cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_check2_marks_tree_shaking_feature_pruning_contract_complete

echo "[tree-shaking] drawer minimal feature tree"
DRAWER_TREE_OUTPUT="$(cargo tree -e features -i ui -p ui --no-default-features --features "$DRAWER_MIN_FEATURES")"
echo "$DRAWER_TREE_OUTPUT"

if ! grep -q 'feature "component-drawer" (command-line)' <<<"$DRAWER_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: component-drawer" >&2
  exit 1
fi

if ! grep -q 'feature "inject-css" (command-line)' <<<"$DRAWER_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: inject-css for drawer minimal tree" >&2
  exit 1
fi

if grep -q 'all-components' <<<"$DRAWER_TREE_OUTPUT"; then
  echo "[tree-shaking] drawer minimal feature tree should not pull all-components" >&2
  exit 1
fi

echo "[tree-shaking] drawer minimal wasm check"
cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features "$DRAWER_MIN_FEATURES"

echo "[tree-shaking] bottom-sheet feature registration + gated aggregation contract"
cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_tree_shaking_contract_is_feature_gated_end_to_end
cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget
cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_check2_marks_tree_shaking_feature_pruning_contract_complete

echo "[tree-shaking] bottom-sheet minimal feature tree"
BOTTOM_SHEET_TREE_OUTPUT="$(cargo tree -e features -i ui -p ui --no-default-features --features "$BOTTOM_SHEET_MIN_FEATURES")"
echo "$BOTTOM_SHEET_TREE_OUTPUT"

if ! grep -q 'feature "component-bottom_sheet" (command-line)' <<<"$BOTTOM_SHEET_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: component-bottom_sheet" >&2
  exit 1
fi

if ! grep -q 'feature "inject-css" (command-line)' <<<"$BOTTOM_SHEET_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: inject-css for bottom-sheet minimal tree" >&2
  exit 1
fi

if grep -q 'all-components' <<<"$BOTTOM_SHEET_TREE_OUTPUT"; then
  echo "[tree-shaking] bottom-sheet minimal feature tree should not pull all-components" >&2
  exit 1
fi

echo "[tree-shaking] bottom-sheet minimal wasm check"
cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features "$BOTTOM_SHEET_MIN_FEATURES"

echo "[tree-shaking] list feature registration + gated aggregation contract"
cargo test -p ui-list list_tree_shaking_contract_uses_feature_gates_and_no_unconditional_registry_path
cargo test -p ui-list list_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget
cargo test -p ui-list list_check2_marks_tree_shaking_feature_pruning_contract_complete
cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_tree_shaking_contract_uses_component_feature_gates
cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget
cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_check2_marks_tree_shaking_feature_pruning_contract_complete

echo "[tree-shaking] list minimal feature tree"
LIST_TREE_OUTPUT="$(cargo tree -e features -i ui -p ui --no-default-features --features "$LIST_MIN_FEATURES")"
echo "$LIST_TREE_OUTPUT"

if ! grep -q 'feature "component-list" (command-line)' <<<"$LIST_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: component-list" >&2
  exit 1
fi

if ! grep -q 'feature "inject-css" (command-line)' <<<"$LIST_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: inject-css for list minimal tree" >&2
  exit 1
fi

if grep -q 'all-components' <<<"$LIST_TREE_OUTPUT"; then
  echo "[tree-shaking] list minimal feature tree should not pull all-components" >&2
  exit 1
fi

echo "[tree-shaking] list minimal wasm check"
cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features "$LIST_MIN_FEATURES"

echo "[tree-shaking] menu feature registration + gated aggregation contract"
cargo test -p ui-menu menu_tree_shaking_contract_is_feature_gated_and_budget_guarded
cargo test -p ui-menu menu_check2_marks_tree_shaking_feature_pruning_contract_complete

echo "[tree-shaking] menu minimal feature tree"
MENU_TREE_OUTPUT="$(cargo tree -e features -i ui -p ui --no-default-features --features "$MENU_MIN_FEATURES")"
echo "$MENU_TREE_OUTPUT"

if ! grep -q 'feature "component-menu" (command-line)' <<<"$MENU_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: component-menu" >&2
  exit 1
fi

if ! grep -q 'feature "inject-css" (command-line)' <<<"$MENU_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: inject-css for menu minimal tree" >&2
  exit 1
fi

if grep -q 'all-components' <<<"$MENU_TREE_OUTPUT"; then
  echo "[tree-shaking] menu minimal feature tree should not pull all-components" >&2
  exit 1
fi

echo "[tree-shaking] menu minimal wasm check"
cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features "$MENU_MIN_FEATURES"

echo "[tree-shaking] fieldset feature registration + gated aggregation contract"
cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_tree_shaking_keeps_component_feature_and_css_boundaries

echo "[tree-shaking] fieldset minimal feature tree"
FIELDSET_TREE_OUTPUT="$(cargo tree -e features -i ui -p ui --no-default-features --features "$FIELDSET_MIN_FEATURES")"
echo "$FIELDSET_TREE_OUTPUT"

if ! grep -q 'feature "component-fieldset" (command-line)' <<<"$FIELDSET_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: component-fieldset" >&2
  exit 1
fi

if ! grep -q 'feature "inject-css" (command-line)' <<<"$FIELDSET_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: inject-css for fieldset minimal tree" >&2
  exit 1
fi

if grep -q 'all-components' <<<"$FIELDSET_TREE_OUTPUT"; then
  echo "[tree-shaking] fieldset minimal feature tree should not pull all-components" >&2
  exit 1
fi

echo "[tree-shaking] fieldset minimal wasm check"
cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features "$FIELDSET_MIN_FEATURES"

echo "[tree-shaking] command feature registration + gated aggregation contract"
cargo test -p ui-command --lib command_tree_shaking_contract_is_feature_gated
cargo test -p ui-command --lib command_check2_marks_tree_shaking_feature_pruning_contract_complete
cargo test -p ui-command --lib command_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget

echo "[tree-shaking] command minimal feature tree"
COMMAND_TREE_OUTPUT="$(cargo tree -e features -i ui -p ui --no-default-features --features "$COMMAND_MIN_FEATURES")"
echo "$COMMAND_TREE_OUTPUT"

if ! grep -q 'feature "component-command" (command-line)' <<<"$COMMAND_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: component-command" >&2
  exit 1
fi

if ! grep -q 'feature "inject-css" (command-line)' <<<"$COMMAND_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: inject-css for command minimal tree" >&2
  exit 1
fi

if grep -q 'all-components' <<<"$COMMAND_TREE_OUTPUT"; then
  echo "[tree-shaking] command minimal feature tree should not pull all-components" >&2
  exit 1
fi

echo "[tree-shaking] command minimal wasm check"
cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features "$COMMAND_MIN_FEATURES"

echo "[tree-shaking] command-dialog minimal feature tree"
COMMAND_DIALOG_TREE_OUTPUT="$(cargo tree -e features -i ui -p ui --no-default-features --features "$COMMAND_DIALOG_MIN_FEATURES")"
echo "$COMMAND_DIALOG_TREE_OUTPUT"

if ! grep -q 'feature "component-command_dialog" (command-line)' <<<"$COMMAND_DIALOG_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: component-command_dialog" >&2
  exit 1
fi

if ! grep -q 'feature "inject-css" (command-line)' <<<"$COMMAND_DIALOG_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: inject-css for command-dialog minimal tree" >&2
  exit 1
fi

if grep -q 'all-components' <<<"$COMMAND_DIALOG_TREE_OUTPUT"; then
  echo "[tree-shaking] command-dialog minimal feature tree should not pull all-components" >&2
  exit 1
fi

echo "[tree-shaking] dialog minimal feature tree"
DIALOG_TREE_OUTPUT="$(cargo tree -e features -i ui -p ui --no-default-features --features "$DIALOG_MIN_FEATURES")"
echo "$DIALOG_TREE_OUTPUT"

if ! grep -q 'feature "component-dialog" (command-line)' <<<"$DIALOG_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: component-dialog" >&2
  exit 1
fi

if ! grep -q 'feature "inject-css" (command-line)' <<<"$DIALOG_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: inject-css for dialog minimal tree" >&2
  exit 1
fi

if grep -q 'all-components' <<<"$DIALOG_TREE_OUTPUT"; then
  echo "[tree-shaking] dialog minimal feature tree should not pull all-components" >&2
  exit 1
fi

echo "[tree-shaking] dialog minimal wasm check"
cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features "$DIALOG_MIN_FEATURES"

echo "[tree-shaking] hover-card feature registration + gated aggregation contract"
cargo test -p ui --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_tree_shaking_contract_stays_feature_gated_in_package_and_demo_modes
cargo test -p ui --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget
cargo test -p ui --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_check2_marks_tree_shaking_feature_pruning_contract_complete

echo "[tree-shaking] hover-card minimal feature tree"
HOVER_CARD_TREE_OUTPUT="$(cargo tree -e features -i ui -p ui --no-default-features --features "$HOVER_CARD_MIN_FEATURES")"
echo "$HOVER_CARD_TREE_OUTPUT"

if ! grep -q 'feature "component-hover_card" (command-line)' <<<"$HOVER_CARD_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: component-hover_card" >&2
  exit 1
fi

if ! grep -q 'feature "inject-css" (command-line)' <<<"$HOVER_CARD_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: inject-css for hover-card minimal tree" >&2
  exit 1
fi

if grep -q 'all-components' <<<"$HOVER_CARD_TREE_OUTPUT"; then
  echo "[tree-shaking] hover-card minimal feature tree should not pull all-components" >&2
  exit 1
fi

echo "[tree-shaking] hover-card minimal wasm check"
cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features "$HOVER_CARD_MIN_FEATURES"

echo "[tree-shaking] circular-progress feature registration + gated aggregation contract"
cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_tree_shaking_contract_is_feature_gated_and_prunable_for_package_and_source_modes
cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget
cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_check2_marks_tree_shaking_feature_pruning_contract_complete

echo "[tree-shaking] circular-progress minimal feature tree"
CIRCULAR_PROGRESS_TREE_OUTPUT="$(cargo tree -e features -i ui -p ui --no-default-features --features "$CIRCULAR_PROGRESS_MIN_FEATURES")"
echo "$CIRCULAR_PROGRESS_TREE_OUTPUT"

if ! grep -q 'feature "component-circular_progress" (command-line)' <<<"$CIRCULAR_PROGRESS_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: component-circular_progress" >&2
  exit 1
fi

if ! grep -q 'feature "inject-css" (command-line)' <<<"$CIRCULAR_PROGRESS_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: inject-css for circular-progress minimal tree" >&2
  exit 1
fi

if grep -q 'all-components' <<<"$CIRCULAR_PROGRESS_TREE_OUTPUT"; then
  echo "[tree-shaking] circular-progress minimal feature tree should not pull all-components" >&2
  exit 1
fi

echo "[tree-shaking] circular-progress minimal wasm check"
cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features "$CIRCULAR_PROGRESS_MIN_FEATURES"

echo "[tree-shaking] chart feature registration + gated aggregation contract"
cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_tree_shaking_contract_stays_feature_gated_in_package_and_demo_modes
cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget
cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_check2_marks_tree_shaking_feature_pruning_contract_complete

echo "[tree-shaking] chart minimal feature tree"
CHART_TREE_OUTPUT="$(cargo tree -e features -i ui -p ui --no-default-features --features "$CHART_MIN_FEATURES")"
echo "$CHART_TREE_OUTPUT"

if ! grep -q 'feature "component-chart" (command-line)' <<<"$CHART_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: component-chart" >&2
  exit 1
fi

if ! grep -q 'feature "inject-css" (command-line)' <<<"$CHART_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: inject-css for chart minimal tree" >&2
  exit 1
fi

if grep -q 'all-components' <<<"$CHART_TREE_OUTPUT"; then
  echo "[tree-shaking] chart minimal feature tree should not pull all-components" >&2
  exit 1
fi

echo "[tree-shaking] chart minimal wasm check"
cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features "$CHART_MIN_FEATURES"

echo "[tree-shaking] carousel feature registration + gated aggregation contract"
cargo test -p ui --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_tree_shaking_contract_keeps_feature_gated_entrypoints
cargo test -p ui --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget
cargo test -p ui --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_check2_marks_tree_shaking_feature_pruning_contract_complete

echo "[tree-shaking] carousel minimal feature tree"
CAROUSEL_TREE_OUTPUT="$(cargo tree -e features -i ui -p ui --no-default-features --features "$CAROUSEL_MIN_FEATURES")"
echo "$CAROUSEL_TREE_OUTPUT"

if ! grep -q 'feature "component-carousel" (command-line)' <<<"$CAROUSEL_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: component-carousel" >&2
  exit 1
fi

if ! grep -q 'feature "inject-css" (command-line)' <<<"$CAROUSEL_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: inject-css for carousel minimal tree" >&2
  exit 1
fi

if grep -q 'all-components' <<<"$CAROUSEL_TREE_OUTPUT"; then
  echo "[tree-shaking] carousel minimal feature tree should not pull all-components" >&2
  exit 1
fi

echo "[tree-shaking] carousel minimal wasm check"
cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features "$CAROUSEL_MIN_FEATURES"

echo "[tree-shaking] collapsible feature registration + gated aggregation contract"
cargo test -p ui --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_tree_shaking_contract_keeps_feature_gates_explicit

echo "[tree-shaking] collapsible minimal feature tree"
COLLAPSIBLE_TREE_OUTPUT="$(cargo tree -e features -i ui -p ui --no-default-features --features "$COLLAPSIBLE_MIN_FEATURES")"
echo "$COLLAPSIBLE_TREE_OUTPUT"

if ! grep -q 'feature "component-collapsible" (command-line)' <<<"$COLLAPSIBLE_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: component-collapsible" >&2
  exit 1
fi

if ! grep -q 'feature "inject-css" (command-line)' <<<"$COLLAPSIBLE_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: inject-css for collapsible minimal tree" >&2
  exit 1
fi

if grep -q 'all-components' <<<"$COLLAPSIBLE_TREE_OUTPUT"; then
  echo "[tree-shaking] collapsible minimal feature tree should not pull all-components" >&2
  exit 1
fi

echo "[tree-shaking] collapsible minimal wasm check"
cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features "$COLLAPSIBLE_MIN_FEATURES"

echo "[tree-shaking] preview-link-card feature registration + gated aggregation contract"
cargo test -p ui --lib --no-default-features --features component-preview_link_card,inject-css preview_link_card_tree_shaking_contract_is_feature_gated_and_css_prunable

echo "[tree-shaking] preview-link-card minimal feature tree"
PREVIEW_LINK_CARD_TREE_OUTPUT="$(cargo tree -e features -i ui -p ui --no-default-features --features "$PREVIEW_LINK_CARD_MIN_FEATURES")"
echo "$PREVIEW_LINK_CARD_TREE_OUTPUT"

if ! grep -q 'feature "component-preview_link_card" (command-line)' <<<"$PREVIEW_LINK_CARD_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: component-preview_link_card" >&2
  exit 1
fi

if ! grep -q 'feature "inject-css" (command-line)' <<<"$PREVIEW_LINK_CARD_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: inject-css for preview-link-card minimal tree" >&2
  exit 1
fi

if grep -q 'all-components' <<<"$PREVIEW_LINK_CARD_TREE_OUTPUT"; then
  echo "[tree-shaking] preview-link-card minimal feature tree should not pull all-components" >&2
  exit 1
fi

echo "[tree-shaking] preview-link-card minimal wasm check"
cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features "$PREVIEW_LINK_CARD_MIN_FEATURES"

echo "[tree-shaking] sidebar feature registration + gated aggregation contract"
cargo test -p ui --lib --no-default-features --features component-sidebar,inject-css sidebar_tree_shaking_keeps_component_feature_and_css_boundaries
cargo test -p ui --lib --no-default-features --features component-sidebar,inject-css sidebar_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget
cargo test -p ui --lib --no-default-features --features component-sidebar,inject-css sidebar_check2_marks_tree_shaking_feature_pruning_contract_complete

echo "[tree-shaking] sidebar minimal feature tree"
SIDEBAR_TREE_OUTPUT="$(cargo tree -e features -i ui -p ui --no-default-features --features "$SIDEBAR_MIN_FEATURES")"
echo "$SIDEBAR_TREE_OUTPUT"

if ! grep -q 'feature "component-sidebar" (command-line)' <<<"$SIDEBAR_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: component-sidebar" >&2
  exit 1
fi

if ! grep -q 'feature "inject-css" (command-line)' <<<"$SIDEBAR_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: inject-css for sidebar minimal tree" >&2
  exit 1
fi

if grep -q 'all-components' <<<"$SIDEBAR_TREE_OUTPUT"; then
  echo "[tree-shaking] sidebar minimal feature tree should not pull all-components" >&2
  exit 1
fi

echo "[tree-shaking] sidebar minimal wasm check"
cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features "$SIDEBAR_MIN_FEATURES"

echo "[tree-shaking] minimal feature tree"
MIN_TREE_OUTPUT="$(cargo tree -e features -i ui -p ui --no-default-features --features "$MIN_FEATURES")"
echo "$MIN_TREE_OUTPUT"

if ! grep -q 'feature "component-accordion" (command-line)' <<<"$MIN_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: component-accordion" >&2
  exit 1
fi

if ! grep -q 'feature "inject-css" (command-line)' <<<"$MIN_TREE_OUTPUT"; then
  echo "[tree-shaking] missing command-line feature: inject-css" >&2
  exit 1
fi

if grep -q 'all-components' <<<"$MIN_TREE_OUTPUT"; then
  echo "[tree-shaking] unexpected all-components in minimal feature tree" >&2
  exit 1
fi

echo "[tree-shaking] reverse dependency tree (web-demo)"
WEB_DEMO_TREE_OUTPUT="$(cargo tree -e features -i ui -p web-demo)"
echo "$WEB_DEMO_TREE_OUTPUT"

if grep -q 'all-components' <<<"$WEB_DEMO_TREE_OUTPUT"; then
  echo "[tree-shaking] web-demo should not pull all-components" >&2
  exit 1
fi

if ! grep -q 'web-demo-components' <<<"$WEB_DEMO_TREE_OUTPUT"; then
  echo "[tree-shaking] web-demo should pull web-demo-components feature bundle" >&2
  exit 1
fi

echo "[tree-shaking] minimal wasm check"
cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features "$MIN_FEATURES"

echo "[tree-shaking] minimal wasm release build for budget"
cargo build -p ui --target wasm32-unknown-unknown --release --no-default-features --features "$MIN_FEATURES"

if [[ ! -f "$BUDGET_FILE" ]]; then
  echo "[tree-shaking] missing budget file: $BUDGET_FILE" >&2
  exit 1
fi

# shellcheck source=/dev/null
source "$BUDGET_FILE"

if [[ -z "${TREE_SHAKING_BASELINE_RLIB_BYTES:-}" || -z "${TREE_SHAKING_MAX_RATIO_PERCENT:-}" ]]; then
  echo "[tree-shaking] budget file must define TREE_SHAKING_BASELINE_RLIB_BYTES and TREE_SHAKING_MAX_RATIO_PERCENT" >&2
  exit 1
fi

LATEST_RLIB="$(ls -1t target/wasm32-unknown-unknown/release/deps/libui_components-*.rlib | head -n 1)"
CURRENT_BYTES="$(stat -c '%s' "$LATEST_RLIB")"
MAX_BYTES=$((TREE_SHAKING_BASELINE_RLIB_BYTES * TREE_SHAKING_MAX_RATIO_PERCENT / 100))

echo "[tree-shaking] budget check"
echo "  latest rlib: $LATEST_RLIB"
echo "  current bytes: $CURRENT_BYTES"
echo "  baseline bytes: $TREE_SHAKING_BASELINE_RLIB_BYTES"
echo "  max ratio: ${TREE_SHAKING_MAX_RATIO_PERCENT}%"
echo "  max bytes: $MAX_BYTES"

if (( CURRENT_BYTES > MAX_BYTES )); then
  echo "[tree-shaking] size regression: $CURRENT_BYTES > $MAX_BYTES" >&2
  exit 1
fi

echo "[tree-shaking] OK"
