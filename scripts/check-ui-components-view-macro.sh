#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[view-macro] contract: button view split"
cargo test -p ui-components --test button_semantics button_view_macro_complexity_is_split_into_semantic_subrenders

echo "[view-macro] contract: button function-first split"
cargo test -p ui-components --test button_semantics button_view_functional_split_prefers_plain_functions_over_local_components

echo "[view-macro] contract: button static fragment constantization"
cargo test -p ui-components --test button_semantics button_static_fragments_are_constantized_with_stable_a11y_semantics

echo "[view-macro] contract: checkbox view macro split"
cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_view_macro_complexity_is_split_into_semantic_subrenders_locally

echo "[view-macro] contract: checkbox function-first split"
cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_view_functional_split_prefers_plain_functions_over_local_components_locally

echo "[view-macro] contract: checkbox static fragment constantization"
cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_static_fragments_are_constantized_with_stable_semantics_locally

echo "[view-macro] contract: checkbox-group view macro complexity"
cargo test -p ui-components --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_view_macro_complexity_is_small_and_does_not_require_semantic_subrenders

echo "[view-macro] contract: checkbox-group function-first split"
cargo test -p ui-components --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_view_functional_split_prefers_plain_functions_over_local_components

echo "[view-macro] contract: checkbox-group static fragment constantization"
cargo test -p ui-components --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_static_fragments_are_constantized_or_absent_for_simple_layout

echo "[view-macro] contract: share-button view macro split"
cargo test -p ui-components --test share_button_semantics share_button_view_macro_complexity_is_split_into_semantic_subrenders

echo "[view-macro] contract: flip-card view macro complexity"
cargo test -p ui-flip-card flip_card_view_macro_complexity_is_small_and_does_not_require_semantic_subrenders

echo "[view-macro] contract: flip-card function-first split"
cargo test -p ui-flip-card flip_card_view_functional_split_prefers_plain_functions_over_local_components

echo "[view-macro] contract: flip-card static fragment scope"
cargo test -p ui-flip-card flip_card_static_fragments_are_constantized_or_absent_for_simple_layout

echo "[view-macro] contract: alert-dialog view macro split"
cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_view_macro_complexity_is_split_into_semantic_subrenders

echo "[view-macro] contract: alert-dialog function-first split"
cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_view_functional_split_prefers_plain_functions_over_local_components

echo "[view-macro] contract: alert-dialog static fragment constantization"
cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_static_fragments_are_constantized_with_templated_type_icons

echo "[view-macro] contract: dialog view macro split"
cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_view_macro_complexity_is_bounded_with_semantic_subblocks

echo "[view-macro] contract: dialog function-first split"
cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_view_functional_split_prefers_plain_functions_over_local_components

echo "[view-macro] contract: dialog static fragment constantization"
cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_static_fragments_are_constantized_with_accessible_close_icon_template

echo "[view-macro] contract: drawer view macro split"
cargo test -p ui-components --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_view_macro_complexity_is_split_into_semantic_subrenders

echo "[view-macro] contract: drawer function-first split"
cargo test -p ui-components --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_view_functional_split_prefers_plain_functions_over_local_components

echo "[view-macro] contract: drawer static fragment constantization"
cargo test -p ui-components --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_static_fragments_are_constantized_or_absent_for_simple_overlay_layout

echo "[view-macro] contract: tag view macro split"
cargo test -p ui-components --test tag_semantics --no-default-features --features component-tag,inject-css tag_view_macro_complexity_is_split_into_semantic_subrenders

echo "[view-macro] contract: tag function-first split"
cargo test -p ui-components --test tag_semantics --no-default-features --features component-tag,inject-css tag_view_functional_split_prefers_plain_functions_over_local_components

echo "[view-macro] contract: tag static fragment constantization"
cargo test -p ui-components --test tag_semantics --no-default-features --features component-tag,inject-css tag_static_fragments_are_constantized_with_stable_semantics

echo "[view-macro] contract: tag-group view macro split"
cargo test -p ui-components --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_view_macro_complexity_is_split_into_semantic_subrenders

echo "[view-macro] contract: tag-group function-first split"
cargo test -p ui-components --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_view_functional_split_prefers_plain_functions_over_local_components

echo "[view-macro] contract: tag-group static fragment constantization"
cargo test -p ui-components --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_static_fragments_are_constantized_with_stable_semantics

echo "[view-macro] contract: tabs view macro split"
cargo test -p ui-components --test tabs_semantics tabs_view_macro_complexity_is_split_into_semantic_subrenders

echo "[view-macro] contract: tabs function-first split"
cargo test -p ui-components --test tabs_semantics tabs_view_functional_split_prefers_plain_functions_over_local_components

echo "[view-macro] contract: tabs static fragment constantization"
cargo test -p ui-components --test tabs_semantics tabs_static_fragments_are_constantized_with_stable_semantics

echo "[view-macro] contract: well view macro complexity"
cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_view_macro_complexity_is_small_and_does_not_require_semantic_subrenders

echo "[view-macro] contract: well function-first simple split"
cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_view_functional_split_prefers_no_extra_local_components_for_simple_layout

echo "[view-macro] contract: swatch view macro complexity"
cargo test -p ui-components --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_view_macro_complexity_is_small_and_does_not_require_semantic_subrenders

echo "[view-macro] contract: swatch function-first simple split"
cargo test -p ui-components --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_view_functional_split_prefers_no_extra_local_components_for_simple_layout

echo "[view-macro] contract: circular-progress view macro complexity"
cargo test -p ui-components --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_view_macro_complexity_is_small_and_does_not_require_semantic_subrenders

echo "[view-macro] contract: circular-progress function-first simple split"
cargo test -p ui-components --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_view_functional_split_prefers_no_extra_local_components_for_simple_layout

echo "[view-macro] contract: circular-progress static fragment scope"
cargo test -p ui-components --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_static_fragments_are_constantized_or_absent_for_simple_indicator_layout

echo "[view-macro] contract: swatch static fragment scope"
cargo test -p ui-components --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_static_fragments_are_constantized_or_absent_for_simple_indicator_layout

echo "[view-macro] contract: swatch functional split keeps stable semantic markers"
cargo test -p ui-components --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_functional_split_keeps_semantic_markers_stable_for_test_selectors

echo "[view-macro] contract: textarea view macro complexity"
cargo test -p ui-components --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_view_macro_complexity_is_bounded_with_semantic_subblocks

echo "[view-macro] contract: textarea function-first split"
cargo test -p ui-components --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_view_functional_split_prefers_plain_functions_over_local_components

echo "[view-macro] contract: textarea static fragment constantization"
cargo test -p ui-components --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_static_fragments_are_constantized_or_absent_for_simple_input_layout

echo "[view-macro] contract: time-field view macro split"
cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_view_macro_complexity_is_split_into_semantic_subrenders

echo "[view-macro] contract: time-field function-first split"
cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_view_functional_split_prefers_plain_functions_over_local_components

echo "[view-macro] contract: time-field static fragment constantization"
cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_static_fragments_are_constantized_with_stable_semantics

echo "[view-macro] contract: form-field view macro split"
cargo test -p ui-components --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_view_macro_complexity_is_controlled_by_semantic_subview_split

echo "[view-macro] contract: form-field function-first split"
cargo test -p ui-components --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_view_functional_split_prefers_plain_functions_over_extra_local_components

echo "[view-macro] contract: form-field static fragment scope"
cargo test -p ui-components --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_static_fragments_are_constantized_or_absent_for_simple_layout

echo "[view-macro] contract: scroll-area view macro complexity"
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_view_macro_complexity_is_small_and_does_not_require_semantic_subrenders

echo "[view-macro] contract: scroll-area function-first simple split"
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_view_functional_split_prefers_no_extra_local_components_for_simple_layout

echo "[view-macro] contract: scroll-area static fragment scope"
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_static_fragments_are_constantized_or_absent_for_simple_layout

echo "[view-macro] contract: slider view macro split"
cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_view_macro_complexity_is_split_into_semantic_subrenders

echo "[view-macro] contract: slider function-first split"
cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_view_functional_split_prefers_plain_functions_over_local_components

echo "[view-macro] contract: slider static fragment constantization"
cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_static_fragments_are_constantized_with_stable_semantics

echo "[view-macro] contract: chart view macro split"
cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_view_macro_complexity_is_split_into_semantic_subrenders

echo "[view-macro] contract: chart function-first split"
cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_view_functional_split_prefers_plain_functions_over_local_components

echo "[view-macro] contract: chart static fragment constantization"
cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_static_fragments_are_constantized_with_stable_semantics

echo "[view-macro] contract: collapsible view macro split"
cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_view_macro_complexity_is_split_into_semantic_subrenders

echo "[view-macro] contract: collapsible function-first split"
cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_view_functional_split_prefers_plain_functions_over_local_components

echo "[view-macro] contract: collapsible static fragment constantization"
cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_static_fragments_are_constantized_or_absent_for_simple_layout

echo "[view-macro] contract: carousel view macro split"
cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_view_macro_complexity_is_split_into_semantic_subrenders

echo "[view-macro] contract: carousel function-first split"
cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_view_functional_split_prefers_plain_functions_over_local_components

echo "[view-macro] contract: carousel static fragment scope"
cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_static_fragments_are_constantized_or_absent_for_simple_layout

echo "[view-macro] contract: color-editor view macro split"
cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_view_macro_complexity_is_split_into_semantic_subrenders

echo "[view-macro] contract: color-editor function-first split"
cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_view_functional_split_prefers_plain_functions_over_local_components

echo "[view-macro] contract: color-editor static fragment scope"
cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_static_fragments_are_constantized_or_absent_for_simple_layout

echo "[view-macro] contract: color-swatch view macro complexity"
cargo test -p ui-components --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_view_macro_complexity_is_small_and_does_not_require_semantic_subrenders

echo "[view-macro] contract: color-swatch function-first split"
cargo test -p ui-components --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_view_functional_split_prefers_no_extra_local_components_for_simple_layout

echo "[view-macro] contract: color-swatch static fragment scope"
cargo test -p ui-components --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_static_fragments_are_constantized_or_absent_for_simple_indicator_layout

echo "[view-macro] contract: color-thumb view macro complexity"
cargo test -p ui-components --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_view_macro_complexity_is_split_into_semantic_subrenders

echo "[view-macro] contract: color-thumb function-first split"
cargo test -p ui-components --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_view_functional_split_prefers_plain_functions_over_local_components

echo "[view-macro] contract: color-thumb static fragment constantization"
cargo test -p ui-components --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_static_fragments_are_constantized_or_absent_for_simple_layout

echo "[view-macro] contract: color-swatch-picker view macro split"
cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_view_macro_complexity_is_split_into_semantic_subrenders

echo "[view-macro] contract: color-swatch-picker function-first split"
cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_view_functional_split_prefers_plain_functions_over_local_components

echo "[view-macro] contract: color-swatch-picker static fragment scope"
cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_static_fragments_are_constantized_or_absent_for_simple_layout

echo "[view-macro] contract: color-slider view macro split"
cargo test -p ui-color-slider color_slider_view_macro_complexity_is_split_into_semantic_subrenders

echo "[view-macro] contract: color-slider function-first split"
cargo test -p ui-color-slider color_slider_view_functional_split_prefers_plain_functions_over_local_components

echo "[view-macro] contract: color-slider static fragment scope"
cargo test -p ui-color-slider color_slider_static_fragments_are_constantized_or_absent_for_simple_layout

echo "[view-macro] contract: autocomplete view macro split"
cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_view_macro_complexity_is_split_into_semantic_subrenders

echo "[view-macro] contract: autocomplete function-first split"
cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_view_functional_split_prefers_plain_functions_over_local_components

echo "[view-macro] contract: autocomplete static fragment scope"
cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_static_fragments_are_constantized_or_absent_for_simple_combobox_layout

echo "[view-macro] contract: error-view view macro complexity"
cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_view_macro_complexity_is_bounded_with_semantic_subblocks

echo "[view-macro] contract: error-view function-first split"
cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_view_functional_split_prefers_plain_functions_over_local_components

echo "[view-macro] contract: error-view static fragment constantization"
cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_static_fragments_are_constantized_or_absent_for_simple_layout

echo "[view-macro] contract: fieldset view macro split"
cargo test -p ui-components --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_view_macro_complexity_is_split_into_semantic_subblocks

echo "[view-macro] contract: fieldset function-first split"
cargo test -p ui-components --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_view_functional_split_prefers_plain_functions_over_local_components

echo "[view-macro] contract: fieldset static fragment constantization"
cargo test -p ui-components --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_static_fragments_are_constantized_or_absent_for_simple_layout

echo "[view-macro] contract: combo-box view macro split"
cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_view_macro_complexity_is_split_into_semantic_subrenders

echo "[view-macro] contract: combo-box function-first split"
cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_view_functional_split_prefers_plain_functions_over_extra_local_components

echo "[view-macro] contract: combo-box static fragment constantization"
cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_static_fragments_are_constantized_with_stable_semantics

echo "[view-macro] contract: color-picker view macro split"
cargo test -p ui-color-picker color_picker_view_macro_complexity_is_split_into_semantic_subrenders

echo "[view-macro] contract: color-picker function-first split"
cargo test -p ui-color-picker color_picker_view_functional_split_prefers_plain_functions_over_local_components

echo "[view-macro] contract: color-picker static fragment constantization"
cargo test -p ui-color-picker color_picker_static_fragments_are_constantized_with_stable_semantics

echo "[view-macro] contract: hover-card view macro split"
cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_view_macro_complexity_is_split_into_semantic_subrenders

echo "[view-macro] contract: hover-card function-first split"
cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_view_functional_split_prefers_plain_functions_over_local_components

echo "[view-macro] contract: hover-card static fragment scope"
cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_static_fragments_are_constantized_or_absent_for_simple_layout

echo "[view-macro] contract: list view macro split"
cargo test -p ui-components --test list_module_semantics --no-default-features --features component-list,inject-css list_view_macro_complexity_is_split_into_semantic_subrenders

echo "[view-macro] contract: list function-first split"
cargo test -p ui-components --test list_module_semantics --no-default-features --features component-list,inject-css list_view_functional_split_prefers_plain_functions_over_local_components

echo "[view-macro] contract: list static fragment constantization"
cargo test -p ui-components --test list_module_semantics --no-default-features --features component-list,inject-css list_static_fragments_are_constantized_with_stable_a11y_markers

echo "[view-macro] contract: menu view macro split"
cargo test -p ui-menu menu_view_macro_complexity_is_split_into_semantic_subblocks

echo "[view-macro] contract: menu function-first split"
cargo test -p ui-menu menu_view_functional_split_prefers_plain_functions_over_local_components

echo "[view-macro] contract: menu static fragment constantization"
cargo test -p ui-menu menu_static_fragments_are_constantized_with_stable_semantics

echo "[view-macro] OK"
