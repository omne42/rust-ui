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
fn combo_box_docs_section(source: &str) -> &str {
    let start = source
        .find("pub(super) fn combo_box() -> AnyView")
        .expect("combo_box docs function should exist");
    let tail = &source[start..];
    let end = tail
        .find("pub(super) fn autocomplete() -> AnyView")
        .expect("combo_box docs section should end before autocomplete docs function");
    &tail[..end]
}

#[test]
fn combo_box_does_not_expose_logic_or_view_modules() {
    let source = load_source("../../components/combo-box/src/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ComboBox internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn combo_box_uses_logic_state_model() {
    let view_source = load_source("../../components/combo-box/src/view.rs");
    let logic_source = load_source("../../components/combo-box/src/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/combo_box.rs");

    for needle in [
        "pub use ui_state_primitives::combo_box::{",
        "ComboBoxStateInput",
        "ComboBoxState",
        "RootDataState",
        "resolve_root_data_state",
        "normalize_optional_text",
        "normalize_id_base",
        "normalize_disabled_indices",
        "resolve_state",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "ComboBox logic should include `{needle}` while consuming centralized ui-state-primitives."
        );
    }

    for needle in [
        "pub struct ComboBoxStateInput",
        "pub struct ComboBoxState",
        "pub fn normalize_optional_text(",
        "pub fn normalize_id_base(",
        "pub fn normalize_disabled_indices(",
        "pub fn filter_indices(",
        "pub fn map_selected_to_filtered(",
        "pub fn map_filtered_to_original(",
        "pub fn resolve_state(",
    ] {
        assert!(
            primitive_source.contains(needle),
            "ComboBox primitive source should define `{needle}`."
        );
    }

    for needle in [
        "let root_state = logic::normalize_root_state(logic::RootStateInput {",
        "logic::filter_indices(items.as_ref(), &query.get(), has_typed.get())",
        "logic::map_selected_to_filtered(selected_index.get(), &filtered_indices.get())",
        "logic::map_filtered_to_original(filtered_index, &indices)",
        "logic::resolve_root_data_state(is_open.get(), state.is_disabled).as_attr()",
        "let state = root_state.state;",
        "let class = root_state.class_name;",
    ] {
        assert!(
            view_source.contains(needle),
            "ComboBox view should derive wrapper state through logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn combo_box_logic_does_not_reimplement_reusable_state_primitives() {
    let logic_source = load_source("../../components/combo-box/src/logic.rs");

    for forbidden in [
        "pub fn normalize_disabled_indices(",
        "pub fn filter_indices(",
        "pub fn map_selected_to_filtered(",
        "pub fn map_filtered_to_original(",
        "pub fn resolve_state(",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "ComboBox logic should not reimplement reusable primitive `{forbidden}`; it must consume ui-state-primitives instead.",
        );
    }
}

#[test]
fn combo_box_component_has_store_adapter_boundary() {
    let view_source = load_source("../../components/combo-box/src/view.rs");
    let logic_source = load_source("../../components/combo-box/src/logic.rs");

    for forbidden in ["GlobalState", "AppState", "Store<", "SignalStore", "apps::"] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "ComboBox should not bind app/business store type `{forbidden}` directly.",
        );
    }
}

#[test]
fn combo_box_discrete_data_state_is_enum_backed() {
    let view_source = load_source("../../components/combo-box/src/view.rs");
    let logic_source = load_source("../../components/combo-box/src/logic.rs");

    for needle in [
        "pub enum RootDataState",
        "Open,",
        "Disabled,",
        "Closed,",
        "pub fn resolve_root_data_state(",
        "pub const fn as_attr(self) -> &'static str",
    ] {
        assert!(
            logic_source.contains(needle),
            "ComboBox logic should model discrete data-state with `{needle}`."
        );
    }

    assert!(
        view_source
            .contains("logic::resolve_root_data_state(is_open.get(), state.is_disabled).as_attr()"),
        "ComboBox view should map data-state via typed enum contract instead of inline string branching."
    );
}

#[test]
fn combo_box_supports_controlled_and_uncontrolled_open_state() {
    let source = load_source("../../components/combo-box/src/view.rs");
    let logic_source = load_source("../../components/combo-box/src/logic.rs");

    for needle in [
        "is_open: Option<Signal<bool>>",
        "open: Option<Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<Callback<bool>>",
        "pub struct OpenStateInput",
        "pub struct OpenState",
        "pub fn normalize_open_state(",
    ] {
        assert!(
            source.contains(needle) || logic_source.contains(needle),
            "ComboBox should accept `{needle}` for controlled/uncontrolled open state."
        );
    }
}

#[test]
fn combo_box_wires_open_value_change_default_triplet_into_headless_state() {
    let source = load_source("../../components/combo-box/src/view.rs");

    for needle in [
        "let normalized_open_state = logic::normalize_open_state(logic::OpenStateInput {",
        "let open_state = overlay_open::use_controllable_open_state_traced(",
        "\"combo-box\",",
        "open,",
        "default_open,",
        "on_open_change,",
        "let is_open = open_state.open;",
        "let set_open = open_state.request_open_change;",
    ] {
        assert!(
            source.contains(needle),
            "ComboBox open axis should wire `{needle}` for stable controlled/uncontrolled semantics.",
        );
    }
}

#[test]
fn combo_box_supports_is_prefixed_boolean_props_with_legacy_aliases() {
    let source = load_source("../../components/combo-box/src/view.rs");
    let logic_source = load_source("../../components/combo-box/src/logic.rs");

    for needle in [
        "is_disabled: Option<bool>",
        "disabled: bool",
        "is_required: Option<Signal<bool>>",
        "required: Option<Signal<bool>>",
        "is_invalid: Option<Signal<bool>>",
        "invalid: Option<Signal<bool>>",
        "pub struct AccessibilityStateInput",
        "pub struct AccessibilityState",
        "pub fn normalize_accessibility_state(",
        "is_disabled: input.is_disabled.unwrap_or(input.disabled)",
        "let required = input",
        ".is_required",
        ".or(input.required)",
        "let invalid = input",
        ".is_invalid",
        ".or(input.invalid)",
    ] {
        assert!(
            source.contains(needle) || logic_source.contains(needle),
            "ComboBox API naming contract should include `{needle}`."
        );
    }

    for needle in [
        "let accessibility_state =",
        "logic::normalize_accessibility_state(logic::AccessibilityStateInput {",
        "let is_disabled = accessibility_state.is_disabled;",
        "let required = accessibility_state.required;",
        "let invalid = accessibility_state.invalid;",
    ] {
        assert!(
            source.contains(needle),
            "ComboBox view should consume normalized accessibility state via `{needle}`."
        );
    }
}

#[test]
fn combo_box_view_does_not_inline_default_fallback_rules() {
    let source = load_source("../../components/combo-box/src/view.rs");

    for forbidden in [
        "is_disabled.unwrap_or(disabled)",
        "is_required.or(required)",
        "is_invalid.or(invalid)",
        "is_open.or(open)",
        "unwrap_or_else(|| Signal::derive(|| false))",
        "logic::normalize_id_base(",
        "logic::normalize_label(",
        "logic::resolve_placeholder(",
        "logic::resolve_state(",
    ] {
        assert!(
            !source.contains(forbidden),
            "ComboBox view.rs should not own fallback/priority rule `{forbidden}`; keep it in logic.rs.",
        );
    }
}

#[test]
fn combo_box_normalizes_label_placeholder_and_id_base() {
    let view_source = load_source("../../components/combo-box/src/view.rs");
    let logic_source = load_source("../../components/combo-box/src/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/combo_box.rs");

    for needle in [
        "normalize_label(",
        "resolve_placeholder(",
        "normalize_id_base(",
    ] {
        assert!(
            logic_source.contains(needle),
            "ComboBox logic should use `{needle}` to keep text and id semantics stable."
        );
    }

    assert!(
        view_source.contains("logic::normalize_root_state(logic::RootStateInput {"),
        "ComboBox view should delegate normalization to logic::normalize_root_state."
    );

    for needle in [
        "pub const DEFAULT_LABEL: &str = \"Options\"",
        "pub const DEFAULT_ID_BASE: &str = \"combo-box\"",
        "pub const DEFAULT_PLACEHOLDER: &str = \"Select…\"",
    ] {
        assert!(
            primitive_source.contains(needle),
            "ComboBox primitives should provide fallback semantics via `{needle}`."
        );
    }
}

#[test]
fn combo_box_escape_stops_propagation_when_open() {
    let source = load_source("../../components/combo-box/src/view.rs");

    for needle in [
        "let key_result = aria.handlers.on_input_key_down.run(key);",
        "if key_result.handled {",
        "if key_result.stop_propagation {",
    ] {
        assert!(
            source.contains(needle),
            "ComboBox should consume typed headless keydown outcomes via `{needle}` to keep keyboard semantics out of view.rs."
        );
    }
}

#[test]
fn combo_box_passes_lang_dir_and_headless_aria_controls_contract() {
    let source = load_source("../../components/combo-box/src/view.rs");

    for needle in [
        "lang: Option<String>",
        "dir: Option<A11yDirection>",
        "lang,",
        "dir,",
        "aria-controls=move || aria.input.aria_controls.get()",
        "lang=aria.input.lang.clone()",
        "dir=aria.input.dir",
        "lang=aria.listbox.lang.clone()",
        "dir=aria.listbox.dir",
    ] {
        assert!(
            source.contains(needle),
            "ComboBox should wire `{needle}` so locale + aria-controls semantics come from ui-headless contract."
        );
    }
}

#[test]
fn combo_box_panel_is_portaled_and_uses_popover_positioning() {
    let source = load_source("../../components/combo-box/src/view.rs");

    for needle in [
        "<Portal>",
        "use_popover_position",
        "data-ui-overlay-portal",
        "--ui-popover-top",
    ] {
        assert!(
            source.contains(needle),
            "ComboBox panel should include `{needle}` for baseline-style popover behavior."
        );
    }
}

#[test]
fn combo_box_panel_exposes_option_and_empty_state_slots() {
    let source = load_source("../../components/combo-box/src/view.rs");

    for needle in [
        "data-slot=\"combo-box-listbox\"",
        "data-empty=move || filtered_indices.get().is_empty().then_some(\"true\")",
        "data-slot=\"combo-box-option\"",
        "data-focused=move || (active_index.get() == filtered_index).then_some(\"true\")",
        "data-slot=\"combo-box-empty\"",
    ] {
        assert!(
            source.contains(needle),
            "ComboBox panel should expose `{needle}` for deterministic style/test hooks."
        );
    }
}

#[test]
fn combo_box_uses_presence_for_motion_safe_unmounting() {
    let source = load_source("../../components/combo-box/src/view.rs");

    for needle in [
        "use_presence(is_open)",
        "on_exit_complete=presence.finish_exit",
    ] {
        assert!(
            source.contains(needle),
            "ComboBox should use `{needle}` so popover exit motion can finish before unmount."
        );
    }
}

#[test]
fn combo_box_emits_baseline_style_state_data_attributes() {
    let source = load_source("../../components/combo-box/src/view.rs");

    for attr in [
        "data-slot=\"combo-box\"",
        "data-state=move ||",
        "data-open=move || is_open.get().then_some(\"true\")",
        "data-closed=move || (!is_open.get()).then_some(\"true\")",
        "data-disabled=state.is_disabled.then_some(\"true\")",
        "data-enabled=state.is_enabled.then_some(\"true\")",
        "data-empty=move || (filtered_count.get() == 0).then_some(\"true\")",
        "data-has-items=state.has_items.then_some(\"true\")",
        "data-has-filtered-items=move || (filtered_count.get() > 0).then_some(\"true\")",
        "data-selection-empty=move || selected_index.get().is_none().then_some(\"true\")",
        "data-has-selection=move || selected_index.get().is_some().then_some(\"true\")",
        "data-invalid=move || invalid.get().then_some(\"true\")",
        "data-valid=move || (!invalid.get()).then_some(\"true\")",
        "data-required=move || required.get().then_some(\"true\")",
        "data-optional=move || (!required.get()).then_some(\"true\")",
        "data-has-description=state.has_description.then_some(\"true\")",
        "data-has-error=state.has_error.then_some(\"true\")",
        "data-has-disabled-options=state.has_disabled_options.then_some(\"true\")",
        "data-controlled=state.is_controlled.then_some(\"true\")",
        "data-uncontrolled=state.is_uncontrolled.then_some(\"true\")",
        "data-label-source=state.label_source_attr",
        "data-description-source=state.description_source_attr",
        "data-error-source=state.error_source_attr",
        "data-placeholder-source=state.placeholder_source_attr",
        "data-id-source=state.id_source_attr",
        "data-class-source=state.class_source_attr",
        "data-motion-source=state.motion_source_attr",
        "data-custom-label=state.has_custom_label.then_some(\"true\")",
        "data-custom-description=state.has_custom_description.then_some(\"true\")",
        "data-custom-error=state.has_custom_error.then_some(\"true\")",
        "data-custom-placeholder=state.has_custom_placeholder.then_some(\"true\")",
        "data-custom-id=state.has_custom_id_base.then_some(\"true\")",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-custom-motion=state.has_custom_motion.then_some(\"true\")",
        "data-typed=move || has_typed.get().then_some(\"true\")",
        "data-count=state.item_count.to_string()",
        "data-filtered-count=move || filtered_count.get().to_string()",
        "data-disabled-option-count=state.disabled_option_count.to_string()",
        "data-slot=\"combo-box-trigger\"",
    ] {
        assert!(
            source.contains(attr),
            "ComboBox should set `{attr}` to support baseline-style styling and state inspection."
        );
    }
}

#[test]
fn combo_box_panel_styles_use_fixed_positioning_and_transform_origin_by_placement() {
    let source = load_source("../../components/combo-box/src/styles.rs");

    for needle in [
        "position: fixed;",
        "var(--ui-popover-top",
        "data-placement=\"bottom-start\"",
        ".ui-combo-box__empty",
    ] {
        assert!(
            source.contains(needle),
            "ComboBox styles should include `{needle}` for popover layout and empty-state rendering."
        );
    }
}

#[test]
fn combo_box_styles_include_controlled_and_disabled_option_markers() {
    let source = load_source("../../components/combo-box/src/styles.rs");

    for needle in [
        ".ui-combo-box--controlled",
        ".ui-combo-box[data-controlled=\"true\"]",
        ".ui-combo-box--has-disabled-options",
        ".ui-combo-box[data-has-disabled-options=\"true\"]",
        ".ui-combo-box--empty",
        ".ui-combo-box[data-empty=\"true\"]",
        ".ui-combo-box[data-label-source=\"custom\"]",
        ".ui-combo-box[data-custom-label=\"true\"]",
        ".ui-combo-box--custom-label",
        ".ui-combo-box[data-description-source=\"custom\"]",
        ".ui-combo-box[data-custom-description=\"true\"]",
        ".ui-combo-box--custom-description",
        ".ui-combo-box[data-error-source=\"custom\"]",
        ".ui-combo-box[data-custom-error=\"true\"]",
        ".ui-combo-box--custom-error",
        ".ui-combo-box[data-placeholder-source=\"custom\"]",
        ".ui-combo-box[data-custom-placeholder=\"true\"]",
        ".ui-combo-box--custom-placeholder",
        ".ui-combo-box[data-id-source=\"custom\"]",
        ".ui-combo-box[data-custom-id=\"true\"]",
        ".ui-combo-box--custom-id",
        ".ui-combo-box[data-class-source=\"custom\"]",
        ".ui-combo-box[data-custom-class=\"true\"]",
        ".ui-combo-box--custom-class",
        ".ui-combo-box[data-motion-source=\"custom\"]",
        ".ui-combo-box[data-custom-motion=\"true\"]",
        ".ui-combo-box--custom-motion",
    ] {
        assert!(
            source.contains(needle),
            "ComboBox styles should include `{needle}` for stable state-marker contracts."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn combo_box_motion_contract_exposes_popover_and_highlight_customization() {
    let mod_source = load_source("../../components/combo-box/src/mod.rs");
    let motion_source = load_source("../../components/combo-box/src/motion.rs");

    for needle in [
        "pub mod motion;",
        "pub use motion::ComboBoxMotion;",
        "pub struct ComboBoxMotion",
        "pub popover: PopoverMotion",
        "pub highlight: ActiveHighlightMotion",
        "fn default_motion_uses_default_popover_and_highlight_motion()",
        "fn supports_custom_popover_and_highlight_motion_contracts()",
    ] {
        assert!(
            mod_source.contains(needle) || motion_source.contains(needle),
            "ComboBox motion contract should include `{needle}` for baseline-style spring customization."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn combo_box_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("../../components/combo-box/src/motion.rs");
    let view_source = load_source("../../components/combo-box/src/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: ComboBoxMotion) -> ComboBoxMotion",
        "popover: sanitize_popover_motion(motion.popover)",
        "highlight: sanitize_highlight(motion.highlight)",
        "fn sanitize_motion_falls_back_for_invalid_nested_values()",
    ] {
        assert!(
            motion_source.contains(needle),
            "ComboBox motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    assert!(
        view_source.contains("let motion = crate::motion::sanitize_motion(motion);"),
        "ComboBox view should sanitize motion before attaching popover and active-highlight motion.",
    );
}

#[test]
fn combo_box_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let section = combo_box_docs_section(&source);

    for needle in [
        "title=\"ComboBox\"",
        "slug=\"combo-box\"",
        "description=\"Combobox with input + listbox + popover, baseline-style root attrs, and baseline-level panel/highlight motion.\"",
        "title=\"展示：多场景对比\"",
        "title=\"Workbench（展示 + Config + Code + CSS Test）\"",
        "test_css_source=workbench_test_css",
        "test_config_signal=workbench_actual_config",
        "data-slot=\"combo-box-showcase\"",
        "data-slot=\"combo-box-workbench-controls\"",
        "data-slot=\"combo-box-workbench-canvas\"",
        "<ComboBox",
        "is_open=controlled_open",
        "is_disabled=true",
    ] {
        assert!(
            section.contains(needle),
            "collections docs page should include `{needle}` for combo-box coverage.",
        );
    }
}

#[test]
fn combo_box_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let section = combo_box_docs_section(&source);

    for needle in [
        "id_base=\"docs-combo-box\".to_string()",
        "label=\"Language\".to_string()",
        "disabled_indices=vec![4]",
        "description=\"Pick one runtime language\".to_string()",
        "error=\"Language is required\".to_string()",
        "on_press=Callback::new(move |_| set_invalid.update(|value| *value = !*value))",
        "\"selected: \"",
        "id_base=\"docs-combo-box-controlled\".to_string()",
        "on_open_change=on_open_change",
        "description=\"Open state is externally controlled\".to_string()",
        "\"open: \"",
        "id_base=\"docs-combo-box-disabled\".to_string()",
        "id_base=\"docs-combo-box-empty\".to_string()",
        "placeholder=\"No options\".to_string()",
        "\"disabled selected: \"",
        "\"empty selected: \"",
    ] {
        assert!(
            section.contains(needle),
            "combo-box docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn combo_box_readme_documents_display_config_code_css_test_sections() {
    let source = load_source("../../components/combo-box/src/README.md");

    for needle in [
        "## 展示 (Display)",
        "## Config (Workbench Settings)",
        "## Code (Workbench Snippet)",
        "## CSS Test (Scoped CSS)",
        "collections.rs` 的 `combo_box()`",
        "受控 open（`is_open` + `on_open_change`）",
        "test_css_source",
        "test_config_signal",
    ] {
        assert!(
            source.contains(needle),
            "combo_box README should contain `{needle}` to lock workbench docs contract."
        );
    }
}

#[test]
fn combo_box_check2_has_no_unchecked_checklist_items() {
    let check2_source = load_source("../../components/combo-box/src/check2.md");

    assert!(
        !check2_source.contains("- [ ]"),
        "combo_box/check2.md should not keep unchecked checklist items after sequential verification."
    );
}

#[test]
fn combo_box_check2_marks_async_scope_as_explicit_na() {
    let check2_source = load_source("../../components/combo-box/src/check2.md");

    assert!(
        check2_source.contains("N/A：`ComboBox` 当前仅做本地筛选与选择，无远程请求和异步状态轴"),
        "combo_box/check2.md should explicitly mark async contract as N/A for current component scope."
    );
}

#[test]
fn combo_box_check2_marks_streaming_scope_as_optional_with_snapshot_fallback() {
    let check2_source = load_source("../../components/combo-box/src/check2.md");

    for needle in [
        "归类为 `Streaming Optional`",
        "`fallback=snapshot`",
        "`Snapshot` 渲染为基线",
    ] {
        assert!(
            check2_source.contains(needle),
            "combo_box/check2.md should keep streaming-scope governance marker `{needle}`."
        );
    }
}

#[test]
fn combo_box_check2_documents_semantic_e2e_selector_and_ready_wait_contract() {
    let check2_source = load_source("../../components/combo-box/src/check2.md");

    for needle in [
        "e2e/tests/docs_app_components_coverage.spec.mjs",
        "`data-slot=\"combo-box\"`",
        "`body:not(:has(#boot))`",
    ] {
        assert!(
            check2_source.contains(needle),
            "combo_box/check2.md should keep e2e stability marker `{needle}`."
        );
    }
}

#[test]
fn combo_box_feature_graph_declares_required_motion_dependencies() {
    let cargo_toml = load_source("Cargo.toml");

    assert!(
        cargo_toml.contains(
            "component-combo_box = [\n    \"component-active_highlight\",\n    \"component-popover\",\n    \"dep:ui-combo-box\",\n]"
        ),
        "ui-components feature graph should declare combo_box -> active_highlight/popover/ui-combo-box dependencies for minimal-feature builds."
    );
}
