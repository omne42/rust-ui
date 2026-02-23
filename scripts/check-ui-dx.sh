#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[dx] contract: playground css hot-reload path"
# cargo test -p ui --test button_semantics button_dx_playground_supports_css_hot_reload_without_wasm_rebuild
cargo test -p ui --test button_semantics --no-default-features --features component-button,inject-css button_dx_playground_supports_css_hot_reload_without_wasm_rebuild

echo "[dx] contract: button workbench optional state persistence"
# cargo test -p ui --test button_semantics button_dx_workbench_supports_optional_state_persistence_and_isolated_canvas
cargo test -p ui --test button_semantics --no-default-features --features component-button,inject-css button_dx_workbench_supports_optional_state_persistence_and_isolated_canvas

echo "[dx] contract: button docs product copy-paste-ready + streaming/snapshot + source-first imports"
cargo test -p ui --test button_semantics --no-default-features --features component-button,inject-css button_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot
cargo test -p ui --test button_semantics --no-default-features --features component-button,inject-css button_check2_documents_docs_product_copy_paste_ready_rules
cargo test -p ui --test button_semantics --no-default-features --features component-button,inject-css button_dx_check_script_covers_docs_product_copy_paste_ready_contract

echo "[dx] contract: button docs examples + api/state matrix sync with logic API/defaults"
cargo test -p ui --test button_semantics --no-default-features --features component-button,inject-css button_check2_documents_docs_sync_and_state_matrix_rules
cargo test -p ui --test button_semantics --no-default-features --features component-button,inject-css button_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults

echo "[dx] contract: button documentation-as-product keeps beginner-first docs entry"
cargo test -p ui --test button_semantics --no-default-features --features component-button,inject-css button_check2_documents_documentation_as_product_rules
cargo test -p ui --test button_semantics --no-default-features --features component-button,inject-css button_documentation_entry_exists_with_beginner_first_progression

echo "[dx] contract: button interactive playground docs acceptance surface"
cargo test -p ui --test button_semantics --no-default-features --features component-button,inject-css button_check2_documents_interactive_playground_rules
cargo test -p ui --test button_semantics --no-default-features --features component-button,inject-css button_docs_app_provides_interactive_playground_for_props_state_and_preview
cargo test -p ui --test button_semantics --no-default-features --features component-button,inject-css button_interactive_playground_reuses_repeatable_semantic_e2e_flow

echo "[dx] contract: collapsible interactive playground docs acceptance surface"
cargo test -p ui --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_check2_documents_interactive_playground_rules
cargo test -p ui --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_docs_app_provides_interactive_playground_for_props_state_and_preview
cargo test -p ui --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_interactive_playground_reuses_repeatable_semantic_e2e_flow
cargo test -p ui --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_dx_check_script_covers_interactive_playground_contract

echo "[dx] contract: button source-first docs are copy-paste-ready with real paths and deps"
cargo test -p ui --test button_semantics --no-default-features --features component-button,inject-css button_check2_documents_source_first_copy_paste_ready_rules
cargo test -p ui --test button_semantics --no-default-features --features component-button,inject-css button_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies

echo "[dx] contract: button heroui benchmark strategy + docs entry synchronization"
cargo test -p ui --test button_semantics --no-default-features --features component-button,inject-css button_check2_documents_heroui_benchmark_docs_sync_rules
cargo test -p ui --test button_semantics --no-default-features --features component-button,inject-css button_heroui_strategy_and_component_docs_are_synchronized_and_indexable

echo "[dx] contract: button-copy workbench optional state persistence"
# cargo test -p ui --test button_copy_semantics button_copy_dx_workbench_supports_optional_state_persistence_and_isolated_canvas
cargo test -p ui --test button_copy_semantics --no-default-features --features component-button_copy,inject-css button_copy_dx_workbench_supports_optional_state_persistence_and_isolated_canvas

echo "[dx] contract: action-button playground css hot-reload path"
# cargo test -p ui --test action_button_semantics action_button_dx_playground_supports_css_hot_reload_without_wasm_rebuild
cargo test -p ui --test action_button_semantics --no-default-features --features component-button,inject-css action_button_dx_playground_supports_css_hot_reload_without_wasm_rebuild

echo "[dx] contract: action-button workbench optional state persistence"
# cargo test -p ui --test action_button_semantics action_button_dx_workbench_supports_optional_state_persistence_and_isolated_canvas
cargo test -p ui --test action_button_semantics --no-default-features --features component-button,inject-css action_button_dx_workbench_supports_optional_state_persistence_and_isolated_canvas

echo "[dx] contract: well reuses playground css hot-reload and isolated canvas"
# cargo test -p ui --test well_semantics --no-default-features --features component-well,inject-css well_dx_playground_supports_css_hot_reload_without_wasm_rebuild
# cargo test -p ui --test well_semantics --no-default-features --features component-well,inject-css well_dx_non_interactive_scope_keeps_isolated_canvas_and_marks_persist_state_na
cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_dx_playground_supports_css_hot_reload_without_wasm_rebuild
cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_dx_non_interactive_scope_keeps_isolated_canvas_and_marks_persist_state_na

echo "[dx] contract: tabs playground css hot-reload + workbench persistence"
# cargo test -p ui --test tabs_semantics tabs_dx_playground_supports_css_hot_reload_without_wasm_rebuild
# cargo test -p ui --test tabs_semantics tabs_dx_workbench_supports_optional_state_persistence_and_isolated_canvas
cargo test -p ui --test tabs_semantics --no-default-features --features component-tabs,inject-css tabs_dx_playground_supports_css_hot_reload_without_wasm_rebuild
cargo test -p ui --test tabs_semantics --no-default-features --features component-tabs,inject-css tabs_dx_workbench_supports_optional_state_persistence_and_isolated_canvas

echo "[dx] contract: tag playground css hot-reload + isolated canvas"
cargo test -p ui --test tag_semantics --no-default-features --features component-tag,inject-css tag_dx_playground_supports_css_hot_reload_without_wasm_rebuild
cargo test -p ui --test tag_semantics --no-default-features --features component-tag,inject-css tag_dx_workbench_keeps_context_and_isolated_canvas_with_optional_persist_na

echo "[dx] contract: tag-group playground css hot-reload + isolated canvas"
cargo test -p ui --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_dx_playground_supports_css_hot_reload_without_wasm_rebuild
cargo test -p ui --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_dx_workbench_keeps_context_and_isolated_canvas_with_optional_persist_na

echo "[dx] contract: swatch playground css hot-reload + isolated canvas"
cargo test -p ui --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_dx_playground_supports_css_hot_reload_without_wasm_rebuild
cargo test -p ui --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_dx_interactive_scope_keeps_isolated_canvas_and_context_visible_with_optional_persist_na

echo "[dx] contract: circular-progress playground css hot-reload + isolated canvas"
cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_dx_playground_supports_css_hot_reload_without_wasm_rebuild
cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_dx_non_interactive_scope_keeps_isolated_canvas_and_marks_persist_state_na
echo "[dx] contract: circular-progress docs product copy-paste-ready + streaming/snapshot + source-first imports"
cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot
cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_docs_are_source_first_copy_paste_ready_with_imports_copy_button_and_sync
cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_check2_documents_docs_product_copy_paste_ready_rules
cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_dx_check_script_covers_docs_product_copy_paste_ready_contract
echo "[dx] contract: circular-progress source-first docs are copy-paste-ready with real paths and deps"
cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_check2_documents_source_first_copy_paste_ready_rules
cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies
echo "[dx] contract: circular-progress heroui benchmark strategy + docs entry synchronization"
cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_check2_documents_heroui_benchmark_docs_sync_rules
cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_heroui_strategy_and_component_docs_are_synchronized_and_indexable
cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_dx_check_script_covers_heroui_benchmark_docs_sync_contract
echo "[dx] contract: circular-progress docs examples + api/state matrix sync with logic API/defaults"
cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_check2_documents_docs_sync_and_state_matrix_rules
cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults
cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_dx_check_script_covers_docs_sync_and_state_matrix_contract
echo "[dx] contract: circular-progress documentation-as-product keeps beginner-first docs entry"
cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_check2_documents_documentation_as_product_rules
cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_documentation_entry_exists_with_beginner_first_progression
cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_dx_check_script_covers_documentation_as_product_contract
echo "[dx] contract: circular-progress interactive playground docs acceptance surface"
cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_check2_documents_interactive_playground_rules
cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_docs_app_provides_interactive_playground_for_props_state_and_preview
cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_interactive_playground_reuses_repeatable_semantic_e2e_flow
cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_dx_check_script_covers_interactive_playground_contract

echo "[dx] contract: textarea playground css hot-reload + isolated canvas"
cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_dx_playground_supports_css_hot_reload_without_wasm_rebuild
cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_dx_interactive_scope_keeps_isolated_canvas_and_context_visible_with_optional_persist_na
cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_docs_app_provides_interactive_playground_for_props_state_and_preview
cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_interactive_playground_reuses_repeatable_semantic_e2e_flow

echo "[dx] contract: time-field playground css hot-reload + isolated canvas"
cargo test -p ui --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_dx_playground_supports_css_hot_reload_without_wasm_rebuild
cargo test -p ui --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_dx_interactive_scope_keeps_isolated_canvas_and_context_visible_with_optional_persist_na
cargo test -p ui --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_docs_app_provides_interactive_playground_for_props_state_and_preview
cargo test -p ui --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_interactive_playground_reuses_repeatable_semantic_e2e_flow

echo "[dx] contract: field docs product copy-paste-ready + streaming/snapshot contract"
cargo test -p ui-field field_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot
echo "[dx] contract: field source-first docs are copy-paste-ready with real paths and deps"
cargo test -p ui-field field_check2_documents_source_first_copy_paste_ready_rules
cargo test -p ui-field field_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies
echo "[dx] contract: field heroui benchmark strategy + docs entry synchronization"
cargo test -p ui-field field_check2_documents_heroui_benchmark_docs_sync_rules
cargo test -p ui-field field_heroui_strategy_and_component_docs_are_synchronized_and_indexable
echo "[dx] contract: field docs examples + api/state matrix sync with logic API/defaults"
cargo test -p ui-field field_check2_documents_docs_sync_and_state_matrix_rules
cargo test -p ui-field field_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults
echo "[dx] contract: field documentation-as-product keeps beginner-first docs entry"
cargo test -p ui-field field_check2_documents_documentation_as_product_rules
cargo test -p ui-field field_documentation_entry_exists_with_beginner_first_progression
echo "[dx] contract: field dx script coverage includes documentation-as-product checks"
cargo test -p ui-field field_dx_check_script_covers_documentation_as_product_contract
echo "[dx] contract: field interactive playground docs acceptance surface"
cargo test -p ui-field field_check2_documents_interactive_playground_rules
cargo test -p ui-field field_docs_app_provides_interactive_playground_for_props_state_and_preview
cargo test -p ui-field field_interactive_playground_reuses_repeatable_semantic_e2e_flow
echo "[dx] contract: field dx script coverage includes interactive playground checks"
cargo test -p ui-field field_dx_check_script_covers_interactive_playground_contract
echo "[dx] contract: field dx script coverage includes source-first copy-paste-ready checks"
cargo test -p ui-field field_dx_check_script_covers_source_first_copy_paste_ready_contract
echo "[dx] contract: field dx script coverage includes heroui benchmark docs-sync checks"
cargo test -p ui-field field_dx_check_script_covers_heroui_benchmark_docs_sync_contract

echo "[dx] contract: fieldset playground css hot-reload + isolated workbench"
cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_dx_playground_supports_css_hot_reload_without_wasm_rebuild
cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_dx_workbench_keeps_context_and_isolated_canvas_with_optional_persist_na
echo "[dx] contract: fieldset docs product copy-paste-ready + streaming/snapshot contract"
cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot
echo "[dx] contract: fieldset docs examples + api/state matrix sync with logic API/defaults"
cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_check2_documents_docs_sync_and_state_matrix_rules
cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults
echo "[dx] contract: fieldset documentation-as-product keeps beginner-first docs entry"
cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_check2_documents_documentation_as_product_rules
cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_documentation_entry_exists_with_beginner_first_progression
echo "[dx] contract: fieldset interactive playground docs acceptance surface"
cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_check2_documents_interactive_playground_rules
cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_docs_app_provides_interactive_playground_for_props_state_and_preview
cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_interactive_playground_reuses_repeatable_semantic_e2e_flow

echo "[dx] contract: fieldset source-first docs are copy-paste-ready with real paths and deps"
cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_check2_documents_source_first_copy_paste_ready_rules
cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies
echo "[dx] contract: fieldset heroui benchmark strategy + docs entry synchronization"
cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_check2_documents_heroui_benchmark_docs_sync_rules
cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_heroui_strategy_and_component_docs_are_synchronized_and_indexable

echo "[dx] contract: form-field playground css hot-reload + isolated canvas"
cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na
echo "[dx] contract: form-field docs product copy-paste-ready + streaming/snapshot contract"
cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot
echo "[dx] contract: form-field docs examples + api/state matrix sync with logic API/defaults"
cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_check2_documents_docs_sync_and_state_matrix_rules
cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults
echo "[dx] contract: form-field documentation-as-product keeps beginner-first docs entry"
cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_check2_documents_documentation_as_product_rules
cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_documentation_entry_exists_with_beginner_first_progression
echo "[dx] contract: form-field interactive playground covers props/state preview + repeatable flow"
cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_docs_app_provides_interactive_playground_for_props_state_and_preview
cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_interactive_playground_reuses_repeatable_semantic_e2e_flow
echo "[dx] contract: form-field source-first docs are copy-paste-ready with real paths and deps"
cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_check2_documents_source_first_copy_paste_ready_rules
cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies
echo "[dx] contract: form-field dx script coverage includes source-first copy-paste-ready checks"
cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_dx_check_script_covers_source_first_copy_paste_ready_contract
echo "[dx] contract: form-field heroui benchmark strategy + docs entry synchronization"
cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_check2_documents_heroui_benchmark_docs_sync_rules
cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_heroui_strategy_and_component_docs_are_synchronized_and_indexable
echo "[dx] contract: form-field dx script coverage includes heroui benchmark docs-sync checks"
cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_dx_check_script_covers_heroui_benchmark_docs_sync_contract

echo "[dx] contract: date-input-group playground css hot-reload + isolated canvas"
cargo test -p ui-date-input-group date_input_group_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na

echo "[dx] contract: date-input-group docs product copy-paste-ready + streaming/snapshot contract"
cargo test -p ui-date-input-group date_input_group_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot

echo "[dx] contract: date-input-group documentation-as-product beginner-first docs entry"
cargo test -p ui-date-input-group date_input_group_documentation_entry_exists_with_beginner_first_progression
cargo test -p ui-date-input-group date_input_group_check2_documents_documentation_as_product_rules
cargo test -p ui-date-input-group date_input_group_dx_check_script_covers_documentation_as_product_contract

echo "[dx] contract: date-input-group docs examples + state-matrix sync with logic API/defaults"
cargo test -p ui-date-input-group date_input_group_check2_documents_docs_sync_and_state_matrix_rules
cargo test -p ui-date-input-group date_input_group_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults
cargo test -p ui-date-input-group date_input_group_dx_check_script_covers_docs_sync_and_state_matrix_contract

echo "[dx] contract: date-input-group interactive playground docs acceptance surface"
cargo test -p ui-date-input-group date_input_group_check2_documents_interactive_playground_rules
cargo test -p ui-date-input-group date_input_group_docs_app_provides_interactive_playground_for_props_state_and_preview
cargo test -p ui-date-input-group date_input_group_interactive_playground_reuses_repeatable_semantic_e2e_flow
cargo test -p ui-date-input-group date_input_group_dx_check_script_covers_interactive_playground_contract

echo "[dx] contract: date-input-group source-first docs are copy-paste-ready with real paths and deps"
cargo test -p ui-date-input-group date_input_group_check2_documents_source_first_copy_paste_ready_rules
cargo test -p ui-date-input-group date_input_group_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies
cargo test -p ui-date-input-group date_input_group_dx_check_script_covers_source_first_copy_paste_ready_contract
cargo test -p ui-date-input-group date_input_group_check2_marks_source_first_copy_paste_ready_contract_complete

echo "[dx] contract: date-input-group heroui benchmark strategy + docs entry synchronization"
cargo test -p ui-date-input-group date_input_group_check2_documents_heroui_benchmark_docs_sync_rules
cargo test -p ui-date-input-group date_input_group_heroui_strategy_and_component_docs_are_synchronized_and_indexable
cargo test -p ui-date-input-group date_input_group_dx_check_script_covers_heroui_benchmark_docs_sync_contract
cargo test -p ui-date-input-group date_input_group_check2_marks_heroui_benchmark_docs_sync_contract_complete

echo "[dx] contract: checkbox-field playground css hot-reload + isolated canvas"
cargo test -p ui --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na

echo "[dx] contract: checkbox-field docs product copy-paste-ready + streaming/snapshot contract"
cargo test -p ui --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot
echo "[dx] contract: checkbox-field docs examples + api/state matrix sync with logic API/defaults"
cargo test -p ui --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_check2_documents_docs_sync_and_state_matrix_rules
cargo test -p ui --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults
echo "[dx] contract: checkbox-field documentation-as-product keeps beginner-first docs entry"
cargo test -p ui --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_check2_documents_documentation_as_product_rules
cargo test -p ui --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_documentation_entry_exists_with_beginner_first_progression
echo "[dx] contract: checkbox-field interactive playground docs acceptance surface"
cargo test -p ui --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_check2_documents_interactive_playground_rules
cargo test -p ui --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_docs_app_provides_interactive_playground_for_props_state_and_preview
cargo test -p ui --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_interactive_playground_reuses_repeatable_semantic_e2e_flow
echo "[dx] contract: checkbox-field source-first docs are copy-paste-ready with real paths and deps"
cargo test -p ui --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_check2_documents_source_first_copy_paste_ready_rules
cargo test -p ui --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies
echo "[dx] contract: checkbox-field heroui benchmark strategy + docs entry synchronization"
cargo test -p ui --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_check2_documents_heroui_benchmark_docs_sync_rules
cargo test -p ui --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_heroui_strategy_and_component_docs_are_synchronized_and_indexable
echo "[dx] contract: checkbox-field dx script coverage includes heroui benchmark docs-sync checks"
cargo test -p ui --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_dx_check_script_covers_heroui_benchmark_docs_sync_contract

echo "[dx] contract: checkbox playground css hot-reload + isolated canvas"
cargo test -p ui --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na_locally
echo "[dx] contract: checkbox docs product copy-paste-ready + streaming/snapshot contract"
cargo test -p ui --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot
echo "[dx] contract: checkbox docs examples + api/state matrix sync with logic API/defaults"
cargo test -p ui --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_check2_documents_docs_sync_and_state_matrix_rules
cargo test -p ui --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults
echo "[dx] contract: checkbox documentation-as-product keeps beginner-first docs entry"
cargo test -p ui --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_check2_documents_documentation_as_product_rules
cargo test -p ui --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_documentation_entry_exists_with_beginner_first_progression
echo "[dx] contract: checkbox interactive playground docs acceptance surface"
cargo test -p ui --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_check2_documents_interactive_playground_rules
cargo test -p ui --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_docs_app_provides_interactive_playground_for_props_state_and_preview
cargo test -p ui --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_interactive_playground_reuses_repeatable_semantic_e2e_flow
echo "[dx] contract: checkbox source-first docs are copy-paste-ready with real paths and deps"
cargo test -p ui --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_check2_documents_source_first_copy_paste_ready_rules
cargo test -p ui --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies
echo "[dx] contract: checkbox heroui benchmark strategy + docs entry synchronization"
cargo test -p ui --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_check2_documents_heroui_benchmark_docs_sync_rules
cargo test -p ui --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_heroui_strategy_and_component_docs_are_synchronized_and_indexable

echo "[dx] contract: checkbox-group playground css hot-reload + isolated canvas"
cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na
echo "[dx] contract: checkbox-group docs examples + api/state matrix sync with logic API/defaults"
cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_check2_documents_docs_sync_and_state_matrix_rules
cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults
echo "[dx] contract: checkbox-group documentation-as-product keeps beginner-first docs entry"
cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_check2_documents_documentation_as_product_rules
cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_documentation_entry_exists_with_beginner_first_progression
echo "[dx] contract: checkbox-group interactive playground docs acceptance surface"
cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_check2_documents_interactive_playground_rules
cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_docs_app_provides_interactive_playground_for_props_state_and_preview
cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_interactive_playground_reuses_repeatable_semantic_e2e_flow
echo "[dx] contract: checkbox-group source-first docs are copy-paste-ready with real paths and deps"
cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_check2_documents_source_first_copy_paste_ready_rules
cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies
echo "[dx] contract: checkbox-group heroui benchmark strategy + docs entry synchronization"
cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_check2_documents_heroui_benchmark_docs_sync_rules
cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_heroui_strategy_and_component_docs_are_synchronized_and_indexable
echo "[dx] contract: checkbox-group dx script coverage includes heroui benchmark docs-sync checks"
cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_dx_check_script_covers_heroui_benchmark_docs_sync_contract
echo "[dx] contract: checkbox-group docs product copy-paste-ready + streaming/snapshot contract"
cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot

echo "[dx] contract: bottom-sheet playground css hot-reload + isolated workbench"
cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_dx_playground_supports_hot_reload_context_and_isolated_workbench
echo "[dx] contract: bottom-sheet docs product copy-paste-ready + streaming/snapshot + state matrix"
cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot
echo "[dx] contract: bottom-sheet docs examples + api/state matrix sync with logic API/defaults"
cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_check2_documents_docs_sync_and_state_matrix_rules
cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults
echo "[dx] contract: bottom-sheet documentation-as-product keeps beginner-first docs entry"
cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_check2_documents_documentation_as_product_rules
cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_documentation_entry_exists_with_beginner_first_progression
echo "[dx] contract: bottom-sheet interactive playground docs acceptance surface"
cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_check2_documents_interactive_playground_rules
cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_docs_app_provides_interactive_playground_for_props_state_and_preview
cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_interactive_playground_reuses_repeatable_semantic_e2e_flow
echo "[dx] contract: bottom-sheet source-first docs are copy-paste-ready with real paths and deps"
cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_check2_documents_source_first_copy_paste_ready_rules
cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies
echo "[dx] contract: bottom-sheet heroui benchmark strategy + docs entry synchronization"
cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_check2_documents_heroui_benchmark_docs_sync_rules
cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_heroui_strategy_and_component_docs_are_synchronized_and_indexable

echo "[dx] contract: coachmark playground css hot-reload + isolated workbench"
cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_dx_playground_supports_hot_reload_context_and_isolated_workbench
echo "[dx] contract: coachmark docs product copy-paste-ready + streaming/snapshot + source-first"
cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot
cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_docs_are_source_first_copy_paste_ready_with_imports_copy_button_and_sync
cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_check2_documents_docs_product_copy_paste_ready_rules
cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_dx_check_script_covers_docs_product_copy_paste_ready_contract
echo "[dx] contract: coachmark heroui benchmark strategy + docs entry synchronization"
cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_check2_documents_heroui_benchmark_docs_sync_rules
cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_heroui_strategy_and_component_docs_are_synchronized_and_indexable
cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_dx_check_script_covers_heroui_benchmark_docs_sync_contract
cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_check2_marks_heroui_benchmark_docs_sync_contract_complete
echo "[dx] contract: coachmark source-first docs checklist completion + gate coverage"
cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_check2_documents_source_first_copy_paste_ready_rules
cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_dx_check_script_covers_source_first_copy_paste_ready_contract
cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_check2_marks_source_first_copy_paste_ready_item_complete
echo "[dx] contract: coachmark docs examples + api/state matrix sync with logic API/defaults"
cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_check2_documents_docs_sync_and_state_matrix_rules
cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults
cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_dx_check_script_covers_docs_sync_and_state_matrix_contract
echo "[dx] contract: coachmark documentation-as-product keeps beginner-first docs entry"
cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_check2_documents_documentation_as_product_rules
cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_documentation_entry_exists_with_beginner_first_progression
cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_dx_check_script_covers_documentation_as_product_contract
echo "[dx] contract: coachmark interactive playground docs acceptance surface"
cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_check2_documents_interactive_playground_rules
cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_docs_app_provides_interactive_playground_for_props_state_and_preview
cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_interactive_playground_reuses_repeatable_semantic_e2e_flow
cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_dx_check_script_covers_interactive_playground_contract

echo "[dx] contract: slider playground css hot-reload + isolated canvas"
cargo test -p ui --test slider_semantics --no-default-features --features component-slider,inject-css slider_dx_playground_supports_css_hot_reload_without_wasm_rebuild
cargo test -p ui --test slider_semantics --no-default-features --features component-slider,inject-css slider_dx_interactive_scope_keeps_isolated_canvas_and_context_visible_with_optional_persist_na

echo "[dx] contract: scroll-area playground css hot-reload + isolated canvas"
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_dx_playground_supports_css_hot_reload_without_wasm_rebuild
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_dx_interactive_scope_keeps_isolated_canvas_and_context_visible_with_optional_persist_na
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_docs_app_provides_interactive_playground_for_props_state_and_preview
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_interactive_playground_reuses_repeatable_semantic_e2e_flow

echo "[dx] contract: alert-dialog playground css hot-reload + isolated canvas"
cargo test -p ui --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_dx_playground_supports_css_hot_reload_without_wasm_rebuild
cargo test -p ui --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_dx_interactive_scope_keeps_isolated_canvas_and_context_visible_with_optional_persist_na
echo "[dx] contract: alert-dialog docs product copy-paste-ready + streaming/snapshot + source-first imports"
cargo test -p ui --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot
cargo test -p ui --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_docs_are_source_first_copy_paste_ready_with_imports_copy_button_and_sync
cargo test -p ui --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_check2_documents_docs_product_copy_paste_ready_rules
cargo test -p ui --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_dx_check_script_covers_docs_product_copy_paste_ready_contract
echo "[dx] contract: alert-dialog source-first docs are copy-paste-ready with real paths and deps"
cargo test -p ui --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_check2_documents_source_first_copy_paste_ready_rules
cargo test -p ui --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies
echo "[dx] contract: alert-dialog docs examples + api/state matrix sync with logic API/defaults"
cargo test -p ui --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_check2_documents_docs_sync_and_state_matrix_rules
cargo test -p ui --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults
echo "[dx] contract: alert-dialog documentation-as-product keeps beginner-first docs entry"
cargo test -p ui --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_check2_documents_documentation_as_product_rules
cargo test -p ui --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_documentation_entry_exists_with_beginner_first_progression
echo "[dx] contract: alert-dialog heroui benchmark strategy + docs entry synchronization"
cargo test -p ui --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_check2_documents_heroui_benchmark_docs_sync_rules
cargo test -p ui --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_heroui_strategy_and_component_docs_are_synchronized_and_indexable
echo "[dx] contract: alert-dialog interactive playground docs acceptance surface"
cargo test -p ui --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_check2_documents_interactive_playground_rules
cargo test -p ui --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_docs_app_provides_interactive_playground_for_props_state_and_preview
cargo test -p ui --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_interactive_playground_reuses_repeatable_semantic_e2e_flow

echo "[dx] contract: modal playground css hot-reload + isolated canvas"
cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_dx_playground_supports_css_hot_reload_without_wasm_rebuild
cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_dx_interactive_scope_keeps_isolated_canvas_and_context_visible_with_optional_persist_na
cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_docs_are_copy_paste_ready_with_imports_and_streaming_snapshot_contract
echo "[dx] contract: modal source-first docs are copy-paste-ready with real paths and deps"
cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_check2_documents_source_first_copy_paste_ready_rules
cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies
echo "[dx] contract: modal docs examples + api/state matrix sync with logic API/defaults"
cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_check2_documents_docs_sync_and_state_matrix_rules
cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults
echo "[dx] contract: modal documentation-as-product keeps beginner-first docs entry"
cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_check2_documents_documentation_as_product_rules
cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_documentation_entry_exists_with_beginner_first_progression
echo "[dx] contract: modal heroui benchmark strategy + docs entry synchronization"
cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_check2_documents_heroui_benchmark_docs_sync_rules
cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_heroui_strategy_and_component_docs_are_synchronized_and_indexable
echo "[dx] contract: modal interactive playground docs acceptance surface"
cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_check2_documents_interactive_playground_rules
cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_docs_app_provides_interactive_playground_for_props_state_and_preview
cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_interactive_playground_reuses_repeatable_semantic_e2e_flow

echo "[dx] contract: overlays playground css hot-reload + isolated workbench"
cargo test -p ui-overlays overlays_dx_playground_supports_css_hot_reload_and_context_preserving_isolated_workbench
echo "[dx] contract: overlays docs product copy-paste-ready + streaming/snapshot + state matrix"
cargo test -p ui-overlays overlays_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot
echo "[dx] contract: overlays source-first docs are copy-paste-ready with real paths and deps"
cargo test -p ui-overlays overlays_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies

echo "[dx] contract: drawer playground css hot-reload + isolated canvas"
cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_dx_playground_supports_css_hot_reload_without_wasm_rebuild
cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_dx_interactive_scope_keeps_isolated_canvas_and_context_visible_with_optional_persist_na
echo "[dx] contract: drawer documentation-as-product keeps beginner-first docs entry"
cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_check2_documents_documentation_as_product_rules
cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_documentation_entry_exists_with_beginner_first_progression
echo "[dx] contract: drawer interactive playground docs acceptance surface"
cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_check2_documents_interactive_playground_rules
cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_docs_app_provides_interactive_playground_for_props_state_and_preview
cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_interactive_playground_reuses_repeatable_semantic_e2e_flow
echo "[dx] contract: drawer source-first docs are copy-paste-ready with real paths and deps"
cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_check2_documents_source_first_copy_paste_ready_rules
cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies
echo "[dx] contract: drawer docs product copy-paste-ready + streaming/snapshot contract"
cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_docs_are_copy_paste_ready_with_imports_and_streaming_snapshot_contract
echo "[dx] contract: drawer docs examples + api/state matrix sync with logic API/defaults"
cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_check2_documents_docs_sync_and_state_matrix_rules
cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults
echo "[dx] contract: drawer heroui benchmark strategy + docs entry synchronization"
cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_check2_documents_heroui_benchmark_docs_sync_rules
cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_heroui_strategy_and_component_docs_are_synchronized_and_indexable

echo "[dx] contract: hover-card playground css hot-reload + isolated canvas"
cargo test -p ui --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_dx_playground_supports_css_hot_reload_without_wasm_rebuild
cargo test -p ui --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_dx_interactive_scope_keeps_isolated_canvas_and_context_visible_with_optional_persist_na
echo "[dx] contract: hover-card docs product copy-paste-ready + streaming/snapshot + source-first imports"
cargo test -p ui --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_docs_are_copy_paste_ready_with_imports_and_streaming_snapshot_contract
echo "[dx] contract: hover-card docs examples + api/state matrix sync with logic API/defaults"
cargo test -p ui --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_check2_documents_docs_sync_and_state_matrix_rules
cargo test -p ui --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults
echo "[dx] contract: hover-card interactive playground docs acceptance surface"
cargo test -p ui --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_check2_documents_interactive_playground_rules
cargo test -p ui --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_docs_app_provides_interactive_playground_for_props_state_and_preview
cargo test -p ui --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_interactive_playground_reuses_repeatable_semantic_e2e_flow
echo "[dx] contract: hover-card documentation-as-product keeps beginner-first docs entry"
cargo test -p ui --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_check2_documents_documentation_as_product_rules
cargo test -p ui --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_documentation_entry_exists_with_beginner_first_progression

echo "[dx] contract: list playground css hot-reload + isolated canvas"
cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na
echo "[dx] contract: list docs product copy-paste-ready + streaming/snapshot contract"
cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot

echo "[dx] contract: menu playground css hot-reload + context-preserving isolated workbench"
cargo test -p ui-menu menu_dx_playground_supports_css_hot_reload_and_context_preserving_isolated_workbench

echo "[dx] contract: flip-card playground css hot-reload + isolated canvas"
cargo test -p ui-flip-card flip_card_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na
cargo test -p ui-flip-card flip_card_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot
cargo test -p ui-flip-card flip_card_docs_examples_parameter_state_matrix_sync_with_logic_defaults
echo "[dx] contract: flip-card interactive playground docs acceptance surface"
cargo test -p ui-flip-card flip_card_check2_documents_interactive_playground_rules
cargo test -p ui-flip-card flip_card_docs_app_provides_interactive_playground_for_props_state_and_preview
cargo test -p ui-flip-card flip_card_interactive_playground_reuses_repeatable_semantic_e2e_flow
echo "[dx] contract: flip-card dx script coverage includes interactive playground checks"
cargo test -p ui-flip-card flip_card_dx_check_script_covers_interactive_playground_contract
echo "[dx] contract: flip-card source-first docs are copy-paste-ready with real paths and deps"
cargo test -p ui-flip-card flip_card_check2_documents_source_first_copy_paste_ready_rules
cargo test -p ui-flip-card flip_card_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies
echo "[dx] contract: flip-card dx script coverage includes source-first copy-paste-ready checks"
cargo test -p ui-flip-card flip_card_dx_check_script_covers_source_first_copy_paste_ready_contract
echo "[dx] contract: flip-card heroui benchmark strategy + docs entry synchronization"
cargo test -p ui-flip-card flip_card_check2_documents_heroui_benchmark_docs_sync_rules
cargo test -p ui-flip-card flip_card_heroui_strategy_and_component_docs_are_synchronized_and_indexable
echo "[dx] contract: flip-card dx script coverage includes heroui benchmark docs-sync checks"
cargo test -p ui-flip-card flip_card_dx_check_script_covers_heroui_benchmark_docs_sync_contract

echo "[dx] contract: dialog workbench css hot-reload + context-preserving isolated canvas"
cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_dx_playground_supports_hot_reload_context_and_isolated_workbench

echo "[dx] contract: dialog docs product copy-paste-ready + streaming/snapshot + source-first imports"
cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_docs_are_copy_paste_ready_with_imports_and_streaming_snapshot_contract
cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_check2_documents_docs_product_copy_paste_ready_rules
cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_check2_documents_source_first_copy_paste_ready_rules
cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies

echo "[dx] contract: dialog docs examples + state-matrix sync with logic API/defaults"
cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_check2_documents_docs_sync_and_state_matrix_rules
cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults

echo "[dx] contract: dialog documentation-as-product keeps beginner-first docs entry"
cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_check2_documents_documentation_as_product_rules
cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_documentation_entry_exists_with_beginner_first_progression

echo "[dx] contract: dialog interactive playground covers props/state preview + repeatable flow"
cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_check2_documents_interactive_playground_rules
cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_docs_app_provides_interactive_playground_for_props_state_and_preview
cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_interactive_playground_reuses_repeatable_semantic_e2e_flow

echo "[dx] contract: combo-box playground css hot-reload + workbench optional persistence"
cargo test -p ui --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_dx_playground_supports_css_hot_reload_without_wasm_rebuild
cargo test -p ui --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_dx_workbench_supports_optional_state_persistence_and_isolated_canvas
cargo test -p ui --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_docs_app_provides_interactive_playground_for_props_state_and_preview
cargo test -p ui --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_interactive_playground_reuses_repeatable_semantic_e2e_flow
cargo test -p ui --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_docs_are_source_first_copy_paste_ready_with_imports_copy_button_and_sync
cargo test -p ui --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot
cargo test -p ui --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults
cargo test -p ui --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_documentation_as_product_keeps_beginner_path_before_advanced_sections

echo "[dx] contract: autocomplete playground css hot-reload + workbench optional persistence"
cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_dx_playground_supports_css_hot_reload_without_wasm_rebuild
cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_dx_workbench_supports_optional_state_persistence_and_isolated_canvas
cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot

echo "[dx] contract: autocomplete docs examples + api/state matrix sync with logic API/defaults"
cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_check2_documents_docs_sync_and_state_matrix_rules
cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults

echo "[dx] contract: autocomplete documentation-as-product keeps beginner-first docs entry"
cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_check2_documents_documentation_as_product_rules
cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_documentation_entry_exists_with_beginner_first_progression

echo "[dx] contract: autocomplete interactive playground docs acceptance surface"
cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_check2_documents_interactive_playground_rules
cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_docs_app_provides_interactive_playground_for_props_state_and_preview
cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_interactive_playground_reuses_repeatable_semantic_e2e_flow

echo "[dx] contract: autocomplete source-first docs are copy-paste-ready with real paths and deps"
cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_check2_documents_source_first_copy_paste_ready_rules
cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies

echo "[dx] contract: autocomplete heroui benchmark strategy + docs entry synchronization"
cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_check2_documents_heroui_benchmark_docs_sync_rules
cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_heroui_strategy_and_component_docs_are_synchronized_and_indexable

echo "[dx] contract: drop-zone playground css hot-reload + workbench optional persistence"
cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_dx_playground_supports_css_hot_reload_without_wasm_rebuild
cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_dx_workbench_supports_optional_state_persistence_and_isolated_canvas
echo "[dx] contract: drop-zone docs product copy-paste-ready + streaming/snapshot contract"
cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot

echo "[dx] contract: breadcrumb docs product copy-paste-ready + streaming/snapshot + source-first imports"
cargo test -p ui --test breadcrumb_semantics --no-default-features --features component-breadcrumb,inject-css breadcrumb_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot

echo "[dx] contract: breadcrumb docs examples + state-matrix sync with logic API/defaults"
cargo test -p ui --test breadcrumb_semantics --no-default-features --features component-breadcrumb,inject-css breadcrumb_check2_documents_docs_sync_and_state_matrix_rules
cargo test -p ui --test breadcrumb_semantics --no-default-features --features component-breadcrumb,inject-css breadcrumb_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults

echo "[dx] contract: breadcrumb documentation-as-product keeps beginner-first docs entry"
cargo test -p ui --test breadcrumb_semantics --no-default-features --features component-breadcrumb,inject-css breadcrumb_check2_documents_documentation_as_product_rules
cargo test -p ui --test breadcrumb_semantics --no-default-features --features component-breadcrumb,inject-css breadcrumb_documentation_entry_exists_with_beginner_first_progression

echo "[dx] contract: breadcrumb interactive playground docs acceptance surface"
cargo test -p ui --test breadcrumb_semantics --no-default-features --features component-breadcrumb,inject-css breadcrumb_check2_documents_interactive_playground_rules
cargo test -p ui --test breadcrumb_semantics --no-default-features --features component-breadcrumb,inject-css breadcrumb_docs_app_provides_interactive_playground_for_props_state_and_preview
cargo test -p ui --test breadcrumb_semantics --no-default-features --features component-breadcrumb,inject-css breadcrumb_interactive_playground_reuses_repeatable_semantic_e2e_flow

echo "[dx] contract: breadcrumb source-first docs are copy-paste-ready with real paths and deps"
cargo test -p ui --test breadcrumb_semantics --no-default-features --features component-breadcrumb,inject-css breadcrumb_check2_documents_source_first_copy_paste_ready_rules
cargo test -p ui --test breadcrumb_semantics --no-default-features --features component-breadcrumb,inject-css breadcrumb_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies

echo "[dx] contract: breadcrumb heroui benchmark strategy + docs entry synchronization"
cargo test -p ui --test breadcrumb_semantics --no-default-features --features component-breadcrumb,inject-css breadcrumb_check2_documents_heroui_benchmark_docs_sync_rules
cargo test -p ui --test breadcrumb_semantics --no-default-features --features component-breadcrumb,inject-css breadcrumb_heroui_strategy_and_component_docs_are_synchronized_and_indexable

echo "[dx] contract: drop-zone docs examples + state-matrix sync with logic API/defaults"
cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_check2_documents_docs_sync_and_state_matrix_rules
cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults

echo "[dx] contract: drop-zone documentation-as-product keeps beginner-first docs entry"
cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_check2_documents_documentation_as_product_rules
cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_documentation_entry_exists_with_beginner_first_progression

echo "[dx] contract: drop-zone interactive playground docs acceptance surface"
cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_check2_documents_interactive_playground_rules
cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_docs_app_provides_interactive_playground_for_props_state_and_preview

echo "[dx] contract: drop-zone source-first docs are copy-paste-ready with real paths and deps"
cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_check2_documents_source_first_copy_paste_ready_rules
cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies

echo "[dx] contract: drop-zone heroui benchmark strategy + docs entry synchronization"
cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_check2_documents_heroui_benchmark_docs_sync_rules
cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_heroui_strategy_and_component_docs_are_synchronized_and_indexable

echo "[dx] contract: error-view playground css hot-reload + isolated canvas"
cargo test -p ui --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na
cargo test -p ui --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot

echo "[dx] contract: error-view docs examples + state-matrix sync with logic API/defaults"
cargo test -p ui --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_check2_documents_docs_sync_and_state_matrix_rules
cargo test -p ui --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults

echo "[dx] contract: error-view source-first docs are copy-paste-ready with real paths and deps"
cargo test -p ui --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_check2_documents_source_first_copy_paste_ready_rules
cargo test -p ui --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies

echo "[dx] contract: error-view heroui benchmark docs stay synchronized and indexable"
cargo test -p ui --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_check2_documents_heroui_benchmark_docs_sync_rules
cargo test -p ui --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_heroui_strategy_and_component_docs_are_synchronized_and_indexable

echo "[dx] contract: error-view documentation-as-product keeps beginner-first docs"
cargo test -p ui --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_check2_documents_documentation_as_product_rules
cargo test -p ui --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_documentation_entry_exists_with_beginner_first_progression

echo "[dx] contract: error-view interactive playground docs acceptance surface"
cargo test -p ui --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_check2_documents_interactive_playground_rules
cargo test -p ui --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_docs_app_provides_interactive_playground_for_props_state_and_preview

echo "[dx] contract: command docs product copy-paste-ready + streaming/snapshot contract"
cargo test -p ui-command --lib command_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot

echo "[dx] contract: command docs examples + state-matrix sync with logic API/defaults"
cargo test -p ui-command --lib command_check2_documents_docs_sync_and_state_matrix_rules
cargo test -p ui-command --lib command_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults

echo "[dx] contract: command source-first copy-paste-ready docs"
cargo test -p ui-command --lib command_check2_documents_source_first_copy_paste_ready_rules
cargo test -p ui-command --lib command_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies

echo "[dx] contract: command interactive playground docs acceptance surface"
cargo test -p ui-command --lib command_check2_documents_interactive_playground_rules
cargo test -p ui-command --lib command_docs_app_provides_interactive_playground_for_props_state_and_preview

echo "[dx] contract: command documentation-as-product beginner-first docs"
cargo test -p ui-command --lib command_check2_documents_documentation_as_product_rules
cargo test -p ui-command --lib command_documentation_entry_exists_with_beginner_first_progression

echo "[dx] contract: command heroui benchmark strategy + docs entry synchronization"
cargo test -p ui-command --lib command_check2_documents_heroui_benchmark_docs_sync_rules
cargo test -p ui-command --lib command_heroui_strategy_and_component_docs_are_synchronized_and_indexable

echo "[dx] contract: command workbench css hot-reload + isolated canvas"
cargo test -p ui-command --lib command_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na

echo "[dx] contract: command-dialog docs product copy-paste-ready + streaming/snapshot contract"
cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_docs_are_copy_paste_ready_with_imports_and_streaming_snapshot_contract
cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_check2_documents_source_first_copy_paste_ready_rules
cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies

echo "[dx] contract: command-dialog docs examples + state-matrix sync with logic API/defaults"
cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_check2_documents_docs_sync_and_state_matrix_rules
cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults

echo "[dx] contract: command-dialog documentation-as-product keeps beginner-first doc entry"
cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_check2_documents_documentation_as_product_rules
cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_documentation_entry_exists_with_beginner_first_progression

echo "[dx] contract: command-dialog workbench css hot-reload + optional context persistence"
cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_dx_workbench_supports_optional_state_persistence_and_isolated_canvas
cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_docs_app_provides_interactive_playground_for_props_state_and_preview
cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_interactive_playground_reuses_repeatable_semantic_e2e_flow

echo "[dx] contract: chart playground css hot-reload + workbench optional persistence"
cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_dx_playground_supports_css_hot_reload_without_wasm_rebuild
cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_dx_workbench_supports_optional_state_persistence_and_isolated_canvas
echo "[dx] contract: chart docs product copy-paste-ready + streaming/snapshot + source-first imports"
cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot
cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_docs_are_source_first_copy_paste_ready_with_imports_copy_button_and_sync
cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_check2_documents_docs_product_copy_paste_ready_rules
cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_dx_check_script_covers_docs_product_copy_paste_ready_contract

echo "[dx] contract: chart docs examples + api/state matrix sync with logic API/defaults"
cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_check2_documents_docs_sync_and_state_matrix_rules
cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults
cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_dx_check_script_covers_docs_sync_and_state_matrix_contract

echo "[dx] contract: chart heroui benchmark strategy + docs entry synchronization"
cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_check2_documents_heroui_benchmark_docs_sync_rules
cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_heroui_strategy_and_component_docs_are_synchronized_and_indexable
cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_dx_check_script_covers_heroui_benchmark_docs_sync_contract

echo "[dx] contract: chart documentation-as-product keeps beginner-first docs entry"
cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_check2_documents_documentation_as_product_rules
cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_documentation_entry_exists_with_beginner_first_progression
cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_dx_check_script_covers_documentation_as_product_contract

echo "[dx] contract: chart interactive playground docs acceptance surface"
cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_check2_documents_interactive_playground_rules
cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_docs_app_provides_interactive_playground_for_props_state_and_preview
cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_interactive_playground_reuses_repeatable_semantic_e2e_flow
cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_dx_check_script_covers_interactive_playground_contract

echo "[dx] contract: chart source-first docs are copy-paste-ready with real paths and deps"
cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_check2_documents_source_first_copy_paste_ready_rules
cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies
cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_dx_check_script_covers_source_first_copy_paste_ready_contract

echo "[dx] contract: carousel playground css hot-reload + workbench optional persistence"
cargo test -p ui --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_dx_playground_supports_css_hot_reload_without_wasm_rebuild
cargo test -p ui --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_dx_workbench_supports_optional_state_persistence_and_isolated_canvas
echo "[dx] contract: carousel docs product copy-paste-ready + streaming/snapshot + source-first imports"
cargo test -p ui --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot
cargo test -p ui --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_docs_are_source_first_copy_paste_ready_with_imports_copy_button_and_sync
cargo test -p ui --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_check2_documents_docs_product_copy_paste_ready_rules
cargo test -p ui --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_dx_check_script_covers_docs_product_copy_paste_ready_contract

echo "[dx] contract: carousel docs examples + api/state matrix sync with logic API/defaults"
cargo test -p ui --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_check2_documents_docs_sync_and_state_matrix_rules
cargo test -p ui --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults

echo "[dx] contract: carousel documentation-as-product keeps beginner-first docs entry"
cargo test -p ui --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_check2_documents_documentation_as_product_rules
cargo test -p ui --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_documentation_entry_exists_with_beginner_first_progression

echo "[dx] contract: carousel interactive playground docs acceptance surface"
cargo test -p ui --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_check2_documents_interactive_playground_rules
cargo test -p ui --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_docs_app_provides_interactive_playground_for_props_state_and_preview
cargo test -p ui --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_interactive_playground_reuses_repeatable_semantic_e2e_flow

echo "[dx] contract: carousel source-first docs are copy-paste-ready with real paths and deps"
cargo test -p ui --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_check2_documents_source_first_copy_paste_ready_rules
cargo test -p ui --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies

echo "[dx] contract: carousel heroui benchmark strategy + docs entry synchronization"
cargo test -p ui --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_check2_documents_heroui_benchmark_docs_sync_rules
cargo test -p ui --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_heroui_strategy_and_component_docs_are_synchronized_and_indexable
cargo test -p ui --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_dx_check_script_covers_heroui_benchmark_docs_sync_contract

echo "[dx] contract: color-editor playground css hot-reload + isolated workbench"
cargo test -p ui --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na
cargo test -p ui --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_docs_app_provides_interactive_playground_for_props_state_and_preview
cargo test -p ui --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_interactive_playground_reuses_repeatable_semantic_e2e_flow

echo "[dx] contract: color-swatch playground css hot-reload + isolated canvas"
cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na
cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot

echo "[dx] contract: color-swatch docs examples + state-matrix sync with logic API/defaults"
cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_check2_documents_docs_sync_and_state_matrix_rules
cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults

echo "[dx] contract: color-swatch documentation-as-product beginner-first docs"
cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_check2_documents_documentation_as_product_rules
cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_documentation_entry_exists_with_beginner_first_progression

echo "[dx] contract: color-swatch interactive playground docs acceptance surface"
cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_check2_documents_interactive_playground_rules
cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_docs_app_provides_interactive_playground_for_props_state_and_preview

echo "[dx] contract: color-swatch source-first docs are copy-paste-ready with real paths and deps"
cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_check2_documents_source_first_copy_paste_ready_rules
cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies

echo "[dx] contract: color-swatch heroui benchmark strategy + docs entry synchronization"
cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_check2_documents_heroui_benchmark_docs_sync_rules
cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_heroui_strategy_and_component_docs_are_synchronized_and_indexable

echo "[dx] contract: color-thumb playground css hot-reload + isolated demo"
cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_dx_playground_supports_css_hot_reload_and_context_with_optional_persist_na

echo "[dx] contract: color-thumb docs product copy-paste-ready + state matrix + streaming/snapshot"
cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot

echo "[dx] contract: color-thumb docs examples + parameter/state matrix sync with logic API/defaults"
cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_check2_documents_docs_sync_and_state_matrix_rules
cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults
cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_dx_check_script_covers_docs_sync_and_state_matrix_contract

echo "[dx] contract: color-thumb interactive playground docs acceptance surface"
cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_check2_documents_interactive_playground_rules
cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_docs_app_provides_interactive_playground_for_props_state_and_preview
cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_dx_check_script_covers_interactive_playground_contract

echo "[dx] contract: color-thumb source-first docs are copy-paste-ready with real paths and deps"
cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_check2_documents_source_first_copy_paste_ready_rules
cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies
cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_dx_check_script_covers_source_first_copy_paste_ready_contract

echo "[dx] contract: color-thumb heroui benchmark strategy + docs entry synchronization"
cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_check2_documents_heroui_benchmark_docs_sync_rules
cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_heroui_strategy_and_component_docs_are_synchronized_and_indexable

echo "[dx] contract: color-swatch-picker playground css hot-reload + isolated canvas"
cargo test -p ui --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na

echo "[dx] contract: color-picker workbench css hot-reload + optional context preserve"
cargo test -p ui-color-picker color_picker_dx_workbench_supports_hot_style_feedback_and_optional_state_preserve

echo "[dx] contract: color-picker interactive playground docs acceptance surface"
cargo test -p ui-color-picker color_picker_docs_app_provides_interactive_playground_for_props_state_and_preview
cargo test -p ui-color-picker color_picker_interactive_playground_reuses_repeatable_semantic_e2e_flow

echo "[dx] contract: color-slider workbench css hot-reload + optional context persistence"
cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_dx_workbench_supports_hot_style_feedback_and_optional_state_preserve
cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_docs_app_provides_interactive_playground_for_props_state_and_preview
cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_interactive_playground_reuses_repeatable_semantic_e2e_flow

echo "[dx] contract: color-wheel docs product copy-paste-ready playground matrix"
cargo test -p ui --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot

echo "[dx] contract: color-wheel docs examples + parameter/state matrix sync with logic API/defaults"
cargo test -p ui --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_check2_documents_docs_sync_and_state_matrix_rules
cargo test -p ui --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults
cargo test -p ui --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_dx_check_script_covers_docs_sync_and_state_matrix_contract

echo "[dx] contract: color-wheel documentation-as-product beginner-first docs"
cargo test -p ui --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_check2_documents_documentation_as_product_rules
cargo test -p ui --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_documentation_entry_exists_with_beginner_first_progression
cargo test -p ui --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_dx_check_script_covers_documentation_as_product_contract

echo "[dx] contract: color-wheel interactive playground docs acceptance surface"
cargo test -p ui --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_check2_documents_interactive_playground_rules
cargo test -p ui --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_docs_app_provides_interactive_playground_for_props_state_and_preview
cargo test -p ui --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_dx_check_script_covers_interactive_playground_contract

echo "[dx] contract: color-wheel source-first docs are copy-paste-ready with real paths and deps"
cargo test -p ui --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_check2_documents_source_first_copy_paste_ready_rules
cargo test -p ui --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies
cargo test -p ui --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_dx_check_script_covers_source_first_copy_paste_ready_contract

echo "[dx] contract: color-wheel heroui benchmark strategy + docs entry synchronization"
cargo test -p ui --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_check2_documents_heroui_benchmark_docs_sync_rules
cargo test -p ui --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_heroui_strategy_and_component_docs_are_synchronized_and_indexable

echo "[dx] contract: color-wheel workbench css hot-reload + optional context persistence"
cargo test -p ui --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_dx_workbench_supports_optional_state_persistence_and_isolated_canvas

echo "[dx] contract: collapsible docs product copy-paste-ready + state matrix + streaming/snapshot + source-first imports"
cargo test -p ui --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot
cargo test -p ui --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_docs_are_source_first_copy_paste_ready_with_imports_copy_button_and_sync
cargo test -p ui --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_check2_documents_docs_product_copy_paste_ready_rules
cargo test -p ui --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_dx_check_script_covers_docs_product_copy_paste_ready_contract

echo "[dx] contract: collapsible source-first docs are copy-paste-ready with real paths and deps"
cargo test -p ui --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_check2_documents_source_first_copy_paste_ready_rules
cargo test -p ui --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies
cargo test -p ui --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_dx_check_script_covers_source_first_copy_paste_ready_contract

echo "[dx] contract: collapsible heroui benchmark strategy + docs entry synchronization"
cargo test -p ui --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_check2_documents_heroui_benchmark_docs_sync_rules
cargo test -p ui --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_heroui_strategy_and_component_docs_are_synchronized_and_indexable
cargo test -p ui --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_dx_check_script_covers_heroui_benchmark_docs_sync_contract

echo "[dx] contract: collapsible docs examples + api/state matrix sync with logic API/defaults"
cargo test -p ui --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_check2_documents_docs_sync_and_state_matrix_rules
cargo test -p ui --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults
cargo test -p ui --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_dx_check_script_covers_docs_sync_and_state_matrix_contract

echo "[dx] contract: collapsible documentation-as-product keeps beginner-first docs entry"
cargo test -p ui --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_check2_documents_documentation_as_product_rules
cargo test -p ui --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_documentation_entry_exists_with_beginner_first_progression
cargo test -p ui --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_dx_check_script_covers_documentation_as_product_contract

echo "[dx] contract: collapsible playground css hot-reload + isolated canvas"
cargo test -p ui --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na

echo "[dx] OK"
