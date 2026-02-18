#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[contract-hygiene] contract: no temporary patch markers in button paths"
cargo test -p ui-components --test button_semantics button_contract_consistency_has_no_temporary_patch_markers

echo "[contract-hygiene] contract: no unwrap/expect in non-test button+accordion code"
cargo test -p ui-components --test button_accordion_hygiene button_and_accordion_non_test_code_forbids_unwrap_and_expect

echo "[contract-hygiene] contract: no side-effect result swallowing in button+accordion code"
cargo test -p ui-components --test button_accordion_hygiene button_and_accordion_non_test_code_forbids_let_result_swallowing

echo "[contract-hygiene] contract: well agent-contract schema-like markers + whitelist-safe render path"
cargo test -p ui-components --test well_semantics --no-default-features --features component-well,inject-css well_agent_contract_markers_are_schema_like_and_machine_readable
cargo test -p ui-components --test well_semantics --no-default-features --features component-well,inject-css well_agent_contract_render_path_is_whitelist_safe_and_script_injection_free

echo "[contract-hygiene] contract: well semantics tests stay contract-first (data/aria/role/source over snapshot)"
cargo test -p ui-components --test well_semantics --no-default-features --features component-well,inject-css well_check2_documents_semantics_first_testing_rules
cargo test -p ui-components --test well_semantics --no-default-features --features component-well,inject-css well_semantics_suite_is_contract_first_not_snapshot_only
cargo test -p ui-components --test well_semantics --no-default-features --features component-well,inject-css well_semantic_markers_changed_in_view_must_be_covered_by_semantics_tests

echo "[contract-hygiene] contract: tabs agent-contract schema-like markers + whitelist-safe render path"
cargo test -p ui-components --test tabs_semantics tabs_agent_contract_markers_are_schema_like_and_machine_readable
cargo test -p ui-components --test tabs_semantics tabs_agent_contract_render_path_is_whitelist_safe_and_script_injection_free

echo "[contract-hygiene] contract: tabs semantics tests stay contract-first (data/aria/role/source over snapshot)"
cargo test -p ui-components --test tabs_semantics tabs_check2_documents_semantics_first_testing_rules
cargo test -p ui-components --test tabs_semantics tabs_semantics_suite_is_contract_first_not_snapshot_only
cargo test -p ui-components --test tabs_semantics tabs_semantic_markers_changed_in_view_must_be_covered_by_semantics_tests

echo "[contract-hygiene] contract: tag agent-contract schema-like markers + whitelist-safe render path"
cargo test -p ui-components --test tag_semantics --no-default-features --features component-tag,inject-css tag_agent_contract_is_schema_typed_and_machine_readable
cargo test -p ui-components --test tag_semantics --no-default-features --features component-tag,inject-css tag_agent_contract_render_path_is_whitelist_safe_and_script_injection_free
cargo test -p ui-components --test tag_semantics --no-default-features --features component-tag,inject-css tag_check2_documents_semantics_first_testing_rules
cargo test -p ui-components --test tag_semantics --no-default-features --features component-tag,inject-css tag_semantics_suite_is_contract_first_not_snapshot_only
cargo test -p ui-components --test tag_semantics --no-default-features --features component-tag,inject-css tag_semantic_markers_changed_in_view_must_be_covered_by_semantics_tests

echo "[contract-hygiene] contract: tag-group agent-contract schema-like markers + whitelist-safe render path"
cargo test -p ui-components --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_agent_contract_is_schema_typed_and_machine_readable
cargo test -p ui-components --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_agent_contract_render_path_is_whitelist_safe_and_script_injection_free
cargo test -p ui-components --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_check2_documents_semantics_first_testing_rules
cargo test -p ui-components --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_semantics_suite_is_contract_first_not_snapshot_only
cargo test -p ui-components --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_semantic_markers_changed_in_view_must_be_covered_by_semantics_tests

echo "[contract-hygiene] contract: swatch agent-contract schema-like markers + whitelist-safe render path"
cargo test -p ui-components --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_agent_contract_is_schema_typed_and_machine_readable
cargo test -p ui-components --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_agent_contract_render_path_is_whitelist_safe_and_script_injection_free
cargo test -p ui-components --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_check2_documents_semantics_first_testing_rules
cargo test -p ui-components --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_semantics_suite_is_contract_first_not_snapshot_only
cargo test -p ui-components --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_semantic_markers_changed_in_view_must_be_covered_by_semantics_tests

echo "[contract-hygiene] contract: textarea agent-contract schema-like markers + whitelist-safe render path"
cargo test -p ui-components --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_agent_contract_markers_are_schema_like_and_machine_readable
cargo test -p ui-components --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_agent_contract_render_path_is_whitelist_safe_and_script_injection_free

echo "[contract-hygiene] contract: textarea semantics tests stay contract-first (data/aria/role/source over snapshot)"
cargo test -p ui-components --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_check2_documents_semantics_first_testing_rules
cargo test -p ui-components --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_semantics_suite_is_contract_first_not_snapshot_only
cargo test -p ui-components --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_semantic_markers_changed_in_view_must_be_covered_by_semantics_tests
cargo test -p ui-components --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_check2_documents_docs_sync_and_state_matrix_rules
cargo test -p ui-components --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_docs_examples_sync_with_logic_api_names_and_state_matrix
cargo test -p ui-components --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_docs_entry_exists_as_readme_or_equivalent_docs_app_page
cargo test -p ui-components --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_docs_are_beginner_friendly_with_default_then_advanced_path
cargo test -p ui-components --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_docs_hello_world_snippet_is_zero_threshold_and_not_architecture_wiring
cargo test -p ui-components --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_check2_marks_documentation_as_product_complete
cargo test -p ui-components --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_check2_documents_source_first_copy_paste_ready_rules
cargo test -p ui-components --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_docs_are_copy_paste_ready_with_imports_copy_button_and_sync
cargo test -p ui-components --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes
cargo test -p ui-components --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_check2_marks_heroui_strategy_and_component_docs_sync_complete

echo "[contract-hygiene] contract: time-field agent-contract schema-like markers + whitelist-safe render path"
cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_agent_contract_is_schema_typed_and_machine_readable
cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_agent_contract_render_path_is_whitelist_safe_and_script_injection_free
cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_check2_documents_semantics_first_testing_rules
cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_semantics_suite_prioritizes_contract_assertions_over_snapshots
cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_semantic_markers_changed_in_view_must_be_covered_by_semantics_tests
cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_docs_entry_exists_as_readme_or_equivalent_docs_app_page
cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_docs_are_beginner_friendly_with_default_then_advanced_path
cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_docs_hello_world_snippet_is_zero_threshold_and_not_architecture_wiring
cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_check2_marks_documentation_as_product_complete
cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_check2_documents_source_first_copy_paste_ready_rules
cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_docs_are_copy_paste_ready_with_imports_copy_button_and_sync
cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes
cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_check2_marks_heroui_strategy_and_component_docs_sync_complete

echo "[contract-hygiene] contract: slider agent-contract schema-like markers + whitelist-safe render path"
cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_check2_documents_agent_contract_schema_governance_rules
cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_agent_contract_is_schema_typed_and_machine_readable
cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing
cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_agent_contract_render_path_is_whitelist_safe_and_script_injection_free
cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_check2_documents_semantics_first_testing_rules
cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_semantics_suite_is_contract_first_not_snapshot_only
cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_semantic_markers_changed_in_view_must_be_covered_by_semantics_tests
cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_check2_documents_docs_sync_and_state_matrix_rules
cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_docs_examples_sync_with_logic_api_names_and_state_matrix
cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_check2_documents_documentation_as_product_rules
cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_docs_entry_exists_as_readme_or_equivalent_docs_app_page
cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_docs_are_beginner_friendly_with_default_then_advanced_path
cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_docs_hello_world_snippet_is_zero_threshold_and_not_architecture_wiring
cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_check2_documents_source_first_copy_paste_ready_rules
cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_docs_are_copy_paste_ready_with_imports_copy_button_and_sync
cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes
cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_check2_marks_heroui_strategy_and_component_docs_sync_complete
cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_check2_documents_explicit_forbidden_antipattern_rules
cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_forbidden_antipatterns_keep_state_primitives_dom_free_and_headless_visual_free
cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_forbidden_antipatterns_keep_key_state_decisions_out_of_view
cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_forbidden_antipatterns_block_parallel_array_api_and_platform_type_leaks
cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_forbidden_antipatterns_avoid_temporary_patch_drift_and_keep_primitives_sunk
cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_check2_documents_final_merge_gate_rules
cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_final_merge_gate_capabilities_are_backed_by_contract_tests
cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_final_merge_gate_marks_full_repo_gate_as_component_scoped_na

echo "[contract-hygiene] contract: scroll-area agent-contract schema-like markers + whitelist-safe render path"
cargo test -p ui-components --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_check2_documents_agent_contract_schema_governance_rules
cargo test -p ui-components --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_agent_contract_is_schema_typed_and_machine_readable
cargo test -p ui-components --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing
cargo test -p ui-components --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_agent_contract_render_path_is_whitelist_safe_and_script_injection_free
cargo test -p ui-components --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_check2_documents_semantics_first_testing_rules
cargo test -p ui-components --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_semantics_suite_is_contract_first_not_snapshot_only
cargo test -p ui-components --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_semantic_markers_changed_in_view_must_be_covered_by_semantics_tests
cargo test -p ui-components --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_check2_documents_docs_sync_and_state_matrix_rules
cargo test -p ui-components --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_docs_examples_sync_with_logic_api_names_and_state_matrix
cargo test -p ui-components --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_docs_entry_exists_as_readme_or_equivalent_docs_app_page
cargo test -p ui-components --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_docs_are_beginner_friendly_with_default_then_advanced_path
cargo test -p ui-components --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_docs_hello_world_snippet_is_zero_threshold_and_not_architecture_wiring
cargo test -p ui-components --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_check2_marks_documentation_as_product_complete
cargo test -p ui-components --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_check2_documents_source_first_copy_paste_ready_rules
cargo test -p ui-components --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_docs_are_copy_paste_ready_with_imports_copy_button_and_sync
cargo test -p ui-components --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_check2_marks_source_first_copy_paste_ready_complete
cargo test -p ui-components --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes
cargo test -p ui-components --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_check2_marks_heroui_strategy_and_component_docs_sync_complete
cargo test -p ui-components --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_check2_documents_explicit_forbidden_antipattern_rules
cargo test -p ui-components --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_forbidden_antipatterns_keep_state_primitives_dom_free_and_headless_visual_free
cargo test -p ui-components --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_forbidden_antipatterns_keep_key_state_decisions_out_of_view
cargo test -p ui-components --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_forbidden_antipatterns_block_parallel_array_api_and_platform_type_leaks
cargo test -p ui-components --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_forbidden_antipatterns_avoid_temporary_patch_drift_and_keep_primitives_sunk
cargo test -p ui-components --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_check2_documents_final_merge_gate_rules
cargo test -p ui-components --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_final_merge_gate_capabilities_are_backed_by_contract_tests
cargo test -p ui-components --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_final_merge_gate_marks_full_repo_gate_as_deferred_by_requirement
cargo test -p ui-components --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_check2_has_no_unchecked_checklist_items

echo "[contract-hygiene] contract: aspect-ratio check2 evidence + no unchecked items"
cargo test -p ui-components --test aspect_ratio_semantics --no-default-features --features component-aspect_ratio,inject-css aspect_ratio_check2_marks_core_sections_complete
cargo test -p ui-components --test aspect_ratio_semantics --no-default-features --features component-aspect_ratio,inject-css aspect_ratio_check2_has_no_unchecked_checklist_items

echo "[contract-hygiene] contract: divider check2 evidence + no unchecked items"
cargo test -p ui-components --test divider_semantics --no-default-features --features component-divider,inject-css divider_check2_marks_core_sections_complete
cargo test -p ui-components --test divider_semantics --no-default-features --features component-divider,inject-css divider_check2_has_no_unchecked_checklist_items

echo "[contract-hygiene] contract: field-label check2 evidence + no unchecked items"
cargo test -p ui-components --test field_label_semantics --no-default-features --features component-field_label,inject-css field_label_check2_marks_core_sections_complete
cargo test -p ui-components --test field_label_semantics --no-default-features --features component-field_label,inject-css field_label_check2_has_no_unchecked_checklist_items

echo "[contract-hygiene] OK"
