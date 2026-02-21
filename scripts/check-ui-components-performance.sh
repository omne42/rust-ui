#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[perf] contract: button performance governance"
cargo test -p ui-components --test button_semantics button_performance_governance_contract_is_budgeted_traceable_and_blocking

echo "[perf] contract: button semantics/perf matrix"
cargo test -p ui-components --test button_semantics button_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement

echo "[perf] contract: button semantic test priority"
cargo test -p ui-components --test button_semantics button_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks

echo "[perf] contract: input performance governance"
cargo test -p ui-components --test input_semantics --no-default-features --features component-input,inject-css input_performance_governance_contract_is_budgeted_traceable_and_blocking

echo "[perf] contract: description performance governance"
cargo test -p ui-components --test description_semantics description_performance_governance_contract_is_mount_only_traceable_and_blocking

echo "[perf] contract: checkbox performance governance"
cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_performance_governance_budget_is_defined_and_blocking_locally
cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement
cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks

echo "[perf] contract: checkbox-field performance governance"
cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_performance_governance_contract_is_budgeted_traceable_and_blocking
cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement
echo "[perf] contract: checkbox-field semantic test priority"
cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks

echo "[perf] contract: checkbox-group performance governance"
cargo test -p ui-components --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_performance_governance_contract_is_budgeted_traceable_and_blocking
echo "[perf] contract: checkbox-group semantics/perf matrix"
cargo test -p ui-components --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement
echo "[perf] contract: checkbox-group semantic test priority"
cargo test -p ui-components --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks

echo "[perf] contract: swatch performance governance"
cargo test -p ui-components --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_performance_governance_budget_is_defined_and_blocking

echo "[perf] contract: circular-progress performance governance"
cargo test -p ui-components --test circular_progress_semantics circular_progress_performance_governance_budget_is_defined_and_blocking
cargo test -p ui-components --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement
echo "[perf] contract: circular-progress semantic test priority"
cargo test -p ui-components --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks

echo "[perf] contract: avatar-group performance governance"
cargo test -p ui-components --test avatar_group_semantics --no-default-features --features component-avatar_group,inject-css avatar_group_performance_governance_budget_is_defined_and_blocking

echo "[perf] contract: button-copy performance governance"
cargo test -p ui-components --test button_copy_semantics button_copy_performance_governance_budget_is_defined_and_blocking

echo "[perf] contract: action-button performance governance"
cargo test -p ui-components --test action_button_semantics action_button_performance_governance_budget_is_defined_and_blocking

echo "[perf] contract: share-button performance governance"
cargo test -p ui-components --test share_button_semantics share_button_performance_governance_budget_is_defined_and_blocking

echo "[perf] contract: action-bar performance governance"
cargo test -p ui-components --test action_bar_semantics --no-default-features --features component-action_bar,inject-css action_bar_performance_governance_budget_is_defined_and_blocking

echo "[perf] contract: flip-card performance governance"
cargo test -p ui-flip-card flip_card_performance_governance_budget_is_defined_and_blocking
cargo test -p ui-flip-card flip_card_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement

echo "[perf] contract: alert-dialog performance governance"
cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_performance_governance_budget_is_defined_and_blocking
echo "[perf] contract: alert-dialog semantics/perf matrix"
cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement
echo "[perf] contract: alert-dialog semantic test priority"
cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks

echo "[perf] contract: dialog performance governance"
cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_performance_governance_budget_is_defined_and_blocking
cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement

echo "[perf] contract: hover-card performance governance"
cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_performance_governance_contract_is_budgeted_traceable_and_blocking

echo "[perf] contract: hover-card semantics/perf matrix"
cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement

echo "[perf] contract: calendar performance governance"
cargo test -p ui-calendar calendar_performance_governance_budget_is_defined_traceable_and_blocking

echo "[perf] contract: bottom-sheet performance governance"
cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_performance_governance_contract_is_mount_only_traceable_and_blocking

echo "[perf] contract: bottom-sheet semantics/perf matrix"
cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement
echo "[perf] contract: bottom-sheet semantic test priority"
cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks

echo "[perf] contract: tag performance governance"
cargo test -p ui-components --test tag_semantics --no-default-features --features component-tag,inject-css tag_performance_governance_budget_is_defined_and_blocking

echo "[perf] contract: tag-group performance governance"
cargo test -p ui-components --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_performance_governance_budget_is_defined_and_blocking

echo "[perf] contract: textarea performance governance"
cargo test -p ui-components --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_performance_governance_contract_is_budgeted_traceable_and_blocking

echo "[perf] contract: time-field performance governance"
cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_performance_governance_budget_is_defined_and_blocking

echo "[perf] contract: form-field performance governance"
cargo test -p ui-components --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_performance_governance_budget_is_defined_traceable_and_blocking
echo "[perf] contract: form-field semantic test priority"
cargo test -p ui-components --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks

echo "[perf] contract: field performance governance"
cargo test -p ui-field field_performance_governance_contract_is_budgeted_traceable_and_blocking
echo "[perf] contract: field semantics/perf matrix"
cargo test -p ui-field field_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement
echo "[perf] contract: field semantic test priority"
cargo test -p ui-field field_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks

echo "[perf] contract: slider performance governance"
cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_performance_governance_budget_is_defined_and_blocking

echo "[perf] contract: scroll-area performance governance"
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_performance_governance_budget_is_defined_and_blocking

echo "[perf] contract: breadcrumb performance governance"
cargo test -p ui-components --test breadcrumb_semantics breadcrumb_performance_governance_contract_is_budgeted_traceable_and_blocking

echo "[perf] contract: breadcrumb semantics/perf matrix"
cargo test -p ui-components --test breadcrumb_semantics breadcrumb_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement

echo "[perf] contract: coachmark performance governance"
cargo test -p ui-components --lib coachmark_performance_governance_budget_is_defined_traceable_and_blocking

echo "[perf] contract: coachmark semantics/perf matrix"
cargo test -p ui-components --lib coachmark_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement

echo "[perf] contract: chart performance governance"
cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_performance_governance_contract_is_budgeted_traceable_and_blocking

echo "[perf] contract: chart semantics/perf matrix"
cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement
echo "[perf] contract: chart semantic-first checks data/aria/role/state-source over snapshot assertions"
cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_check2_documents_semantics_tests_priority_rules

echo "[perf] contract: collapsible performance governance"
cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_performance_governance_contract_is_budgeted_traceable_and_blocking

echo "[perf] contract: collapsible semantics/perf matrix"
cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement

echo "[perf] contract: color-loupe performance governance"
cargo test -p ui-components --test color_loupe_semantics color_loupe_performance_governance_contract_is_budgeted_traceable_and_blocking

echo "[perf] contract: color-swatch performance governance"
cargo test -p ui-components --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_performance_governance_contract_is_budgeted_traceable_and_blocking
cargo test -p ui-components --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement

echo "[perf] contract: color-thumb performance governance"
cargo test -p ui-components --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_performance_governance_contract_is_budgeted_traceable_and_blocking
cargo test -p ui-components --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement

echo "[perf] contract: color-editor performance governance"
cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_performance_governance_contract_is_budgeted_traceable_and_blocking
cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement

echo "[perf] contract: color-slider performance governance"
cargo test -p ui-color-slider color_slider_performance_governance_contract_is_budgeted_traceable_and_blocking
cargo test -p ui-components --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement

echo "[perf] contract: color-wheel performance governance"
cargo test -p ui-components --test color_wheel_semantics color_wheel_performance_governance_contract_is_budgeted_traceable_and_blocking
cargo test -p ui-components --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement

echo "[perf] contract: color-picker performance governance"
cargo test -p ui-color-picker color_picker_performance_governance_contract_is_budgeted_traceable_and_blocking
cargo test -p ui-color-picker color_picker_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement

echo "[perf] contract: date-input-group semantics/perf matrix"
cargo test -p ui-date-input-group date_input_group_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement
echo "[perf] contract: date-input-group semantic test priority"
cargo test -p ui-date-input-group date_input_group_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks

echo "[perf] contract: color-swatch-picker performance governance"
cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_performance_governance_contract_is_budgeted_traceable_and_blocking
cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement

echo "[perf] contract: autocomplete performance governance"
cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_performance_governance_budget_is_defined_and_blocking

echo "[perf] contract: autocomplete semantics/perf matrix"
cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement

echo "[perf] contract: autocomplete semantic test priority"
cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks

echo "[perf] contract: error-view performance governance"
cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_performance_governance_contract_is_budgeted_traceable_and_blocking

echo "[perf] contract: error-view semantics/perf matrix"
cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement

echo "[perf] contract: error-view semantic test priority"
cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks

echo "[perf] contract: fieldset performance governance"
cargo test -p ui-components --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_performance_governance_contract_is_budgeted_traceable_and_blocking

echo "[perf] contract: fieldset semantics/perf matrix"
cargo test -p ui-components --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement

echo "[perf] contract: fieldset semantic test priority"
cargo test -p ui-components --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks

echo "[perf] contract: combo-box performance governance"
cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_performance_governance_budget_is_defined_and_blocking

echo "[perf] contract: combo-box semantics/perf matrix"
cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement

echo "[perf] contract: combo-box semantic test priority"
cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks

echo "[perf] contract: list performance governance"
cargo test -p ui-components --test list_module_semantics --no-default-features --features component-list,inject-css list_performance_governance_contract_is_budgeted_traceable_and_blocking_via_global_gates

echo "[perf] contract: list semantics/perf matrix"
cargo test -p ui-components --test list_module_semantics --no-default-features --features component-list,inject-css list_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement

echo "[perf] contract: menu performance governance"
cargo test -p ui-menu menu_performance_governance_contract_is_budgeted_traceable_and_blocking

echo "[perf] contract: menu semantics/perf matrix"
cargo test -p ui-menu menu_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement

echo "[perf] contract: command-dialog performance governance"
cargo test -p ui-components --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_performance_governance_contract_is_mount_only_traceable_and_blocking

echo "[perf] contract: command-dialog semantics/perf matrix"
cargo test -p ui-components --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement

echo "[perf] contract: command performance governance"
cargo test -p ui-command --lib command_performance_governance_budget_is_mount_only_traceable_and_blocking

echo "[perf] contract: command semantics/perf matrix"
cargo test -p ui-command --lib command_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement

echo "[perf] contract: modal performance governance"
cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_performance_governance_contract_is_budgeted_traceable_and_blocking

echo "[perf] contract: modal semantics/perf matrix"
cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement

echo "[perf] contract: modal semantic test priority"
cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks

echo "[perf] contract: overlays performance governance"
cargo test -p ui-overlays overlays_performance_governance_contract_is_budgeted_traceable_and_blocking

echo "[perf] contract: overlays semantics/perf matrix"
cargo test -p ui-overlays overlays_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement

echo "[perf] contract: overlays semantic test priority"
cargo test -p ui-overlays overlays_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks

echo "[perf] contract: drawer performance governance"
cargo test -p ui-components --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_performance_governance_contract_is_budgeted_traceable_and_blocking

echo "[perf] contract: drawer semantics/perf matrix"
cargo test -p ui-components --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement

echo "[perf] contract: drawer semantic test priority"
cargo test -p ui-components --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks

echo "[perf] contract: carousel performance governance"
cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_performance_governance_contract_is_mount_only_traceable_and_blocking

echo "[perf] contract: carousel semantics/perf matrix"
cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement

echo "[perf] contract: carousel semantic test priority"
cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks

echo "[perf] contract: empty-state performance governance"
cargo test -p ui-components --test empty_state_semantics --no-default-features --features component-empty_state,inject-css empty_state_performance_governance_contract_is_mount_only_traceable_and_blocking

echo "[perf] contract: drop-zone performance governance"
cargo test -p ui-components --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_performance_governance_contract_is_budgeted_traceable_and_blocking

echo "[perf] contract: drop-zone semantics/perf matrix"
cargo test -p ui-components --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement

echo "[perf] contract: drop-zone semantic test priority"
cargo test -p ui-components --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks

echo "[perf] contract: docs perf probe budgets"
cargo test -p ui-components --test accordion_semantics docs_perf_probe_budgets_are_wired_for_component_pages

echo "[perf] contract: render_count follow-up tracking"
cargo test -p ui-components --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan

echo "[perf] OK"
