use std::fs;
use std::path::Path;

fn resolve_source_path(rel_path: &str) -> Option<std::path::PathBuf> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));

    let mut candidates = vec![manifest_dir.join(rel_path)];

    if let Some(component_rel_path) = rel_path.strip_prefix("../../components/") {
        let direct = workspace_dir.join("components").join(component_rel_path);
        candidates.push(direct.clone());

        let parts: Vec<&str> = component_rel_path.split('/').collect();
        if parts.len() > 3 && parts.get(1) == Some(&"src") && parts.get(2) == parts.first() {
            let collapsed = workspace_dir
                .join("components")
                .join(parts[0])
                .join("src")
                .join(parts[3..].join("/"));
            candidates.push(collapsed);
        }
    }

    if let Some(src_rel_path) = rel_path.strip_prefix("src/") {
        let segments: Vec<&str> = src_rel_path.split('/').collect();
        let components_root = workspace_dir.join("components");

        if let Ok(entries) = fs::read_dir(&components_root) {
            let component_dirs: Vec<String> = entries
                .flatten()
                .filter_map(|entry| {
                    let path = entry.path();
                    path.is_dir()
                        .then(|| entry.file_name().to_string_lossy().to_string())
                })
                .collect();

            for component_dir in component_dirs {
                for start in 0..segments.len() {
                    for end in start..segments.len() {
                        let name = segments[start..=end]
                            .iter()
                            .map(|segment| segment.replace('_', "-"))
                            .collect::<Vec<_>>()
                            .join("-");

                        if name != component_dir {
                            continue;
                        }

                        if end + 1 >= segments.len() {
                            candidates
                                .push(components_root.join(&component_dir).join("src/mod.rs"));
                            candidates
                                .push(components_root.join(&component_dir).join("src/check2.md"));
                            candidates.push(components_root.join(&component_dir).join("check2.md"));
                            continue;
                        }

                        let suffix = segments[end + 1..].join("/");
                        candidates.push(
                            components_root
                                .join(&component_dir)
                                .join("src")
                                .join(&suffix),
                        );
                        candidates.push(
                            components_root
                                .join(&component_dir)
                                .join("test")
                                .join(&suffix),
                        );

                        if suffix == "check2.md" {
                            candidates.push(components_root.join(&component_dir).join("check2.md"));
                        }
                    }
                }
            }
        }
    }

    candidates.into_iter().find(|path| path.exists())
}

fn load_source(rel_path: &str) -> String {
    let path = resolve_source_path(rel_path)
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join(rel_path));

    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn path_exists(rel_path: &str) -> bool {
    resolve_source_path(rel_path)
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join(rel_path))
        .exists()
}
#[test]
fn dialog_does_not_expose_logic_module() {
    let source = load_source("src/dialog/mod.rs");

    assert!(
        !source.contains("pub mod logic"),
        "Dialog's `logic` module should stay private to avoid leaking implementation details into the public API."
    );
}

#[test]
fn dialog_component_has_local_semantics_test_module() {
    let source = load_source("../../components/dialog/test/semantics.rs");

    for needle in [
        "fn dialog_component_keeps_expected_file_boundaries()",
        "fn dialog_logic_and_view_follow_layering_contract()",
        "fn dialog_public_surface_avoids_web_sys_types()",
        "fn dialog_non_composite_api_keeps_explicit_single_tree_contract()",
        "fn dialog_has_no_dragging_macro_micro_state_machine_contracts()",
        "fn dialog_has_no_two_pass_geometry_measurement_contracts()",
        "fn dialog_has_no_collection_registration_protocol_contracts()",
        "fn dialog_has_no_slot_projection_strategy_contracts()",
        "fn dialog_has_no_environment_stream_subscription_contracts()",
        "fn dialog_has_no_event_light_cone_bulk_collection_contracts()",
        "fn dialog_has_no_causality_bus_trace_id_contracts()",
        "fn dialog_has_no_foreign_zone_escape_hatch_contracts()",
        "fn dialog_hydration_path_avoids_time_random_uuid_and_uses_deterministic_ids()",
        "fn dialog_ssr_cross_platform_contract_is_cfg_guarded_and_non_wasm_safe()",
        "fn dialog_preserves_ui_headless_web_ssr_compile_error_mutex_contract()",
        "fn dialog_reduced_motion_ssr_wasm_branch_contract_is_preserved()",
        "fn dialog_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe()",
        "fn dialog_ui_components_fixed_entry_files_follow_layered_boundaries()",
        "fn dialog_component_directory_standard_files_follow_contract_and_na_paths()",
        "fn dialog_file_placement_discipline_is_strict_for_component_scope()",
        "fn dialog_hyper_structure_builder_spec_is_not_applicable_for_simple_component()",
        "fn dialog_context_compression_manifest_and_rbi_projection_are_present_and_current()",
        "fn dialog_check2_documents_agent_contract_schema_governance_rules()",
        "fn dialog_agent_contract_is_schema_typed_and_machine_readable()",
        "fn dialog_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing()",
        "fn dialog_agent_contract_render_path_is_whitelist_safe_and_script_injection_free()",
        "fn dialog_contract_hygiene_script_covers_agent_contract_schema_guards()",
        "fn dialog_check2_documents_streaming_definition_is_llm_output_only_with_two_modes()",
        "fn dialog_streaming_display_modes_are_limited_to_streaming_and_snapshot()",
        "fn dialog_streaming_script_covers_two_mode_definition_contract()",
        "fn dialog_check2_documents_snapshot_as_default_baseline_capability()",
        "fn dialog_snapshot_baseline_consumes_complete_result_and_renders_stably()",
        "fn dialog_streaming_script_covers_snapshot_baseline_contract()",
        "fn dialog_check2_documents_streaming_required_optional_classification_rules()",
        "fn dialog_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous()",
        "fn dialog_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer()",
        "fn dialog_streaming_script_covers_required_optional_classification_contract()",
        "fn dialog_rust_hygiene_forbids_unwrap_expect_and_ignored_results_in_component_sources()",
        "fn dialog_rust_hygiene_string_hotspots_are_coalesced_with_cow_static_str()",
        "fn dialog_rust_hygiene_script_enforces_repo_level_hygiene_guards()",
        "fn dialog_performance_governance_budget_is_defined_and_blocking()",
        "fn dialog_view_macro_complexity_is_bounded_with_semantic_subblocks()",
        "fn dialog_view_functional_split_prefers_plain_functions_over_local_components()",
        "fn dialog_static_fragments_are_constantized_with_accessible_close_icon_template()",
        "fn dialog_inner_html_usage_is_explicitly_na_and_guarded()",
        "fn dialog_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated()",
        "fn dialog_dx_playground_supports_hot_reload_context_and_isolated_workbench()",
        "fn dialog_docs_are_copy_paste_ready_with_imports_and_streaming_snapshot_contract()",
        "fn dialog_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies()",
        "fn dialog_dx_check_script_covers_docs_product_copy_paste_ready_contract()",
        "fn dialog_check2_documents_docs_product_copy_paste_ready_rules()",
        "fn dialog_dx_check_script_covers_source_first_copy_paste_ready_contract()",
        "fn dialog_check2_documents_source_first_copy_paste_ready_rules()",
        "fn dialog_check2_marks_source_first_copy_paste_ready_contract_complete()",
        "fn dialog_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes()",
        "fn dialog_contract_hygiene_script_covers_heroui_strategy_doc_sync_contract()",
        "fn dialog_check2_marks_heroui_strategy_and_component_docs_sync_complete()",
        "fn dialog_check2_documents_docs_sync_and_state_matrix_rules()",
        "fn dialog_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults()",
        "fn dialog_dx_check_script_covers_docs_sync_state_matrix_contract()",
        "fn dialog_check2_marks_docs_sync_and_state_matrix_contract_complete()",
        "fn dialog_check2_documents_documentation_as_product_rules()",
        "fn dialog_documentation_entry_exists_with_beginner_first_progression()",
        "fn dialog_dx_check_script_covers_documentation_as_product_contract()",
        "fn dialog_check2_marks_documentation_as_product_contract_complete()",
        "fn dialog_check2_documents_interactive_playground_rules()",
        "fn dialog_docs_app_provides_interactive_playground_for_props_state_and_preview()",
        "fn dialog_interactive_playground_reuses_repeatable_semantic_e2e_flow()",
        "fn dialog_dx_check_script_covers_interactive_playground_contract()",
        "fn dialog_check2_marks_interactive_playground_contract_complete()",
        "fn dialog_check2_documents_semantics_first_testing_rules()",
        "fn dialog_semantics_suite_is_contract_first_not_snapshot_only()",
        "fn dialog_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks()",
        "fn dialog_contract_hygiene_script_covers_semantics_first_contract_guards()",
        "fn dialog_check2_documents_e2e_selector_and_stable_wait_rules()",
        "fn dialog_e2e_selector_contract_uses_semantic_markers_and_settled_waits()",
        "fn dialog_e2e_contract_covers_ready_and_settled_conditions_for_overlay_paths()",
        "fn dialog_e2e_check_script_covers_selector_and_settled_wait_contract()",
        "fn dialog_check2_marks_e2e_selector_stability_item_complete()",
        "fn dialog_check2_documents_e2e_repeatable_key_flow_rules()",
        "fn dialog_e2e_key_flow_is_repeatable_and_failure_points_are_semantic()",
        "fn dialog_e2e_check_script_covers_selector_and_key_flow_contracts()",
        "fn dialog_check2_marks_e2e_repeatable_key_flow_contract_complete()",
        "fn dialog_version_deprecation_migration_is_na_without_major_breaking_upgrade()",
        "fn dialog_version_deprecation_migration_script_covers_engineering_gate()",
        "fn dialog_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries()",
        "fn dialog_styles_use_defensive_variable_fallback_chain()",
        "fn dialog_cascade_layer_and_runtime_style_contract_is_enforced()",
        "fn dialog_tree_shaking_contract_registers_component_feature_and_gates_lib_css_aggregation()",
        "fn dialog_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget()",
        "fn dialog_check2_documents_tree_shaking_feature_pruning_requirements()",
        "fn dialog_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()",
        "fn dialog_semantics_and_performance_script_covers_contract()",
        "fn dialog_check2_marks_semantics_and_performance_regression_contract_complete()",
    ] {
        assert!(
            source.contains(needle),
            "Dialog component should keep local semantics regression `{needle}`."
        );
    }
}

#[test]
fn dialog_feature_gate_declares_required_component_dependencies() {
    let source = load_source("Cargo.toml");

    assert!(
        source.contains("component-dialog = [\"component-overlay\", \"component-button\"]",),
        "component-dialog feature must depend on component-overlay + component-button so minimal feature builds stay valid.",
    );
}

#[test]
fn dialog_tree_shaking_contract_registers_component_feature_and_gates_lib_css_aggregation() {
    let cargo_source = load_source("Cargo.toml");
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");

    assert!(
        cargo_source.contains("component-dialog = [\"component-overlay\", \"component-button\"]"),
        "ui-components feature tree should register `component-dialog` with minimal dependency chain.",
    );
    assert!(
        cargo_source.contains("web-demo-components = [")
            && cargo_source.contains("\"component-dialog\","),
        "ui-components feature bundles should include component-dialog registration.",
    );
    assert!(
        cargo_source.contains("all-components = [")
            && cargo_source.contains("\"component-dialog\","),
        "all-components bundle should explicitly include component-dialog.",
    );

    for required in [
        "#[cfg(feature = \"component-dialog\")]",
        "#[path = \"../../../components/dialog/src/mod.rs\"]",
        "pub mod dialog;",
        "pub use dialog::{Dialog, DialogMotion, DialogSize};",
    ] {
        assert!(
            lib_source.contains(required),
            "ui-components lib should keep dialog gate marker `{required}`.",
        );
    }

    let mut css_gate_is_adjacent = false;
    let css_lines: Vec<&str> = css_source.lines().collect();
    for (idx, line) in css_lines.iter().enumerate() {
        if line.contains("out.push_str(crate::dialog::styles::CSS);")
            && idx > 0
            && css_lines[idx - 1].contains("#[cfg(feature = \"component-dialog\")]")
        {
            css_gate_is_adjacent = true;
        }
    }
    assert!(
        css_gate_is_adjacent,
        "dialog CSS aggregation should be directly gated by component feature in css.rs.",
    );
}

#[test]
fn dialog_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget() {
    let source = load_source("../../scripts/check-ui-components-tree-shaking.sh");

    for needle in [
        "DIALOG_MIN_FEATURES=\"component-dialog,inject-css\"",
        "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_tree_shaking_contract_registers_component_feature_and_gates_lib_css_aggregation",
        "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "dialog minimal feature tree",
        "cargo tree -e features -i ui-components -p ui-components --no-default-features --features \"$DIALOG_MIN_FEATURES\"",
        "missing command-line feature: component-dialog",
        "missing command-line feature: inject-css for dialog minimal tree",
        "dialog minimal feature tree should not pull all-components",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features \"$DIALOG_MIN_FEATURES\"",
    ] {
        assert!(
            source.contains(needle),
            "tree-shaking check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn dialog_module_exposes_slot_and_state_contracts() {
    let source = load_source("src/dialog/mod.rs");

    for needle in [
        "pub enum DialogSlot",
        "pub struct DialogPartStateInput",
        "pub struct DialogPartState",
        "DEFAULT_ID_BASE",
        "DEFAULT_TITLE",
        "DEFAULT_CLOSE_LABEL",
        "DEFAULT_SHOW_CLOSE_BUTTON",
        "DEFAULT_SIZE",
    ] {
        assert!(
            source.contains(needle),
            "dialog::mod should include `{needle}` contracts."
        );
    }
}

#[test]
fn dialog_logic_exposes_state_helpers() {
    let source = load_source("src/dialog/logic.rs");

    for needle in [
        "pub enum DialogSize",
        "pub fn as_attr(self) -> &'static str",
        "pub type DialogOpenMode = dialog_state::DialogOpenMode;",
        "pub type DialogCloseButtonVisibility = dialog_state::DialogCloseButtonVisibility;",
        "pub type DialogCloseButtonPropSource = dialog_state::DialogCloseButtonPropSource;",
        "pub struct DialogOpenStateInput",
        "pub struct DialogOpenState",
        "pub fn normalize_open_state(input: DialogOpenStateInput) -> DialogOpenState",
        "pub struct DialogCloseConfigInput",
        "pub struct DialogCloseConfig",
        "pub fn normalize_close_config(input: DialogCloseConfigInput) -> DialogCloseConfig",
        "pub struct DialogExitConfig",
        "pub fn normalize_exit_config(on_exit_complete: Option<Callback<()>>) -> DialogExitConfig",
        "pub fn can_request_close(mode: DialogOpenMode, has_open_change_handler: bool) -> bool",
        "pub struct DialogPartStatesInput",
        "pub struct DialogPartStates",
        "pub struct DialogPartClasses",
        "pub fn resolve_part_states(input: DialogPartStatesInput) -> DialogPartStates",
        "pub fn resolve_part_classes(",
        "dialog_state::resolve_open_state_contract(dialog_state::DialogOpenStateContractInput {",
        "dialog_state::resolve_close_button_contract(",
        "dialog_state::can_request_close(mode, has_open_change_handler)",
        "pub fn state_attr(has_description: bool)",
        "pub fn description_attr(has_description: bool)",
        "pub fn footer_attr(has_footer: bool)",
        "pub fn close_button_attr(show_close_button: bool)",
        "pub fn normalize_optional_text(value: Option<String>)",
        "pub fn normalize_required_text(value: String, fallback: &'static str)",
        "pub fn normalize_id_base(value: String)",
        "pub fn resolve_state(input: DialogPartStateInput) -> DialogPartState",
        "pub fn compose_class_name(base_class_name: Option<String>, state: DialogPartState)",
    ] {
        assert!(
            source.contains(needle),
            "Dialog logic should include `{needle}` for centralized state/source contracts."
        );
    }
}

#[test]
fn dialog_view_uses_logic_contracts_and_source_markers() {
    let source = load_source("src/dialog/view.rs");

    for needle in [
        "logic::normalize_id_base(id_base)",
        "let can_request_close = logic::can_request_close(",
        "let open_state = logic::normalize_open_state(logic::DialogOpenStateInput {",
        "let open_state = use_controllable_open_state_traced(",
        "let is_open = open_state.open;",
        "let request_open_change = open_state.request_open_change;",
        "logic::normalize_required_text(title, logic::DEFAULT_TITLE)",
        "logic::normalize_optional_text(description)",
        "logic::normalize_optional_text(class_name)",
        "let part_states = logic::resolve_part_states(logic::DialogPartStatesInput {",
        "let part_classes = logic::resolve_part_classes(class_name, part_states);",
        "data-slot=root_state.slot_attr",
        "data-state=root_state.state_attr",
        "data-open-mode=open_mode_attr",
        "data-open-source=open_source_attr",
        "data-open-change-source=open_change_source_attr",
        "data-open-prop-source=open_prop_source_attr",
        "data-size=root_state.size_attr",
        "data-description=root_state.description_attr",
        "data-footer=root_state.footer_attr",
        "data-close-button=root_state.close_button_attr",
        "data-size-source=root_state.size_source_attr",
        "data-id-source=root_state.id_source_attr",
        "data-title-source=root_state.title_source_attr",
        "data-description-source=root_state.description_source_attr",
        "data-footer-source=root_state.footer_source_attr",
        "data-close-source=root_state.close_source_attr",
        "data-class-source=root_state.class_source_attr",
        "data-motion-source=root_state.motion_source_attr",
        "data-exit-source=root_state.exit_source_attr",
        "data-custom-size=root_state.has_custom_size.then_some(\"true\")",
        "data-custom-id=root_state.has_custom_id_base.then_some(\"true\")",
        "data-custom-title=root_state.has_custom_title.then_some(\"true\")",
        "data-custom-description=root_state.has_custom_description.then_some(\"true\")",
        "data-custom-close=(root_state.close_source_attr == \"custom\").then_some(\"true\")",
        "data-custom-motion=root_state.has_custom_motion.then_some(\"true\")",
        "data-custom-exit=root_state.has_on_exit_complete.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "Dialog view should include `{needle}` for stable marker contracts."
        );
    }
}

#[test]
fn dialog_public_api_uses_prefixed_bool_names_with_compat_aliases() {
    let source = load_source("src/dialog/view.rs");

    for needle in [
        "#[prop(optional)] is_open: Option<Signal<bool>>",
        "#[prop(optional)] open: Option<Signal<bool>>",
        "#[prop(optional)] default_open: Option<bool>",
        "#[prop(optional)] on_open_change: Option<Callback<bool>>",
        "#[prop(optional)] on_close: Option<OnPress>",
        "#[prop(optional, default = logic::DEFAULT_SHOW_CLOSE_BUTTON)] is_close_button_visible: bool",
        "#[prop(optional)] show_close_button: Option<bool>",
        "let open_state = logic::normalize_open_state(logic::DialogOpenStateInput {",
        "let open_state = use_controllable_open_state_traced(",
        "let is_open = open_state.open;",
        "let request_open_change = open_state.request_open_change;",
        "let can_request_close = logic::can_request_close(",
        "let close_config = logic::normalize_close_config(logic::DialogCloseConfigInput {",
        "let close_button_visibility = close_config.close_button_visibility;",
        "let close_label = close_config.close_label;",
        "let exit_config = logic::normalize_exit_config(on_exit_complete);",
        "let on_exit_complete = StoredValue::new(exit_config.on_exit_complete);",
        "let part_states = logic::resolve_part_states(logic::DialogPartStatesInput {",
        "let part_classes = logic::resolve_part_classes(class_name, part_states);",
        "request_open_change.run(false);",
    ] {
        assert!(
            source.contains(needle),
            "Dialog API naming should include `{needle}` for prefixed naming + migration compatibility."
        );
    }

    for forbidden in [
        "if close_label.trim().is_empty()",
        "show_close_button.unwrap_or(is_close_button_visible)",
        "on_exit_complete.unwrap_or_else",
        "logic::resolve_state(DialogPartStateInput {",
        "logic::compose_class_name(class_name, root_state)",
    ] {
        assert!(
            !source.contains(forbidden),
            "Dialog view should not keep fallback/default branching `{forbidden}` after logic centralization."
        );
    }
}

#[test]
fn dialog_has_no_async_loading_or_retry_contracts() {
    let source = load_source("src/dialog/view.rs");

    for forbidden in [
        "is_loading",
        "aria-busy",
        "aria_busy",
        "on_retry",
        "use_async_action",
        "data-loading",
        "data-error",
        "data-retry",
        "spawn_local",
        "spawn(",
    ] {
        assert!(
            !source.contains(forbidden),
            "Dialog has no async protocol and should not expose `{forbidden}`."
        );
    }

    for needle in [
        "#[prop(optional)] on_close: Option<OnPress>",
        "let close_action: OnPress = Callback::new(move |_| {",
        "request_open_change.run(false);",
    ] {
        assert!(
            source.contains(needle),
            "Dialog should keep sync close flow evidence via `{needle}`."
        );
    }
}

#[test]
fn dialog_non_composite_api_rejects_parallel_item_array_contracts() {
    let source = load_source("src/dialog/view.rs");

    for needle in ["pub fn Dialog(", "title: String", "children: ChildrenFn"] {
        assert!(
            source.contains(needle),
            "Dialog should keep explicit single-tree API contract `{needle}`."
        );
    }

    for forbidden in [
        "labels: Vec",
        "titles: Vec",
        "panels: Vec",
        "item_specs",
        "ItemSpec",
    ] {
        assert!(
            !source.contains(forbidden),
            "Dialog should not expose parallel-array/config-sugar API `{forbidden}`."
        );
    }
}

#[test]
fn dialog_has_no_dragging_macro_micro_state_machine_contracts() {
    let view_source = load_source("src/dialog/view.rs");
    let logic_source = load_source("src/dialog/logic.rs");
    let motion_source = load_source("src/dialog/motion.rs");

    for forbidden in [
        "on_drag",
        "dragging",
        "DragEnd",
        "Action::DragEnd",
        "pointermove",
        "mousemove",
        "touchmove",
        "request_animation_frame",
        "requestAnimationFrame",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "Dialog should not expose drag macro/micro state machine contract `{forbidden}`."
        );
    }

    for needle in [
        "use_controllable_open_state_traced(",
        "request_open_change.run(false);",
        "crate::overlay::motion::attach_motion(",
    ] {
        assert!(
            view_source.contains(needle) || motion_source.contains(needle),
            "Dialog should keep open/close + motion delegation contract `{needle}`."
        );
    }
}

#[test]
fn dialog_has_no_two_pass_geometry_measurement_contracts() {
    let view_source = load_source("src/dialog/view.rs");
    let logic_source = load_source("src/dialog/logic.rs");
    let motion_source = load_source("src/dialog/motion.rs");

    for forbidden in [
        "getBoundingClientRect",
        "ResizeObserver",
        "IntersectionObserver",
        "client_width",
        "client_height",
        "offset_width",
        "offset_height",
        "scroll_width",
        "scroll_height",
        "Intent",
        "Rectification",
        "placement_rect",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "Dialog should not expose two-pass geometry measurement contract `{forbidden}`."
        );
    }

    for needle in [
        "use_controllable_open_state_traced(",
        "request_open_change.run(false);",
        "crate::overlay::motion::attach_motion(",
    ] {
        assert!(
            view_source.contains(needle) || motion_source.contains(needle),
            "Dialog should keep open/close + overlay delegation contract `{needle}`."
        );
    }
}

#[test]
fn dialog_has_no_collection_registration_protocol_contracts() {
    let view_source = load_source("src/dialog/view.rs");
    let logic_source = load_source("src/dialog/logic.rs");

    for forbidden in [
        "RegistrationContext",
        "Register",
        "Unregister",
        "items_order",
        "HashSet",
        "BTreeSet",
        "roving_index",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "Dialog should not expose collection registration contract `{forbidden}`."
        );
    }

    for needle in ["pub fn Dialog(", "title: String", "children: ChildrenFn"] {
        assert!(
            view_source.contains(needle),
            "Dialog should keep explicit single-tree container API contract `{needle}`."
        );
    }
}

#[test]
fn dialog_has_no_slot_projection_strategy_contracts() {
    let view_source = load_source("src/dialog/view.rs");
    let logic_source = load_source("src/dialog/logic.rs");
    let motion_source = load_source("src/dialog/motion.rs");

    for forbidden in [
        "KeepAlive",
        "Lazy",
        "Eager",
        "NotifyHidden",
        "notify_hidden",
        "slot_projection",
        "projection_mode",
        "set_interval",
        "set_timeout",
        "requestAnimationFrame",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "Dialog should not expose slot projection strategy contract `{forbidden}`."
        );
    }

    for needle in ["pub fn Dialog(", "title: String", "children: ChildrenFn"] {
        assert!(
            view_source.contains(needle),
            "Dialog should keep explicit single-panel container API contract `{needle}`."
        );
    }
}

#[test]
fn dialog_has_no_environment_stream_subscription_contracts() {
    let view_source = load_source("src/dialog/view.rs");
    let logic_source = load_source("src/dialog/logic.rs");
    let motion_source = load_source("src/dialog/motion.rs");

    for forbidden in [
        "ResizeObserver",
        "IntersectionObserver",
        "BreakpointChanged",
        "window.add_event_listener",
        "add_event_listener",
        "matchMedia",
        "match_media",
        "debounce",
        "throttle",
        "on_resize",
        "on_theme",
        "on_intersection",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "Dialog should not expose env-stream subscription contract `{forbidden}`."
        );
    }

    for needle in ["pub fn Dialog(", "title: String", "children: ChildrenFn"] {
        assert!(
            view_source.contains(needle),
            "Dialog should keep explicit single-panel API contract `{needle}`."
        );
    }
}

#[test]
fn dialog_has_no_event_light_cone_bulk_collection_contracts() {
    let view_source = load_source("src/dialog/view.rs");
    let logic_source = load_source("src/dialog/logic.rs");

    for forbidden in [
        "ContextBus",
        "Context Bus",
        "SelectionState::All",
        "SelectionState",
        "bulk_select",
        "select_all",
        "prop drilling",
        "selector_query",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "Dialog should not expose event-light-cone contract `{forbidden}`."
        );
    }

    for needle in ["pub fn Dialog(", "title: String", "children: ChildrenFn"] {
        assert!(
            view_source.contains(needle),
            "Dialog should keep explicit single-panel API contract `{needle}`."
        );
    }
}

#[test]
fn dialog_has_no_causality_bus_trace_id_contracts() {
    let view_source = load_source("src/dialog/view.rs");
    let logic_source = load_source("src/dialog/logic.rs");
    let motion_source = load_source("src/dialog/motion.rs");

    for forbidden in [
        "TraceId",
        "trace_id",
        "causality_bus",
        "CausalityBus",
        "bus_broadcast",
        "derived_command",
        "dispatch_command",
        "subscriber",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "Dialog should not expose causality-bus trace contract `{forbidden}`."
        );
    }

    for needle in [
        "use_controllable_open_state_traced(",
        "request_open_change.run(false);",
        "pub fn Dialog(",
    ] {
        assert!(
            view_source.contains(needle),
            "Dialog should keep local open/close causal chain contract `{needle}`."
        );
    }
}

#[test]
fn dialog_has_no_foreign_zone_escape_hatch_contracts() {
    let view_source = load_source("src/dialog/view.rs");
    let logic_source = load_source("src/dialog/logic.rs");
    let mod_source = load_source("src/dialog/mod.rs");

    for forbidden in [
        "ECharts",
        "echarts",
        "mapbox",
        "leaflet",
        "openlayers",
        "google.maps",
        "YieldControl",
        "CleanupForeign",
        "ForeignZone",
        "Foreign Zone",
        "HtmlCanvasElement",
        "JsValue",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !mod_source.contains(forbidden),
            "Dialog should not expose imperative third-party escape hatch contract `{forbidden}`."
        );
    }

    for needle in [
        "pub fn Dialog(",
        "use_controllable_open_state_traced(",
        "request_open_change.run(false);",
    ] {
        assert!(
            view_source.contains(needle),
            "Dialog should keep declarative open/close contract `{needle}`."
        );
    }
}

#[test]
fn dialog_hydration_path_avoids_time_random_uuid_and_uses_deterministic_ids() {
    let view_source = load_source("src/dialog/view.rs");
    let logic_source = load_source("src/dialog/logic.rs");
    let mod_source = load_source("src/dialog/mod.rs");
    let root_source = load_source("src/root.rs");

    for forbidden in [
        "SystemTime::now",
        "UNIX_EPOCH",
        "Instant::now",
        "Date::now",
        "js_sys::Date::now",
        "Uuid::new_v4",
        "uuid::",
        "rand::",
        "thread_rng",
        "getrandom",
        "Math::random",
        "fastrand",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !mod_source.contains(forbidden),
            "Dialog hydration init should not depend on time/random id source `{forbidden}`."
        );
    }

    for needle in [
        "let id_base = logic::normalize_id_base(id_base);",
        "let title_id = format!(\"{id_base}-title\")",
        "let description_id = format!(\"{id_base}-description\")",
    ] {
        assert!(
            view_source.contains(needle),
            "Dialog should keep deterministic id derivation contract `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional, default = 1)] id_seed: u64,",
        "provide_ui_id_provider(id_seed);",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should keep deterministic IdProvider seed injection contract `{needle}`."
        );
    }
}

#[test]
fn dialog_ssr_cross_platform_contract_is_cfg_guarded_and_non_wasm_safe() {
    let view_source = load_source("src/dialog/view.rs");
    let logic_source = load_source("src/dialog/logic.rs");
    let motion_source = load_source("src/dialog/motion.rs");
    let overlay_motion_source = load_source("../../components/overlay/src/motion.rs");
    let headless_lib_source = load_source("../ui-headless/src/lib.rs");
    let motion_lib_source = load_source("../ui-motion/src/lib.rs");

    for forbidden in [
        "web_sys::",
        "web-sys",
        "js_sys::",
        "wasm_bindgen",
        "window.",
        "document.",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "Dialog non-wasm path should not leak browser-only token `{forbidden}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
    ] {
        assert!(
            overlay_motion_source.contains(needle),
            "Overlay motion should keep explicit cfg split contract `{needle}`."
        );
    }

    assert!(
        headless_lib_source.contains(
            "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")"
        ),
        "ui-headless should keep compile-time web/ssr mutex guard."
    );

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            motion_lib_source.contains(needle),
            "ui-motion should keep wasm/non-wasm backend split contract `{needle}`."
        );
    }
}

#[test]
fn dialog_preserves_ui_headless_web_ssr_compile_error_mutex_contract() {
    let view_source = load_source("src/dialog/view.rs");
    let headless_lib_source = load_source("../ui-headless/src/lib.rs");
    let headless_cargo_source = load_source("../ui-headless/Cargo.toml");

    for needle in [
        "use ui_headless::{A11yDirection, locale_attrs, use_controllable_open_state_traced};",
        "let open_state = use_controllable_open_state_traced(",
    ] {
        assert!(
            view_source.contains(needle),
            "Dialog should keep ui-headless contract mounting `{needle}`."
        );
    }

    assert!(
        headless_lib_source.contains(
            "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")"
        ),
        "ui-headless must keep compile-time web/ssr mutex guard."
    );

    for needle in [
        "default = [\"web\"]",
        "web = [\"leptos/csr\"]",
        "ssr = [\"leptos/ssr\"]",
    ] {
        assert!(
            headless_cargo_source.contains(needle),
            "ui-headless feature wiring should keep `{needle}`."
        );
    }
}

#[test]
fn dialog_reduced_motion_ssr_wasm_branch_contract_is_preserved() {
    let view_source = load_source("src/dialog/view.rs");
    let overlay_motion_source = load_source("../../components/overlay/src/motion.rs");
    let motion_lib_source = load_source("../ui-motion/src/lib.rs");
    let spring_source = load_source("../ui-motion/src/spring.rs");

    for needle in [
        "let motion = crate::dialog::motion::sanitize_motion(motion);",
        "motion=motion.overlay",
    ] {
        assert!(
            view_source.contains(needle),
            "Dialog view should keep stable motion contract mounting `{needle}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "if !is_open.get() {",
        "finish_exit.run(());",
    ] {
        assert!(
            overlay_motion_source.contains(needle),
            "Overlay motion should keep wasm/non-wasm predictable downgrade contract `{needle}`."
        );
    }

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "true",
    ] {
        assert!(
            motion_lib_source.contains(needle),
            "ui-motion non-wasm backend should keep reduced-motion no-op stub `{needle}`."
        );
    }

    for needle in [
        "if crate::web::prefers_reduced_motion() {",
        "(self.inner.apply.borrow_mut())(target);",
        "if let Some(on_rest) = self.inner.on_rest.borrow_mut().as_mut() {",
    ] {
        assert!(
            spring_source.contains(needle),
            "ui-motion spring runtime should keep reduced-motion immediate-settle contract `{needle}`."
        );
    }
}

#[test]
fn dialog_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe() {
    let dialog_motion_source = load_source("src/dialog/motion.rs");
    let dialog_motion_test_source = load_source("../../components/dialog/test/motion.rs");
    let overlay_motion_source = load_source("../../components/overlay/src/motion.rs");
    let ui_motion_source = load_source("../ui-motion/src/lib.rs");
    let dialog_view_source = load_source("src/dialog/view.rs");
    let script_source = load_source("../../scripts/check-ui-components-platforms.sh");
    let checklist_source = load_source("../../components/dialog/check2.md");

    for needle in [
        "pub struct DialogMotion {",
        "pub overlay: crate::overlay::OverlayMotion,",
        "pub fn sanitize_motion(motion: DialogMotion) -> DialogMotion",
        "crate::overlay::motion::sanitize_motion(motion.overlay)",
        "pub fn attach_motion(",
        "crate::overlay::motion::attach_motion(node_ref, is_open, finish_exit, motion.overlay);",
    ] {
        assert!(
            dialog_motion_source.contains(needle),
            "dialog motion module should keep component-scoped contract mapping `{needle}`.",
        );
    }

    for needle in [
        "fn default_motion_uses_default_overlay_motion_contract()",
        "fn supports_custom_overlay_motion_contract()",
        "stiffness: 225.0",
        "damping: 21.0",
        "fn sanitize_motion_delegates_to_overlay_contract()",
    ] {
        assert!(
            dialog_motion_test_source.contains(needle),
            "dialog motion regression should include `{needle}`.",
        );
    }

    for needle in [
        "pub struct OverlayMotion {",
        "pub spring: ui_motion::spring::SpringConfig,",
        "stiffness: if value.stiffness.is_finite() && value.stiffness > 0.0 {",
        "damping: if value.damping.is_finite() && value.damping > 0.0 {",
        "pub fn sanitize_motion(motion: OverlayMotion) -> OverlayMotion",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "finish_exit.run(());",
    ] {
        assert!(
            overlay_motion_source.contains(needle),
            "overlay motion should keep stiffness/damping contract + platform-safe attach path `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_source.contains(needle),
            "ui-motion should keep reduced-motion/non-wasm no-op base contract `{needle}`.",
        );
    }

    for needle in [
        "let motion = crate::dialog::motion::sanitize_motion(motion);",
        "motion=motion.overlay",
    ] {
        assert!(
            dialog_view_source.contains(needle),
            "dialog view should sanitize and forward contract via overlay attach path `{needle}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe";
    assert!(
        script_source.contains(script_needle),
        "platform check script should include `{script_needle}`.",
    );

    for needle in [
        "- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。",
        "DialogMotion",
        "overlay::motion::sanitize_motion",
        "overlay::motion::attach_motion",
        "stiffness: 225.0",
        "damping: 21.0",
        "pub fn prefers_reduced_motion() -> bool",
        "dialog_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe",
        "scripts/check-ui-components-platforms.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            checklist_source.contains(needle),
            "dialog check2 motion section should reference `{needle}`.",
        );
    }
}

#[test]
fn dialog_ui_components_fixed_entry_files_follow_layered_boundaries() {
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let active_highlight_source = load_source("../ui-visual-primitive/src/active_highlight.rs");
    let controllable_state_source = load_source("../ui-headless/src/controllable_state.rs");
    let presence_source = load_source("../ui-headless/src/presence.rs");
    let a11y_source = load_source("../ui-headless/src/a11y.rs");
    let script_source = load_source("../../scripts/check-ui-components-entrypoints.sh");
    let checklist_source = load_source("../../components/dialog/check2.md");

    for needle in [
        "mod css;",
        "pub mod root;",
        "pub use root::UiRoot;",
        "#[cfg(feature = \"component-dialog\")]",
        "pub mod dialog;",
        "pub use dialog::{Dialog, DialogMotion, DialogSize};",
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String)",
        "css::push_components_css(out);",
    ] {
        assert!(
            lib_source.contains(needle),
            "ui-components lib entry should keep marker `{needle}`.",
        );
    }

    for forbidden in [
        "pub mod css;",
        "leptos::web_sys",
        "web_sys::",
        "wasm_bindgen",
    ] {
        assert!(
            !lib_source.contains(forbidden),
            "ui-components lib entry should not leak platform/internal marker `{forbidden}`.",
        );
    }

    for needle in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-dialog\")]",
        "out.push_str(crate::dialog::styles::CSS);",
        "#[cfg(feature = \"component-active_highlight\")]",
        "out.push_str(ui_visual_primitive::active_highlight::CSS);",
        "#[cfg(not(feature = \"inject-css\"))]",
        "pub fn push_components_css(_out: &mut String) {}",
    ] {
        assert!(
            css_source.contains(needle),
            "ui-components css registry should keep feature-gated marker `{needle}`.",
        );
    }

    for needle in [
        "use ui_headless::{UiI18n, provide_ui_i18n, provide_ui_id_provider};",
        "provide_ui_i18n(i18n);",
        "provide_ui_id_provider(id_seed);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "if let Some(overrides) = semantic_overrides.get_value() {",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "ui_layout::push_components_css(&mut out);",
        "data-theme-scheme",
        "data-theme-color",
        "data-theme-system",
        "data-theme-scale",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should keep centralized theme/i18n marker `{needle}`.",
        );
    }

    for needle in [
        "pub const CSS: &str =",
        "pub struct ActiveHighlightMotion",
        "struct ActiveHighlightMotionDriver",
        "pub fn attach_active_highlight_motion(",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            active_highlight_source.contains(needle),
            "active_highlight shared primitive should keep marker `{needle}`.",
        );
    }

    for forbidden in [
        "Dialog",
        "Accordion",
        "Button",
        "aria-",
        "data-slot",
        "on:click",
    ] {
        assert!(
            !active_highlight_source.contains(forbidden),
            "active_highlight should stay generic and avoid component business marker `{forbidden}`.",
        );
    }

    for forbidden in ["src/overlay_open.rs", "src/presence.rs", "src/a11y.rs"] {
        assert!(
            !path_exists(forbidden),
            "ui-components forbidden entrypoint file should not exist: `{forbidden}`.",
        );
    }

    for required in [
        "../ui-headless/src/controllable_state.rs",
        "../ui-headless/src/presence.rs",
        "../ui-headless/src/a11y.rs",
    ] {
        assert!(
            path_exists(required),
            "ui-headless canonical primitive file should exist: `{required}`.",
        );
    }

    for needle in [
        "pub fn use_controllable_open_state_traced(",
        "pub fn use_presence(",
        "pub fn aria_controls_when_open(",
    ] {
        assert!(
            controllable_state_source.contains(needle)
                || presence_source.contains(needle)
                || a11y_source.contains(needle),
            "headless canonical primitive files should keep marker `{needle}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_ui_components_fixed_entry_files_follow_layered_boundaries";
    assert!(
        script_source.contains(script_needle),
        "entrypoints check script should include `{script_needle}`.",
    );

    for needle in [
        "- [x] `ui-components` 固定入口文件落点正确。",
        "dialog_ui_components_fixed_entry_files_follow_layered_boundaries",
        "scripts/check-ui-components-entrypoints.sh",
        "crates/ui-components/src/lib.rs",
        "crates/ui-components/src/css.rs",
        "crates/ui-components/src/root.rs",
        "crates/ui-visual-primitive/src/active_highlight.rs",
        "crates/ui-headless/src/controllable_state.rs",
        "crates/ui-headless/src/presence.rs",
        "crates/ui-headless/src/a11y.rs",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            checklist_source.contains(needle),
            "dialog check2 fixed-entry-files section should reference `{needle}`.",
        );
    }
}

#[test]
fn dialog_component_directory_standard_files_follow_contract_and_na_paths() {
    let module_source = load_source("src/dialog/mod.rs");
    let logic_source = load_source("src/dialog/logic.rs");
    let styles_source = load_source("src/dialog/styles.rs");
    let view_source = load_source("src/dialog/view.rs");
    let motion_source = load_source("src/dialog/motion.rs");
    let script_source = load_source("../../scripts/check-ui-components-component-files.sh");
    let checklist_source = load_source("../../components/dialog/check2.md");

    for required in [
        "src/dialog/mod.rs",
        "src/dialog/logic.rs",
        "src/dialog/styles.rs",
        "src/dialog/view.rs",
        "src/dialog/motion.rs",
    ] {
        assert!(
            path_exists(required),
            "dialog component directory should include `{required}`.",
        );
    }

    for forbidden_file in ["src/dialog/render.rs", "src/dialog/spec.rs"] {
        assert!(
            !path_exists(forbidden_file),
            "dialog component directory should keep `{forbidden_file}` absent.",
        );
    }

    for required in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use view::Dialog;",
        "pub use motion::DialogMotion;",
    ] {
        assert!(
            module_source.contains(required),
            "mod.rs should keep minimal stable export marker `{required}`.",
        );
    }
    for forbidden in ["pub mod logic", "pub mod view", "mod render;", "mod spec;"] {
        assert!(
            !module_source.contains(forbidden),
            "mod.rs should not over-export or drift to `{forbidden}`.",
        );
    }

    for required in [
        "pub struct DialogOpenStateInput",
        "pub struct DialogOpenState",
        "pub fn normalize_open_state(input: DialogOpenStateInput) -> DialogOpenState",
        "pub fn normalize_close_config(input: DialogCloseConfigInput) -> DialogCloseConfig",
        "pub fn resolve_part_states(input: DialogPartStatesInput) -> DialogPartStates",
        "pub fn resolve_part_classes(",
    ] {
        assert!(
            logic_source.contains(required),
            "logic.rs should keep normalization/derivation marker `{required}`.",
        );
    }
    for forbidden in ["web_sys::", "window()", "document()", "NodeRef", "view!"] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should stay free of DOM/render token `{forbidden}`.",
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        "var(--ui-",
        ".ui-dialog",
        ".ui-dialog__title[data-slot=\"dialog-title\"]",
    ] {
        assert!(
            styles_source.contains(required),
            "styles.rs should keep token-first CSS marker `{required}`.",
        );
    }
    for forbidden in [
        "#[component]",
        "use ui_headless",
        "use leptos",
        "Dialog title",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "styles.rs should avoid render/headless/business text token `{forbidden}`.",
        );
    }

    for required in [
        "#[component]",
        "pub fn Dialog(",
        "use ui_headless::{A11yDirection, locale_attrs, use_controllable_open_state_traced};",
        "let open_state = logic::normalize_open_state(logic::DialogOpenStateInput {",
        "let open_state = use_controllable_open_state_traced(",
        "let motion = crate::dialog::motion::sanitize_motion(motion);",
        "<Overlay",
        "data-slot=root_state.slot_attr",
    ] {
        assert!(
            view_source.contains(required),
            "view.rs should keep render + headless mount marker `{required}`.",
        );
    }
    for forbidden in [
        "@keyframes",
        "pub const CSS: &str",
        "request_animation_frame",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should avoid styling/runtime engine token `{forbidden}`.",
        );
    }

    for required in [
        "pub struct DialogMotion",
        "pub fn sanitize_motion(motion: DialogMotion) -> DialogMotion",
        "pub fn attach_motion(",
        "crate::overlay::motion::sanitize_motion(motion.overlay)",
        "crate::overlay::motion::attach_motion(node_ref, is_open, finish_exit, motion.overlay);",
    ] {
        assert!(
            motion_source.contains(required),
            "motion.rs should keep semantic->motion contract mapping marker `{required}`.",
        );
    }
    for forbidden in ["request_animation_frame", "web_sys::", "set_timeout"] {
        assert!(
            !motion_source.contains(forbidden),
            "motion.rs should avoid runtime engine token `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_component_directory_standard_files_follow_contract_and_na_paths";
    assert!(
        script_source.contains(script_needle),
        "component-files script should include `{script_needle}`.",
    );

    for needle in [
        "- [x] 组件目录标准文件落点正确。",
        "dialog_component_directory_standard_files_follow_contract_and_na_paths",
        "scripts/check-ui-components-component-files.sh",
        "components/dialog/src/mod.rs",
        "components/dialog/src/logic.rs",
        "components/dialog/src/styles.rs",
        "components/dialog/src/view.rs",
        "components/dialog/src/motion.rs",
        "components/dialog/src/spec.rs",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            checklist_source.contains(needle),
            "dialog check2 component-directory-standard section should reference `{needle}`.",
        );
    }
}

#[test]
fn dialog_file_placement_discipline_is_strict_for_component_scope() {
    dialog_component_directory_standard_files_follow_contract_and_na_paths();
}

#[test]
fn dialog_check2_marks_file_placement_discipline_contract_complete() {
    let source = load_source("../../components/dialog/check2.md");

    for needle in [
        "- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。",
        "dialog_file_placement_discipline_is_strict_for_component_scope",
        "dialog_component_directory_standard_files_follow_contract_and_na_paths",
        "scripts/check-ui-components-component-files.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "dialog check2 file-placement-discipline section should reference `{needle}`.",
        );
    }
}

#[test]
fn dialog_hyper_structure_builder_spec_is_not_applicable_for_simple_component() {
    let mod_source = load_source("src/dialog/mod.rs");
    let readme_source = load_source("../../components/dialog/src/README.md");
    let protocol_source = load_source("src/dialog/protocol.rs");
    let script_source = load_source("../../scripts/check-ui-components-component-files.sh");

    assert!(
        !path_exists("src/dialog/spec.rs"),
        "Dialog should not add `spec.rs` unless there is a stable external schema/builder contract.",
    );
    assert!(
        path_exists("src/button/spec.rs"),
        "button should remain the canonical complex component carrying `spec.rs`.",
    );

    for forbidden in ["mod spec", "pub mod spec", "spec::", "DialogSpec"] {
        assert!(
            !mod_source.contains(forbidden),
            "dialog module boundary should not expose spec module marker `{forbidden}`.",
        );
    }

    for forbidden in ["Spec::new(", ".render()", "DialogSpec"] {
        assert!(
            !readme_source.contains(forbidden),
            "dialog docs should not force Hyper-Structure builder marker `{forbidden}`.",
        );
    }

    for forbidden in ["impl DialogComponentSpec", "pub fn new(", "fn render("] {
        assert!(
            !protocol_source.contains(forbidden),
            "dialog protocol should stay schema-only and avoid builder token `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_hyper_structure_builder_spec_is_not_applicable_for_simple_component";
    assert!(
        script_source.contains(script_needle),
        "component-files script should include `{script_needle}`.",
    );
}

#[test]
fn dialog_check2_marks_hyper_structure_builder_item_complete() {
    let source = load_source("../../components/dialog/check2.md");

    for needle in [
        "- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。（N/A：`Dialog` 为标准 overlay 装配组件，当前无稳定外部 Schema DSL 与 builder 需求，不引入 `spec.rs` 与 `*Spec::new()...render()` 链路；`protocol.rs` 仅保留最小版本化序列化契约。）",
        "dialog_hyper_structure_builder_spec_is_not_applicable_for_simple_component",
        "scripts/check-ui-components-component-files.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "dialog check2 Hyper-Structure builder section should reference `{needle}`.",
        );
    }
}

#[test]
fn dialog_context_compression_manifest_and_rbi_projection_are_present_and_current() {
    for required_file in ["src/dialog/Component.toml", "src/dialog/dialog.rbi"] {
        assert!(
            path_exists(required_file),
            "dialog context-compression artifact should exist: `{required_file}`.",
        );
    }

    let manifest_source = load_source("src/dialog/Component.toml");
    let rbi_source = load_source("src/dialog/dialog.rbi");
    let script_source = load_source("../../scripts/check-ui-components-component-files.sh");

    for needle in [
        "schema_version = \"1\"",
        "name = \"Dialog\"",
        "crate = \"ui-dialog\"",
        "rbi = \"dialog.rbi\"",
        "name = \"is_open\"",
        "name = \"open\"",
        "name = \"default_open\"",
        "name = \"on_open_change\"",
        "name = \"on_close\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "dialog Component.toml should keep context-compression marker `{needle}`.",
        );
    }

    for needle in [
        "pub enum DialogSlot",
        "pub use crate::logic::DialogSize;",
        "pub use crate::motion::DialogMotion;",
        "pub const DEFAULT_ID_BASE: &str;",
        "pub fn sanitize_motion(",
        "pub fn attach_motion(",
        "pub fn Dialog(",
        "children: leptos::children::ChildrenFn",
    ] {
        assert!(
            rbi_source.contains(needle),
            "dialog RBI projection should keep signature marker `{needle}`.",
        );
    }

    let needle = "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_context_compression_manifest_and_rbi_projection_are_present_and_current";
    assert!(
        script_source.contains(needle),
        "component-files check script should enforce `{needle}`.",
    );
}

#[test]
fn dialog_check2_marks_context_compression_manifest_and_rbi_contract_complete() {
    let source = load_source("../../components/dialog/check2.md");

    assert!(
        source.contains("- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。"),
        "dialog check2 should mark context-compression manifest/rbi gate complete.",
    );

    for needle in [
        "components/dialog/src/Component.toml",
        "components/dialog/src/dialog.rbi",
        "dialog_context_compression_manifest_and_rbi_projection_are_present_and_current",
        "scripts/check-ui-components-component-files.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "dialog check2 context-compression section should reference `{needle}`.",
        );
    }
}

#[test]
fn dialog_check2_documents_agent_contract_schema_governance_rules() {
    let checklist_source = load_source("../../components/dialog/check2.md");

    for required in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "关键交互组件必须输出稳定机器可读语义（至少 `data-*` + 状态来源标记；复杂组件建议补 `data-ui-schema`）。",
        "Agent 消费字段应来自类型化 schema 生成，不允许散落字符串拼接。",
        "契约字段需可追溯到组件状态轴与动作语义（intent/action/state/source）。",
        "配置到组件的渲染链路必须走白名单能力边界，禁止任意脚本注入。",
        "dialog_agent_contract_is_schema_typed_and_machine_readable",
        "dialog_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing",
        "dialog_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
        "scripts/check-ui-components-contract-hygiene.sh",
    ] {
        assert!(
            checklist_source.contains(required),
            "dialog checklist should keep Agent Contract governance rule `{required}`.",
        );
    }
}

#[test]
fn dialog_agent_contract_is_schema_typed_and_machine_readable() {
    let logic_source = load_source("src/dialog/logic.rs");
    let view_source = load_source("src/dialog/view.rs");

    for needle in [
        "pub const DIALOG_AGENT_SCHEMA: &str = \"ui.dialog.agent-contract\";",
        "pub enum DialogAgentSchemaVersion",
        "pub enum DialogAgentIntent",
        "pub enum DialogAgentAction",
        "pub enum DialogAgentState",
        "pub enum DialogAgentSource",
        "pub enum DialogAgentConfigPolicy",
        "pub struct DialogAgentContract",
        "pub struct DialogAgentContractInput",
        "pub fn dialog_agent_source_from_open_mode(mode: DialogOpenMode) -> DialogAgentSource",
        "pub fn resolve_agent_contract(input: DialogAgentContractInput) -> DialogAgentContract",
    ] {
        assert!(
            logic_source.contains(needle),
            "dialog logic should keep typed agent contract marker `{needle}`.",
        );
    }

    for needle in [
        "let agent_contract = Signal::derive(move || {",
        "logic::resolve_agent_contract(logic::DialogAgentContractInput {",
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-ui-schema-version=move || agent_contract.get().schema_version.as_str()",
        "data-ui-intent=move || agent_contract.get().intent.as_str()",
        "data-ui-action=move || agent_contract.get().action.as_str()",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "data-ui-source=move || agent_contract.get().source.as_str()",
        "data-ui-open-change-source=move || agent_contract.get().open_change_source",
        "data-ui-config-policy=move || agent_contract.get().config_policy.as_str()",
    ] {
        assert!(
            view_source.contains(needle),
            "dialog view should mount schemaized agent marker `{needle}`.",
        );
    }
}

#[test]
fn dialog_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing() {
    let logic_source = load_source("src/dialog/logic.rs");
    let view_source = load_source("src/dialog/view.rs");
    let manifest_source = load_source("src/dialog/Component.toml");

    for needle in [
        "DialogAgentSchemaVersion::V1",
        "DialogAgentIntent::OverlayInteraction",
        "DialogAgentAction::OpenClose",
        "DialogAgentState::Open",
        "DialogAgentState::Closed",
        "DialogAgentSource::Controlled",
        "DialogAgentSource::Uncontrolled",
        "DialogAgentConfigPolicy::Whitelist",
    ] {
        assert!(
            logic_source.contains(needle),
            "dialog agent contract should derive fields from typed enums via `{needle}`.",
        );
    }

    for forbidden in [
        "data-ui-schema=format!(",
        "data-ui-schema-version=format!(",
        "data-ui-intent=format!(",
        "data-ui-action=format!(",
        "data-ui-state=format!(",
        "data-ui-source=format!(",
        "data-ui-config-policy=format!(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "dialog view should avoid free-form agent marker string splicing `{forbidden}`.",
        );
    }

    for needle in [
        "name = \"data-ui-schema\"",
        "name = \"data-ui-schema-version\"",
        "name = \"data-ui-intent\"",
        "name = \"data-ui-action\"",
        "name = \"data-ui-state\"",
        "name = \"data-ui-source\"",
        "name = \"data-ui-config-policy\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "dialog manifest should project typed agent marker `{needle}`.",
        );
    }
}

#[test]
fn dialog_agent_contract_render_path_is_whitelist_safe_and_script_injection_free() {
    let manifest_source = load_source("src/dialog/Component.toml");
    let view_source = load_source("src/dialog/view.rs");
    let logic_source = load_source("src/dialog/logic.rs");
    let protocol_source = load_source("src/dialog/protocol.rs");

    for needle in [
        "[[agent_contract_whitelist]]",
        "name = \"render_path\"",
        "allowed = [",
        "typed_state_from_ui_state_primitives::dialog::resolve_open_state_contract",
        "typed_agent_contract_from_logic::resolve_agent_contract",
        "typed_render_mount_from_view::render_content",
        "blocked = [",
        "\"inner_html\"",
        "\"dangerously_set_inner_html\"",
        "\"<script\"",
        "\"javascript:\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "dialog manifest should keep whitelist-safe render-path marker `{needle}`.",
        );
    }

    let combined = [
        view_source.as_str(),
        logic_source.as_str(),
        protocol_source.as_str(),
    ]
    .join("\n");
    for forbidden in [
        "inner_html",
        "set_inner_html",
        "dangerously_set_inner_html",
        "insert_adjacent_html",
        "<script",
        "javascript:",
    ] {
        assert!(
            !combined.contains(forbidden),
            "dialog render path should stay whitelist-safe without `{forbidden}`.",
        );
    }
}

#[test]
fn dialog_contract_hygiene_script_covers_agent_contract_schema_guards() {
    let source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_check2_documents_agent_contract_schema_governance_rules",
        "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_agent_contract_is_schema_typed_and_machine_readable",
        "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing",
        "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
    ] {
        assert!(
            source.contains(needle),
            "contract-hygiene script should include `{needle}`.",
        );
    }
}

#[test]
fn dialog_check2_marks_agent_contract_schema_governance_complete() {
    let source = load_source("../../components/dialog/check2.md");

    for needle in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "dialog_check2_documents_agent_contract_schema_governance_rules",
        "dialog_agent_contract_is_schema_typed_and_machine_readable",
        "dialog_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing",
        "dialog_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
        "dialog_contract_hygiene_script_covers_agent_contract_schema_guards",
        "scripts/check-ui-components-contract-hygiene.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "dialog check2 should keep Agent Contract governance marker `{needle}`.",
        );
    }
}

#[test]
fn dialog_check2_documents_streaming_definition_is_llm_output_only_with_two_modes() {
    let source = load_source("../../components/dialog/check2.md");

    for needle in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
    ] {
        assert!(
            source.contains(needle),
            "dialog check2 should pin streaming two-mode definition marker `{needle}`.",
        );
    }
}

#[test]
fn dialog_streaming_display_modes_are_limited_to_streaming_and_snapshot() {
    let logic_source = load_source("src/dialog/logic.rs");
    let view_source = load_source("src/dialog/view.rs");
    let manifest_source = load_source("src/dialog/Component.toml");
    let rbi_source = load_source("src/dialog/dialog.rbi");

    for needle in [
        "pub enum DialogAgentStreamMode {",
        "Streaming,",
        "Snapshot,",
        "Self::Streaming => \"streaming\"",
        "Self::Snapshot => \"snapshot\"",
        "stream_mode: DialogAgentStreamMode::Snapshot,",
        "stream_fallback: DialogAgentStreamMode::Snapshot,",
    ] {
        assert!(
            logic_source.contains(needle),
            "dialog logic should keep stream-mode type marker `{needle}`.",
        );
    }

    for needle in [
        "data-stream-mode=move || agent_contract.get().stream_mode.as_str()",
        "data-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
    ] {
        assert!(
            view_source.contains(needle),
            "dialog view should expose stream-mode marker `{needle}`.",
        );
    }

    for needle in [
        "name = \"data-stream-mode\"",
        "ty = \"streaming | snapshot\"",
        "name = \"data-stream-fallback\"",
        "name = \"llm_streaming_two_display_modes_only\"",
        "name = \"stream_mode\"",
        "name = \"stream_fallback\"",
        "values = [\"streaming\", \"snapshot\"]",
    ] {
        assert!(
            manifest_source.contains(needle),
            "dialog manifest should keep stream-definition marker `{needle}`.",
        );
    }

    for needle in [
        "pub enum DialogAgentStreamMode {",
        "pub stream_mode: DialogAgentStreamMode,",
        "pub stream_fallback: DialogAgentStreamMode,",
    ] {
        assert!(
            rbi_source.contains(needle),
            "dialog RBI should keep stream-definition projection marker `{needle}`.",
        );
    }

    for forbidden in ["token-by-token", "delta-patch-mode", "chunk-stream-mode"] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !manifest_source.contains(forbidden)
                && !rbi_source.contains(forbidden),
            "dialog stream contract should avoid undefined mode token `{forbidden}`.",
        );
    }
}

#[test]
fn dialog_streaming_script_covers_two_mode_definition_contract() {
    let source = load_source("../../scripts/check-ui-components-streaming.sh");

    for needle in [
        "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_check2_documents_streaming_definition_is_llm_output_only_with_two_modes",
        "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_streaming_display_modes_are_limited_to_streaming_and_snapshot",
    ] {
        assert!(
            source.contains(needle),
            "streaming check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn dialog_check2_marks_streaming_two_mode_definition_complete() {
    let source = load_source("../../components/dialog/check2.md");

    for needle in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "dialog_check2_documents_streaming_definition_is_llm_output_only_with_two_modes",
        "dialog_streaming_display_modes_are_limited_to_streaming_and_snapshot",
        "dialog_streaming_script_covers_two_mode_definition_contract",
        "scripts/check-ui-components-streaming.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "dialog check2 should keep streaming two-mode evidence marker `{needle}`.",
        );
    }
}

#[test]
fn dialog_check2_documents_snapshot_as_default_baseline_capability() {
    let source = load_source("../../components/dialog/check2.md");

    for needle in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
    ] {
        assert!(
            source.contains(needle),
            "dialog check2 should pin snapshot baseline marker `{needle}`.",
        );
    }
}

#[test]
fn dialog_snapshot_baseline_consumes_complete_result_and_renders_stably() {
    let logic_source = load_source("src/dialog/logic.rs");
    let view_source = load_source("src/dialog/view.rs");

    for needle in [
        "stream_mode: DialogAgentStreamMode::Snapshot,",
        "stream_fallback: DialogAgentStreamMode::Snapshot,",
        "pub fn resolve_agent_contract(input: DialogAgentContractInput) -> DialogAgentContract",
    ] {
        assert!(
            logic_source.contains(needle),
            "dialog logic should keep snapshot baseline contract marker `{needle}`.",
        );
    }

    for needle in [
        "let id_base = logic::normalize_id_base(id_base);",
        "let title = logic::normalize_required_text(title, logic::DEFAULT_TITLE);",
        "let description = logic::normalize_optional_text(description);",
        "let close_config = logic::normalize_close_config(logic::DialogCloseConfigInput {",
        "let part_states = logic::resolve_part_states(logic::DialogPartStatesInput {",
        "let part_classes = logic::resolve_part_classes(class_name, part_states);",
        "data-stream-mode=move || agent_contract.get().stream_mode.as_str()",
        "data-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "data-state=root_state.state_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "dialog view should keep stable snapshot render marker `{needle}`.",
        );
    }
}

#[test]
fn dialog_streaming_script_covers_snapshot_baseline_contract() {
    let source = load_source("../../scripts/check-ui-components-streaming.sh");

    for needle in [
        "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_check2_documents_snapshot_as_default_baseline_capability",
        "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_snapshot_baseline_consumes_complete_result_and_renders_stably",
    ] {
        assert!(
            source.contains(needle),
            "streaming check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn dialog_check2_marks_snapshot_baseline_capability_complete() {
    let source = load_source("../../components/dialog/check2.md");

    for needle in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "dialog_check2_documents_snapshot_as_default_baseline_capability",
        "dialog_snapshot_baseline_consumes_complete_result_and_renders_stably",
        "dialog_streaming_script_covers_snapshot_baseline_contract",
        "scripts/check-ui-components-streaming.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "dialog check2 should keep snapshot baseline evidence marker `{needle}`.",
        );
    }
}

#[test]
fn dialog_check2_documents_streaming_required_optional_classification_rules() {
    let source = load_source("../../components/dialog/check2.md");

    for needle in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
    ] {
        assert!(
            source.contains(needle),
            "dialog check2 should keep streaming required/optional rule `{needle}`.",
        );
    }
}

#[test]
fn dialog_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous() {
    let logic_source = load_source("src/dialog/logic.rs");
    let view_source = load_source("src/dialog/view.rs");

    for needle in [
        "stream_support: DialogAgentStreamSupport::Optional,",
        "output_status: DialogAgentOutputStatus::Verified,",
        "DialogAgentOutputStatus::Draft",
        "DialogAgentOutputStatus::CommitReady",
        "<Overlay",
        "aria_labelledby=title_id.clone()",
        "data-ui-stream-support=move || agent_contract.get().stream_support.as_str()",
        "data-stream-mode=move || agent_contract.get().stream_mode.as_str()",
        "data-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
        "data-output-status=move || agent_contract.get().output_status.as_str()",
        "data-state=root_state.state_attr",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "data-ui-source=move || agent_contract.get().source.as_str()",
    ] {
        assert!(
            logic_source.contains(needle) || view_source.contains(needle),
            "dialog optional-streaming scope should keep semantic continuity marker `{needle}`.",
        );
    }
}

#[test]
fn dialog_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer() {
    let view_source = load_source("src/dialog/view.rs");
    let logic_source = load_source("src/dialog/logic.rs");
    let mod_source = load_source("src/dialog/mod.rs");
    let motion_source = load_source("src/dialog/motion.rs");
    let combined = format!("{view_source}\n{logic_source}\n{mod_source}\n{motion_source}");

    for forbidden in [
        "on_retry",
        "retry(",
        "reconnect",
        "backoff",
        "resume_stream",
        "validate_stream",
        "stream_error",
        "disconnect",
    ] {
        assert!(
            !combined.contains(forbidden),
            "dialog should keep validation/retry/resilience policy outside component layer; found `{forbidden}`.",
        );
    }
}

#[test]
fn dialog_streaming_script_covers_required_optional_classification_contract() {
    let source = load_source("../../scripts/check-ui-components-streaming.sh");

    for needle in [
        "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_check2_documents_streaming_required_optional_classification_rules",
        "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
    ] {
        assert!(
            source.contains(needle),
            "streaming check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn dialog_check2_marks_streaming_required_optional_classification_complete() {
    let source = load_source("../../components/dialog/check2.md");

    for needle in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "dialog_check2_documents_streaming_required_optional_classification_rules",
        "dialog_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "dialog_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
        "dialog_streaming_script_covers_required_optional_classification_contract",
        "scripts/check-ui-components-streaming.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "dialog check2 should keep streaming required/optional evidence marker `{needle}`.",
        );
    }
}

#[test]
fn dialog_rust_hygiene_forbids_unwrap_expect_and_ignored_results_in_component_sources() {
    let mut combined = String::new();
    for rel_path in [
        "src/dialog/lib.rs",
        "src/dialog/mod.rs",
        "src/dialog/logic.rs",
        "src/dialog/motion.rs",
        "src/dialog/protocol.rs",
        "src/dialog/styles.rs",
        "src/dialog/view.rs",
    ] {
        combined.push_str(&load_source(rel_path));
        combined.push('\n');
    }

    for forbidden in [".unwrap(", ".expect(", "let _ ="] {
        assert!(
            !combined.contains(forbidden),
            "dialog non-test sources should avoid rust-hygiene violation `{forbidden}`.",
        );
    }
}

#[test]
fn dialog_rust_hygiene_string_hotspots_are_coalesced_with_cow_static_str() {
    let logic_source = load_source("src/dialog/logic.rs");

    for required in [
        "use std::borrow::Cow;",
        "let mut classes: Vec<Cow<'static, str>> = vec![Cow::Borrowed(state.base_class)];",
        "Cow::Borrowed(\"ui-dialog--with-description\")",
        "Cow::Borrowed(\"ui-dialog--custom-motion\")",
        "classes.push(Cow::Owned(base_class_name));",
    ] {
        assert!(
            logic_source.contains(required),
            "dialog string hotspot path should keep Cow marker `{required}`.",
        );
    }
}

#[test]
fn dialog_rust_hygiene_script_enforces_repo_level_hygiene_guards() {
    let script_source = load_source("../../scripts/check-rust-hygiene.sh");

    for required in [
        r#"'\.(unwrap|unwrap_err|expect)\s*\('"#,
        r#"'^[[:space:]]*let[[:space:]]+_[[:space:]]*='"#,
        "string clone hotspots (prefer Cow<'static, str>)",
        "find crates apps -type f -name '*.rs' -path '*/src/*' | sort",
    ] {
        assert!(
            script_source.contains(required),
            "rust-hygiene gate script should enforce `{required}`.",
        );
    }
}

#[test]
fn dialog_check2_marks_rust_hygiene_contract_complete() {
    let source = load_source("../../components/dialog/check2.md");

    for needle in [
        "- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。",
        "dialog_rust_hygiene_forbids_unwrap_expect_and_ignored_results_in_component_sources",
        "dialog_rust_hygiene_string_hotspots_are_coalesced_with_cow_static_str",
        "dialog_rust_hygiene_script_enforces_repo_level_hygiene_guards",
        "./scripts/check-rust-hygiene.sh",
    ] {
        assert!(
            source.contains(needle),
            "dialog check2 rust hygiene section should reference `{needle}`.",
        );
    }
}

#[test]
fn dialog_check2_marks_tree_shaking_feature_pruning_contract_complete() {
    let source = load_source("../../components/dialog/check2.md");

    for needle in [
        "- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui-components` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。",
        "dialog_tree_shaking_contract_registers_component_feature_and_gates_lib_css_aggregation",
        "dialog_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "scripts/check-ui-components-tree-shaking.sh",
        "cargo tree -e features -i ui-components -p ui-components --no-default-features --features component-dialog,inject-css",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "dialog check2 tree-shaking section should reference `{needle}`.",
        );
    }
}

#[test]
fn dialog_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()
{
    let local_semantics = include_str!("../../../components/dialog/test/semantics.rs");
    let aggregated_semantics = load_source("tests/dialog_semantics.rs");
    let dialog_view_source = load_source("../../components/dialog/src/view.rs");
    let overlay_view_source = load_source("../../components/overlay/src/view.rs");
    let focus_trap_source = load_source("../../crates/ui-headless/src/focus_trap.rs");
    let todo_source = load_source("../../docs/plan/TODO.md");

    for required_test in [
        "fn dialog_logic_and_view_follow_layering_contract()",
        "fn dialog_performance_governance_budget_is_defined_and_blocking()",
        "fn dialog_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()",
    ] {
        assert!(
            local_semantics.contains(required_test) && aggregated_semantics.contains(required_test),
            "semantic/performance regression suite should include `{required_test}` in local and aggregated tests."
        );
    }

    for marker in [
        "data-state=root_state.state_attr",
        "data-open-source=open_source_attr",
        "data-open-change-source=open_change_source_attr",
        "data-open-prop-source=open_prop_source_attr",
        "aria_labelledby=title_id.clone()",
        "aria_describedby=description_id.clone()",
    ] {
        assert!(
            dialog_view_source.contains(marker),
            "Dialog view should expose semantic/data marker `{marker}`."
        );
    }

    for marker in [
        "role=role",
        "aria-modal=\"true\"",
        "aria-labelledby=move || aria_labelledby.get()",
        "aria-describedby=move || aria_describedby.get()",
        "on:keydown=on_key_down",
    ] {
        assert!(
            overlay_view_source.contains(marker),
            "Overlay view should expose aria/focus interaction marker `{marker}`."
        );
    }

    for marker in [
        "focus_manager_push_trap(FocusTrapFrame {",
        "focus_manager_pop_trap",
        "restore_focus_chain(",
    ] {
        assert!(
            focus_trap_source.contains(marker),
            "ui-headless focus manager stack should expose focus-flow marker `{marker}`."
        );
    }

    for marker in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(marker),
            "render_count follow-up governance should include `{marker}`."
        );
    }
}

#[test]
fn dialog_semantics_and_performance_script_covers_contract() {
    let script_source = load_source("../../scripts/check-ui-components-performance.sh");

    for marker in [
        "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_performance_governance_budget_is_defined_and_blocking",
        "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "cargo test -p ui-components --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            script_source.contains(marker),
            "performance check script should include `{marker}`."
        );
    }
}

#[test]
fn dialog_check2_marks_semantics_and_performance_regression_contract_complete() {
    let check2_source = load_source("../../components/dialog/check2.md");

    for marker in [
        "- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。",
        "dialog_logic_and_view_follow_layering_contract",
        "dialog_performance_governance_budget_is_defined_and_blocking",
        "dialog_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "`render_count` 自动化回归仍在仓库统一 follow-up",
        "scripts/check-ui-components-performance.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(marker),
            "dialog check2 semantic/performance section should include `{marker}`."
        );
    }
}

#[test]
fn dialog_performance_governance_budget_is_defined_and_blocking() {
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let perf_probe_source = load_source("../../apps/docs-app/src/perf_probe.rs");
    let coverage_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let check2_source = load_source("src/dialog/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-performance.sh");
    let view_source = load_source("src/dialog/view.rs");

    for needle in [
        "\"button\" => UiPerfBudget {",
        "max_mount_ms: 24.0,",
        "\"input\" => UiPerfBudget {",
        "max_mount_ms: 28.0,",
        "\"dialog\" => UiPerfBudget {",
        "max_mount_ms: 36.0,",
        "max_update_ms: Some(12.0),",
        "max_heap_kb: Some(640.0),",
    ] {
        assert!(
            shell_source.contains(needle),
            "docs shell should keep dialog performance budget token `{needle}`."
        );
    }

    for needle in [
        "component_doc!(\"Dialog\", \"dialog\", \"Overlays\", overlays::dialog)",
        "\"dialog\"",
    ] {
        assert!(
            pages_source.contains(needle),
            "Dialog docs page should remain in coverage traversal via `{needle}`."
        );
    }

    for needle in [
        "data-perf-mount-ms",
        "data-perf-budget-ms",
        "data-perf-budget-update-ms",
        "data-perf-budget-heap-kb",
        "data-perf-violation",
        "data-perf-observability",
        "(mount_ms > budget.max_mount_ms).then_some(\"true\")",
        "\"mount-plus-budget\"",
    ] {
        assert!(
            perf_probe_source.contains(needle),
            "UiPerfProbe should expose performance regression marker `{needle}`."
        );
    }

    for needle in [
        "toHaveAttribute(\"data-perf-mount-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-budget-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-observability\", /mount/);",
        "not.toHaveAttribute(\"data-perf-violation\", \"true\");",
    ] {
        assert!(
            coverage_source.contains(needle),
            "docs coverage e2e should enforce repeatable perf regression guard `{needle}`."
        );
    }

    for needle in [
        "let trace = ui_headless::use_ui_trace();",
        "trace.emit(",
        "fn render_events(trace: ui_headless::UiTrace) -> AnyView",
    ] {
        assert!(
            debug_overlay_source.contains(needle),
            "debug overlay should keep trace-based perf attribution token `{needle}`."
        );
    }

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "performance governance should keep render_count follow-up marker `{needle}`."
        );
    }

    for needle in [
        "性能治理：关键路径有预算",
        "渲染次数预算为 `1`",
        "render_count",
        "若当前测试框架暂不支持精确渲染计数",
        "等价证据",
        "dialog_performance_governance_budget_is_defined_and_blocking",
    ] {
        assert!(
            check2_source.contains(needle),
            "Dialog checklist should keep performance governance marker `{needle}`."
        );
    }

    for needle in [
        "data-state=root_state.state_attr",
        "data-open-source=open_source_attr",
        "data-open-change-source=open_change_source_attr",
        "data-size-source=root_state.size_source_attr",
        "data-motion-source=root_state.motion_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "Dialog view should expose attribution marker `{needle}` for perf triage.",
        );
    }

    let script_needle = "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_performance_governance_budget_is_defined_and_blocking";
    assert!(
        script_source.contains(script_needle),
        "performance gate script should include `{script_needle}`.",
    );
}

#[test]
fn dialog_view_macro_complexity_is_bounded_with_semantic_subblocks() {
    let view_source = load_source("src/dialog/view.rs");
    let script_source = load_source("../../scripts/check-ui-components-view-macro.sh");
    let check2_source = load_source("src/dialog/check2.md");

    for needle in [
        "fn render_dialog_close_section(",
        "struct DialogHeaderRenderInput",
        "fn render_dialog_header_section(input: DialogHeaderRenderInput) -> AnyView",
        "fn render_dialog_body_section(",
        "fn render_dialog_footer_section(",
        "let close_view = render_dialog_close_section(",
        "let header_view = render_dialog_header_section(DialogHeaderRenderInput {",
        "let body_view = render_dialog_body_section(body_state, body_class, children());",
        "let footer_view = render_dialog_footer_section(root_state, footer_state, footer_class, footer);",
        "{close_view}",
        "{header_view}",
        "{body_view}",
        "{footer_view}",
    ] {
        assert!(
            view_source.contains(needle),
            "Dialog view should keep semantic sub-block split marker `{needle}`."
        );
    }

    assert!(
        view_source.matches("view! {").count() <= 8,
        "Dialog view should keep `view!` macro expansion bounded after semantic sub-block extraction.",
    );

    let script_needle = "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_view_macro_complexity_is_bounded_with_semantic_subblocks";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`.",
    );

    for needle in [
        "- [x] `view!` 宏复杂度受控：",
        "render_dialog_close_section",
        "render_dialog_header_section",
        "dialog_view_macro_complexity_is_bounded_with_semantic_subblocks",
    ] {
        assert!(
            check2_source.contains(needle),
            "Dialog checklist should keep view-macro complexity evidence `{needle}`.",
        );
    }
}

#[test]
fn dialog_view_functional_split_prefers_plain_functions_over_local_components() {
    let view_source = load_source("src/dialog/view.rs");
    let script_source = load_source("../../scripts/check-ui-components-view-macro.sh");
    let check2_source = load_source("src/dialog/check2.md");

    for needle in [
        "fn render_dialog_close_section(",
        "struct DialogHeaderRenderInput",
        "fn render_dialog_header_section(input: DialogHeaderRenderInput) -> AnyView",
        "fn render_dialog_body_section(",
        "fn render_dialog_footer_section(",
        "let close_view = render_dialog_close_section(",
        "let header_view = render_dialog_header_section(DialogHeaderRenderInput {",
        "let body_view = render_dialog_body_section(body_state, body_class, children());",
        "let footer_view = render_dialog_footer_section(root_state, footer_state, footer_class, footer);",
        "pub fn Dialog(",
    ] {
        assert!(
            view_source.contains(needle),
            "Dialog view should keep function-first split marker `{needle}`."
        );
    }

    assert_eq!(
        view_source.matches("#[component]").count(),
        1,
        "Dialog should keep a single public component entrypoint and avoid local component sprawl.",
    );

    for forbidden in [
        "#[component]\nfn render_dialog_close_section(",
        "#[component]\nfn render_dialog_header_section(",
        "#[component]\nfn render_dialog_body_section(",
        "#[component]\nfn render_dialog_footer_section(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Dialog helper should remain plain function, not nested component `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_view_functional_split_prefers_plain_functions_over_local_components";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`.",
    );

    for needle in [
        "- [x] 函数式拆分优先：",
        "render_dialog_close_section",
        "render_dialog_header_section",
        "dialog_view_functional_split_prefers_plain_functions_over_local_components",
    ] {
        assert!(
            check2_source.contains(needle),
            "Dialog checklist should keep functional-split evidence `{needle}`.",
        );
    }
}

#[test]
fn dialog_static_fragments_are_constantized_with_accessible_close_icon_template() {
    let view_source = load_source("src/dialog/view.rs");
    let script_source = load_source("../../scripts/check-ui-components-view-macro.sh");
    let check2_source = load_source("src/dialog/check2.md");

    for needle in [
        "const DIALOG_CLOSE_ICON_VIEWBOX: &str = \"0 0 20 20\";",
        "const DIALOG_CLOSE_ICON_PATH_D: &str = \"M5 5l10 10M15 5L5 15\";",
        "const DIALOG_CLOSE_ICON_STROKE_WIDTH: &str = \"1.5\";",
        "fn render_dialog_close_icon() -> impl IntoView",
        "<svg viewBox=DIALOG_CLOSE_ICON_VIEWBOX fill=\"none\" aria-hidden=\"true\">",
        "d=DIALOG_CLOSE_ICON_PATH_D",
        "stroke_width=DIALOG_CLOSE_ICON_STROKE_WIDTH",
        "{render_dialog_close_icon()}",
        "aria_label=close_label",
    ] {
        assert!(
            view_source.contains(needle),
            "Dialog view should keep static-fragment constantization marker `{needle}`."
        );
    }

    for forbidden in [
        "<svg viewBox=\"0 0 20 20\" fill=\"none\" aria-hidden=\"true\">",
        "d=\"M5 5l10 10M15 5L5 15\"",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Dialog should avoid scattered inline static icon literal `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_static_fragments_are_constantized_with_accessible_close_icon_template";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`.",
    );

    for needle in [
        "- [x] 静态片段常量化：",
        "render_dialog_close_icon",
        "DIALOG_CLOSE_ICON_PATH_D",
        "dialog_static_fragments_are_constantized_with_accessible_close_icon_template",
    ] {
        assert!(
            check2_source.contains(needle),
            "Dialog checklist should keep static-fragment evidence `{needle}`.",
        );
    }
}

#[test]
fn dialog_inner_html_usage_is_explicitly_na_and_guarded() {
    for rel_path in [
        "src/dialog/mod.rs",
        "src/dialog/logic.rs",
        "src/dialog/styles.rs",
        "src/dialog/view.rs",
        "src/dialog/motion.rs",
        "src/dialog/protocol.rs",
        "../../components/dialog/src/README.md",
    ] {
        let source = load_source(rel_path);
        for forbidden in [
            "inner_html",
            "set_inner_html",
            "dangerously_set_inner_html",
            "markdown_to_html(",
            "<script",
            "javascript:",
            "onerror=",
            "onload=",
            "fetch(",
            "reqwest",
            "http://",
            "https://",
        ] {
            assert!(
                !source.contains(forbidden),
                "dialog source `{rel_path}` must not contain raw-html injection token `{forbidden}`."
            );
        }
    }

    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_dialog.rs");
    for forbidden in [
        "inner_html",
        "set_inner_html",
        "dangerously_set_inner_html",
        "markdown_to_html(",
        "<script",
        "javascript:",
        "onerror=",
        "onload=",
        "fetch(",
        "reqwest",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "dialog docs page must not contain raw-html injection token `{forbidden}`."
        );
    }

    let script_source = load_source("../../scripts/check-ui-components-inner-html.sh");
    let script_needle = "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_inner_html_usage_is_explicitly_na_and_guarded";
    assert!(
        script_source.contains(script_needle),
        "inner-html gate script should include `{script_needle}`."
    );

    let check2_source = load_source("../../components/dialog/check2.md");
    for needle in [
        "- [x] `inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。",
        "N/A：`Dialog` 当前无 `inner_html` 使用点",
        "dialog_inner_html_usage_is_explicitly_na_and_guarded",
        "scripts/check-ui-components-inner-html.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "Dialog checklist should keep inner-html safety evidence `{needle}`.",
        );
    }
}

#[test]
fn dialog_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated() {
    let check2_source = load_source("../../components/dialog/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-wasm-debug.sh");
    let component_cargo_source = load_source("../../components/dialog/Cargo.toml");
    let ui_components_cargo_source = load_source("Cargo.toml");
    let ui_components_lib_source = load_source("src/lib.rs");
    let docs_app_source = load_source("../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let trace_source = load_source("../ui-headless/src/trace.rs");
    let view_source = load_source("src/dialog/view.rs");
    let logic_source = load_source("src/dialog/logic.rs");
    let motion_source = load_source("src/dialog/motion.rs");
    let readme_source = load_source("../../components/dialog/src/README.md");

    for needle in ["[features]", "default = []"] {
        assert!(
            component_cargo_source.contains(needle),
            "dialog crate feature boundary should include `{needle}`."
        );
    }

    for forbidden in [
        "wasm-debug",
        "dialog-wasm-debug",
        "dialog_wasm_debug",
        "component-dialog-wasm-debug",
    ] {
        assert!(
            !component_cargo_source.contains(forbidden),
            "dialog crate should not expose component-local wasm debug feature `{forbidden}`.",
        );
    }

    assert!(
        ui_components_cargo_source
            .contains("button-wasm-debug = [\"component-button\", \"dep:tracing\"]"),
        "ui-components should keep shared wasm-debug feature marker `button-wasm-debug`.",
    );

    for forbidden in [
        "dialog-wasm-debug",
        "dialog_wasm_debug",
        "component-dialog-wasm-debug",
    ] {
        assert!(
            !ui_components_cargo_source.contains(forbidden),
            "ui-components should not define dialog-local wasm debug feature `{forbidden}`.",
        );
    }

    let all_components_start = ui_components_cargo_source
        .find("all-components = [")
        .expect("all-components feature list should exist");
    let all_components_end = ui_components_cargo_source[all_components_start..]
        .find("\n\ndev-all-components")
        .map(|offset| all_components_start + offset)
        .expect("all-components list should end before dev-all-components");
    let all_components_block =
        &ui_components_cargo_source[all_components_start..all_components_end];
    assert!(
        !all_components_block.contains("button-wasm-debug"),
        "shared wasm-debug feature must stay out of all-components production path.",
    );

    for needle in [
        "macro_rules! wasm_debug_proxy",
        "pub(crate) use wasm_debug_proxy;",
    ] {
        assert!(
            ui_components_lib_source.contains(needle),
            "ui-components root should keep wasm-debug isolation marker `{needle}`."
        );
    }

    for needle in [
        "let debug_overlay_enabled = cfg!(debug_assertions);",
        "provide_ui_trace(debug_overlay_enabled);",
        "<Show when=move || debug_overlay_enabled>",
        "<debug_overlay::UiDebugOverlay enabled=true />",
    ] {
        assert!(
            docs_app_source.contains(needle),
            "docs-app should keep dev-only debug overlay entry marker `{needle}`."
        );
    }

    for needle in [
        "pub enum UiTraceEventKind",
        "OpenChange {",
        "open: bool",
        "pub struct UiTraceEvent",
        "pub ts_ms: u64",
        "pub component: &'static str",
        "pub kind: UiTraceEventKind",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
    ] {
        assert!(
            trace_source.contains(needle),
            "ui-headless trace contract should keep marker `{needle}`."
        );
    }

    for needle in [
        "fn render_events(trace: ui_headless::UiTrace) -> AnyView",
        "events.into_iter().rev().take(40)",
        "fn render_event(event: ui_headless::UiTraceEvent) -> AnyView",
        "ui_headless::UiTraceEventKind::OpenChange { open }",
        "let ts_ms = event.ts_ms;",
        "format!(\"{ts_ms}ms\")",
        "data-slot=\"ui-debug-overlay-events\"",
        "data-slot=\"ui-debug-overlay-event\"",
        "data-component=component",
        "data-kind=kind_attr",
    ] {
        assert!(
            debug_overlay_source.contains(needle),
            "debug overlay should keep replayable timeline marker `{needle}`.",
        );
    }

    for needle in [
        "use_controllable_open_state_traced(",
        "data-open-source=open_source_attr",
        "data-open-change-source=open_change_source_attr",
        "data-open-prop-source=open_prop_source_attr",
        "data-custom-open-change=has_custom_on_open_change.then_some(\"true\")",
    ] {
        assert!(
            view_source.contains(needle),
            "dialog should expose state/source markers for debug attribution `{needle}`.",
        );
    }

    for forbidden in [
        "dialog-wasm-debug",
        "request_replay.run(",
        "render_debug_panel(",
        "#[prop(optional)] debug",
        "data-debug-source",
        "debug_overlay::UiDebugOverlay",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !readme_source.contains(forbidden),
            "dialog runtime/public surface should not leak wasm-debug internals `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated";
    assert!(
        script_source.contains(script_needle),
        "wasm-debug gate script should include `{script_needle}`.",
    );

    for needle in [
        "- [x] WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。",
        "dialog_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated",
        "scripts/check-ui-components-wasm-debug.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "dialog checklist should keep wasm-debug governance marker `{needle}`.",
        );
    }
}

#[test]
fn dialog_dx_playground_supports_hot_reload_context_and_isolated_workbench() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_dialog.rs");
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");
    let check2_source = load_source("../../components/dialog/check2.md");

    for needle in [
        "title=\"Interactive Playground\"",
        "description=\"展示 / Config / Code / CSS Test 集成工作台（含多场景对比）。\"",
        "code_signal=workbench_code",
        "test_css_source=workbench_test_css_source",
        "test_source_path=\"components/dialog/src/styles.rs\".to_string()",
        "test_config_signal=workbench_actual_config",
        "controls=move || view!",
        "title=\"Scenario Comparison\"",
        "code_signal=scenario_code",
    ] {
        assert!(
            docs_source.contains(needle),
            "dialog docs workbench should keep DX surface marker `{needle}`."
        );
    }

    for needle in [
        "ui_components::dialog::styles::CSS",
        "let (workbench_open_raw, set_workbench_open_raw) = signal(false);",
        "let (workbench_present, set_workbench_present) = signal(workbench_open.get_untracked());",
        "if workbench_open.get() {",
        "set_workbench_present.set(true);",
        "<Show when=move || workbench_present.get()>",
        "on_exit_complete=on_workbench_exit_complete",
        "let (workbench_with_description, set_workbench_with_description) = signal(true);",
        "let (workbench_show_close, set_workbench_show_close) = signal(true);",
        "let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);",
        "let (workbench_custom_class, set_workbench_custom_class) = signal(false);",
    ] {
        assert!(
            docs_source.contains(needle),
            "dialog docs should keep context-preserving workbench marker `{needle}`."
        );
    }

    let script_needle = "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_dx_playground_supports_hot_reload_context_and_isolated_workbench";
    assert!(
        script_source.contains(script_needle),
        "dx gate script should include `{script_needle}`.",
    );

    for needle in [
        "- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
        "dialog_dx_playground_supports_hot_reload_context_and_isolated_workbench",
        "scripts/check-ui-components-dx.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "dialog checklist should keep DX governance marker `{needle}`.",
        );
    }
}

#[test]
fn dialog_docs_are_copy_paste_ready_with_imports_and_streaming_snapshot_contract() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_dialog.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");

    for needle in [
        "const DIALOG_DOC_IMPORTS: &str =",
        "code_imports=DIALOG_DOC_IMPORTS.to_string()",
        "title=\"Hello World\"",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled\"",
        "title=\"Streaming / Snapshot Contract\"",
        "Streaming is optional; fallback stays snapshot.",
        "data-requested-stream-mode=move || stream_requested_mode.get()",
        "data-requested-output-status=move || stream_requested_output_status.get()",
        "effective component markers: data-stream-mode=snapshot data-stream-fallback=snapshot data-output-status=verified",
    ] {
        assert!(
            docs_source.contains(needle),
            "dialog docs should keep copy-ready + streaming/snapshot contract `{needle}`.",
        );
    }

    for needle in [
        "const DEFAULT_PLAYGROUND_IMPORTS: &str =",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "missing_import_lines(&raw, &imports)",
    ] {
        assert!(
            playground_source.contains(needle),
            "playground copy pipeline should keep import completion marker `{needle}`.",
        );
    }
}

#[test]
fn dialog_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_dialog.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let code_block_view_source = load_source("../../components/code-block/src/view.rs");

    for needle in [
        "data-slot=\"dialog-source-first\"",
        "<h3>\"Source-first Copy-Paste\"</h3>",
        "<code>\"Show code\"</code>",
        "DIALOG_DOC_IMPORTS",
        "compose_copy_ready_code",
        "code_imports=DIALOG_DOC_IMPORTS.to_string()",
        "Dependency prerequisites",
        "component-dialog",
        "inject-css",
        "data-slot=\"dialog-source-paths\"",
        "components/dialog/src/mod.rs",
        "components/dialog/src/logic.rs",
        "components/dialog/src/view.rs",
        "components/dialog/src/styles.rs",
        "components/dialog/src/motion.rs",
    ] {
        assert!(
            docs_source.contains(needle),
            "dialog source-first docs should contain `{needle}`.",
        );
    }

    for needle in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "code_imports: Option<String>",
        "<CodeBlock code=resolved_code.get() />",
    ] {
        assert!(
            playground_source.contains(needle),
            "docs playground copy-ready pipeline should contain `{needle}`.",
        );
    }

    for needle in [
        "class_name=\"ui-code-block__copy-button\".to_string()",
        "copy_to_clipboard_aria_label",
    ] {
        assert!(
            code_block_view_source.contains(needle),
            "CodeBlock should keep one-click copy affordance token `{needle}`.",
        );
    }
}

#[test]
fn dialog_dx_check_script_covers_docs_product_copy_paste_ready_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for needle in [
        "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_docs_are_copy_paste_ready_with_imports_and_streaming_snapshot_contract",
        "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_check2_documents_docs_product_copy_paste_ready_rules",
        "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce dialog docs-product guard `{needle}`.",
        );
    }
}

#[test]
fn dialog_check2_documents_docs_product_copy_paste_ready_rules() {
    let source = load_source("../../components/dialog/check2.md");

    for needle in [
        "- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。",
        "Hello World",
        "State Matrix",
        "Controlled vs Uncontrolled",
        "Streaming / Snapshot Contract",
        "DIALOG_DOC_IMPORTS",
        "compose_copy_ready_code",
        "dialog_docs_are_copy_paste_ready_with_imports_and_streaming_snapshot_contract",
        "dialog_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
        "dialog_dx_check_script_covers_docs_product_copy_paste_ready_contract",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "dialog check2 docs-product section should reference `{needle}`.",
        );
    }
}

#[test]
fn dialog_check2_documents_docs_sync_and_state_matrix_rules() {
    let checklist_source = load_source("../../components/dialog/check2.md");

    for required in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "组件行为或参数变更必须同步更新 `apps/docs-app` 示例与说明。",
        "文档示例需覆盖至少一组状态矩阵（受控/非受控、disabled、size/variant 等）。",
        "文档中的 API 名称与默认值必须和 `logic.rs` 当前实现一致。",
    ] {
        assert!(
            checklist_source.contains(required),
            "dialog checklist should keep docs-sync/state-matrix rule `{required}`.",
        );
    }
}

#[test]
fn dialog_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_dialog.rs");
    let view_source = load_source("src/dialog/view.rs");
    let logic_source = load_source("src/dialog/logic.rs");

    dialog_docs_page_covers_primary_playgrounds();
    dialog_docs_playgrounds_lock_state_matrix_contract_values();

    for needle in [
        "pub(super) fn dialog() -> AnyView",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled\"",
        "state_matrix_options.clone()",
        "is_open=if state_matrix_is_controlled.get() {",
        "default_open=if state_matrix_is_controlled.get() {",
        "on_open_change=if state_matrix_is_controlled.get() {",
        "size=state_matrix_size.get()",
        "is_close_button_visible=state_matrix_show_close.get()",
        "is_open=Some(compare_controlled_open)",
        "on_open_change=Some(on_compare_controlled_open_change.clone())",
        "default_open=Some(true)",
        "description=\"default_open initializes once; subsequent transitions stay internal.\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "dialog docs examples should keep state-matrix/API sync marker `{needle}`.",
        );
    }

    for needle in [
        "#[prop(optional)] is_open: Option<Signal<bool>>",
        "#[prop(optional)] open: Option<Signal<bool>>",
        "#[prop(optional)] default_open: Option<bool>",
        "#[prop(optional)] on_open_change: Option<Callback<bool>>",
        "#[prop(optional, default = logic::DEFAULT_SIZE)] size: DialogSize",
        "#[prop(optional, default = logic::DEFAULT_SHOW_CLOSE_BUTTON)] is_close_button_visible: bool",
        "#[prop(optional)] show_close_button: Option<bool>",
    ] {
        assert!(
            view_source.contains(needle),
            "dialog view public API should keep `{needle}` for docs/runtime sync.",
        );
    }

    for needle in [
        "pub const DEFAULT_OPEN: bool = false;",
        "pub const DEFAULT_SHOW_CLOSE_BUTTON: bool = dialog_state::DEFAULT_SHOW_CLOSE_BUTTON;",
        "pub const DEFAULT_SIZE: DialogSize = match dialog_state::DEFAULT_SIZE {",
    ] {
        assert!(
            view_source.contains(needle) || logic_source.contains(needle),
            "dialog parameter model marker `{needle}` should remain in implementation."
        );
    }
}

#[test]
fn dialog_dx_check_script_covers_docs_sync_state_matrix_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for needle in [
        "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_check2_documents_docs_sync_and_state_matrix_rules",
        "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce docs-sync/state-matrix guard `{needle}`.",
        );
    }
}

#[test]
fn dialog_check2_marks_docs_sync_and_state_matrix_contract_complete() {
    let source = load_source("../../components/dialog/check2.md");

    for needle in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "dialog_check2_documents_docs_sync_and_state_matrix_rules",
        "dialog_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
        "dialog_dx_check_script_covers_docs_sync_state_matrix_contract",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "dialog check2 docs-sync/state-matrix section should reference `{needle}`.",
        );
    }
}

#[test]
fn dialog_check2_documents_documentation_as_product_rules() {
    let checklist_source = load_source("../../components/dialog/check2.md");

    for required in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。",
        "文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。",
        "“只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。",
    ] {
        assert!(
            checklist_source.contains(required),
            "dialog checklist should keep documentation-as-product rule `{required}`.",
        );
    }
}

#[test]
fn dialog_documentation_entry_exists_with_beginner_first_progression() {
    let readme = load_source("src/dialog/README.md");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_dialog.rs");

    for needle in [
        "# Dialog",
        "## Hello World",
        "## 常见用法",
        "## 先用起来，再进阶",
        "<Dialog id_base=\"docs-dialog-hello\".to_string() title=\"Hello dialog\".to_string() default_open=Some(true)>",
        "默认路径：`Hello World -> Dialog`（先跑通）",
        "进阶调参：`Interactive Playground -> Scenario Comparison`",
    ] {
        assert!(
            readme.contains(needle),
            "dialog README should include beginner-friendly marker `{needle}`.",
        );
    }

    for needle in [
        "pub(super) fn dialog() -> AnyView",
        "title=\"Dialog\"",
        "slug=\"dialog\"",
        "title=\"Hello World\"",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled\"",
        "title=\"Interactive Playground\"",
        "title=\"Scenario Comparison\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "dialog docs entry should include `{needle}`.",
        );
    }

    let readme_hello = readme
        .find("## Hello World")
        .expect("Dialog README should include Hello World section");
    let readme_common = readme
        .find("## 常见用法")
        .expect("Dialog README should include common usage section");
    let readme_progressive = readme
        .find("## 先用起来，再进阶")
        .expect("Dialog README should include beginner-to-advanced section");
    let readme_display = readme
        .find("## 展示区（Display）")
        .expect("Dialog README should include display section");
    assert!(
        readme_hello < readme_common
            && readme_common < readme_progressive
            && readme_progressive < readme_display,
        "Dialog README should keep default path before advanced guidance.",
    );

    let docs_hello = docs_source
        .find("title=\"Hello World\"")
        .expect("Dialog docs should include Hello World playground");
    let docs_matrix = docs_source
        .find("title=\"State Matrix\"")
        .expect("Dialog docs should include state matrix playground");
    let docs_controlled = docs_source
        .find("title=\"Controlled vs Uncontrolled\"")
        .expect("Dialog docs should include controlled/uncontrolled playground");
    let docs_advanced = docs_source
        .find("title=\"Interactive Playground\"")
        .expect("Dialog docs should include interactive playground");

    assert!(
        docs_hello < docs_matrix
            && docs_matrix < docs_controlled
            && docs_controlled < docs_advanced,
        "Dialog docs should keep beginner-first order before advanced controls.",
    );
}

#[test]
fn dialog_dx_check_script_covers_documentation_as_product_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for needle in [
        "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_check2_documents_documentation_as_product_rules",
        "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_documentation_entry_exists_with_beginner_first_progression",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce documentation-as-product contract `{needle}`.",
        );
    }
}

#[test]
fn dialog_check2_marks_documentation_as_product_contract_complete() {
    let check2_source = load_source("../../components/dialog/check2.md");

    for needle in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "dialog_check2_documents_documentation_as_product_rules",
        "dialog_documentation_entry_exists_with_beginner_first_progression",
        "dialog_dx_check_script_covers_documentation_as_product_contract",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "dialog check2 should keep documentation-as-product evidence marker `{needle}`.",
        );
    }
}

#[test]
fn dialog_check2_documents_interactive_playground_rules() {
    let checklist_source = load_source("../../components/dialog/check2.md");

    for required in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "Playground 至少支持基础 props 调整、状态切换、交互反馈观察。",
        "对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。",
        "Playground 作为验收面，需可重复复现关键交互路径。",
    ] {
        assert!(
            checklist_source.contains(required),
            "dialog checklist should keep interactive-playground rule `{required}`.",
        );
    }
}

#[test]
fn dialog_docs_app_provides_interactive_playground_for_props_state_and_preview() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_dialog.rs");

    dialog_docs_page_covers_primary_playgrounds();
    dialog_docs_playgrounds_lock_state_matrix_contract_values();

    for needle in [
        "title=\"Interactive Playground\"",
        "description=\"展示 / Config / Code / CSS Test 集成工作台（含多场景对比）。\"",
        "code_signal=workbench_code",
        "test_css_source=workbench_test_css_source",
        "test_source_path=\"components/dialog/src/styles.rs\".to_string()",
        "test_config_signal=workbench_actual_config",
        "controls=move || view!",
        "data-slot=\"dialog-workbench\"",
        "data-slot=\"dialog-e2e-open-workbench\"",
        "id_base=\"docs-dialog-workbench\".to_string()",
        "size=workbench_size.get()",
        "show_close_button=workbench_show_close.get()",
        "title=\"Streaming / Snapshot Contract\"",
        "data-requested-stream-mode=move || stream_requested_mode.get()",
        "data-requested-output-status=move || stream_requested_output_status.get()",
        "effective component markers: data-stream-mode=snapshot data-stream-fallback=snapshot data-output-status=verified",
    ] {
        assert!(
            docs_source.contains(needle),
            "dialog docs playground should keep interactive marker `{needle}`.",
        );
    }
}

#[test]
fn dialog_interactive_playground_reuses_repeatable_semantic_e2e_flow() {
    let e2e_source = load_source("../../e2e/tests/docs_app_dialog_contract.spec.mjs");

    for needle in [
        "docs-app dialog interactive + comparison playgrounds stay contract-stable",
        "await page.goto(\"/#/components/dialog\");",
        "const workbench = page.locator('[data-slot=\"dialog-workbench\"]').first();",
        "[data-slot=\"dialog-e2e-open-workbench\"]",
        "[data-slot=\"dialog-e2e-close-workbench\"]",
        "docs-app dialog key flow is repeatable with semantic breakpoints",
        "await page.keyboard.press(\"Enter\");",
        "await page.reload();",
        "await expectDialogSettledClosed(workbenchPanel, workbenchDialog, workbenchOverlay);",
    ] {
        assert!(
            e2e_source.contains(needle),
            "dialog interactive playground should keep repeatable semantic e2e marker `{needle}`."
        );
    }
}

#[test]
fn dialog_dx_check_script_covers_interactive_playground_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for needle in [
        "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_check2_documents_interactive_playground_rules",
        "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_interactive_playground_reuses_repeatable_semantic_e2e_flow",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce dialog interactive playground guard `{needle}`.",
        );
    }
}

#[test]
fn dialog_check2_marks_interactive_playground_contract_complete() {
    let source = load_source("../../components/dialog/check2.md");

    for needle in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "dialog_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "dialog_interactive_playground_reuses_repeatable_semantic_e2e_flow",
        "dialog_dx_check_script_covers_interactive_playground_contract",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "dialog check2 interactive-playground section should reference `{needle}`.",
        );
    }
}

#[test]
fn dialog_dx_check_script_covers_source_first_copy_paste_ready_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for needle in [
        "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_check2_documents_source_first_copy_paste_ready_rules",
        "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce dialog source-first guard `{needle}`.",
        );
    }
}

#[test]
fn dialog_check2_documents_source_first_copy_paste_ready_rules() {
    let source = load_source("../../components/dialog/check2.md");

    for needle in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
        "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
        "文档代码与当前实现必须同步，防止示例漂移。",
        "dialog_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
        "dialog_dx_check_script_covers_source_first_copy_paste_ready_contract",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "dialog check2 source-first section should reference `{needle}`.",
        );
    }
}

#[test]
fn dialog_check2_marks_source_first_copy_paste_ready_contract_complete() {
    let source = load_source("../../components/dialog/check2.md");

    for needle in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "dialog_check2_documents_source_first_copy_paste_ready_rules",
        "dialog_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
        "dialog_dx_check_script_covers_source_first_copy_paste_ready_contract",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "dialog check2 source-first completion section should reference `{needle}`.",
        );
    }
}

#[test]
fn dialog_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes() {
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let docs_index_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_dialog.rs");
    let readme_source = load_source("../../components/dialog/src/README.md");
    let view_source = load_source("src/dialog/view.rs");
    let logic_source = load_source("src/dialog/logic.rs");

    for needle in [
        "### Dialog 同步记录（2026-02-20）",
        "`Dialog` 参数主轴保持 `is_open/open + on_open_change + default_open`、`size`、`is_close_button_visible/show_close_button`、`close_label`、`motion`、`on_close`、`class_name/lang/dir`",
        "component_doc!(\"Dialog\", \"dialog\", \"Overlays\", overlays::dialog)",
        "`#/components/dialog` 可索引访问。",
        "components/dialog/src/README.md",
        "研究文档补充判定：本轮为 Dialog 参数语义与文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。",
        "参数语义若变更，必须先同步本策略文档与 docs 入口",
    ] {
        assert!(
            strategy_source.contains(needle) || docs_index_source.contains(needle),
            "dialog HeroUI/doc sync record should include `{needle}`."
        );
    }

    for needle in [
        "pub(super) fn dialog() -> AnyView",
        "title=\"Dialog\"",
        "slug=\"dialog\"",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled\"",
        "title=\"Interactive Playground\"",
    ] {
        assert!(
            docs_page_source.contains(needle),
            "dialog docs entry should keep indexable marker `{needle}`."
        );
    }

    for needle in ["# Dialog", "## Hello World", "## 展示区（Display）"] {
        assert!(
            readme_source.contains(needle),
            "dialog README should keep docs entry marker `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional)] is_open: Option<Signal<bool>>",
        "#[prop(optional)] open: Option<Signal<bool>>",
        "#[prop(optional)] default_open: Option<bool>",
        "#[prop(optional)] on_open_change: Option<Callback<bool>>",
        "#[prop(optional)] on_close: Option<OnPress>",
        "#[prop(optional, default = logic::DEFAULT_SIZE)] size: DialogSize",
        "#[prop(optional, default = logic::DEFAULT_SHOW_CLOSE_BUTTON)] is_close_button_visible: bool",
        "#[prop(optional)] show_close_button: Option<bool>",
        "#[prop(optional, default = logic::DEFAULT_CLOSE_LABEL)] close_label: &'static str",
        "#[prop(optional)] motion: DialogMotion",
    ] {
        assert!(
            view_source.contains(needle),
            "dialog view should keep parameter-model marker `{needle}`."
        );
    }

    for needle in [
        "pub const DEFAULT_OPEN: bool = false;",
        "pub const DEFAULT_SHOW_CLOSE_BUTTON: bool = dialog_state::DEFAULT_SHOW_CLOSE_BUTTON;",
        "pub const DEFAULT_SIZE: DialogSize = match dialog_state::DEFAULT_SIZE {",
        "pub const DEFAULT_CLOSE_LABEL: &str = dialog_state::DEFAULT_CLOSE_LABEL;",
    ] {
        assert!(
            logic_source.contains(needle),
            "dialog logic should keep default normalization marker `{needle}`."
        );
    }
}

#[test]
fn dialog_contract_hygiene_script_covers_heroui_strategy_doc_sync_contract() {
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes",
        "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_check2_marks_heroui_strategy_and_component_docs_sync_complete",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene check script should enforce `{needle}`."
        );
    }
}

#[test]
fn dialog_check2_marks_heroui_strategy_and_component_docs_sync_complete() {
    let source = load_source("../../components/dialog/check2.md");

    for needle in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。",
        "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
        "“仅代码更新无文档更新”在接口变更场景下直接判不通过。",
        "dialog_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes",
        "dialog_contract_hygiene_script_covers_heroui_strategy_doc_sync_contract",
        "scripts/check-ui-components-contract-hygiene.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "dialog check2 HeroUI/doc sync section should reference `{needle}`.",
        );
    }
}

#[test]
fn dialog_check2_documents_semantics_first_testing_rules() {
    let checklist_source = load_source("../../components/dialog/check2.md");

    for required in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "每个交互组件至少有对应 `*_semantics.rs` 测试覆盖关键状态轴与动作语义。",
        "断言应聚焦语义契约（状态来源/可访问性/键盘路径），快照仅作补充。",
        "新增/变更语义字段必须同步补测试，否则不得打勾。",
    ] {
        assert!(
            checklist_source.contains(required),
            "dialog checklist should keep semantics-first testing rule `{required}`.",
        );
    }
}

#[test]
fn dialog_semantics_suite_is_contract_first_not_snapshot_only() {
    let semantics_source = load_source("tests/dialog_semantics.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_dialog_contract.spec.mjs");

    for required in [
        "dialog_view_uses_logic_contracts_and_source_markers",
        "dialog_wires_aria_ids_and_optional_description_semantics",
        "dialog_e2e_contract_uses_semantic_selectors",
        "dialog_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
        "dialog_e2e_contract_covers_ready_and_settled_conditions_for_overlay_paths",
        "dialog_check2_documents_e2e_repeatable_key_flow_rules",
        "dialog_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
        "data-state=root_state.state_attr",
        "data-open-mode=open_mode_attr",
        "data-open-source=open_source_attr",
        "data-open-change-source=open_change_source_attr",
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-stream-mode=move || agent_contract.get().stream_mode.as_str()",
        "data-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
        "data-output-status=move || agent_contract.get().output_status.as_str()",
    ] {
        assert!(
            semantics_source.contains(required),
            "dialog semantic test suite should assert contract marker `{required}`.",
        );
    }

    for required in [
        "docs-app dialog exposes stable role/source markers",
        "docs-app dialog closes via escape",
        "docs-app dialog interactive + comparison playgrounds stay contract-stable",
        "await page.locator('[data-slot=\"dialog-e2e-open-marker\"]').first().click();",
        "await waitForWasmReady(page);",
        "expectDialogReady(page, overlayPanel, dialogRoot)",
        "expectDialogSettledClosed(overlayPanel, dialogRoot, overlayRoot)",
        "docs-app dialog key flow is repeatable with semantic breakpoints",
        "await openDefaultButton.focus();",
        "await page.keyboard.press(\"Enter\");",
        "await page.reload();",
        "await expect(dialogRoot).toHaveAttribute(\"data-state\", \"with-description\")",
        "await expect(dialogRoot).toHaveAttribute(\"data-motion-source\", \"custom\")",
        "await overlayPanel.press(\"Escape\");",
    ] {
        assert!(
            e2e_source.contains(required),
            "dialog e2e should keep role/data/aria-path marker `{required}`.",
        );
    }

    let forbidden = [
        ["assert", "_snapshot!("].concat(),
        ["insta::assert", "_snapshot!("].concat(),
        ["to_match", "_snapshot("].concat(),
        ["image", "_snapshot("].concat(),
        ["toHave", "Screenshot"].concat(),
        ["toMatch", "Snapshot"].concat(),
    ];

    for forbidden in forbidden {
        assert!(
            !semantics_source.contains(&forbidden) && !e2e_source.contains(&forbidden),
            "dialog semantics should not rely on snapshot-only assertion `{forbidden}` as primary signal.",
        );
    }
}

#[test]
fn dialog_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks() {
    let view_source = load_source("src/dialog/view.rs");
    let semantics_source = load_source("tests/dialog_semantics.rs");

    for marker in [
        "data-slot=root_state.slot_attr",
        "data-state=root_state.state_attr",
        "data-open-mode=open_mode_attr",
        "data-id-source=root_state.id_source_attr",
        "data-title-source=root_state.title_source_attr",
        "data-description-source=root_state.description_source_attr",
        "data-close-source=root_state.close_source_attr",
        "data-motion-source=root_state.motion_source_attr",
        "data-open-source=open_source_attr",
        "data-open-change-source=open_change_source_attr",
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "data-stream-mode=move || agent_contract.get().stream_mode.as_str()",
        "data-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
        "data-output-status=move || agent_contract.get().output_status.as_str()",
    ] {
        assert!(
            view_source.contains(marker),
            "dialog view should expose semantic marker `{marker}`.",
        );
        assert!(
            semantics_source.contains(marker),
            "dialog semantic marker `{marker}` changed without matching semantics assertion update.",
        );
    }
}

#[test]
fn dialog_contract_hygiene_script_covers_semantics_first_contract_guards() {
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_check2_documents_semantics_first_testing_rules",
        "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_semantics_suite_is_contract_first_not_snapshot_only",
        "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn dialog_check2_documents_e2e_selector_and_stable_wait_rules() {
    let checklist_source = load_source("../../components/dialog/check2.md");

    for required in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。",
        "WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。",
        "若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。",
    ] {
        assert!(
            checklist_source.contains(required),
            "dialog checklist should keep e2e selector/stable-wait rule `{required}`.",
        );
    }
}

#[test]
fn dialog_e2e_selector_contract_uses_semantic_markers_and_settled_waits() {
    let e2e_source = load_source("../../e2e/tests/docs_app_dialog_contract.spec.mjs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_dialog.rs");

    for needle in [
        "page.goto(\"/#/components/dialog\")",
        "body:not(:has(#boot))",
        "waitForWasmReady(page)",
        "[data-slot=\"dialog-e2e-open-default\"]",
        "[data-slot=\"dialog-e2e-open-marker\"]",
        "[data-slot=\"dialog-e2e-open-workbench\"]",
        "[data-slot=\"dialog-e2e-open-compare-default\"]",
        "[data-slot=\"dialog-e2e-open-compare-compact\"]",
        "[data-slot=\"dialog-e2e-open-compare-motion\"]",
        "[data-slot=\"overlay-panel\"][role=\"dialog\"]",
        "[data-slot=\"dialog\"]",
        "[data-slot=\"overlay\"]",
        "expectDialogReady(page, overlayPanel, dialogRoot)",
        "expectDialogSettledClosed(",
        "toHaveAttribute(\"data-open\", \"true\")",
        "toHaveAttribute(\"data-ui-schema\", \"dialog\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "dialog e2e selector/stable-wait contract should include `{needle}`.",
        );
    }

    for needle in [
        "data-slot=\"dialog-e2e-open-default\"",
        "data-slot=\"dialog-e2e-open-marker\"",
        "data-slot=\"dialog-e2e-open-workbench\"",
        "data-slot=\"dialog-e2e-open-compare-default\"",
        "data-slot=\"dialog-e2e-open-compare-compact\"",
        "data-slot=\"dialog-e2e-open-compare-motion\"",
        "data-slot=\"dialog-e2e-close-marker\"",
        "data-slot=\"dialog-e2e-close-workbench\"",
        "data-slot=\"dialog-e2e-close-compare-default\"",
        "data-slot=\"dialog-e2e-close-compare-compact\"",
        "data-slot=\"dialog-e2e-close-compare-motion\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "dialog docs source should keep e2e semantic anchor `{needle}`.",
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "sleep(",
        "getByText(",
        "toHaveScreenshot(",
        "toMatchSnapshot(",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "dialog e2e contract should avoid flaky/snapshot selector token `{forbidden}`.",
        );
    }
}

#[test]
fn dialog_e2e_contract_covers_ready_and_settled_conditions_for_overlay_paths() {
    let e2e_source = load_source("../../e2e/tests/docs_app_dialog_contract.spec.mjs");

    for needle in [
        "async function expectDialogReady(page, overlayPanel, dialogRoot)",
        "async function expectDialogSettledClosed(overlayPanel, dialogRoot, overlayRoot)",
        "toHaveAttribute(\"data-state\", \"open\")",
        "toHaveAttribute(\"data-open\", \"true\")",
        "toHaveAttribute(\"data-stream-mode\", \"snapshot\")",
        "toHaveAttribute(\"data-stream-fallback\", \"snapshot\")",
        "toHaveAttribute(\"data-output-status\", \"verified\")",
        "await overlayPanel.press(\"Escape\");",
        "await overlayPanel.locator('[data-slot=\"dialog-e2e-close-marker\"]').first().click();",
        "await workbenchPanel.locator('[data-slot=\"dialog-e2e-close-workbench\"]').first().click();",
        "await defaultPanel.locator('[data-slot=\"dialog-e2e-close-compare-default\"]').first().click();",
        "await compactPanel.locator('[data-slot=\"dialog-e2e-close-compare-compact\"]').first().click();",
        "await motionPanel.locator('[data-slot=\"dialog-e2e-close-compare-motion\"]').first().click();",
        "await expectDialogSettledClosed(overlayPanel, dialogRoot, overlayRoot);",
        "await expectDialogSettledClosed(workbenchPanel, workbenchDialog, workbenchOverlay);",
        "await expectDialogSettledClosed(defaultPanel, defaultDialog, defaultOverlay);",
        "await expectDialogSettledClosed(compactPanel, compactDialog, compactOverlay);",
        "await expectDialogSettledClosed(motionPanel, motionDialog, motionOverlay);",
    ] {
        assert!(
            e2e_source.contains(needle),
            "dialog e2e ready/settled contract should include `{needle}`.",
        );
    }
}

#[test]
fn dialog_check2_documents_e2e_repeatable_key_flow_rules() {
    let checklist_source = load_source("../../components/dialog/check2.md");

    for required in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。",
        "回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。",
        "高风险路径（overlay、focus、keyboard、async）优先进入回归集合。",
    ] {
        assert!(
            checklist_source.contains(required),
            "dialog checklist should keep repeatable key-flow rule `{required}`.",
        );
    }
}

#[test]
fn dialog_e2e_key_flow_is_repeatable_and_failure_points_are_semantic() {
    let e2e_source = load_source("../../e2e/tests/docs_app_dialog_contract.spec.mjs");

    for needle in [
        "docs-app dialog key flow is repeatable with semantic breakpoints",
        "await openDefaultButton.focus();",
        "await expect(openDefaultButton).toBeFocused();",
        "await page.keyboard.press(\"Enter\");",
        "await expect(defaultDialog).toHaveAttribute(\"data-state\", \"with-description\")",
        "await expect(defaultDialog).toHaveAttribute(\"data-close-button\", \"shown\")",
        "await defaultPanel.press(\"Escape\");",
        "await page.reload();",
        "await openWorkbenchButton.focus();",
        "await expect(openWorkbenchButton).toBeFocused();",
        "await expect(workbenchDialog).toHaveAttribute(\"data-state\", \"with-description\")",
        "await expect(workbenchDialog).toHaveAttribute(\"data-close-button\", \"shown\")",
        "await closeWorkbenchButton.focus();",
        "await expect(closeWorkbenchButton).toBeFocused();",
        "await expectDialogSettledClosed(workbenchPanel, workbenchDialog, workbenchOverlay);",
    ] {
        assert!(
            e2e_source.contains(needle),
            "dialog repeatable key-flow contract should include `{needle}`.",
        );
    }

    for forbidden in [
        "toHaveScreenshot(",
        "toMatchSnapshot(",
        "waitForTimeout(",
        "setTimeout(",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "dialog repeatable key-flow should avoid flaky/non-semantic token `{forbidden}`.",
        );
    }
}

#[test]
fn dialog_e2e_check_script_covers_selector_and_settled_wait_contract() {
    assert!(
        path_exists("../../scripts/check-ui-components-e2e-dialog.sh"),
        "dialog e2e check script should exist.",
    );

    let script_source = load_source("../../scripts/check-ui-components-e2e-dialog.sh");

    for needle in [
        "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_check2_documents_e2e_selector_and_stable_wait_rules",
        "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
        "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_e2e_contract_covers_ready_and_settled_conditions_for_overlay_paths",
    ] {
        assert!(
            script_source.contains(needle),
            "dialog e2e check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn dialog_e2e_check_script_covers_selector_and_key_flow_contracts() {
    assert!(
        path_exists("../../scripts/check-ui-components-e2e-dialog.sh"),
        "dialog e2e check script should exist.",
    );

    let script_source = load_source("../../scripts/check-ui-components-e2e-dialog.sh");

    for needle in [
        "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_check2_documents_e2e_selector_and_stable_wait_rules",
        "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
        "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
    ] {
        assert!(
            script_source.contains(needle),
            "dialog e2e key-flow check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn dialog_check2_marks_e2e_selector_stability_item_complete() {
    let check2_source = load_source("../../components/dialog/check2.md");

    for needle in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "components/dialog/test/semantics.rs::dialog_check2_documents_e2e_selector_and_stable_wait_rules",
        "components/dialog/test/semantics.rs::dialog_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
        "components/dialog/test/semantics.rs::dialog_e2e_contract_covers_ready_and_settled_conditions_for_overlay_paths",
        "components/dialog/test/semantics.rs::dialog_e2e_check_script_covers_selector_and_settled_wait_contract",
        "components/dialog/test/dialog_semantics.rs::dialog_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
        "scripts/check-ui-components-e2e-dialog.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "dialog check2 e2e selector stability section should include `{needle}`.",
        );
    }
}

#[test]
fn dialog_check2_marks_e2e_repeatable_key_flow_contract_complete() {
    let check2_source = load_source("../../components/dialog/check2.md");

    for needle in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "components/dialog/test/semantics.rs::dialog_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
        "components/dialog/test/semantics.rs::dialog_e2e_check_script_covers_selector_and_key_flow_contracts",
        "components/dialog/test/dialog_semantics.rs::dialog_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
        "scripts/check-ui-components-e2e-dialog.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "dialog check2 repeatable key-flow section should include `{needle}`.",
        );
    }
}

#[test]
fn dialog_version_deprecation_migration_is_na_without_major_breaking_upgrade() {
    let manifest_source = load_source("../../components/dialog/src/Component.toml");
    let rbi_source = load_source("../../components/dialog/src/dialog.rbi");
    let mod_source = load_source("../../components/dialog/src/mod.rs");
    let logic_source = load_source("../../components/dialog/src/logic.rs");
    let view_source = load_source("../../components/dialog/src/view.rs");
    let styles_source = load_source("../../components/dialog/src/styles.rs");
    let motion_source = load_source("../../components/dialog/src/motion.rs");
    let protocol_source = load_source("../../components/dialog/src/protocol.rs");
    let check2_source = load_source("../../components/dialog/check2.md");

    for needle in [
        "schema_version = \"1\"",
        "name = \"Dialog\"",
        "crate = \"ui-dialog\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "dialog manifest should keep stable v1 schema marker `{needle}`."
        );
    }

    for needle in [
        "pub fn Dialog(",
        "is_open: Option<leptos::prelude::Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<leptos::prelude::Callback<bool>>",
    ] {
        assert!(
            rbi_source.contains(needle),
            "dialog RBI should keep stable public API marker `{needle}`."
        );
    }

    let combined = format!(
        "{mod_source}\n{logic_source}\n{view_source}\n{styles_source}\n{motion_source}\n{protocol_source}"
    );
    for forbidden in [
        "migrate_v1_to_v2",
        "migrate_v2_to_v3",
        "deprecation_window",
        "deprecated_since",
        "schema_version = \"2\"",
        "contract.v2",
        "SchemaRegistry",
    ] {
        assert!(
            !manifest_source.contains(forbidden)
                && !rbi_source.contains(forbidden)
                && !combined.contains(forbidden),
            "dialog should not introduce major-version migration marker `{forbidden}` in current scope."
        );
    }

    for needle in [
        "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。",
        "N/A：本次 `Dialog` 未发生跨大版本 API 破坏升级",
        "schema_version = \"1\"",
        "dialog_version_deprecation_migration_is_na_without_major_breaking_upgrade",
        "scripts/check-ui-components-engineering.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "dialog/check2.md should keep version-migration governance marker `{needle}`."
        );
    }
}

#[test]
fn dialog_version_deprecation_migration_script_covers_engineering_gate() {
    let script_source = load_source("../../scripts/check-ui-components-engineering.sh");

    let marker = "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_version_deprecation_migration_is_na_without_major_breaking_upgrade";
    assert!(
        script_source.contains(marker),
        "engineering check script should enforce `{marker}`."
    );
}

#[test]
fn dialog_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries() {
    let protocol_source = load_source("src/dialog/protocol.rs");
    let protocol_test_source = load_source("../../components/dialog/test/protocol.rs");
    let mod_source = load_source("src/dialog/mod.rs");
    let logic_source = load_source("src/dialog/logic.rs");
    let view_source = load_source("src/dialog/view.rs");
    let styles_source = load_source("src/dialog/styles.rs");
    let motion_source = load_source("src/dialog/motion.rs");
    let readme_source = load_source("../../components/dialog/src/README.md");
    let checklist_source = load_source("../../components/dialog/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-engineering.sh");

    let spec_path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../components/dialog/src/spec.rs");
    assert!(
        !spec_path.exists(),
        "Dialog should not introduce `spec.rs`; protocol is carried by `src/protocol.rs`."
    );

    for needle in [
        "use serde::{Deserialize, Serialize};",
        "pub enum DialogComponentSchemaVersion",
        "#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]",
        "#[serde(rename_all = \"snake_case\")]",
        "pub struct DialogComponentSpec",
        "#[serde(default)]",
        "pub schema_version: DialogComponentSchemaVersion",
    ] {
        assert!(
            protocol_source.contains(needle),
            "dialog protocol should keep structured serde schema marker `{needle}`.",
        );
    }

    for needle in [
        "fn protocol_types_implement_serde_contract()",
        "assert_serde::<DialogComponentSchemaVersion>();",
        "assert_serde::<DialogComponentSpec>();",
    ] {
        assert!(
            protocol_test_source.contains(needle),
            "dialog protocol tests should keep serde contract marker `{needle}`.",
        );
    }

    let combined = [
        mod_source.as_str(),
        logic_source.as_str(),
        view_source.as_str(),
        styles_source.as_str(),
        motion_source.as_str(),
    ]
    .join("\n");

    assert!(
        combined.contains("use_controllable_open_state_traced(") && combined.contains("\"dialog\""),
        "dialog should reuse shared traced controllable-state semantics.",
    );

    for forbidden in [
        "tracing::span!(",
        "tracing::event!(",
        "#[tracing::instrument]",
        "target: \"ui_components::dialog::",
        "const DIALOG_TRACE_TARGET",
    ] {
        assert!(
            !combined.contains(forbidden),
            "dialog should avoid component-local tracing semantic drift token `{forbidden}`.",
        );
    }

    for source in [
        mod_source.as_str(),
        logic_source.as_str(),
        view_source.as_str(),
        styles_source.as_str(),
        motion_source.as_str(),
        protocol_source.as_str(),
        readme_source.as_str(),
    ] {
        for forbidden in [
            "tokio",
            "tokio::",
            "async_std",
            "async_std::",
            "async-std",
            "runtime::Handle",
            "smol::",
            "spawn_blocking(",
        ] {
            assert!(
                !source.contains(forbidden),
                "dialog engineering contract should not leak runtime marker `{forbidden}`.",
            );
        }
    }

    let script_needle = "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries";
    assert!(
        script_source.contains(script_needle),
        "engineering gate script should include `{script_needle}`.",
    );

    for needle in [
        "- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "dialog_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries",
        "scripts/check-ui-components-engineering.sh",
    ] {
        assert!(
            checklist_source.contains(needle),
            "dialog checklist should keep engineering governance marker `{needle}`.",
        );
    }
}

#[test]
fn dialog_styles_use_defensive_variable_fallback_chain() {
    let styles_source = load_source("src/dialog/styles.rs");
    let checklist_source = load_source("../../components/dialog/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for needle in [
        "var(--ui-overlay-panel-min-width, var(--ui-fallback-overlay-panel-min-width))",
        "var(--ui-space-md, var(--ui-fallback-space-md))",
        "var(--ui-space-xs, var(--ui-fallback-space-xs))",
        "var(--ui-space-sm, var(--ui-fallback-space-sm))",
        "var(--ui-space-3xs, var(--ui-fallback-space-3xs))",
        "var(--ui-heading-h5-font-size,",
        "var(--ui-fallback-heading-h5-font-size)",
        "var(--ui-heading-h5-line-height,",
        "var(--ui-fallback-heading-h5-line-height)",
        "var(--ui-font-size-150, var(--ui-fallback-font-size-150))",
        "var(--ui-line-height-150, var(--ui-fallback-line-height-150))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
    ] {
        assert!(
            styles_source.contains(needle),
            "dialog styles should keep defensive fallback variable marker `{needle}`.",
        );
    }

    for forbidden in ["var(--ui-space-3xs);", "#", "px", "rem", "em", "vh", "vw"] {
        assert!(
            !styles_source.contains(forbidden),
            "dialog styles should avoid hardcoded terminal literal `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_styles_use_defensive_variable_fallback_chain";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene script should include `{script_needle}`.",
    );

    for needle in [
        "- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。",
        "dialog_styles_use_defensive_variable_fallback_chain",
        "scripts/check-ui-components-contract-hygiene.sh",
    ] {
        assert!(
            checklist_source.contains(needle),
            "dialog checklist should keep defensive-variable marker `{needle}`.",
        );
    }
}

#[test]
fn dialog_cascade_layer_and_runtime_style_contract_is_enforced() {
    let css_entry_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let view_source = load_source("../../components/dialog/src/view.rs");
    let styles_source = load_source("../../components/dialog/src/styles.rs");
    let checklist_source = load_source("../../components/dialog/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for needle in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-dialog\")]",
        "out.push_str(crate::dialog::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            css_entry_source.contains(needle),
            "ui-components css entry should enforce cascade-layer contract `{needle}`.",
        );
    }

    for needle in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "<style>{move || css_text.get()}</style>",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should keep centralized css injection contract `{needle}`.",
        );
    }

    assert!(
        !view_source.contains(" style="),
        "dialog view should not embed plain inline style assignments."
    );

    for forbidden in [
        "style=\"top:",
        "style=\"left:",
        "style=\"width:",
        "style=\"height:",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "dialog view should not include fragile inline style token `{forbidden}`."
        );
    }

    for (line_index, line) in view_source.lines().enumerate() {
        if let Some(pos) = line.find("style:") {
            let key = line[pos + "style:".len()..]
                .split(|c: char| c == '=' || c.is_whitespace() || c == '>')
                .next()
                .unwrap_or_default()
                .trim();
            assert!(
                key.starts_with("--"),
                "dialog runtime style should only set css custom properties; found `style:{key}` at line {}.",
                line_index + 1
            );
        }
    }

    for needle in ["pub const CSS: &str", ".ui-dialog", "var(--ui-"] {
        assert!(
            styles_source.contains(needle),
            "dialog styles should remain static token css contract `{needle}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_cascade_layer_and_runtime_style_contract_is_enforced";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene script should include `{script_needle}`."
    );

    for needle in [
        "- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。",
        "dialog_cascade_layer_and_runtime_style_contract_is_enforced",
        "scripts/check-ui-components-contract-hygiene.sh",
        "crates/ui-components/src/css.rs",
        "crates/ui-components/src/root.rs",
        "components/dialog/src/view.rs",
    ] {
        assert!(
            checklist_source.contains(needle),
            "dialog checklist should keep cascade-layer marker `{needle}`."
        );
    }
}

#[test]
fn dialog_wires_aria_ids_and_optional_description_semantics() {
    let source = load_source("src/dialog/view.rs");

    for needle in [
        "let title_id = format!(\"{id_base}-title\")",
        "aria_labelledby=title_id.clone()",
        "let description_id = format!(\"{id_base}-description\")",
        "if root_state.show_description",
        "aria_describedby=description_id.clone()",
        "<Show when=move || root_state.show_description>",
        "data-slot=description_state.slot_attr",
        "data-description-source=description_state.description_source_attr",
    ] {
        assert!(
            source.contains(needle),
            "Dialog should include `{needle}` for stable a11y description wiring."
        );
    }
}

#[test]
fn dialog_supports_headless_locale_contract_with_lang_dir_attrs() {
    let source = load_source("src/dialog/view.rs");

    for needle in [
        "use ui_headless::{A11yDirection, locale_attrs, use_controllable_open_state_traced};",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "let locale = locale_attrs(logic::normalize_optional_text(lang), dir);",
        "lang=move || locale_lang.with_value(|value| value.clone())",
        "dir=locale_dir",
    ] {
        assert!(
            source.contains(needle),
            "Dialog should include `{needle}` to keep headless locale attrs contract stable."
        );
    }
}

#[test]
fn dialog_close_button_uses_button_icon_only_with_aria_label() {
    let source = load_source("src/dialog/view.rs");

    for needle in [
        "data-slot=close_state.slot_attr",
        "<Button",
        "aria_label=close_label",
    ] {
        assert!(
            source.contains(needle),
            "Dialog close button should be accessible and stable (`{needle}`)."
        );
    }
}

#[test]
fn dialog_styles_include_state_and_source_marker_selectors() {
    let source = load_source("src/dialog/styles.rs");

    for selector in [
        ".ui-dialog[data-motion-source=\"custom\"]",
        ".ui-dialog[data-custom-motion=\"true\"]",
        ".ui-dialog--custom-motion",
        ".ui-dialog[data-size-source=\"custom\"]",
        ".ui-dialog[data-custom-size=\"true\"]",
        ".ui-dialog--custom-size",
        ".ui-dialog[data-id-source=\"custom\"]",
        ".ui-dialog[data-custom-id=\"true\"]",
        ".ui-dialog--custom-id",
        ".ui-dialog[data-title-source=\"custom\"]",
        ".ui-dialog[data-custom-title=\"true\"]",
        ".ui-dialog--custom-title",
        ".ui-dialog[data-description-source=\"custom\"]",
        ".ui-dialog[data-custom-description=\"true\"]",
        ".ui-dialog--custom-description",
        ".ui-dialog[data-close-source=\"custom\"]",
        ".ui-dialog[data-custom-close=\"true\"]",
        ".ui-dialog--custom-close",
        ".ui-dialog[data-exit-source=\"custom\"]",
        ".ui-dialog[data-custom-exit=\"true\"]",
        ".ui-dialog--custom-exit",
        ".ui-dialog--with-description",
        ".ui-dialog[data-state=\"with-description\"]",
        ".ui-dialog--title-only",
        ".ui-dialog[data-close-button=\"hidden\"]",
        ".ui-dialog__title[data-slot=\"dialog-title\"]",
        ".ui-dialog__description[data-slot=\"dialog-description\"]",
        ".ui-dialog__body[data-slot=\"dialog-body\"]",
    ] {
        assert!(
            source.contains(selector),
            "Dialog styles should include `{selector}` as stable state/source marker contracts."
        );
    }
}

#[test]
fn dialog_styles_consume_ui_theme_tokens_and_avoid_hardcoded_dialog_size_literals() {
    let source = load_source("src/dialog/styles.rs");

    for needle in [
        "var(--ui-overlay-panel-min-width, var(--ui-fallback-overlay-panel-min-width))",
        "var(--ui-space-md, var(--ui-fallback-space-md))",
        "var(--ui-space-xs, var(--ui-fallback-space-xs))",
        "var(--ui-space-sm, var(--ui-fallback-space-sm))",
        "var(--ui-heading-h5-font-size,",
        "var(--ui-fallback-heading-h5-font-size)",
        "var(--ui-heading-h5-line-height,",
        "var(--ui-fallback-heading-h5-line-height)",
        "var(--ui-font-size-150, var(--ui-fallback-font-size-150))",
        "var(--ui-line-height-150, var(--ui-fallback-line-height-150))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
    ] {
        assert!(
            source.contains(needle),
            "Dialog styles should consume ui-theme token/fallback variable `{needle}`."
        );
    }

    for forbidden in [
        "520px", "380px", "480px", "640px", "16px", "24px", "14px", "20px",
    ] {
        assert!(
            !source.contains(forbidden),
            "Dialog styles should avoid hardcoded dialog sizing/typography literal `{forbidden}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn dialog_motion_contract_exposes_default_and_custom_overlay_checks() {
    let source = load_source("src/dialog/motion.rs");

    for needle in [
        "pub struct DialogMotion",
        "pub overlay: crate::overlay::OverlayMotion",
        "fn default_motion_uses_default_overlay_motion_contract()",
        "fn supports_custom_overlay_motion_contract()",
    ] {
        assert!(
            source.contains(needle),
            "Dialog motion module should include `{needle}` for baseline-level contract coverage."
        );
    }
}

#[test]
fn dialog_docs_page_contains_state_source_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays_dialog.rs");

    for needle in [
        "pub(super) fn dialog() -> AnyView",
        "title=\"Dialog\"",
        "slug=\"dialog\"",
        "State + Source Markers",
        "data-id-source",
        "data-title-source",
        "data-description-source",
        "data-close-source",
        "data-motion-source",
        "<Dialog",
    ] {
        assert!(
            source.contains(needle),
            "dialog docs page should contain `{needle}`."
        );
    }
}

#[test]
fn dialog_docs_page_exposes_interactive_display_config_code_css_test_contract() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays_dialog.rs");

    for needle in [
        "<Playground",
        "title=\"Interactive Playground\"",
        "description=\"展示 / Config / Code / CSS Test 集成工作台（含多场景对比）。\"",
        "code_signal=workbench_code",
        "test_css_source=workbench_test_css_source",
        "test_source_path=\"components/dialog/src/styles.rs\".to_string()",
        "test_config_signal=workbench_actual_config",
        "controls=move || view!",
        "title=\"Scenario Comparison\"",
        "code_signal=scenario_code",
    ] {
        assert!(
            source.contains(needle),
            "dialog docs page should include `{needle}` for interactive display/config/code/css-test + comparison coverage.",
        );
    }
}

#[test]
fn dialog_readme_covers_display_config_code_css_test_and_comparison_sections() {
    let source = load_source("src/dialog/README.md");

    for needle in [
        "## 展示区（Display）",
        "## Config 区",
        "## Code 区",
        "## CSS Test 区",
        "## 多种情况对比显示",
    ] {
        assert!(
            source.contains(needle),
            "dialog README should include `{needle}` for required documentation structure.",
        );
    }
}

#[test]
fn dialog_e2e_contract_uses_semantic_selectors() {
    let source = load_source("../../e2e/tests/docs_app_dialog_contract.spec.mjs");

    for needle in [
        "docs-app dialog exposes stable role/source markers",
        "docs-app dialog interactive + comparison playgrounds stay contract-stable",
        "/#/components/dialog",
        "dialog-e2e-open-default",
        "dialog-e2e-open-marker",
        "dialog-e2e-open-workbench",
        "dialog-e2e-open-compare-default",
        "dialog-e2e-open-compare-compact",
        "dialog-e2e-open-compare-motion",
        "dialog-e2e-close-marker",
        "dialog-e2e-close-workbench",
        "dialog-e2e-close-compare-default",
        "dialog-e2e-close-compare-compact",
        "dialog-e2e-close-compare-motion",
        "expectDialogReady(",
        "expectDialogSettledClosed(",
        "data-slot=\"overlay-panel\"",
        "data-slot=\"overlay\"",
        "role=\"dialog\"",
        "data-id-source",
        "data-title-source",
        "data-description-source",
        "data-close-source",
        "data-motion-source",
        "Escape",
    ] {
        assert!(
            source.contains(needle),
            "dialog e2e contract should include `{needle}` for stable semantic regression coverage."
        );
    }

    for forbidden in ["getByRole(", "getByText(", "waitForTimeout(", "setTimeout("] {
        assert!(
            !source.contains(forbidden),
            "dialog e2e selectors should avoid brittle/non-semantic token `{forbidden}`.",
        );
    }
}

#[test]
fn dialog_motion_module_stays_as_contract_mapping_without_custom_engine() {
    let source = load_source("src/dialog/motion.rs");

    for needle in [
        "pub struct DialogMotion",
        "pub fn sanitize_motion(motion: DialogMotion) -> DialogMotion",
        "overlay: crate::overlay::motion::sanitize_motion(motion.overlay)",
        "pub fn attach_motion(",
        "crate::overlay::motion::attach_motion(node_ref, is_open, finish_exit, motion.overlay);",
    ] {
        assert!(
            source.contains(needle),
            "Dialog motion layer should keep contract-mapping delegate `{needle}`."
        );
    }

    for forbidden in [
        "request_animation_frame",
        "requestAnimationFrame",
        "web_sys::window",
        "Animation::new",
        "keyframe_sampling",
    ] {
        assert!(
            !source.contains(forbidden),
            "Dialog motion should not implement component-local motion driver internals `{forbidden}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn dialog_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("src/dialog/motion.rs");
    let view_source = load_source("src/dialog/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: DialogMotion) -> DialogMotion",
        "overlay: crate::overlay::motion::sanitize_motion(motion.overlay)",
        "fn sanitize_motion_delegates_to_overlay_contract()",
    ] {
        assert!(
            motion_source.contains(needle),
            "Dialog motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    assert!(
        view_source.contains("let motion = crate::dialog::motion::sanitize_motion(motion);"),
        "Dialog view should sanitize motion before forwarding to Overlay.",
    );
}

#[test]
fn dialog_docs_page_locks_custom_motion_marker_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays_dialog.rs");

    for needle in [
        "title=\"State + Source Markers\"",
        "motion=DialogMotion {",
        "overlay: OverlayMotion {",
        "initial_scale: 0.94",
        "initial_y_px: 14.0",
        "data-motion-source",
    ] {
        assert!(
            source.contains(needle),
            "dialog docs page should include `{needle}` for motion/source marker regression stability."
        );
    }
}

#[test]
fn dialog_docs_default_playground_locks_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays_dialog.rs");

    for needle in [
        "<Playground",
        "title=\"Dialog\"",
        "code_signal=code",
        "code_imports=DIALOG_DOC_IMPORTS.to_string()",
        "<Button data-slot=\"dialog-e2e-open-default\" on_press=open_dialog>",
        "id_base=\"docs-dialog\".to_string()",
        "title=\"Dialog title\".to_string()",
        "description=\"Uses Overlay + header/body/footer layout.\".to_string()",
        "<Button variant=ButtonVariant::Secondary on_press=on_close>\"Cancel\"</Button>",
        "<Button on_press=on_close>\"Confirm\"</Button>",
        "on_exit_complete=on_exit_complete",
    ] {
        assert!(
            source.contains(needle),
            "dialog docs default playground should contain `{needle}`.",
        );
    }
}

#[test]
fn dialog_docs_hello_world_provides_minimal_default_entrypoint() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays_dialog.rs");

    for needle in [
        "<Playground",
        "title=\"Hello World\"",
        "code_signal=hello_code",
        "description=\"最小可用默认路径：不需要手动接线 primitives/headless 状态机。\"",
        "<Button on_press=open_hello_dialog>\"Open hello dialog\"</Button>",
        "id_base=\"docs-dialog-hello\".to_string()",
        "title=\"Hello dialog\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "dialog docs hello-world entrypoint should include `{needle}`."
        );
    }

    let hello_code = r#"<Dialog id_base="docs-dialog-hello".to_string() title="Hello dialog".to_string() default_open=Some(true)>
  <div>"Hello dialog body"</div>
</Dialog>"#;
    assert!(
        source.contains(hello_code),
        "dialog hello-world snippet should stay <=5 lines and copy-paste ready."
    );
}

#[test]
fn dialog_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays_dialog.rs");

    for needle in [
        "pub(super) fn dialog() -> AnyView",
        "title=\"Dialog\"",
        "slug=\"dialog\"",
        "description=\"Dialog panel with header/body/footer structure on top of Overlay.\"",
        "title=\"Hello World\"",
        "code_signal=hello_code",
        "title=\"Dialog\"",
        "code_signal=code",
        "title=\"State + Source Markers\"",
        "code_signal=marker_code",
        "data-id-source",
        "data-title-source",
        "data-description-source",
        "data-close-source",
        "data-motion-source",
    ] {
        assert!(
            source.contains(needle),
            "overlays_dialog docs page should include `{needle}` for dialog primary coverage.",
        );
    }
}

#[test]
fn dialog_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays_dialog.rs");

    for needle in [
        "id_base=\"docs-dialog\".to_string()",
        "title=\"Dialog title\".to_string()",
        "description=\"Uses Overlay + header/body/footer layout.\".to_string()",
        "<Button variant=ButtonVariant::Secondary on_press=on_close>\"Cancel\"</Button>",
        "<Button on_press=on_close>\"Confirm\"</Button>",
        "on_exit_complete=on_exit_complete",
        "id_base=\"docs-dialog-marker\".to_string()",
        "title=\"Marker dialog\".to_string()",
        "description=\"Custom size, class, close label, and motion for contract inspection.\"",
        "size=DialogSize::Lg",
        "close_label=\"Dismiss dialog\"",
        "class_name=\"docs-dialog-custom\".to_string()",
        "motion=DialogMotion {",
        "overlay: OverlayMotion {",
        "initial_scale: 0.94",
        "initial_y_px: 14.0",
        "data-slot=\"dialog-e2e-open-marker\"",
        "\"open: \"",
    ] {
        assert!(
            source.contains(needle),
            "dialog docs playgrounds should contain `{needle}`.",
        );
    }
}
