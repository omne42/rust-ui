#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[engineering] contract: serde schema + structured migration errors"
cargo test -p ui-components --test button_semantics button_engineering_contract_uses_serde_schema_and_structured_migration_errors

echo "[engineering] contract: version deprecation migration NA guard"
cargo test -p ui-components --test button_semantics button_version_deprecation_migration_is_na_without_major_breaking_upgrade

echo "[engineering] contract: tracing target semantics"
cargo test -p ui-components --test button_semantics button_engineering_contract_uses_consistent_tracing_targets

echo "[engineering] contract: runtime boundary leakage"
cargo test -p ui-components --test button_semantics button_engineering_contract_avoids_runtime_leaks_in_public_api

echo "[engineering] contract: button-copy tracing + runtime boundary leakage"
cargo test -p ui-components --test button_copy_semantics button_copy_engineering_contract_reuses_button_tracing_and_avoids_runtime_leaks

echo "[engineering] contract: action-button tracing + runtime boundary leakage"
cargo test -p ui-components --test action_button_semantics action_button_engineering_contract_reuses_button_tracing_and_avoids_runtime_leaks

echo "[engineering] contract: well serde/spec NA + tracing semantics + runtime boundary leakage"
# cargo test -p ui-components --test well_semantics --no-default-features --features component-well,inject-css well_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope
# cargo test -p ui-components --test well_semantics --no-default-features --features component-well,inject-css well_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events
# cargo test -p ui-components --test well_semantics --no-default-features --features component-well,inject-css well_engineering_contract_avoids_runtime_leaks_in_public_api_surface
cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope
cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events
cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_engineering_contract_avoids_runtime_leaks_in_public_api_surface

echo "[engineering] contract: tabs serde/spec NA + tracing semantics + runtime boundary leakage"
cargo test -p ui-components --test tabs_semantics tabs_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope
cargo test -p ui-components --test tabs_semantics tabs_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events
cargo test -p ui-components --test tabs_semantics tabs_engineering_contract_avoids_runtime_leaks_in_public_api_surface

echo "[engineering] contract: tag serde/spec NA + tracing semantics + runtime boundary leakage"
cargo test -p ui-components --test tag_semantics --no-default-features --features component-tag,inject-css tag_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope
cargo test -p ui-components --test tag_semantics --no-default-features --features component-tag,inject-css tag_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events
cargo test -p ui-components --test tag_semantics --no-default-features --features component-tag,inject-css tag_engineering_contract_avoids_runtime_leaks_in_public_api_surface

echo "[engineering] contract: tag-group serde/spec NA + tracing semantics + runtime boundary leakage"
cargo test -p ui-components --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope
cargo test -p ui-components --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events
cargo test -p ui-components --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_engineering_contract_avoids_runtime_leaks_in_public_api_surface

echo "[engineering] contract: swatch serde/spec NA + tracing semantics + runtime boundary leakage"
cargo test -p ui-components --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope
cargo test -p ui-components --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events
cargo test -p ui-components --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_engineering_contract_avoids_runtime_leaks_in_public_api_surface

echo "[engineering] contract: textarea serde/spec NA + tracing semantics + runtime boundary leakage"
cargo test -p ui-components --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope
cargo test -p ui-components --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events
cargo test -p ui-components --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_engineering_contract_avoids_runtime_leaks_in_public_api_surface

echo "[engineering] contract: time-field serde/spec NA + tracing semantics + runtime boundary leakage"
cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope
cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events
cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_engineering_contract_avoids_runtime_leaks_in_public_api_surface

echo "[engineering] contract: fieldset serde protocol + tracing semantics + runtime boundary leakage"
cargo test -p ui-components --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_engineering_contract_uses_serde_protocol_and_structured_schema_defaults
cargo test -p ui-components --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_version_deprecation_migration_is_na_without_major_breaking_upgrade
cargo test -p ui-components --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events
cargo test -p ui-components --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_engineering_contract_avoids_runtime_leaks_in_public_api_surface
echo "[engineering] contract: fieldset rust hygiene forbids unwrap/expect + swallowed results + string clone churn"
cargo test -p ui-components --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources
cargo test -p ui-components --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent
cargo test -p ui-components --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_rust_hygiene_script_enforces_repo_level_hygiene_guards

echo "[engineering] contract: form-field serde protocol + tracing semantics + runtime boundary leakage"
cargo test -p ui-components --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries
cargo test -p ui-components --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade

echo "[engineering] contract: form-field rust hygiene forbids unwrap/expect + swallowed results + string clone churn"
cargo test -p ui-components --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources
cargo test -p ui-components --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent
cargo test -p ui-components --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_rust_hygiene_script_enforces_repo_level_hygiene_guards

echo "[engineering] contract: field version deprecation migration N/A gate"
cargo test -p ui-field field_version_deprecation_migration_is_not_required_without_major_breaking_upgrade

echo "[engineering] contract: date-input-group serde/spec NA + tracing semantics + runtime boundary leakage"
cargo test -p ui-date-input-group date_input_group_engineering_capability_contract_is_na_and_runtime_agnostic

echo "[engineering] contract: date-input-group version deprecation migration registry N/A gate"
cargo test -p ui-date-input-group date_input_group_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade

echo "[engineering] contract: date-input-group rust hygiene forbids unwrap/expect + swallowed results + string clone churn"
cargo test -p ui-date-input-group date_input_group_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources
cargo test -p ui-date-input-group date_input_group_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent
cargo test -p ui-date-input-group date_input_group_rust_hygiene_script_enforces_repo_level_hygiene_guards

echo "[engineering] contract: checkbox-field serde/spec NA + tracing semantics + runtime boundary leakage"
cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope
cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events
cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_engineering_contract_avoids_runtime_leaks_in_public_api_surface

echo "[engineering] contract: checkbox-field version deprecation migration N/A gate"
cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_version_deprecation_migration_is_na_without_major_breaking_upgrade

echo "[engineering] contract: checkbox-field rust hygiene forbids unwrap/expect + swallowed results + string clone churn"
cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources
cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent
cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_rust_hygiene_script_enforces_repo_level_hygiene_guards

echo "[engineering] contract: checkbox serde/spec NA + tracing semantics + runtime boundary leakage"
cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope_locally
cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events_locally
cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_engineering_contract_avoids_runtime_leaks_in_public_api_surface_locally
echo "[engineering] contract: checkbox version deprecation migration registry N/A gate"
cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade_locally

echo "[engineering] contract: checkbox rust hygiene forbids unwrap/expect + swallowed results + string clone churn"
cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources
cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent
cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_rust_hygiene_script_enforces_repo_level_hygiene_guards
echo "[engineering] contract: checkbox status-primitives sourcing + two-pass geometry NA"
cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_check2_documents_status_primitives_layer_rules
cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_status_primitives_layer_is_consumed_without_component_local_state_machine
cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_two_pass_geometry_rendering_is_na_and_measurement_free

echo "[engineering] contract: checkbox-group serde/spec NA + tracing semantics + runtime boundary leakage"
cargo test -p ui-components --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope
cargo test -p ui-components --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events
cargo test -p ui-components --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_engineering_contract_avoids_runtime_leaks_in_public_api_surface
echo "[engineering] contract: checkbox-group version deprecation migration registry N/A gate"
cargo test -p ui-components --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade

echo "[engineering] contract: checkbox-group rust hygiene forbids unwrap/expect + swallowed results + string clone churn"
cargo test -p ui-components --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources
cargo test -p ui-components --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent
cargo test -p ui-components --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_rust_hygiene_script_enforces_repo_level_hygiene_guards

echo "[engineering] contract: bottom-sheet serde/spec NA + tracing semantics + runtime boundary leakage"
cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope
cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_version_deprecation_migration_is_na_without_major_breaking_upgrade
cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events
cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_engineering_contract_avoids_runtime_leaks_in_public_api_surface

echo "[engineering] contract: bottom-sheet ui-components fixed entry files follow layered boundaries"
cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_ui_components_fixed_entry_files_follow_layered_boundaries

echo "[engineering] contract: bottom-sheet component directory standard file layout"
cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_component_directory_standard_file_layout_is_enforced

echo "[engineering] contract: bottom-sheet rust hygiene forbids unwrap/expect + swallowed results + string clone churn"
cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources
cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent
cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_rust_hygiene_script_enforces_repo_level_hygiene_guards

echo "[engineering] contract: slider serde/spec NA + tracing semantics + runtime boundary leakage"
cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope
cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events
cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_engineering_contract_avoids_runtime_leaks_in_public_api_surface

echo "[engineering] contract: scroll-area serde/spec NA + tracing semantics + runtime boundary leakage"
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_engineering_contract_avoids_runtime_leaks_in_public_api_surface

echo "[engineering] contract: circular-progress serde/spec NA + tracing semantics + runtime boundary leakage"
cargo test -p ui-components --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope
cargo test -p ui-components --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events
cargo test -p ui-components --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_engineering_contract_avoids_runtime_leaks_in_public_api_surface
cargo test -p ui-components --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_version_deprecation_migration_is_na_without_major_breaking_upgrade
echo "[engineering] contract: circular-progress rust hygiene forbids unwrap/expect + swallowed results + string clone churn"
cargo test -p ui-components --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources
cargo test -p ui-components --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent
cargo test -p ui-components --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_rust_hygiene_script_enforces_repo_level_hygiene_guards

echo "[engineering] contract: coachmark serde protocol + tracing semantics + runtime boundary leakage"
cargo test -p ui-components --lib --no-default-features --features component-coachmark,inject-css coachmark_engineering_contract_uses_serde_protocol_and_structured_schema_defaults
cargo test -p ui-components --lib --no-default-features --features component-coachmark,inject-css coachmark_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events
cargo test -p ui-components --lib --no-default-features --features component-coachmark,inject-css coachmark_engineering_contract_avoids_runtime_leaks_in_public_api_surface

echo "[engineering] contract: coachmark version deprecation migration N/A gate"
cargo test -p ui-components --lib --no-default-features --features component-coachmark,inject-css coachmark_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade

echo "[engineering] contract: coachmark rust hygiene forbids unwrap/expect + swallowed results + string clone churn"
cargo test -p ui-components --lib --no-default-features --features component-coachmark,inject-css coachmark_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources
cargo test -p ui-components --lib --no-default-features --features component-coachmark,inject-css coachmark_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent
cargo test -p ui-components --lib --no-default-features --features component-coachmark,inject-css coachmark_rust_hygiene_script_enforces_repo_level_hygiene_guards

echo "[engineering] contract: alert-dialog serde protocol + tracing semantics + runtime boundary leakage"
cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_engineering_contract_uses_serde_protocol_and_structured_schema_defaults
cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events
cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_engineering_contract_avoids_runtime_leaks_in_public_api_surface
echo "[engineering] contract: alert-dialog version deprecation migration N/A gate"
cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade

echo "[engineering] contract: alert-dialog rust hygiene forbids unwrap/expect + swallowed results + string clone churn"
cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources
cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent
cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_rust_hygiene_script_enforces_repo_level_hygiene_guards

echo "[engineering] contract: modal serde protocol + tracing semantics + runtime boundary leakage"
cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_engineering_contract_uses_serde_protocol_and_structured_schema_defaults
cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_version_deprecation_migration_is_na_without_major_breaking_upgrade
cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events
cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_engineering_contract_avoids_runtime_leaks_in_public_api_surface
cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_check2_marks_kernel_shell_layer_boundary_items_complete

echo "[engineering] contract: breadcrumb version deprecation migration N/A gate"
cargo test -p ui-components --test breadcrumb_semantics --no-default-features --features component-breadcrumb,inject-css breadcrumb_version_deprecation_migration_is_na_without_major_breaking_upgrade

echo "[engineering] contract: calendar version deprecation migration N/A gate"
cargo test -p ui-calendar calendar_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade

echo "[engineering] contract: overlays serde protocol + tracing semantics + runtime boundary leakage"
cargo test -p ui-overlays overlays_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries
echo "[engineering] contract: overlays version deprecation migration N/A gate"
cargo test -p ui-overlays overlays_version_deprecation_migration_is_na_without_major_breaking_upgrade
echo "[engineering] contract: overlays rust hygiene forbids unwrap/expect + swallowed results + string clone churn"
cargo test -p ui-overlays overlays_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources
cargo test -p ui-overlays overlays_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent
cargo test -p ui-overlays overlays_rust_hygiene_script_enforces_repo_level_hygiene_guards
echo "[engineering] contract: modal rust hygiene forbids unwrap/expect + swallowed results + string clone churn"
cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources
cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent
cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_rust_hygiene_script_enforces_repo_level_hygiene_guards

echo "[engineering] contract: drawer serde protocol + tracing semantics + runtime boundary leakage"
cargo test -p ui-components --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_engineering_contract_uses_serde_protocol_and_structured_schema_defaults
cargo test -p ui-components --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_version_deprecation_migration_is_na_without_major_breaking_upgrade
cargo test -p ui-components --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events
cargo test -p ui-components --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_engineering_contract_avoids_runtime_leaks_in_public_api_surface
echo "[engineering] contract: drawer rust hygiene forbids unwrap/expect + swallowed results + string clone churn"
cargo test -p ui-components --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources
cargo test -p ui-components --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent
cargo test -p ui-components --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_rust_hygiene_script_enforces_repo_level_hygiene_guards

echo "[engineering] contract: flip-card serde/spec NA + tracing semantics + runtime boundary leakage"
cargo test -p ui-flip-card flip_card_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope
cargo test -p ui-flip-card flip_card_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events
cargo test -p ui-flip-card flip_card_engineering_contract_avoids_runtime_leaks_in_public_api_surface
cargo test -p ui-flip-card flip_card_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade

echo "[engineering] contract: dialog serde protocol + tracing semantics + runtime boundary leakage"
cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries
cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_version_deprecation_migration_is_na_without_major_breaking_upgrade

echo "[engineering] contract: command-dialog serde/spec NA + tracing semantics + runtime boundary leakage"
cargo test -p ui-components --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope
cargo test -p ui-components --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_version_deprecation_migration_is_na_without_major_breaking_upgrade
cargo test -p ui-components --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events
cargo test -p ui-components --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_engineering_contract_avoids_runtime_leaks_in_public_api_surface

echo "[engineering] contract: command serde protocol + tracing semantics + runtime boundary leakage"
cargo test -p ui-command --lib command_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries
cargo test -p ui-command --lib command_version_deprecation_migration_is_na_without_major_breaking_upgrade
echo "[engineering] contract: command rust hygiene forbids unwrap/expect + swallowed results + string clone churn"
cargo test -p ui-command --lib command_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources
cargo test -p ui-command --lib command_rust_hygiene_string_clone_hotspots_converge_to_cow_or_static_borrow
cargo test -p ui-command --lib command_rust_hygiene_script_enforces_repo_level_hygiene_guards

echo "[engineering] contract: combo-box serde/spec NA + tracing semantics + runtime boundary leakage"
cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope
cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_version_deprecation_migration_is_na_without_major_breaking_upgrade
cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events
cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_engineering_contract_avoids_runtime_leaks_in_public_api_surface

echo "[engineering] contract: autocomplete serde protocol + tracing semantics + runtime boundary leakage"
cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_engineering_contract_uses_serde_protocol_and_structured_schema_defaults
cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events
cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_engineering_contract_avoids_runtime_leaks_in_public_api_surface
cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade

echo "[engineering] contract: drop-zone serde/spec NA + tracing semantics + runtime boundary leakage"
cargo test -p ui-components --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope
cargo test -p ui-components --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_version_deprecation_migration_is_na_without_major_breaking_upgrade
cargo test -p ui-components --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events
cargo test -p ui-components --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_engineering_contract_avoids_runtime_leaks_in_public_api_surface

echo "[engineering] contract: error-view serde protocol + tracing semantics + runtime boundary leakage"
cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries
cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade

echo "[engineering] contract: error-view rust hygiene forbids unwrap/expect + swallowed results + string clone churn"
cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources
cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_rust_hygiene_string_clone_hotspots_converge_to_cow_or_static_borrow
cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_rust_hygiene_script_enforces_repo_level_hygiene_guards

echo "[engineering] contract: chart serde/spec NA + tracing semantics + runtime boundary leakage"
cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope
cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events
cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_engineering_contract_avoids_runtime_leaks_in_public_api_surface

echo "[engineering] contract: chart version deprecation migration registry N/A gate"
cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade

echo "[engineering] contract: chart rust hygiene forbids unwrap/expect + swallowed results + string clone churn"
cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources
cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent
cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_rust_hygiene_script_enforces_repo_level_hygiene_guards

echo "[engineering] contract: color-editor serde/spec NA + tracing semantics + runtime boundary leakage"
cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_engineering_contract_marks_spec_serde_path_as_na_and_keeps_tracing_runtime_boundaries
cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade
cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources
cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_rust_hygiene_string_clone_hotspots_converge_to_cow_or_borrowed_static
cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_rust_hygiene_script_enforces_repo_level_hygiene_guards

echo "[engineering] contract: color-swatch serde/spec NA + tracing semantics + runtime boundary leakage"
cargo test -p ui-components --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_engineering_contract_marks_spec_serde_path_as_na_and_keeps_tracing_runtime_boundaries
cargo test -p ui-components --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade
echo "[engineering] contract: color-swatch rust hygiene forbids unwrap/expect + swallowed results + string clone churn"
cargo test -p ui-components --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources
cargo test -p ui-components --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent
cargo test -p ui-components --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_rust_hygiene_script_enforces_repo_level_hygiene_guards

echo "[engineering] contract: color-thumb serde protocol + tracing semantics + runtime boundary leakage"
cargo test -p ui-components --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries
cargo test -p ui-components --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade
echo "[engineering] contract: color-thumb rust hygiene forbids unwrap/expect + swallowed results + string clone churn"
cargo test -p ui-components --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources
cargo test -p ui-components --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent
cargo test -p ui-components --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_rust_hygiene_script_enforces_repo_level_hygiene_guards

echo "[engineering] contract: color-swatch-picker serde/spec NA + tracing semantics + runtime boundary leakage"
cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_engineering_contract_marks_spec_serde_path_as_na_and_keeps_tracing_runtime_boundaries
cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade
echo "[engineering] contract: color-swatch-picker rust hygiene forbids unwrap/expect + swallowed results + string clone churn"
cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources
cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent
cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_rust_hygiene_script_enforces_repo_level_hygiene_guards

echo "[engineering] contract: color-slider serde protocol + tracing semantics + runtime boundary leakage"
cargo test -p ui-components --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries
cargo test -p ui-components --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade
cargo test -p ui-components --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources
cargo test -p ui-components --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_rust_hygiene_string_clone_hotspots_converge_to_cow_or_borrowed_static
cargo test -p ui-components --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_rust_hygiene_script_enforces_repo_level_hygiene_guards

echo "[engineering] contract: color-wheel serde protocol + tracing semantics + runtime boundary leakage"
cargo test -p ui-components --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries
cargo test -p ui-components --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade

echo "[engineering] contract: color-wheel rust hygiene forbids unwrap/expect + swallowed results + string clone churn"
cargo test -p ui-components --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources
cargo test -p ui-components --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent
cargo test -p ui-components --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_rust_hygiene_script_enforces_repo_level_hygiene_guards

echo "[engineering] contract: color-picker serde protocol + tracing semantics + runtime boundary leakage"
cargo test -p ui-color-picker color_picker_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries
cargo test -p ui-color-picker color_picker_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade

echo "[engineering] contract: color-picker rust hygiene forbids unwrap/expect + swallowed results + string clone churn"
cargo test -p ui-color-picker color_picker_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources
cargo test -p ui-color-picker color_picker_rust_hygiene_string_clone_hotspots_converge_to_cow_or_borrowed_static
cargo test -p ui-color-picker color_picker_rust_hygiene_script_enforces_repo_level_hygiene_guards

echo "[engineering] contract: hover-card serde/spec NA + tracing semantics + runtime boundary leakage"
cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope
cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events
cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_engineering_contract_avoids_runtime_leaks_in_public_api_surface
cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_version_deprecation_migration_is_na_without_major_breaking_upgrade

echo "[engineering] contract: list serde protocol + tracing semantics + runtime boundary leakage"
cargo test -p ui-components --test list_module_semantics --no-default-features --features component-list,inject-css list_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries

echo "[engineering] contract: list version deprecation migration registry N/A gate"
cargo test -p ui-components --test list_module_semantics --no-default-features --features component-list,inject-css list_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade

echo "[engineering] contract: list rust hygiene forbids unwrap/expect + swallowed results + string clone churn"
cargo test -p ui-components --test list_module_semantics --no-default-features --features component-list,inject-css list_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources
cargo test -p ui-components --test list_module_semantics --no-default-features --features component-list,inject-css list_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent
cargo test -p ui-components --test list_module_semantics --no-default-features --features component-list,inject-css list_rust_hygiene_script_enforces_repo_level_hygiene_guards

echo "[engineering] contract: menu serde protocol + tracing semantics + runtime boundary leakage"
cargo test -p ui-menu menu_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries
echo "[engineering] contract: menu rust hygiene forbids unwrap/expect + swallowed results + string clone churn"
cargo test -p ui-menu menu_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources
cargo test -p ui-menu menu_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent
cargo test -p ui-menu menu_rust_hygiene_script_enforces_repo_level_hygiene_guards

echo "[engineering] contract: collapsible serde protocol + tracing semantics + runtime boundary leakage"
cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_engineering_contract_uses_serde_protocol_and_structured_schema_defaults
cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events
cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_engineering_contract_avoids_runtime_leaks_in_public_api_surface
cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_version_deprecation_migration_is_na_without_major_breaking_upgrade

echo "[engineering] contract: carousel serde/spec NA + tracing semantics + runtime boundary leakage"
cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope
cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events
cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_engineering_contract_avoids_runtime_leaks_in_public_api_surface

echo "[engineering] contract: carousel version deprecation migration registry N/A gate"
cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade

echo "[engineering] contract: carousel rust hygiene forbids unwrap/expect + swallowed results + string clone churn"
cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources
cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent
cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_rust_hygiene_script_enforces_repo_level_hygiene_guards

echo "[engineering] OK"
