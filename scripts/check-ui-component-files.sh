#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[component-files] contract: required file layout"
cargo test -p ui --test button_semantics button_component_directory_has_standard_file_layout

echo "[component-files] contract: mod.rs minimal stable exports"
cargo test -p ui --test button_semantics button_mod_rs_keeps_minimal_stable_exports

echo "[component-files] contract: logic/styles/view/motion/spec responsibilities"
cargo test -p ui --test button_semantics button_component_file_responsibilities_remain_scoped

echo "[component-files] contract: spec.rs stays scarce and versioned for complex button only"
cargo test -p ui --test button_semantics button_spec_file_contract_is_scarce_and_has_versioned_regression_coverage

echo "[component-files] contract: button hyper-structure builder exposes new->render chain"
cargo test -p ui --test button_semantics button_hyper_structure_builder_spec_contract_is_available_for_complex_component

echo "[component-files] contract: button context-compression manifest + rbi projection"
cargo test -p ui --test button_semantics button_context_compression_manifest_and_rbi_projection_are_present_and_current

echo "[component-files] contract: form-field standard file layout + scoped responsibilities (motion/spec N/A)"
cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_component_directory_standard_files_follow_contract_and_na_paths

echo "[component-files] contract: form-field file-placement discipline (no render.rs, motion/spec N/A)"
cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_file_placement_discipline_is_strict_for_component_scope

echo "[component-files] contract: form-field hyper-structure builder spec is explicitly N/A"
cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_hyper_structure_builder_spec_is_not_applicable_for_simple_component

echo "[component-files] contract: form-field context-compression manifest + rbi projection"
cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_context_compression_manifest_and_rbi_projection_are_present_and_synced

echo "[component-files] contract: well required file layout + export boundary + scoped responsibilities"
# cargo test -p ui --test well_semantics --no-default-features --features component-well,inject-css well_component_directory_has_standard_file_layout
# cargo test -p ui --test well_semantics --no-default-features --features component-well,inject-css well_mod_rs_keeps_minimal_stable_exports
# cargo test -p ui --test well_semantics --no-default-features --features component-well,inject-css well_component_file_responsibilities_remain_scoped
cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_component_directory_has_standard_file_layout
cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_mod_rs_keeps_minimal_stable_exports
cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_component_file_responsibilities_remain_scoped

echo "[component-files] contract: tabs required file layout + export boundary + scoped responsibilities"
cargo test -p ui --test tabs_semantics tabs_component_directory_has_standard_file_layout
cargo test -p ui --test tabs_semantics tabs_mod_rs_keeps_minimal_stable_exports
cargo test -p ui --test tabs_semantics tabs_component_file_responsibilities_remain_scoped

echo "[component-files] contract: tag required file layout + export boundary + scoped responsibilities"
cargo test -p ui --test tag_semantics --no-default-features --features component-tag,inject-css tag_component_directory_has_standard_file_layout
cargo test -p ui --test tag_semantics --no-default-features --features component-tag,inject-css tag_mod_rs_keeps_minimal_stable_exports
cargo test -p ui --test tag_semantics --no-default-features --features component-tag,inject-css tag_component_file_responsibilities_remain_scoped

echo "[component-files] contract: tag-group required file layout + export boundary + scoped responsibilities"
cargo test -p ui --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_component_directory_has_standard_file_layout
cargo test -p ui --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_mod_rs_keeps_minimal_stable_exports
cargo test -p ui --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_component_file_responsibilities_remain_scoped

echo "[component-files] contract: swatch required file layout + export boundary + scoped responsibilities"
cargo test -p ui --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_component_directory_has_standard_file_layout
cargo test -p ui --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_mod_rs_keeps_minimal_stable_exports
cargo test -p ui --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_component_file_responsibilities_remain_scoped

echo "[component-files] contract: textarea required file layout + export boundary + scoped responsibilities"
cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_component_directory_has_standard_file_layout
cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_mod_rs_keeps_minimal_stable_exports
cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_component_file_responsibilities_remain_scoped

echo "[component-files] contract: time-field required file layout + export boundary + scoped responsibilities"
cargo test -p ui --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_component_directory_has_standard_file_layout
cargo test -p ui --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_mod_rs_keeps_minimal_stable_exports
cargo test -p ui --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_component_file_responsibilities_remain_scoped

echo "[component-files] contract: list required file layout + scoped responsibilities"
cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_component_directory_has_standard_file_layout

echo "[component-files] contract: menu standard file layout + scoped responsibilities"
cargo test -p ui-menu menu_component_directory_standard_files_follow_contract_and_na_spec

echo "[component-files] contract: menu file-placement discipline in AI struct-first section"
cargo test -p ui-menu menu_file_placement_discipline_is_strict_for_component_scope

echo "[component-files] contract: menu hyper-structure builder spec is explicitly N/A"
cargo test -p ui-menu menu_hyper_structure_builder_spec_is_not_applicable_for_simple_component

echo "[component-files] contract: menu context-compression manifest + rbi projection"
cargo test -p ui-menu menu_context_compression_manifest_and_rbi_projection_are_present_and_current

echo "[component-files] contract: list context-compression manifest + rbi projection"
cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_context_compression_manifest_and_rbi_projection_are_present_and_current

echo "[component-files] contract: command-dialog standard file layout + scoped responsibilities"
cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_component_directory_standard_files_follow_contract_and_na_paths

echo "[component-files] contract: dialog standard file layout + scoped responsibilities"
cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_component_directory_standard_files_follow_contract_and_na_paths

echo "[component-files] contract: dialog file-placement discipline in AI struct-first section"
cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_file_placement_discipline_is_strict_for_component_scope

echo "[component-files] contract: dialog hyper-structure builder spec contract is explicitly N/A"
cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_hyper_structure_builder_spec_is_not_applicable_for_simple_component

echo "[component-files] contract: dialog context-compression manifest + rbi projection"
cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_context_compression_manifest_and_rbi_projection_are_present_and_current

echo "[component-files] contract: error-view standard file layout + scoped responsibilities"
cargo test -p ui --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_component_files_respect_layered_responsibility_boundaries

echo "[component-files] contract: error-view keeps spec.rs out of simple component surface"
cargo test -p ui --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_does_not_introduce_spec_rs_for_simple_component

echo "[component-files] contract: error-view file-placement discipline in AI struct-first section"
cargo test -p ui --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_file_placement_discipline_is_strict_for_component_scope

echo "[component-files] contract: error-view hyper-structure builder spec is explicitly N/A"
cargo test -p ui --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_hyper_structure_builder_spec_is_not_applicable_for_simple_component

echo "[component-files] contract: error-view context-compression manifest + rbi projection"
cargo test -p ui --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_context_compression_manifest_and_rbi_projection_are_present_and_current

echo "[component-files] contract: command-dialog file-placement discipline in AI struct-first section"
cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_file_placement_discipline_is_strict_for_component_scope

echo "[component-files] contract: command-dialog hyper-structure builder spec contract is explicitly N/A"
cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_hyper_structure_builder_spec_is_not_applicable_for_simple_component

echo "[component-files] contract: command-dialog context-compression manifest + rbi projection"
cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_context_compression_manifest_and_rbi_projection_are_present_and_current

echo "[component-files] contract: combo-box standard file layout + scoped responsibilities"
cargo test -p ui --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_component_directory_standard_files_follow_contract_and_na_paths

echo "[component-files] contract: autocomplete standard file layout + scoped responsibilities"
cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_component_directory_standard_files_follow_contract_and_na_spec

echo "[component-files] contract: autocomplete file-placement discipline in AI struct-first section"
cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_file_placement_discipline_is_strict_for_component_scope

echo "[component-files] contract: autocomplete hyper-structure builder spec is explicitly N/A"
cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_hyper_structure_builder_spec_is_not_applicable_for_simple_component

echo "[component-files] contract: autocomplete context-compression manifest + rbi projection"
cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_context_compression_manifest_and_rbi_projection_are_present_and_current

echo "[component-files] contract: drop-zone standard file layout + scoped responsibilities"
cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_component_directory_standard_files_follow_contract_and_na_paths

echo "[component-files] contract: drop-zone hyper-structure builder spec is explicitly N/A"
cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_hyper_structure_builder_spec_is_not_applicable_for_simple_component

echo "[component-files] contract: drop-zone context-compression manifest + rbi projection"
cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_context_compression_manifest_and_rbi_projection_are_present_and_current

echo "[component-files] contract: modal standard file layout + scoped responsibilities"
cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_component_directory_standard_files_follow_contract_and_na_paths

echo "[component-files] contract: modal file-placement discipline in AI struct-first section"
cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_file_placement_discipline_is_strict_for_component_scope

echo "[component-files] contract: modal hyper-structure builder spec contract is explicitly N/A"
cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_hyper_structure_builder_spec_is_not_applicable_for_simple_component

echo "[component-files] contract: modal context-compression manifest + rbi projection"
cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_context_compression_manifest_and_rbi_projection_are_present_and_current

echo "[component-files] contract: drawer standard file layout + scoped responsibilities"
cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_component_directory_standard_files_follow_contract_and_na_paths

echo "[component-files] contract: drawer file-placement discipline in AI struct-first section"
cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_file_placement_discipline_is_strict_for_component_scope

echo "[component-files] contract: drawer hyper-structure builder spec contract is explicitly N/A"
cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_hyper_structure_builder_spec_is_not_applicable_for_simple_component

echo "[component-files] contract: drawer context-compression manifest + rbi projection"
cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_context_compression_manifest_and_rbi_projection_are_present_and_current

echo "[component-files] contract: combo-box file-placement discipline in AI struct-first section"
cargo test -p ui --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_file_placement_discipline_is_strict_and_protocol_free

echo "[component-files] contract: combo-box hyper-structure builder spec contract is explicitly N/A"
cargo test -p ui --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_hyper_structure_builder_spec_is_not_applicable_for_simple_component

echo "[component-files] contract: combo-box context-compression manifest + rbi projection"
cargo test -p ui --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_context_compression_manifest_and_rbi_projection_are_present_and_current

echo "[component-files] contract: checkbox-field required file layout + export boundary + scoped responsibilities"
cargo test -p ui --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_component_directory_has_standard_file_layout
cargo test -p ui --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_mod_rs_keeps_minimal_stable_exports
cargo test -p ui --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_component_file_responsibilities_remain_scoped
cargo test -p ui --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_file_placement_discipline_is_strict_and_protocol_free
cargo test -p ui --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_spec_file_is_not_introduced_for_simple_component
cargo test -p ui --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_context_compression_manifest_and_rbi_projection_are_present_and_current

echo "[component-files] contract: checkbox-group standard file layout + scoped responsibilities"
cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_component_directory_standard_files_follow_contract_and_na_paths

echo "[component-files] contract: checkbox-group file-placement discipline (no render.rs, spec.rs N/A)"
cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_file_placement_discipline_is_strict_for_component_scope

echo "[component-files] contract: checkbox-group hyper-structure builder spec is explicitly N/A"
cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_hyper_structure_builder_spec_is_not_applicable_for_simple_component

echo "[component-files] contract: checkbox-group context-compression manifest + rbi projection"
cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_context_compression_manifest_and_rbi_projection_are_present_and_current

echo "[component-files] contract: date-input-group standard file layout + scoped responsibilities"
cargo test -p ui-date-input-group date_input_group_component_directory_standard_files_follow_contract_and_na_paths

echo "[component-files] contract: date-input-group hyper-structure builder spec is explicitly N/A"
cargo test -p ui-date-input-group date_input_group_hyper_structure_builder_spec_is_not_applicable_for_simple_component

echo "[component-files] contract: date-input-group context-compression manifest + rbi projection"
cargo test -p ui-date-input-group date_input_group_context_compression_manifest_and_rbi_projection_are_present_and_current

echo "[component-files] contract: checkbox standard file layout + scoped responsibilities"
cargo test -p ui --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_component_directory_standard_files_follow_contract_and_na_paths_locally

echo "[component-files] contract: alert-dialog standard file layout + scoped responsibilities"
cargo test -p ui --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_component_directory_standard_files_follow_contract_and_no_spec

echo "[component-files] contract: alert-dialog file-placement discipline in AI struct-first section"
cargo test -p ui --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_file_placement_discipline_is_strict_for_component_scope

echo "[component-files] contract: alert-dialog hyper-structure builder spec is explicitly N/A"
cargo test -p ui --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_hyper_structure_builder_spec_is_not_applicable_for_simple_component

echo "[component-files] contract: alert-dialog context-compression manifest + rbi projection"
cargo test -p ui --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_context_compression_manifest_and_rbi_projection_are_present_and_current

echo "[component-files] contract: command standard file layout + scoped responsibilities"
cargo test -p ui-command --lib command_component_directory_standard_files_follow_contract_and_na_spec

echo "[component-files] contract: command file-placement discipline in AI struct-first section"
cargo test -p ui-command --lib command_file_placement_discipline_is_strict_for_component_scope

echo "[component-files] contract: command hyper-structure builder spec is explicitly N/A"
cargo test -p ui-command --lib command_hyper_structure_builder_spec_is_not_applicable_for_simple_component

echo "[component-files] contract: command context-compression manifest + rbi projection"
cargo test -p ui-command --lib command_context_compression_manifest_and_rbi_projection_are_present_and_current

echo "[component-files] contract: color-editor standard file layout + scoped responsibilities"
cargo test -p ui --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_component_directory_standard_files_follow_contract_and_na_paths

echo "[component-files] contract: color-picker standard file layout + scoped responsibilities"
cargo test -p ui-color-picker color_picker_component_directory_standard_files_follow_contract_and_na_paths

echo "[component-files] contract: color-picker file-placement discipline in AI struct-first section"
cargo test -p ui-color-picker color_picker_file_placement_discipline_contract_is_explicit_for_interactive_component_scope

echo "[component-files] contract: color-picker hyper-structure builder spec contract is explicitly N/A"
cargo test -p ui-color-picker color_picker_hyper_structure_builder_spec_is_explicitly_na_for_non_complex_component

echo "[component-files] contract: color-picker context-compression manifest + rbi projection"
cargo test -p ui-color-picker color_picker_context_compression_manifest_and_rbi_projection_are_present_and_current

echo "[component-files] contract: carousel standard file layout + scoped responsibilities"
cargo test -p ui --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_component_directory_standard_files_follow_contract_and_na_paths

echo "[component-files] contract: carousel file-placement discipline in AI struct-first section"
cargo test -p ui --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_file_placement_discipline_is_strict_for_component_scope

echo "[component-files] contract: carousel hyper-structure builder spec is explicitly N/A"
cargo test -p ui --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_hyper_structure_builder_spec_is_not_applicable_for_simple_component

echo "[component-files] contract: carousel context-compression manifest + rbi projection"
cargo test -p ui --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_context_compression_manifest_and_rbi_projection_are_present_and_current

echo "[component-files] contract: flip-card standard file layout + scoped responsibilities"
cargo test -p ui-flip-card flip_card_component_directory_standard_files_follow_contract_and_na_paths

echo "[component-files] contract: flip-card file-placement discipline in AI struct-first section"
cargo test -p ui-flip-card flip_card_file_placement_discipline_is_strict_for_component_scope

echo "[component-files] contract: flip-card hyper-structure builder spec contract is explicitly N/A"
cargo test -p ui-flip-card flip_card_hyper_structure_builder_spec_is_not_applicable_for_simple_component

echo "[component-files] contract: flip-card context-compression manifest + rbi projection"
cargo test -p ui-flip-card flip_card_context_compression_manifest_and_rbi_projection_are_present_and_current

echo "[component-files] contract: color-slider standard file layout + scoped responsibilities"
cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_component_directory_standard_files_follow_contract_and_na_paths

echo "[component-files] contract: color-thumb standard file layout + scoped responsibilities"
cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_component_directory_standard_files_follow_contract_and_na_paths

echo "[component-files] contract: color-swatch standard file layout + scoped responsibilities"
cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_component_directory_standard_files_follow_contract_and_na_paths

echo "[component-files] contract: color-swatch-picker standard file layout + scoped responsibilities"
cargo test -p ui --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_component_directory_standard_files_follow_contract_and_na_paths

echo "[component-files] contract: fieldset standard file layout + scoped responsibilities"
cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_component_directory_standard_files_follow_contract_and_na_paths

echo "[component-files] contract: hover-card standard file layout + scoped responsibilities"
cargo test -p ui --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_component_file_responsibilities_remain_scoped

echo "[component-files] contract: hover-card file-placement discipline in AI struct-first section"
cargo test -p ui --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_file_placement_discipline_is_strict_for_component_scope

echo "[component-files] contract: hover-card hyper-structure builder spec contract is explicitly N/A"
cargo test -p ui --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_hyper_structure_builder_spec_is_not_applicable_for_simple_component

echo "[component-files] contract: hover-card context-compression manifest + rbi projection"
cargo test -p ui --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_context_compression_manifest_and_rbi_projection_are_present_and_current

echo "[component-files] contract: fieldset file-placement discipline in AI struct-first section"
cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_file_placement_discipline_is_strict_for_component_scope

echo "[component-files] contract: fieldset hyper-structure builder spec contract is explicitly N/A"
cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_hyper_structure_builder_spec_is_not_applicable_for_simple_component

echo "[component-files] contract: fieldset context-compression manifest + rbi projection"
cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_context_compression_manifest_and_rbi_projection_are_present_and_current

echo "[component-files] contract: color-swatch-picker strict file-placement discipline (no render/protocol, spec optional)"
cargo test -p ui --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_file_placement_discipline_is_strict_for_component_scope

echo "[component-files] contract: color-swatch-picker hyper-structure builder spec contract is explicitly N/A"
cargo test -p ui --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_hyper_structure_builder_spec_is_not_applicable_for_simple_component

echo "[component-files] contract: color-swatch-picker context-compression manifest + rbi projection"
cargo test -p ui --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_context_compression_manifest_and_rbi_projection_are_present_and_current

echo "[component-files] contract: color-swatch file-placement discipline in AI struct-first section"
cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_file_placement_discipline_contract_is_explicit_for_component_scope

echo "[component-files] contract: color-swatch hyper-structure builder spec contract is explicitly N/A"
cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_hyper_structure_builder_spec_is_explicitly_na_for_non_complex_component

echo "[component-files] contract: color-swatch context-compression manifest + rbi projection"
cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_context_compression_manifest_and_rbi_projection_are_present_and_current

echo "[component-files] contract: color-slider file-placement discipline in AI struct-first section"
cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_file_placement_discipline_contract_is_explicit_for_interactive_component_scope

echo "[component-files] contract: color-thumb file-placement discipline in AI struct-first section"
cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_file_placement_discipline_contract_is_explicit_for_interactive_component_scope

echo "[component-files] contract: color-thumb hyper-structure builder spec contract is explicitly N/A"
cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_hyper_structure_builder_spec_is_not_applicable_for_simple_component

echo "[component-files] contract: color-thumb context-compression manifest + rbi projection"
cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_context_compression_manifest_and_rbi_projection_are_present_and_current

echo "[component-files] contract: color-slider hyper-structure builder spec contract is explicitly N/A"
cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_keeps_spec_rs_out_of_simple_component_surface

echo "[component-files] contract: color-slider context-compression manifest + rbi projection"
cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_context_compression_manifest_and_rbi_projection_are_present_and_current

echo "[component-files] contract: color-wheel context-compression manifest + rbi projection"
cargo test -p ui --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_context_compression_manifest_and_rbi_projection_are_present_and_current

echo "[component-files] contract: color-editor file-placement discipline in AI struct-first section"
cargo test -p ui --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_file_placement_discipline_contract_is_explicit_for_interactive_component_scope

echo "[component-files] contract: color-editor hyper-structure builder spec contract is explicitly N/A"
cargo test -p ui --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_hyper_structure_builder_spec_is_explicitly_na_for_non_complex_component

echo "[component-files] contract: color-editor context-compression manifest + rbi projection"
cargo test -p ui --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_context_compression_manifest_and_rbi_projection_are_present_and_current

echo "[component-files] contract: checkbox file-placement discipline in AI struct-first section"
cargo test -p ui --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_file_placement_discipline_contract_is_explicit_for_interactive_component_scope_locally

echo "[component-files] contract: checkbox hyper-structure builder spec contract is explicitly N/A"
cargo test -p ui --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_hyper_structure_builder_spec_is_explicitly_na_for_non_complex_component_locally

echo "[component-files] contract: checkbox context-compression manifest + rbi projection"
cargo test -p ui --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_context_compression_manifest_and_rbi_are_present_and_consistent_locally

echo "[component-files] contract: bottom-sheet file-placement discipline in AI struct-first section"
cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_file_placement_discipline_contract_is_explicit_for_interactive_component_scope

echo "[component-files] contract: bottom-sheet hyper-structure builder spec contract is explicitly N/A"
cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_hyper_structure_builder_spec_is_explicitly_na_for_non_complex_component

echo "[component-files] contract: bottom-sheet context-compression manifest + rbi projection"
cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_context_compression_manifest_and_rbi_projection_are_present_and_current

echo "[component-files] contract: slider required file layout + export boundary + scoped responsibilities"
cargo test -p ui --test slider_semantics --no-default-features --features component-slider,inject-css slider_component_directory_has_standard_file_layout_and_no_spec_file
cargo test -p ui --test slider_semantics --no-default-features --features component-slider,inject-css slider_mod_rs_keeps_minimal_stable_exports
cargo test -p ui --test slider_semantics --no-default-features --features component-slider,inject-css slider_component_file_responsibilities_remain_scoped

echo "[component-files] contract: circular-progress standard file layout + scoped responsibilities"
cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_component_directory_standard_files_follow_contract_and_na_paths

echo "[component-files] contract: circular-progress hyper-structure spec builder N/A guard"
cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_hyper_structure_builder_spec_contract_is_not_applicable_for_simple_component

echo "[component-files] contract: circular-progress context-compression manifest + rbi projection"
cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_context_compression_manifest_and_rbi_are_present_and_consistent

echo "[component-files] contract: coachmark standard file layout + scoped responsibilities"
cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_component_directory_standard_files_follow_contract_and_na_paths

echo "[component-files] contract: coachmark file-placement discipline in AI struct-first section"
cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_file_placement_discipline_contract_is_explicit_for_interactive_component_scope

echo "[component-files] contract: coachmark hyper-structure builder spec contract is explicitly N/A"
cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_hyper_structure_builder_spec_is_explicitly_na_for_non_complex_component

echo "[component-files] contract: coachmark context-compression manifest + rbi projection"
cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_context_compression_manifest_and_rbi_are_present_and_consistent

echo "[component-files] contract: chart scoped file responsibilities and forbidden render/spec drift"
cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_component_file_responsibilities_stay_layered_and_non_overlapping

echo "[component-files] contract: chart file-placement discipline in AI struct-first section"
cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_file_placement_discipline_is_strict_for_struct_first_scope

echo "[component-files] contract: chart hyper-structure builder spec is explicitly N/A"
cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_hyper_structure_builder_spec_is_not_applicable_for_simple_component

echo "[component-files] contract: chart context-compression manifest + rbi projection"
cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_context_compression_manifest_and_rbi_projection_are_present_and_current

echo "[component-files] contract: scroll-area required file layout + export boundary + scoped responsibilities"
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_component_directory_has_standard_file_layout
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_mod_rs_keeps_minimal_stable_exports
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_component_file_responsibilities_remain_scoped

echo "[component-files] contract: overlays standard file layout + scoped responsibilities"
cargo test -p ui-overlays overlays_component_directory_standard_files_follow_contract_and_na_spec

echo "[component-files] contract: overlays file-placement discipline in AI struct-first section"
cargo test -p ui-overlays overlays_file_placement_discipline_is_strict_for_component_scope

echo "[component-files] contract: overlays hyper-structure builder spec contract is explicitly N/A"
cargo test -p ui-overlays overlays_hyper_structure_builder_spec_is_not_applicable_for_simple_component

echo "[component-files] contract: overlays context-compression manifest + rbi projection"
cargo test -p ui-overlays overlays_context_compression_manifest_and_rbi_projection_are_present_and_current

echo "[component-files] OK"
