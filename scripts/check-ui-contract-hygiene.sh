#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[contract-hygiene] contract: no temporary patch markers in button paths"
cargo test -p ui --test button_semantics button_contract_consistency_has_no_temporary_patch_markers

echo "[contract-hygiene] contract: button styles keep defensive fallback chain with ui-theme SSOT terminals"
cargo test -p ui --test button_semantics button_styles_use_defensive_variable_fallback_chain_locally

echo "[contract-hygiene] contract: button css stays in @layer ui and runtime styles stay css-variable-only"
cargo test -p ui --test button_semantics button_cascade_layer_and_runtime_style_contract_is_enforced_locally

echo "[contract-hygiene] contract: button agent-contract markers stay schema-typed and whitelist-rendered"
cargo test -p ui --test button_semantics button_agent_contract_schema_markers_are_typed_traceable_and_whitelist_rendered

echo "[contract-hygiene] contract: button check2 pins agent-contract schema governance evidence"
cargo test -p ui --test button_semantics button_check2_marks_agent_contract_schema_item_complete

echo "[contract-hygiene] contract: no unwrap/expect in non-test button+accordion code"
cargo test -p ui --test button_accordion_hygiene button_and_accordion_non_test_code_forbids_unwrap_and_expect

echo "[contract-hygiene] contract: no side-effect result swallowing in button+accordion code"
cargo test -p ui --test button_accordion_hygiene button_and_accordion_non_test_code_forbids_let_result_swallowing

echo "[contract-hygiene] contract: legacy line-height ratios are allowlisted only"
cargo test -p ui --test style_rules --no-default-features --features inject-css text_line_height_legacy_ratios_are_allowlisted_and_no_new_ones

echo "[contract-hygiene] contract: checkbox-field css is aggregated in @layer ui and runtime style is css-variable-only"
cargo test -p ui --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_cascade_layer_and_runtime_style_contract_is_enforced

echo "[contract-hygiene] contract: checkbox-field agent-contract schema-like markers + whitelist-safe render path"
cargo test -p ui --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_check2_documents_agent_contract_schema_governance_rules
cargo test -p ui --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_agent_contract_is_schema_typed_and_machine_readable
cargo test -p ui --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing
cargo test -p ui --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_agent_contract_render_path_is_whitelist_safe_and_script_injection_free

echo "[contract-hygiene] contract: checkbox-group forbids inner_html/untrusted html render paths"
cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_inner_html_usage_is_absent_and_untrusted_html_paths_are_blocked

echo "[contract-hygiene] contract: checkbox-group styles keep defensive fallback chain with ui-theme SSOT terminals"
cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_styles_use_defensive_variable_fallback_chain

echo "[contract-hygiene] contract: checkbox-group css is aggregated in @layer ui and runtime style is css-variable-only"
cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_cascade_layer_and_runtime_style_contract_is_enforced

echo "[contract-hygiene] contract: checkbox-group motion contract is built-in and attached with reduced-motion + non-wasm no-op"
cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop

echo "[contract-hygiene] contract: checkbox-group agent-contract markers are schema-typed, traceable, and whitelist-rendered"
cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_agent_contract_is_schema_typed_traceable_and_whitelist_rendered

echo "[contract-hygiene] contract: checkbox styles keep defensive fallback chain with ui-theme SSOT terminals"
cargo test -p ui --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_styles_use_defensive_variable_fallback_chain_locally

echo "[contract-hygiene] contract: checkbox css is aggregated in @layer ui and runtime style is css-variable-only"
cargo test -p ui --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_cascade_layer_and_runtime_style_contract_is_enforced_locally

echo "[contract-hygiene] contract: checkbox agent-contract schema-like markers + whitelist-safe render path"
cargo test -p ui --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_agent_contract_is_schema_typed_and_machine_readable_locally
cargo test -p ui --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_agent_contract_render_path_is_whitelist_safe_and_script_injection_free_locally

echo "[contract-hygiene] contract: circular-progress styles keep defensive fallback chain with ui-theme SSOT terminals"
cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_styles_use_defensive_variable_fallback_chain

echo "[contract-hygiene] contract: chart styles keep defensive fallback chain with ui-theme SSOT terminals"
cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_styles_use_defensive_variable_fallback_chain

echo "[contract-hygiene] contract: chart agent-contract schema-like markers + whitelist-safe render path"
cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_check2_documents_agent_contract_schema_governance_rules
cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_agent_contract_is_schema_typed_and_machine_readable
cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing
cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_agent_contract_render_path_is_whitelist_safe_and_script_injection_free

echo "[contract-hygiene] contract: carousel styles keep defensive fallback chain with ui-theme SSOT terminals"
cargo test -p ui --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_styles_use_defensive_variable_fallback_chain

echo "[contract-hygiene] contract: carousel agent-contract schema markers stay typed, traceable, and whitelist-rendered"
cargo test -p ui --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_agent_contract_schema_markers_are_typed_traceable_and_whitelist_rendered
cargo test -p ui --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_check2_marks_agent_contract_schema_item_complete

echo "[contract-hygiene] contract: flip-card styles keep defensive fallback chain with ui-theme SSOT terminals"
cargo test -p ui-flip-card flip_card_styles_use_defensive_variable_fallback_chain

echo "[contract-hygiene] contract: flip-card css is aggregated in @layer ui and runtime style is css-variable-only"
cargo test -p ui-flip-card flip_card_cascade_layer_and_runtime_style_contract_is_enforced

echo "[contract-hygiene] contract: flip-card motion contract is built-in and safely attached across reduced-motion + non-wasm"
cargo test -p ui-flip-card flip_card_motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop

echo "[contract-hygiene] contract: flip-card rust hygiene forbids unwrap/expect + let _= and converges class-name hotspots to Cow"
cargo test -p ui-flip-card flip_card_rust_hygiene_contract_is_enforced_for_non_test_sources

echo "[contract-hygiene] contract: flip-card agent-contract schema-like markers + whitelist-safe render path"
cargo test -p ui-flip-card flip_card_check2_documents_agent_contract_schema_governance_rules
cargo test -p ui-flip-card flip_card_agent_contract_is_schema_typed_and_machine_readable
cargo test -p ui-flip-card flip_card_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing
cargo test -p ui-flip-card flip_card_agent_contract_render_path_is_whitelist_safe_and_script_injection_free

echo "[contract-hygiene] contract: flip-card semantics priority asserts role/aria/data-source before snapshots"
cargo test -p ui-flip-card flip_card_check2_documents_semantics_first_testing_rules
cargo test -p ui-flip-card flip_card_semantics_suite_is_contract_first_not_snapshot_only
cargo test -p ui-flip-card flip_card_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks

echo "[contract-hygiene] contract: flip-card documentation-as-product stays beginner-friendly with README/docs entry"
cargo test -p ui-flip-card flip_card_check2_documents_documentation_as_product_rules
cargo test -p ui-flip-card flip_card_docs_entry_exists_as_readme_or_equivalent_docs_app_page
cargo test -p ui-flip-card flip_card_docs_are_beginner_friendly_with_default_then_advanced_path
cargo test -p ui-flip-card flip_card_docs_hello_world_snippet_is_zero_threshold_and_not_architecture_wiring

echo "[contract-hygiene] contract: fieldset styles keep defensive fallback chain with ui-theme SSOT terminals"
cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_styles_use_defensive_variable_fallback_chain

echo "[contract-hygiene] contract: form-field styles keep defensive fallback chain with ui-theme SSOT terminals"
cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals

echo "[contract-hygiene] contract: date-input-group styles keep defensive fallback chain with ui-theme SSOT terminals"
cargo test -p ui-date-input-group date_input_group_styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals

echo "[contract-hygiene] contract: date-input-group css is aggregated in @layer ui and runtime style is css-variable-only"
cargo test -p ui-date-input-group date_input_group_cascade_layer_contract_uses_ui_layer_and_css_variable_only_runtime_updates

echo "[contract-hygiene] contract: date-input-group agent-contract schema markers are type-derived and machine readable"
cargo test -p ui-date-input-group date_input_group_agent_contract_is_schema_typed_and_machine_readable

echo "[contract-hygiene] contract: date-input-group agent-contract render path stays whitelist-safe"
cargo test -p ui-date-input-group date_input_group_agent_contract_render_path_is_whitelist_safe_and_script_injection_free

echo "[contract-hygiene] contract: date-input-group streaming term stays limited to llm output render modes"
cargo test -p ui-date-input-group date_input_group_streaming_term_is_limited_to_llm_output_render_modes

echo "[contract-hygiene] contract: date-input-group snapshot output is foundational and stable for complete config"
cargo test -p ui-date-input-group date_input_group_snapshot_is_foundational_and_complete_config_renders_stably

echo "[contract-hygiene] contract: date-input-group streaming is optional with snapshot fallback and explicit output status"
cargo test -p ui-date-input-group date_input_group_streaming_requirement_is_optional_with_snapshot_fallback_and_explicit_status

echo "[contract-hygiene] contract: menu agent-contract schema-like markers are type-derived and machine readable"
cargo test -p ui-menu menu_check2_documents_agent_contract_schema_governance_rules
cargo test -p ui-menu menu_agent_contract_is_schema_typed_and_machine_readable
cargo test -p ui-menu menu_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing
cargo test -p ui-menu menu_agent_contract_render_path_is_whitelist_safe_and_script_injection_free
cargo test -p ui-menu menu_streaming_term_is_limited_to_llm_output_render_modes
echo "[contract-hygiene] contract: menu snapshot output is foundational and stable for complete config"
cargo test -p ui-menu menu_snapshot_is_foundational_and_complete_config_renders_stably
echo "[contract-hygiene] contract: menu streaming is optional with snapshot fallback and explicit output status"
cargo test -p ui-menu menu_streaming_requirement_is_optional_with_snapshot_fallback_and_explicit_status

echo "[contract-hygiene] contract: form-field css is aggregated in @layer ui and runtime style is css-variable-only"
cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_cascade_layer_and_runtime_style_contract_is_enforced

echo "[contract-hygiene] contract: form-field agent-contract schema-like markers + whitelist-safe render path"
cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_check2_documents_agent_contract_schema_governance_rules
cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_agent_contract_is_schema_typed_and_machine_readable
cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing
cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_agent_contract_render_path_is_whitelist_safe_and_script_injection_free

echo "[contract-hygiene] contract: fieldset css is aggregated in @layer ui and runtime style is css-variable-only"
cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_cascade_layer_and_runtime_style_contract_is_enforced

echo "[contract-hygiene] contract: fieldset agent-contract schema-like markers + whitelist-safe render path"
cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_check2_documents_agent_contract_schema_governance_rules
cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_agent_contract_is_schema_typed_and_machine_readable
cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing
cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_agent_contract_render_path_is_whitelist_safe_and_script_injection_free

echo "[contract-hygiene] contract: color-editor styles keep defensive fallback chain with ui-theme SSOT terminals"
cargo test -p ui --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_styles_use_defensive_variable_fallback_chain

echo "[contract-hygiene] contract: color-slider styles keep defensive fallback chain with ui-theme SSOT terminals"
cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_styles_use_defensive_variable_fallback_chain

echo "[contract-hygiene] contract: color-swatch styles keep defensive fallback chain with ui-theme SSOT terminals"
cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_styles_use_defensive_variable_fallback_chain

echo "[contract-hygiene] contract: color-thumb styles keep defensive fallback chain with ui-theme SSOT terminals"
cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_styles_use_defensive_variable_fallback_chain

echo "[contract-hygiene] contract: color-thumb css is aggregated in @layer ui and runtime style is css-variable-only"
cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_cascade_layer_and_runtime_style_contract_is_enforced

echo "[contract-hygiene] contract: color-thumb motion contract is built-in and safely attached across reduced-motion + non-wasm"
cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop

echo "[contract-hygiene] contract: color-thumb agent-contract schema-like markers + whitelist-safe render path"
cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_agent_contract_is_schema_typed_and_machine_readable
cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_agent_contract_render_path_is_whitelist_safe_and_script_injection_free

echo "[contract-hygiene] contract: color-thumb semantics priority asserts role/aria/data-source before snapshots"
cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_semantics_priority_contract_prefers_semantic_assertions_over_snapshot_only

echo "[contract-hygiene] contract: color-thumb documentation-as-product stays beginner-friendly with README/docs entry"
cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_check2_documents_documentation_as_product_rules
cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_docs_entry_exists_as_readme_or_equivalent_docs_app_page
cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_docs_are_beginner_friendly_with_default_then_advanced_path
cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_docs_hello_world_snippet_is_zero_threshold_and_not_architecture_wiring

echo "[contract-hygiene] contract: color-swatch-picker styles keep defensive fallback chain with ui-theme SSOT terminals"
cargo test -p ui --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_styles_use_defensive_variable_fallback_chain

echo "[contract-hygiene] contract: color-swatch-picker css is aggregated in @layer ui and runtime style is css-variable-only"
cargo test -p ui --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_cascade_layer_and_runtime_style_contract_is_enforced

echo "[contract-hygiene] contract: color-swatch-picker agent-contract schema-like markers + whitelist-safe render path"
cargo test -p ui --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_agent_contract_is_schema_typed_and_machine_readable
cargo test -p ui --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_agent_contract_render_path_is_whitelist_safe_and_script_injection_free

echo "[contract-hygiene] contract: color-swatch-picker docs product copy-paste-ready playground matrix"
cargo test -p ui --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_docs_page_covers_primary_playgrounds
cargo test -p ui --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_docs_playgrounds_lock_state_matrix_contract_values
cargo test -p ui --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_docs_are_copy_paste_ready_with_imports_copy_button_and_sync
cargo test -p ui --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_check2_documents_docs_product_copy_paste_ready_contract

echo "[contract-hygiene] contract: color-swatch-picker source-first docs stay copy-paste-ready with import/dependency sync"
cargo test -p ui --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_check2_documents_source_first_copy_paste_ready_rules
cargo test -p ui --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies
cargo test -p ui --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_contract_hygiene_script_covers_source_first_copy_paste_ready_contract

echo "[contract-hygiene] contract: color-swatch-picker HeroUI strategy docs and component docs entry stay synchronized"
cargo test -p ui --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_check2_documents_heroui_benchmark_docs_sync_rules
cargo test -p ui --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_heroui_strategy_and_component_docs_are_synchronized_and_indexable
cargo test -p ui --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_contract_hygiene_script_covers_heroui_benchmark_docs_sync_contract
cargo test -p ui --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_check2_marks_heroui_benchmark_docs_sync_contract_complete

echo "[contract-hygiene] contract: color-swatch-picker documentation-as-product stays beginner-friendly with README/docs entry"
cargo test -p ui --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_check2_documents_documentation_as_product_rules
cargo test -p ui --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_docs_entry_exists_as_readme_or_equivalent_docs_app_page
cargo test -p ui --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_docs_are_beginner_friendly_with_default_then_advanced_path
cargo test -p ui --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_docs_hello_world_snippet_is_zero_threshold_and_not_architecture_wiring

echo "[contract-hygiene] contract: color-swatch-picker docs interactive playground supports live props/state replay and explicit non-spec N/A"
cargo test -p ui --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_check2_documents_interactive_playground_rules
cargo test -p ui --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_docs_interactive_playground_supports_live_props_state_and_feedback_preview
cargo test -p ui --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_docs_interactive_playground_replay_path_is_explicit_and_repeatable
cargo test -p ui --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_docs_interactive_playground_spec_linkage_is_not_applicable_for_non_spec_component

echo "[contract-hygiene] contract: color-swatch-picker docs examples/state matrix remain synced with logic API names and defaults"
cargo test -p ui --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_check2_documents_docs_sync_and_state_matrix_rules
cargo test -p ui --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_docs_examples_parameter_and_state_matrix_stay_synced_with_logic_defaults

echo "[contract-hygiene] contract: color-swatch-picker semantics priority asserts role/aria/data-source before snapshots"
cargo test -p ui --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_semantics_priority_contract_prefers_semantic_assertions_over_snapshot_only

echo "[contract-hygiene] contract: color-swatch-picker explicit forbidden anti-patterns and final merge gate"
cargo test -p ui --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_check2_documents_explicit_forbidden_antipattern_rules
cargo test -p ui --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_forbidden_antipatterns_keep_architecture_boundaries_intact
cargo test -p ui --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_check2_documents_final_merge_gate_rules
cargo test -p ui --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_final_merge_gate_capabilities_are_backed_by_contract_checks
cargo test -p ui --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_check2_has_no_unchecked_checklist_items

echo "[contract-hygiene] contract: color-swatch css is aggregated in @layer ui and runtime style is css-variable-only"
cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_cascade_layer_and_runtime_style_contract_is_enforced

echo "[contract-hygiene] contract: color-swatch agent-contract schema-like markers + whitelist-safe render path"
cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_agent_contract_is_schema_typed_and_machine_readable
cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_agent_contract_render_path_is_whitelist_safe_and_script_injection_free

echo "[contract-hygiene] contract: color-swatch semantics priority asserts role/aria/data-source before snapshots"
cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_semantics_priority_contract_prefers_semantic_assertions_over_snapshot_only

echo "[contract-hygiene] contract: color-slider css is aggregated in @layer ui and runtime style is css-variable-only"
cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_cascade_layer_and_runtime_style_contract_is_enforced

echo "[contract-hygiene] contract: color-slider agent-contract schema-like markers + whitelist-safe render path"
cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_agent_contract_is_schema_typed_and_machine_readable
cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_agent_contract_render_path_is_whitelist_safe_and_script_injection_free

echo "[contract-hygiene] contract: color-wheel agent-contract schema-like markers + whitelist-safe render path"
cargo test -p ui --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_agent_contract_is_schema_typed_and_machine_readable
cargo test -p ui --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_agent_contract_render_path_is_whitelist_safe_and_script_injection_free

echo "[contract-hygiene] contract: color-wheel semantics priority asserts role/aria/data-source before snapshots"
cargo test -p ui --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_check2_documents_semantics_first_testing_rules

echo "[contract-hygiene] contract: color-slider docs product copy-paste-ready playground matrix"
cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_docs_page_covers_primary_playgrounds
cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_docs_playgrounds_lock_state_matrix_contract_values
cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_check2_documents_docs_product_copy_paste_ready_contract

echo "[contract-hygiene] contract: color-slider docs/examples/api-defaults stay synced with state-matrix coverage"
cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_check2_documents_docs_sync_and_state_matrix_rules
cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_docs_examples_parameter_and_state_matrix_stay_synced_with_logic_defaults

echo "[contract-hygiene] contract: color-slider source-first docs stay copy-paste-ready with import/dependency sync"
cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_check2_documents_source_first_copy_paste_ready_rules
cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_docs_are_copy_paste_ready_with_imports_copy_button_and_sync

echo "[contract-hygiene] contract: color-slider documentation-as-product stays beginner-friendly with README/docs entry"
cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_check2_documents_documentation_as_product_rules
cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_docs_entry_exists_as_readme_or_equivalent_docs_app_page
cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_docs_are_beginner_friendly_with_default_then_advanced_path
cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_docs_hello_world_snippet_is_zero_threshold_and_not_architecture_wiring

echo "[contract-hygiene] contract: color-slider HeroUI strategy and docs entry stay synced for parameter model changes"
cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes
cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_check2_marks_heroui_strategy_and_component_docs_sync_complete

echo "[contract-hygiene] contract: color-slider semantics priority asserts role/aria/data-source before snapshots"
cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_semantics_priority_contract_prefers_semantic_assertions_over_snapshot_only

echo "[contract-hygiene] contract: color-picker styles keep defensive fallback chain with ui-theme SSOT terminals"
cargo test -p ui-color-picker color_picker_styles_use_defensive_variable_fallback_chain

echo "[contract-hygiene] contract: color-picker documentation-as-product stays beginner-friendly with README/docs entry"
cargo test -p ui-color-picker color_picker_check2_documents_documentation_as_product_rules
cargo test -p ui-color-picker color_picker_docs_entry_exists_as_readme_or_equivalent_docs_app_page
cargo test -p ui-color-picker color_picker_docs_are_beginner_friendly_with_default_then_advanced_path
cargo test -p ui-color-picker color_picker_docs_hello_world_snippet_is_zero_threshold_and_not_architecture_wiring

echo "[contract-hygiene] contract: color-picker docs product copy-paste-ready playground matrix"
cargo test -p ui-color-picker color_picker_docs_product_contract_is_copy_paste_ready_with_playground_stream_snapshot_and_imports

echo "[contract-hygiene] contract: color-picker source-first docs stay copy-paste-ready with import/dependency sync"
cargo test -p ui-color-picker color_picker_check2_documents_source_first_copy_paste_ready_rules
cargo test -p ui-color-picker color_picker_docs_are_copy_paste_ready_with_imports_copy_button_and_sync

echo "[contract-hygiene] contract: color-picker HeroUI strategy and docs entry stay synced for parameter model changes"
cargo test -p ui-color-picker color_picker_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes
cargo test -p ui-color-picker color_picker_check2_marks_heroui_strategy_and_component_docs_sync_complete

echo "[contract-hygiene] contract: color-picker docs/examples/api-defaults stay synced with state-matrix coverage"
cargo test -p ui-color-picker color_picker_check2_documents_docs_sync_and_state_matrix_rules
cargo test -p ui-color-picker color_picker_docs_examples_parameter_and_state_matrix_stay_synced_with_logic_defaults

echo "[contract-hygiene] contract: color-picker semantics priority asserts role/aria/data-source before snapshots"
cargo test -p ui-color-picker color_picker_semantics_priority_contract_prefers_semantic_assertions_over_snapshot_only

echo "[contract-hygiene] contract: color-picker css stays in @layer ui and runtime style stays css-variable-only"
cargo test -p ui-color-picker color_picker_cascade_layer_and_runtime_style_contract_is_enforced

echo "[contract-hygiene] contract: color-picker agent-contract schema-like markers + whitelist-safe render path"
cargo test -p ui-color-picker color_picker_agent_contract_is_schema_typed_and_machine_readable
cargo test -p ui-color-picker color_picker_agent_contract_render_path_is_whitelist_safe_and_script_injection_free

echo "[contract-hygiene] contract: color-editor css is aggregated in @layer ui and runtime style is css-variable-only"
cargo test -p ui --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_cascade_layer_and_runtime_style_contract_is_enforced

echo "[contract-hygiene] contract: color-editor agent-contract schema-like markers + whitelist-safe render path"
cargo test -p ui --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_agent_contract_is_schema_typed_and_machine_readable
cargo test -p ui --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_agent_contract_render_path_is_whitelist_safe_and_script_injection_free

echo "[contract-hygiene] contract: color-editor HeroUI strategy and docs entry stay synced for parameter model changes"
cargo test -p ui --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes
cargo test -p ui --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_check2_marks_heroui_strategy_and_component_docs_sync_complete

echo "[contract-hygiene] contract: color-editor docs product copy-paste-ready playground matrix"
cargo test -p ui --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_docs_page_covers_primary_playgrounds
cargo test -p ui --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_docs_playgrounds_lock_state_matrix_contract_values
cargo test -p ui --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_docs_examples_parameter_and_state_matrix_stay_synced_with_logic_defaults
cargo test -p ui --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_check2_documents_documentation_as_product_rules
cargo test -p ui --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_docs_entry_exists_as_readme_or_equivalent_docs_app_page
cargo test -p ui --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_docs_are_beginner_friendly_with_default_then_advanced_path
cargo test -p ui --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_check2_documents_source_first_copy_paste_ready_rules
cargo test -p ui --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_docs_are_copy_paste_ready_with_imports_copy_button_and_sync
cargo test -p ui --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_check2_documents_docs_product_copy_paste_ready_contract
cargo test -p ui --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_contract_hygiene_script_covers_source_first_copy_paste_ready_contract
cargo test -p ui --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_semantics_priority_contract_prefers_semantic_assertions_over_snapshot_only

echo "[contract-hygiene] contract: combo-box styles keep defensive fallback chain with ui-theme SSOT terminals"
cargo test -p ui --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_styles_use_defensive_variable_fallback_chain

echo "[contract-hygiene] contract: autocomplete styles keep defensive fallback chain with ui-theme SSOT terminals"
cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_styles_use_defensive_variable_fallback_chain

echo "[contract-hygiene] contract: autocomplete css is aggregated in @layer ui and runtime style is css-variable-only"
cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_cascade_layer_and_runtime_style_contract_is_enforced

echo "[contract-hygiene] contract: autocomplete motion contract is built-in and safely attached across reduced-motion + non-wasm"
cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop

echo "[contract-hygiene] contract: autocomplete ui fixed-entry files stay correctly located and scoped"
cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_ui_components_fixed_entry_files_are_correctly_located_and_scoped

echo "[contract-hygiene] contract: autocomplete agent-contract schema-like markers + whitelist-safe render path"
cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_agent_contract_is_schema_typed_and_machine_readable
cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing
cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_agent_contract_render_path_is_whitelist_safe_and_script_injection_free

echo "[contract-hygiene] contract: autocomplete streaming term is limited to llm output render modes"
cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_streaming_term_is_limited_to_llm_output_render_modes

echo "[contract-hygiene] contract: autocomplete snapshot rendering is foundational and complete-config stable"
cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_snapshot_is_foundational_and_complete_config_renders_stably

echo "[contract-hygiene] contract: autocomplete streaming is optional with snapshot fallback and explicit output status"
cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_streaming_requirement_is_optional_with_snapshot_fallback_and_explicit_status

echo "[contract-hygiene] contract: autocomplete rust hygiene forbids unwrap/expect + let _= in non-test code"
cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_rust_hygiene_forbids_unwrap_expect_and_let_result_swallowing_in_non_test_sources

echo "[contract-hygiene] contract: autocomplete string clone hotspots converge to Cow<'static, str>"
cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_string_clone_hotspots_converge_to_cow_static_str_for_class_tokens

echo "[contract-hygiene] contract: autocomplete check2 keeps rust-hygiene completion evidence pinned"
cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_check2_marks_rust_hygiene_item_complete_with_component_scope

echo "[contract-hygiene] contract: drop-zone styles keep defensive fallback chain with ui-theme SSOT terminals"
cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_styles_use_defensive_variable_fallback_chain

echo "[contract-hygiene] contract: drop-zone css is aggregated in @layer ui and runtime style is css-variable-only"
cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_cascade_layer_and_runtime_style_contract_is_enforced

echo "[contract-hygiene] contract: drop-zone motion contract is built-in and safely attached across reduced-motion + non-wasm"
cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop

echo "[contract-hygiene] contract: drop-zone agent-contract schema-like markers + whitelist-safe render path"
cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_check2_documents_agent_contract_schema_governance_rules
cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_agent_contract_is_schema_typed_and_machine_readable
cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing
cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_agent_contract_render_path_is_whitelist_safe_and_script_injection_free

echo "[contract-hygiene] contract: combo-box css is aggregated in @layer ui and runtime style is css-variable-only"
cargo test -p ui --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_cascade_layer_and_runtime_style_contract_is_enforced

echo "[contract-hygiene] contract: combo-box agent-contract schema-like markers + whitelist-safe render path"
cargo test -p ui --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_check2_documents_agent_contract_schema_governance_rules
cargo test -p ui --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_agent_contract_is_schema_typed_and_machine_readable
cargo test -p ui --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_agent_contract_render_path_is_whitelist_safe_and_script_injection_free

echo "[contract-hygiene] contract: combo-box HeroUI strategy and docs entry stay synced for parameter model changes"
cargo test -p ui --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes
cargo test -p ui --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_check2_marks_heroui_strategy_and_component_docs_sync_complete

echo "[contract-hygiene] contract: chart css is aggregated in @layer ui and runtime style is css-variable-only"
cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_cascade_layer_and_runtime_style_contract_is_enforced

echo "[contract-hygiene] contract: carousel css is aggregated in @layer ui and runtime style is css-variable-only"
cargo test -p ui --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_cascade_layer_and_runtime_style_contract_is_enforced

echo "[contract-hygiene] contract: command styles keep defensive fallback chain with ui-theme SSOT terminals"
cargo test -p ui-command --lib command_styles_use_defensive_variable_fallback_chain

echo "[contract-hygiene] contract: command css is aggregated in @layer ui and runtime style is css-variable-only"
cargo test -p ui-command --lib command_cascade_layer_and_runtime_style_contract_is_enforced

echo "[contract-hygiene] contract: command agent-contract schema-like markers + whitelist-safe render path"
cargo test -p ui-command --lib command_check2_documents_agent_contract_schema_governance_rules
cargo test -p ui-command --lib command_agent_contract_is_schema_typed_and_machine_readable
cargo test -p ui-command --lib command_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing
cargo test -p ui-command --lib command_agent_contract_render_path_is_whitelist_safe_and_script_injection_free

echo "[contract-hygiene] contract: command semantics tests stay contract-first (data/aria/role/source over snapshot)"
cargo test -p ui-command --lib command_check2_documents_semantics_first_testing_rules
cargo test -p ui-command --lib command_semantics_suite_is_contract_first_not_snapshot_only
cargo test -p ui-command --lib command_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks

echo "[contract-hygiene] contract: command-dialog styles keep defensive fallback chain with ui-theme SSOT terminals"
cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_styles_use_defensive_variable_fallback_chain

echo "[contract-hygiene] contract: error-view styles keep defensive fallback chain with ui-theme SSOT terminals"
cargo test -p ui --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_styles_use_defensive_variable_fallback_chain

echo "[contract-hygiene] contract: error-view css is aggregated in @layer ui and runtime style is css-variable-only"
cargo test -p ui --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_cascade_layer_and_runtime_style_contract_is_enforced

echo "[contract-hygiene] contract: error-view agent-contract schema-like markers + whitelist-safe render path"
cargo test -p ui --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_check2_documents_agent_contract_schema_governance_rules
cargo test -p ui --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_agent_contract_is_schema_typed_and_machine_readable
cargo test -p ui --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing
cargo test -p ui --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_agent_contract_render_path_is_whitelist_safe_and_script_injection_free

echo "[contract-hygiene] contract: command-dialog css is aggregated in @layer ui and runtime style is css-variable-only"
cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_cascade_layer_and_runtime_style_contract_is_enforced

echo "[contract-hygiene] contract: command-dialog agent-contract schema-like markers + whitelist-safe render path"
cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_check2_documents_agent_contract_schema_governance_rules
cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_agent_contract_is_schema_typed_and_machine_readable
cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing
cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_agent_contract_render_path_is_whitelist_safe_and_script_injection_free

echo "[contract-hygiene] contract: command-dialog HeroUI strategy and docs entry stay synced for parameter model changes"
cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes
cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_check2_marks_heroui_strategy_and_component_docs_sync_complete

echo "[contract-hygiene] contract: command-dialog semantics tests stay contract-first (data/aria/role/source over snapshot)"
cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_check2_documents_semantics_first_testing_rules
cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_semantics_suite_is_contract_first_not_snapshot_only
cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks

echo "[contract-hygiene] contract: alert-dialog styles keep defensive fallback chain with ui-theme SSOT terminals"
cargo test -p ui --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_styles_use_defensive_variable_fallback_chain

echo "[contract-hygiene] contract: alert-dialog agent-contract schema-like markers + whitelist-safe render path"
cargo test -p ui --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_check2_documents_agent_contract_schema_governance_rules
cargo test -p ui --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_agent_contract_is_schema_typed_and_machine_readable
cargo test -p ui --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing
cargo test -p ui --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_agent_contract_render_path_is_whitelist_safe_and_script_injection_free

echo "[contract-hygiene] contract: modal styles keep defensive fallback chain with ui-theme SSOT terminals"
cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_styles_use_defensive_variable_fallback_chain

echo "[contract-hygiene] contract: hover-card styles keep defensive fallback chain with ui-theme SSOT terminals"
cargo test -p ui --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_styles_use_defensive_variable_fallback_chain

echo "[contract-hygiene] contract: hover-card css is aggregated in @layer ui and runtime style is css-variable-only"
cargo test -p ui --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_cascade_layer_and_runtime_style_contract_is_enforced

echo "[contract-hygiene] contract: hover-card agent-contract schema-like markers + whitelist-safe render path"
cargo test -p ui --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_check2_documents_agent_contract_schema_governance_rules
cargo test -p ui --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_agent_contract_is_schema_typed_and_machine_readable
cargo test -p ui --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing
cargo test -p ui --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_agent_contract_render_path_is_whitelist_safe_and_script_injection_free

echo "[contract-hygiene] contract: hover-card semantics tests stay contract-first (data/aria/role/source over snapshot)"
cargo test -p ui --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_check2_documents_semantics_first_testing_rules
cargo test -p ui --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_semantics_suite_is_contract_first_not_snapshot_only
cargo test -p ui --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks

echo "[contract-hygiene] contract: drawer styles keep defensive fallback chain with ui-theme SSOT terminals"
cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals

echo "[contract-hygiene] contract: drawer css is aggregated in @layer ui and runtime style is css-variable-only"
cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_cascade_layer_and_runtime_style_contract_is_enforced

echo "[contract-hygiene] contract: drawer motion contract is built-in and safely attached across reduced-motion + non-wasm"
cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop

echo "[contract-hygiene] contract: drawer agent-contract schema-like markers + whitelist-safe render path"
cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_check2_documents_agent_contract_schema_governance_rules
cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_agent_contract_is_schema_typed_and_machine_readable
cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing
cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_agent_contract_render_path_is_whitelist_safe_and_script_injection_free

echo "[contract-hygiene] contract: dialog styles keep defensive fallback chain with ui-theme SSOT terminals"
cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_styles_use_defensive_variable_fallback_chain

echo "[contract-hygiene] contract: dialog css is aggregated in @layer ui and runtime style is css-variable-only"
cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_cascade_layer_and_runtime_style_contract_is_enforced

echo "[contract-hygiene] contract: dialog agent-contract schema-like markers + whitelist-safe render path"
cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_check2_documents_agent_contract_schema_governance_rules
cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_agent_contract_is_schema_typed_and_machine_readable
cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing
cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_agent_contract_render_path_is_whitelist_safe_and_script_injection_free

echo "[contract-hygiene] contract: dialog HeroUI strategy and docs entry stay synced for parameter model changes"
cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes
cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_check2_marks_heroui_strategy_and_component_docs_sync_complete

echo "[contract-hygiene] contract: dialog semantics tests stay contract-first (data/aria/role/source over snapshot)"
cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_check2_documents_semantics_first_testing_rules
cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_semantics_suite_is_contract_first_not_snapshot_only
cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks

echo "[contract-hygiene] contract: modal css is aggregated in @layer ui and runtime style is css-variable-only"
cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_cascade_layer_and_runtime_style_contract_is_enforced

echo "[contract-hygiene] contract: modal motion contract is built-in and safely attached across reduced-motion + non-wasm"
cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop

echo "[contract-hygiene] contract: modal agent-contract schema-like markers + whitelist-safe render path"
cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_check2_documents_agent_contract_schema_governance_rules
cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_agent_contract_is_schema_typed_and_machine_readable
cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing
cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_agent_contract_render_path_is_whitelist_safe_and_script_injection_free

echo "[contract-hygiene] contract: alert-dialog css is aggregated in @layer ui and runtime style is css-variable-only"
cargo test -p ui --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_cascade_layer_and_runtime_style_contract_is_enforced

echo "[contract-hygiene] contract: bottom-sheet styles keep defensive fallback chain with ui-theme SSOT terminals"
cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_styles_use_defensive_variable_fallback_chain

echo "[contract-hygiene] contract: bottom-sheet css is aggregated in @layer ui and runtime style is css-variable-only"
cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_cascade_layer_and_runtime_style_contract_is_enforced

echo "[contract-hygiene] contract: bottom-sheet agent-contract schema-like markers + whitelist-safe render path"
cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_check2_documents_agent_contract_schema_governance_rules
cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_agent_contract_is_schema_typed_and_machine_readable
cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing
cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_agent_contract_render_path_is_whitelist_safe_and_script_injection_free

echo "[contract-hygiene] contract: coachmark styles keep defensive fallback chain with ui-theme SSOT terminals"
cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals

echo "[contract-hygiene] contract: collapsible styles keep defensive fallback chain with ui-theme SSOT terminals"
cargo test -p ui --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_styles_use_defensive_variable_fallback_chain

echo "[contract-hygiene] contract: collapsible css is aggregated in @layer ui and runtime style is css-variable-only"
cargo test -p ui --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_cascade_layer_and_runtime_style_contract_is_enforced

echo "[contract-hygiene] contract: collapsible motion contract is built-in and safely attached across reduced-motion + non-wasm"
cargo test -p ui --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop

echo "[contract-hygiene] contract: collapsible ui fixed entry files stay in canonical boundaries"
cargo test -p ui --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_ui_components_fixed_entry_files_follow_contract

echo "[contract-hygiene] contract: collapsible component directory keeps standard file layout boundaries"
cargo test -p ui --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_component_directory_standard_files_stay_in_canonical_layout

echo "[contract-hygiene] contract: collapsible file layout discipline keeps canonical component directory"
cargo test -p ui --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_file_layout_discipline_keeps_canonical_component_directory

echo "[contract-hygiene] contract: collapsible hyper-structure builder spec.rs stays explicit N/A for simple scope"
cargo test -p ui --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_hyper_structure_builder_spec_rs_is_not_applicable_for_simple_component

echo "[contract-hygiene] contract: collapsible context-compression manifest + rbi projection stay present and current"
cargo test -p ui --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_context_compression_manifest_and_rbi_projection_are_present_and_current

echo "[contract-hygiene] contract: collapsible agent-contract schema-like markers + whitelist-safe render path"
cargo test -p ui --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_check2_documents_agent_contract_schema_governance_rules
cargo test -p ui --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_agent_contract_is_schema_typed_and_machine_readable
cargo test -p ui --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing
cargo test -p ui --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_agent_contract_render_path_is_whitelist_safe_and_script_injection_free
cargo test -p ui --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_streaming_term_is_limited_to_llm_output_render_modes
cargo test -p ui --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_snapshot_is_foundational_and_complete_config_renders_stably
cargo test -p ui --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_streaming_requirement_is_optional_with_snapshot_fallback_and_explicit_status

echo "[contract-hygiene] contract: collapsible semantics tests stay contract-first (data/aria/role/source over snapshot)"
cargo test -p ui --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_check2_documents_semantics_first_testing_rules
cargo test -p ui --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_semantics_suite_is_contract_first_not_snapshot_only
cargo test -p ui --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks
cargo test -p ui --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_semantics_first_testing_script_covers_contract

echo "[contract-hygiene] contract: coachmark css is aggregated in @layer ui and runtime style is css-variable-only"
cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_cascade_layer_and_runtime_style_contract_is_enforced

echo "[contract-hygiene] contract: coachmark agent-contract schema-like markers + whitelist-safe render path"
cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_agent_contract_is_schema_typed_and_machine_readable_locally
cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_agent_contract_render_path_is_whitelist_safe_and_script_injection_free_locally

echo "[contract-hygiene] contract: coachmark semantics tests stay contract-first (data/aria/role/source over snapshot)"
cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_check2_documents_semantics_first_testing_rules
cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_semantics_suite_is_contract_first_not_snapshot_only
cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks
cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_semantics_first_testing_script_covers_contract

echo "[contract-hygiene] contract: circular-progress css is aggregated in @layer ui and runtime style is css-variable-only"
cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_cascade_layer_and_runtime_style_contract_is_enforced

echo "[contract-hygiene] contract: well agent-contract schema-like markers + whitelist-safe render path"
cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_agent_contract_markers_are_schema_like_and_machine_readable
cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_agent_contract_render_path_is_whitelist_safe_and_script_injection_free

echo "[contract-hygiene] contract: well semantics tests stay contract-first (data/aria/role/source over snapshot)"
cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_check2_documents_semantics_first_testing_rules
cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_semantics_suite_is_contract_first_not_snapshot_only
cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks

echo "[contract-hygiene] contract: tabs agent-contract schema-like markers + whitelist-safe render path"
cargo test -p ui --test tabs_semantics tabs_agent_contract_markers_are_schema_like_and_machine_readable
cargo test -p ui --test tabs_semantics tabs_agent_contract_render_path_is_whitelist_safe_and_script_injection_free

echo "[contract-hygiene] contract: tabs semantics tests stay contract-first (data/aria/role/source over snapshot)"
cargo test -p ui --test tabs_semantics tabs_check2_documents_semantics_first_testing_rules
cargo test -p ui --test tabs_semantics tabs_semantics_suite_is_contract_first_not_snapshot_only
cargo test -p ui --test tabs_semantics tabs_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks

echo "[contract-hygiene] contract: tag styles keep defensive fallback chain with ui-theme SSOT terminals"
cargo test -p ui --test tag_semantics --no-default-features --features component-tag,inject-css tag_styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals

echo "[contract-hygiene] contract: tag agent-contract schema-like markers + whitelist-safe render path"
cargo test -p ui --test tag_semantics --no-default-features --features component-tag,inject-css tag_agent_contract_is_schema_typed_and_machine_readable
cargo test -p ui --test tag_semantics --no-default-features --features component-tag,inject-css tag_agent_contract_render_path_is_whitelist_safe_and_script_injection_free
cargo test -p ui --test tag_semantics --no-default-features --features component-tag,inject-css tag_check2_documents_semantics_first_testing_rules
cargo test -p ui --test tag_semantics --no-default-features --features component-tag,inject-css tag_semantics_suite_is_contract_first_not_snapshot_only
cargo test -p ui --test tag_semantics --no-default-features --features component-tag,inject-css tag_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks
echo "[contract-hygiene] contract: tag css is aggregated in @layer ui and runtime style is css-variable-only"
cargo test -p ui --test tag_semantics --no-default-features --features component-tag,inject-css tag_cascade_layer_and_runtime_style_contract_is_enforced

echo "[contract-hygiene] contract: tag-group styles keep defensive fallback chain with ui-theme SSOT terminals"
cargo test -p ui --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals

echo "[contract-hygiene] contract: tag-group agent-contract schema-like markers + whitelist-safe render path"
cargo test -p ui --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_agent_contract_is_schema_typed_and_machine_readable
cargo test -p ui --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_agent_contract_render_path_is_whitelist_safe_and_script_injection_free
cargo test -p ui --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_check2_documents_semantics_first_testing_rules
cargo test -p ui --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_semantics_suite_is_contract_first_not_snapshot_only
cargo test -p ui --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks
echo "[contract-hygiene] contract: tag-group css is aggregated in @layer ui and runtime style is css-variable-only"
cargo test -p ui --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_cascade_layer_and_runtime_style_contract_is_enforced

echo "[contract-hygiene] contract: swatch agent-contract schema-like markers + whitelist-safe render path"
cargo test -p ui --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_agent_contract_is_schema_typed_and_machine_readable
cargo test -p ui --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_agent_contract_render_path_is_whitelist_safe_and_script_injection_free
cargo test -p ui --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_check2_documents_semantics_first_testing_rules
cargo test -p ui --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_semantics_suite_is_contract_first_not_snapshot_only
cargo test -p ui --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks

echo "[contract-hygiene] contract: textarea agent-contract schema-like markers + whitelist-safe render path"
cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_agent_contract_markers_are_schema_like_and_machine_readable
cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_agent_contract_render_path_is_whitelist_safe_and_script_injection_free

echo "[contract-hygiene] contract: textarea semantics tests stay contract-first (data/aria/role/source over snapshot)"
cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_check2_documents_semantics_first_testing_rules
cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_semantics_suite_is_contract_first_not_snapshot_only
cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks
cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_check2_documents_docs_sync_and_state_matrix_rules
cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_docs_examples_sync_with_logic_api_names_and_state_matrix
cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_docs_entry_exists_as_readme_or_equivalent_docs_app_page
cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_docs_are_beginner_friendly_with_default_then_advanced_path
cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_docs_hello_world_snippet_is_zero_threshold_and_not_architecture_wiring
cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_check2_marks_documentation_as_product_complete
cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_check2_documents_source_first_copy_paste_ready_rules
cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_docs_are_copy_paste_ready_with_imports_copy_button_and_sync
cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes
cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_check2_marks_heroui_strategy_and_component_docs_sync_complete

echo "[contract-hygiene] contract: time-field agent-contract schema-like markers + whitelist-safe render path"
cargo test -p ui --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_agent_contract_is_schema_typed_and_machine_readable
cargo test -p ui --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_agent_contract_render_path_is_whitelist_safe_and_script_injection_free
cargo test -p ui --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_check2_documents_semantics_first_testing_rules
cargo test -p ui --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_semantics_suite_prioritizes_contract_assertions_over_snapshots
cargo test -p ui --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks
cargo test -p ui --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_docs_entry_exists_as_readme_or_equivalent_docs_app_page
cargo test -p ui --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_docs_are_beginner_friendly_with_default_then_advanced_path
cargo test -p ui --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_docs_hello_world_snippet_is_zero_threshold_and_not_architecture_wiring
cargo test -p ui --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_check2_marks_documentation_as_product_complete
cargo test -p ui --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_check2_documents_source_first_copy_paste_ready_rules
cargo test -p ui --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_docs_are_copy_paste_ready_with_imports_copy_button_and_sync
cargo test -p ui --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes
cargo test -p ui --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_check2_marks_heroui_strategy_and_component_docs_sync_complete

echo "[contract-hygiene] contract: slider agent-contract schema-like markers + whitelist-safe render path"
cargo test -p ui --test slider_semantics --no-default-features --features component-slider,inject-css slider_check2_documents_agent_contract_schema_governance_rules
cargo test -p ui --test slider_semantics --no-default-features --features component-slider,inject-css slider_agent_contract_is_schema_typed_and_machine_readable
cargo test -p ui --test slider_semantics --no-default-features --features component-slider,inject-css slider_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing
cargo test -p ui --test slider_semantics --no-default-features --features component-slider,inject-css slider_agent_contract_render_path_is_whitelist_safe_and_script_injection_free
cargo test -p ui --test slider_semantics --no-default-features --features component-slider,inject-css slider_check2_documents_semantics_first_testing_rules
cargo test -p ui --test slider_semantics --no-default-features --features component-slider,inject-css slider_semantics_suite_is_contract_first_not_snapshot_only
cargo test -p ui --test slider_semantics --no-default-features --features component-slider,inject-css slider_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks
cargo test -p ui --test slider_semantics --no-default-features --features component-slider,inject-css slider_check2_documents_docs_sync_and_state_matrix_rules
cargo test -p ui --test slider_semantics --no-default-features --features component-slider,inject-css slider_docs_examples_sync_with_logic_api_names_and_state_matrix
cargo test -p ui --test slider_semantics --no-default-features --features component-slider,inject-css slider_check2_documents_documentation_as_product_rules
cargo test -p ui --test slider_semantics --no-default-features --features component-slider,inject-css slider_docs_entry_exists_as_readme_or_equivalent_docs_app_page
cargo test -p ui --test slider_semantics --no-default-features --features component-slider,inject-css slider_docs_are_beginner_friendly_with_default_then_advanced_path
cargo test -p ui --test slider_semantics --no-default-features --features component-slider,inject-css slider_docs_hello_world_snippet_is_zero_threshold_and_not_architecture_wiring
cargo test -p ui --test slider_semantics --no-default-features --features component-slider,inject-css slider_check2_documents_source_first_copy_paste_ready_rules
cargo test -p ui --test slider_semantics --no-default-features --features component-slider,inject-css slider_docs_are_copy_paste_ready_with_imports_copy_button_and_sync
cargo test -p ui --test slider_semantics --no-default-features --features component-slider,inject-css slider_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes
cargo test -p ui --test slider_semantics --no-default-features --features component-slider,inject-css slider_check2_marks_heroui_strategy_and_component_docs_sync_complete
cargo test -p ui --test slider_semantics --no-default-features --features component-slider,inject-css slider_check2_documents_explicit_forbidden_antipattern_rules
cargo test -p ui --test slider_semantics --no-default-features --features component-slider,inject-css slider_forbidden_antipatterns_keep_state_primitives_dom_free_and_headless_visual_free
cargo test -p ui --test slider_semantics --no-default-features --features component-slider,inject-css slider_forbidden_antipatterns_keep_key_state_decisions_out_of_view
cargo test -p ui --test slider_semantics --no-default-features --features component-slider,inject-css slider_forbidden_antipatterns_block_parallel_array_api_and_platform_type_leaks
cargo test -p ui --test slider_semantics --no-default-features --features component-slider,inject-css slider_forbidden_antipatterns_avoid_temporary_patch_drift_and_keep_primitives_sunk
cargo test -p ui --test slider_semantics --no-default-features --features component-slider,inject-css slider_check2_documents_final_merge_gate_rules
cargo test -p ui --test slider_semantics --no-default-features --features component-slider,inject-css slider_final_merge_gate_capabilities_are_backed_by_contract_checks
cargo test -p ui --test slider_semantics --no-default-features --features component-slider,inject-css slider_final_merge_gate_marks_full_repo_gate_as_component_scoped_na

echo "[contract-hygiene] contract: circular-progress agent-contract schema-like markers + whitelist-safe render path"
cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_check2_documents_agent_contract_schema_governance_rules
cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_agent_contract_is_schema_typed_and_machine_readable
cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing
cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_agent_contract_render_path_is_whitelist_safe_and_script_injection_free

echo "[contract-hygiene] contract: scroll-area agent-contract schema-like markers + whitelist-safe render path"
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_check2_documents_agent_contract_schema_governance_rules
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_agent_contract_is_schema_typed_and_machine_readable
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_agent_contract_render_path_is_whitelist_safe_and_script_injection_free
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_check2_documents_semantics_first_testing_rules
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_semantics_suite_is_contract_first_not_snapshot_only
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_check2_documents_docs_sync_and_state_matrix_rules
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_docs_examples_sync_with_logic_api_names_and_state_matrix
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_docs_entry_exists_as_readme_or_equivalent_docs_app_page
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_docs_are_beginner_friendly_with_default_then_advanced_path
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_docs_hello_world_snippet_is_zero_threshold_and_not_architecture_wiring
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_check2_marks_documentation_as_product_complete
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_check2_documents_source_first_copy_paste_ready_rules
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_docs_are_copy_paste_ready_with_imports_copy_button_and_sync
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_check2_marks_source_first_copy_paste_ready_complete
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_check2_marks_heroui_strategy_and_component_docs_sync_complete
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_check2_documents_explicit_forbidden_antipattern_rules
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_forbidden_antipatterns_keep_state_primitives_dom_free_and_headless_visual_free
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_forbidden_antipatterns_keep_key_state_decisions_out_of_view
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_forbidden_antipatterns_block_parallel_array_api_and_platform_type_leaks
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_forbidden_antipatterns_avoid_temporary_patch_drift_and_keep_primitives_sunk
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_check2_documents_final_merge_gate_rules
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_final_merge_gate_capabilities_are_backed_by_contract_checks
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_final_merge_gate_marks_full_repo_gate_as_deferred_by_requirement
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_check2_has_no_unchecked_checklist_items

echo "[contract-hygiene] contract: aspect-ratio check2 evidence + no unchecked items"
cargo test -p ui-layout --test aspect_ratio_semantics --no-default-features --features component-aspect_ratio,inject-css aspect_ratio_check2_marks_core_sections_complete
cargo test -p ui-layout --test aspect_ratio_semantics --no-default-features --features component-aspect_ratio,inject-css aspect_ratio_check2_has_no_unchecked_checklist_items

echo "[contract-hygiene] contract: divider check2 evidence + no unchecked items"
cargo test -p ui-layout --test divider_semantics --no-default-features --features component-divider,inject-css divider_check2_marks_core_sections_complete
cargo test -p ui-layout --test divider_semantics --no-default-features --features component-divider,inject-css divider_check2_has_no_unchecked_checklist_items

echo "[contract-hygiene] contract: field-label check2 evidence + no unchecked items"
cargo test -p ui --test field_label_semantics --no-default-features --features component-field_label,inject-css field_label_check2_marks_core_sections_complete
cargo test -p ui --test field_label_semantics --no-default-features --features component-field_label,inject-css field_label_check2_has_no_unchecked_checklist_items

echo "[contract-hygiene] contract: list styles keep defensive fallback chain with ui-theme SSOT terminals"
cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals

echo "[contract-hygiene] contract: list css is aggregated in @layer ui and runtime style stays css-variable-only"
cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_cascade_layer_contract_is_aggregated_in_ui_layer_and_rejects_plain_inline_style_rules

echo "[contract-hygiene] contract: list agent-contract schema-like markers + whitelist-safe render path"
cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_check2_documents_agent_contract_schema_governance_rules
cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_agent_contract_is_schema_typed_and_machine_readable

echo "[contract-hygiene] contract: list semantics priority asserts role/aria/data-source before snapshots"
cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_check2_documents_semantics_first_testing_rules
cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_semantics_suite_is_contract_first_not_snapshot_only
cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks

echo "[contract-hygiene] contract: breadcrumb semantics priority asserts role/aria/data-source before snapshots"
cargo test -p ui --test breadcrumb_semantics --no-default-features --features component-breadcrumb,inject-css breadcrumb_check2_documents_semantics_first_testing_rules
cargo test -p ui --test breadcrumb_semantics --no-default-features --features component-breadcrumb,inject-css breadcrumb_semantics_suite_is_contract_first_not_snapshot_only

echo "[contract-hygiene] contract: overlays agent-contract schema-like markers + whitelist-safe render path"
cargo test -p ui-overlays overlays_agent_contract_is_schema_typed_and_machine_readable
cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_agent_contract_render_path_is_whitelist_safe_and_script_injection_free

echo "[contract-hygiene] contract: menu styles keep defensive fallback chain with ui-theme SSOT terminals"
cargo test -p ui-menu menu_styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals

echo "[contract-hygiene] contract: menu css is aggregated in @layer ui and runtime style stays css-variable-only"
cargo test -p ui-menu menu_cascade_layer_contract_is_aggregated_in_ui_layer_and_runtime_style_is_css_variable_only

echo "[contract-hygiene] contract: menu motion contract is built-in and safely attached across reduced-motion + non-wasm"
cargo test -p ui-menu menu_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe

echo "[contract-hygiene] contract: overlays styles keep defensive fallback chain with ui-theme SSOT terminals"
cargo test -p ui-overlays overlays_styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals

echo "[contract-hygiene] contract: overlays css stays in @layer ui and runtime style stays css-variable-only"
cargo test -p ui-overlays overlays_cascade_layer_and_runtime_style_contract_is_enforced

echo "[contract-hygiene] OK"
