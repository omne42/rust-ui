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
fn combo_box_uses_is_prefixed_boolean_props_without_alias_drift() {
    let source = load_source("../../components/combo-box/src/view.rs");
    let logic_source = load_source("../../components/combo-box/src/logic.rs");

    for needle in [
        "is_disabled: Option<bool>",
        "is_required: Option<Signal<bool>>",
        "is_invalid: Option<Signal<bool>>",
        "pub struct AccessibilityStateInput",
        "pub struct AccessibilityState",
        "pub fn normalize_accessibility_state(",
        "is_disabled: input.is_disabled.unwrap_or(false)",
        "let required = input",
        ".is_required",
        "let invalid = input",
        ".is_invalid",
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

    for forbidden in [
        "#[prop(optional)] disabled: bool",
        "#[prop(optional, into)] required: Option<Signal<bool>>",
        "#[prop(optional, into)] invalid: Option<Signal<bool>>",
        "#[prop(optional)] open: Option<Signal<bool>>",
        ".or(input.required)",
        ".or(input.invalid)",
        "input.is_open.or(input.open)",
    ] {
        assert!(
            !source.contains(forbidden) && !logic_source.contains(forbidden),
            "ComboBox API naming should not keep legacy alias contract `{forbidden}`."
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
fn combo_box_uses_deterministic_id_provider_for_generated_ids() {
    let view_source = load_source("../../components/combo-box/src/view.rs");
    let logic_source = load_source("../../components/combo-box/src/logic.rs");
    let root_source = load_source("src/root.rs");

    for needle in [
        "use_ui_id_provider",
        "let generated_id_base = use_ui_id_provider()",
        "logic::resolve_id_base(id_base, generated_id_base)",
        "has_custom_id_base",
        "pub fn resolve_id_base(id_base: String, generated_id_base: String) -> String",
    ] {
        assert!(
            view_source.contains(needle) || logic_source.contains(needle),
            "ComboBox should use deterministic id provider path via `{needle}`."
        );
    }

    assert!(
        root_source.contains("provide_ui_id_provider(id_seed);"),
        "UiRoot should provide a seeded UiIdProvider so SSR/hydration ids stay deterministic."
    );

    for forbidden in [
        "Uuid::",
        "uuid::",
        "rand::",
        "thread_rng",
        "SystemTime::now",
        "js_sys::Date",
        "getrandom",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "ComboBox id initialization should not depend on runtime randomness/time `{forbidden}`."
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
        "let option_attrs = aria.option_attrs;",
        "data-slot=\"combo-box-option\"",
        "role=move || option_attrs.run(filtered_index).role",
        "aria-selected=move || option_attrs.run(filtered_index).aria_selected",
        "aria-disabled=move || option_attrs.run(filtered_index).aria_disabled",
        "data-selected=move || option_attrs.run(filtered_index).data_selected",
        "data-focused=move || option_attrs.run(filtered_index).data_focused",
        "data-disabled=move || option_attrs.run(filtered_index).data_disabled",
        "data-slot=\"combo-box-empty\"",
    ] {
        assert!(
            source.contains(needle),
            "ComboBox panel should expose `{needle}` for deterministic style/test hooks."
        );
    }
}

#[test]
fn combo_box_wires_pointer_handlers_through_headless_contract() {
    let source = load_source("../../components/combo-box/src/view.rs");

    for needle in [
        "let on_option_pointer_move = aria.handlers.on_option_pointer_move;",
        "let on_option_click = aria.handlers.on_option_click;",
        "on:pointermove=move |_| on_option_pointer_move.run(filtered_index)",
        "on:click=move |_| on_option_click.run(filtered_index)",
    ] {
        assert!(
            source.contains(needle),
            "ComboBox should wire pointer path via typed headless handlers `{needle}`."
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
        "data-focused=move || focus_ring.is_focused.get().then_some(\"true\")",
        "data-focus-visible=move || focus_ring.is_focus_visible.get().then_some(\"true\")",
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
fn combo_box_styles_use_defensive_variable_fallback_chain() {
    let styles_source = load_source("../../components/combo-box/src/styles.rs");
    let theme_css_source = load_source("../ui-theme/src/css.rs");

    for needle in [
        "var(--ui-space-xs, var(--ui-fallback-space-xs))",
        "var(--ui-space-sm, var(--ui-fallback-space-sm))",
        "var(--ui-space-md, var(--ui-fallback-space-md))",
        "var(--ui-font-size-150, var(--ui-fallback-font-size-150))",
        "var(--ui-line-height-150, var(--ui-fallback-line-height-150))",
        "var(--ui-font-size-100, var(--ui-fallback-font-size-100))",
        "var(--ui-line-height-100, var(--ui-fallback-line-height-100))",
        "var(--ui-component-height-100, var(--ui-fallback-component-height-100))",
        "var(--ui-border-width, var(--ui-fallback-border-width))",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-radius-md, var(--ui-fallback-radius-md))",
        "var(--ui-radius-lg, var(--ui-fallback-radius-lg))",
        "var(--ui-bg, var(--ui-fallback-bg))",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
        "var(--ui-focus-ring, var(--ui-fallback-focus-ring))",
        "var(--ui-danger, var(--ui-fallback-danger))",
        "var(--ui-accent-soft, var(--ui-fallback-accent-soft))",
        "var(--ui-shadow-md, var(--ui-fallback-shadow-md))",
        "var(--ui-overlay-panel-min-width, var(--ui-fallback-overlay-panel-min-width))",
        "var(--ui-overlay-viewport-inset, var(--ui-fallback-overlay-viewport-inset))",
        "var(--ui-overlay-z-index, var(--ui-fallback-overlay-z-index))",
        "var(--ui-overlay-enter-offset-y, var(--ui-fallback-overlay-enter-offset-y))",
        "var(--ui-overlay-enter-scale, var(--ui-fallback-overlay-enter-scale))",
    ] {
        assert!(
            styles_source.contains(needle),
            "combo-box styles should keep defensive fallback chain marker `{needle}`."
        );
    }

    for needle in [
        "--ui-fallback-space-xs:",
        "--ui-fallback-space-sm:",
        "--ui-fallback-space-md:",
        "--ui-fallback-font-size-150:",
        "--ui-fallback-line-height-150:",
        "--ui-fallback-font-size-100:",
        "--ui-fallback-line-height-100:",
        "--ui-fallback-component-height-100:",
        "--ui-fallback-border-width:",
        "--ui-fallback-border:",
        "--ui-fallback-radius-md:",
        "--ui-fallback-radius-lg:",
        "--ui-fallback-bg:",
        "--ui-fallback-fg:",
        "--ui-fallback-fg-muted:",
        "--ui-fallback-focus-ring:",
        "--ui-fallback-danger:",
        "--ui-fallback-accent-soft:",
        "--ui-fallback-shadow-md:",
        "--ui-fallback-overlay-panel-min-width:",
        "--ui-fallback-overlay-viewport-inset:",
        "--ui-fallback-overlay-z-index:",
        "--ui-fallback-overlay-enter-offset-y:",
        "--ui-fallback-overlay-enter-scale:",
    ] {
        assert!(
            theme_css_source.contains(needle),
            "ui-theme css should provide fallback terminal `{needle}`."
        );
    }

    for forbidden in ["14px", "20px", "12px", "16px", "240px", "0px"] {
        assert!(
            !styles_source.contains(forbidden),
            "combo-box styles should avoid raw terminal token `{forbidden}`."
        );
    }
}

#[test]
fn combo_box_defensive_variables_check_script_covers_style_fallback_contract() {
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    let needle = "cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_styles_use_defensive_variable_fallback_chain";
    assert!(
        script_source.contains(needle),
        "contract-hygiene check script should enforce `{needle}`."
    );
}

#[test]
fn combo_box_check2_marks_defensive_variables_contract_complete() {
    let source = load_source("../../components/combo-box/check2.md");

    assert!(
        source.contains("- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。"),
        "combo_box check2 should mark defensive-variables gate complete."
    );

    for needle in [
        "combo_box_styles_use_defensive_variable_fallback_chain",
        "combo_box_defensive_variables_check_script_covers_style_fallback_contract",
        "scripts/check-ui-components-contract-hygiene.sh",
        "components/combo-box/src/styles.rs",
        "crates/ui-theme/src/css.rs",
    ] {
        assert!(
            source.contains(needle),
            "combo_box check2 defensive-variables section should reference `{needle}`."
        );
    }
}

#[test]
fn combo_box_cascade_layer_and_runtime_style_contract_is_enforced() {
    let css_entry_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let view_source = load_source("../../components/combo-box/src/view.rs");

    for needle in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-combo_box\")]",
        "out.push_str(crate::combo_box::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            css_entry_source.contains(needle),
            "ui-components css entry should enforce cascade-layer contract `{needle}`."
        );
    }

    for needle in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "<style>{move || css_text.get()}</style>",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should keep centralized css injection contract `{needle}`."
        );
    }

    for forbidden in [
        "style=\"top:",
        "style=\"left:",
        "style=\"right:",
        "style=\"bottom:",
        "style=\"width:",
        "style=\"height:",
        "style=\"position:",
        "style:top=",
        "style:left=",
        "style:right=",
        "style:bottom=",
        "style:width=",
        "style:height=",
        "style:position=",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "combo-box view should not include plain inline style token `{forbidden}`."
        );
    }

    let style_lines: Vec<&str> = view_source
        .lines()
        .filter(|line| line.contains("style="))
        .collect();
    assert_eq!(
        style_lines.len(),
        1,
        "combo-box view should keep a single runtime style binding for css vars."
    );
    assert!(
        style_lines[0].contains("style=panel_vars"),
        "combo-box runtime style binding should route through `panel_vars`."
    );

    for needle in [
        "let panel_vars = move || {",
        "--ui-popover-top: {}px;",
        "--ui-popover-left: {}px;",
        "--ui-popover-anchor-width: {}px;",
        "position.top_px.get()",
        "position.left_px.get()",
        "position.anchor_width_px.get()",
    ] {
        assert!(
            view_source.contains(needle),
            "combo-box runtime style payload should stay css-custom-property-only via `{needle}`."
        );
    }
}

#[test]
fn combo_box_cascade_layer_check_script_covers_contract() {
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    let needle = "cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_cascade_layer_and_runtime_style_contract_is_enforced";
    assert!(
        script_source.contains(needle),
        "contract-hygiene check script should enforce `{needle}`."
    );
}

#[test]
fn combo_box_check2_marks_cascade_layer_contract_complete() {
    let source = load_source("../../components/combo-box/check2.md");

    assert!(
        source.contains("- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。"),
        "combo_box check2 should mark cascade-layer gate complete."
    );

    for needle in [
        "combo_box_cascade_layer_and_runtime_style_contract_is_enforced",
        "combo_box_cascade_layer_check_script_covers_contract",
        "scripts/check-ui-components-contract-hygiene.sh",
        "crates/ui-components/src/css.rs",
        "crates/ui-components/src/root.rs",
        "components/combo-box/src/view.rs",
    ] {
        assert!(
            source.contains(needle),
            "combo_box check2 cascade-layer section should reference `{needle}`."
        );
    }
}

#[test]
fn combo_box_ui_theme_layer_consumes_tokens_without_rebuilding_theme() {
    let styles_source = load_source("../../components/combo-box/src/styles.rs");
    let logic_source = load_source("../../components/combo-box/src/logic.rs");
    let view_source = load_source("../../components/combo-box/src/view.rs");
    let tokens_source = load_source("../ui-theme/src/tokens.rs");
    let theme_source = load_source("../ui-theme/src/theme.rs");
    let css_source = load_source("../ui-theme/src/css.rs");
    let docs_source = load_source("../../docs/spec/styling.md");
    let baseline_source = load_source("../ui-theme/tests/token_scale_baseline.rs");

    for needle in [
        "var(--ui-space-xs)",
        "var(--ui-space-sm)",
        "var(--ui-space-md)",
        "var(--ui-fg)",
        "var(--ui-fg-muted)",
        "var(--ui-bg)",
        "var(--ui-border)",
        "var(--ui-radius-md)",
        "var(--ui-radius-lg)",
        "var(--ui-shadow-md)",
        "var(--ui-focus-ring)",
        "var(--ui-danger)",
        "var(--ui-overlay-panel-min-width",
        "var(--ui-overlay-viewport-inset",
        "var(--ui-overlay-enter-offset-y",
        "var(--ui-overlay-enter-scale",
        "var(--ui-overlay-z-index",
    ] {
        assert!(
            styles_source.contains(needle),
            "ComboBox styles should consume theme token variable `{needle}` from ui-theme output."
        );
    }

    for forbidden in [
        "ui_theme::",
        "ThemeContext",
        "ThemeSystem",
        "ThemeColor",
        "ThemeScale",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "ComboBox component layer should not rebuild theme context/type `{forbidden}`."
        );
    }

    for needle in [
        "pub struct OverlayLayoutTokens",
        "pub struct ThemeTokens",
        "pub fn overlay_layout_tokens(ctx: ThemeContext) -> OverlayLayoutTokens",
        "--ui-overlay-panel-min-width:",
        "--ui-overlay-viewport-inset:",
        "--ui-overlay-enter-offset-y:",
        "--ui-overlay-enter-scale:",
        "--ui-overlay-z-index:",
        "Token 统一基线落点固定",
        "tokens.rs` 定义",
        "theme.rs` 映射",
        "css.rs` 输出变量",
        "WCAG 2.1 AA",
        "fn token_scale_baselines_are_regression_testable()",
        "fn overlay_layout_tokens_follow_scale_baseline()",
    ] {
        assert!(
            tokens_source.contains(needle)
                || theme_source.contains(needle)
                || css_source.contains(needle)
                || docs_source.contains(needle)
                || baseline_source.contains(needle),
            "Theme baseline contract evidence should include `{needle}`."
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
fn combo_box_ui_motion_layer_delegates_popover_driver_to_shared_contract() {
    let motion_source = load_source("../../components/combo-box/src/motion.rs");

    for needle in [
        "pub use ui_popover::PopoverMotion;",
        "ui_popover::motion::sanitize_motion(",
        "ui_popover::motion::attach_motion(",
    ] {
        assert!(
            motion_source.contains(needle),
            "ComboBox motion should delegate popover driver via `{needle}` instead of reimplementing runtime executors."
        );
    }

    for forbidden in ["SpringAnimator::new(", "set_property(\"--ui-popover-"] {
        assert!(
            !motion_source.contains(forbidden),
            "ComboBox motion should not own low-level popover driver details `{forbidden}`."
        );
    }
}

#[test]
fn combo_box_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe() {
    let combo_box_motion_source = load_source("../../components/combo-box/src/motion.rs");
    let combo_box_motion_test_source = load_source("../../components/combo-box/test/motion.rs");
    let combo_box_view_source = load_source("../../components/combo-box/src/view.rs");
    let popover_motion_source = load_source("../../components/popover/src/motion.rs");
    let active_highlight_source =
        load_source("../../crates/ui-visual-primitive/src/active_highlight.rs");
    let ui_motion_lib_source = load_source("../../crates/ui-motion/src/lib.rs");
    let ui_motion_spring_source = load_source("../../crates/ui-motion/src/spring.rs");

    for needle in [
        "pub struct ComboBoxMotion {",
        "pub popover: PopoverMotion,",
        "pub highlight: ActiveHighlightMotion,",
        "fn sanitize_spring(",
        "fn sanitize_popover_spring(",
        "ui_motion::spring::sanitize_config(value, default)",
        "pub fn sanitize_popover_motion(motion: PopoverMotion) -> PopoverMotion",
        "pub fn sanitize_motion(motion: ComboBoxMotion) -> ComboBoxMotion",
        "pub fn attach_popover_motion(",
        "ui_popover::motion::attach_motion(",
        "sanitize_popover_motion(motion),",
    ] {
        assert!(
            combo_box_motion_source.contains(needle),
            "combo-box motion module should keep component-scoped motion contract marker `{needle}`."
        );
    }

    for needle in [
        "fn supports_custom_popover_and_highlight_motion_contracts()",
        "stiffness: 240.0,",
        "damping: 22.0,",
        "fn sanitize_motion_falls_back_for_invalid_nested_values()",
        "stiffness: f64::NAN,",
        "damping: -1.0,",
    ] {
        assert!(
            combo_box_motion_test_source.contains(needle),
            "combo-box motion regression suite should include `{needle}`."
        );
    }

    for needle in [
        "let motion = crate::motion::sanitize_motion(motion);",
        "motion::attach_popover_motion(",
        "motion.popover,",
        "motion.highlight,",
        "ui_visual_primitive::active_highlight::attach_active_highlight_motion(",
    ] {
        assert!(
            combo_box_view_source.contains(needle),
            "combo-box view should attach and forward motion contract via `{needle}`."
        );
    }

    for needle in [
        "pub struct PopoverMotion {",
        "pub spring: ui_motion::spring::SpringConfig,",
        "stiffness: 300.0,",
        "damping: 25.0,",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "if ui_motion::web::prefers_reduced_motion() {",
        "on_exit_complete.run(());",
    ] {
        assert!(
            popover_motion_source.contains(needle),
            "shared popover motion should keep platform-safe reduced-motion/no-op marker `{needle}`."
        );
    }

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            active_highlight_source.contains(needle),
            "active-highlight motion should keep non-wasm no-op marker `{needle}`."
        );
    }

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_lib_source.contains(needle),
            "ui-motion non-wasm stub contract should include `{needle}`."
        );
    }

    for needle in [
        "if crate::web::prefers_reduced_motion() {",
        "self.inner.state.set(SpringState::new(target));",
        "(self.inner.apply.borrow_mut())(target);",
    ] {
        assert!(
            ui_motion_spring_source.contains(needle),
            "ui-motion spring should keep reduced-motion fast path `{needle}`."
        );
    }

    for forbidden in ["request_animation_frame", "web_sys::", "wasm_bindgen::"] {
        assert!(
            !combo_box_motion_source.contains(forbidden),
            "combo-box motion should avoid runtime/backend coupling token `{forbidden}`."
        );
    }
}

#[test]
fn combo_box_motion_contract_platform_script_covers_guard() {
    let source = load_source("../../scripts/check-ui-components-platforms.sh");

    let needle = "cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe";
    assert!(
        source.contains(needle),
        "platform check script should enforce `{needle}`.",
    );
}

#[test]
fn combo_box_check2_marks_motion_contractualization_complete() {
    let source = load_source("../../components/combo-box/check2.md");

    assert!(
        source.contains("- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。"),
        "combo-box check2 should mark motion contractualization gate complete.",
    );

    for needle in [
        "ComboBoxMotion` + `sanitize_motion` + `attach_popover_motion`",
        "supports_custom_popover_and_highlight_motion_contracts",
        "stiffness: 240.0",
        "damping: 22.0",
        "if ui_motion::web::prefers_reduced_motion() {",
        "pub fn prefers_reduced_motion() -> bool",
        "combo_box_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe",
        "combo_box_motion_contract_platform_script_covers_guard",
        "cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "combo-box check2 motion contractualization section should reference `{needle}`.",
        );
    }
}

#[test]
fn combo_box_ui_components_fixed_entry_files_follow_layered_boundaries() {
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let active_highlight_source = load_source("../ui-visual-primitive/src/active_highlight.rs");
    let controllable_state_source =
        load_source("../../crates/ui-headless/src/controllable_state.rs");
    let presence_source = load_source("../../crates/ui-headless/src/presence.rs");
    let a11y_source = load_source("../../crates/ui-headless/src/a11y.rs");

    for needle in [
        "mod css;",
        "pub mod root;",
        "pub use root::UiRoot;",
        "#[cfg(feature = \"component-combo_box\")]",
        "pub use ui_combo_box as combo_box;",
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
        "#[cfg(feature = \"component-combo_box\")]",
        "out.push_str(crate::combo_box::styles::CSS);",
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
        "ComboBox",
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
        "../../crates/ui-headless/src/controllable_state.rs",
        "../../crates/ui-headless/src/presence.rs",
        "../../crates/ui-headless/src/a11y.rs",
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
}

#[test]
fn combo_box_entrypoints_check_script_covers_fixed_entrypoint_contract() {
    let script_source = load_source("../../scripts/check-ui-components-entrypoints.sh");

    let needle = "cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_ui_components_fixed_entry_files_follow_layered_boundaries";
    assert!(
        script_source.contains(needle),
        "entrypoints check script should enforce `{needle}`."
    );
}

#[test]
fn combo_box_check2_marks_ui_components_fixed_entry_files_contract_complete() {
    let source = load_source("../../components/combo-box/check2.md");

    for needle in [
        "- [x] `ui-components` 固定入口文件落点正确。",
        "combo_box_ui_components_fixed_entry_files_follow_layered_boundaries",
        "combo_box_entrypoints_check_script_covers_fixed_entrypoint_contract",
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
            source.contains(needle),
            "combo-box check2 fixed-entry-files section should reference `{needle}`.",
        );
    }
}

#[test]
fn combo_box_component_directory_standard_files_follow_contract_and_na_paths() {
    let module_source = load_source("src/combo_box/mod.rs");
    let logic_source = load_source("src/combo_box/logic.rs");
    let styles_source = load_source("src/combo_box/styles.rs");
    let view_source = load_source("src/combo_box/view.rs");
    let motion_source = load_source("src/combo_box/motion.rs");

    for required in [
        "src/combo_box/mod.rs",
        "src/combo_box/logic.rs",
        "src/combo_box/styles.rs",
        "src/combo_box/view.rs",
        "src/combo_box/motion.rs",
    ] {
        assert!(
            path_exists(required),
            "combo-box component directory should include `{required}`.",
        );
    }

    for forbidden_file in ["src/combo_box/render.rs", "src/combo_box/spec.rs"] {
        assert!(
            !path_exists(forbidden_file),
            "combo-box component directory should keep `{forbidden_file}` absent.",
        );
    }

    for required in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use motion::ComboBoxMotion;",
        "pub use view::ComboBox;",
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
        "pub struct AccessibilityStateInput",
        "pub struct OpenStateInput",
        "pub struct RootStateInput",
        "pub struct RootState",
        "pub fn normalize_accessibility_state(",
        "pub fn normalize_open_state(",
        "pub fn normalize_root_state(",
        "pub fn resolve_root_data_state(",
        "pub fn compose_class_name(",
        "pub use ui_state_primitives::combo_box::{",
    ] {
        assert!(
            logic_source.contains(required),
            "logic.rs should keep normalization/derivation marker `{required}`.",
        );
    }
    for forbidden in [
        "web_sys::",
        "window()",
        "document()",
        "NodeRef",
        "view!",
        "on:click",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should stay free of DOM/render token `{forbidden}`.",
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        ".ui-combo-box {",
        ".ui-combo-box__panel {",
        "var(--ui-",
    ] {
        assert!(
            styles_source.contains(required),
            "styles.rs should keep token-first CSS marker `{required}`.",
        );
    }
    for forbidden in ["#[component]", "use ui_headless", "use leptos", "Select…"] {
        assert!(
            !styles_source.contains(forbidden),
            "styles.rs should avoid render/headless/business text token `{forbidden}`.",
        );
    }

    for required in [
        "#[component]",
        "fn ComboBoxPanel(",
        "pub fn ComboBox(",
        "logic::normalize_open_state(logic::OpenStateInput {",
        "logic::normalize_root_state(logic::RootStateInput {",
        "use_combo_box(ComboBoxOptions {",
        "use_presence(is_open)",
        "data-slot=SLOT_COMBO_BOX",
        "style=panel_vars",
    ] {
        assert!(
            view_source.contains(required),
            "view.rs should keep render + headless mount marker `{required}`.",
        );
    }
    for forbidden in ["@keyframes", ".ui-combo-box {", "request_animation_frame"] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should avoid styling/runtime engine token `{forbidden}`.",
        );
    }

    for required in [
        "pub struct ComboBoxMotion",
        "pub fn sanitize_motion(motion: ComboBoxMotion) -> ComboBoxMotion",
        "pub fn attach_popover_motion(",
        "ui_popover::motion::attach_motion(",
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
}

#[test]
fn combo_box_component_files_check_script_covers_standard_directory_contract() {
    let script_source = load_source("../../scripts/check-ui-components-component-files.sh");

    let needle = "cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_component_directory_standard_files_follow_contract_and_na_paths";
    assert!(
        script_source.contains(needle),
        "component-files check script should enforce `{needle}`.",
    );
}

#[test]
fn combo_box_check2_marks_component_directory_standard_files_contract_complete() {
    let source = load_source("../../components/combo-box/check2.md");

    for needle in [
        "- [x] 组件目录标准文件落点正确。",
        "combo_box_component_directory_standard_files_follow_contract_and_na_paths",
        "combo_box_component_files_check_script_covers_standard_directory_contract",
        "scripts/check-ui-components-component-files.sh",
        "components/combo-box/src/mod.rs",
        "components/combo-box/src/logic.rs",
        "components/combo-box/src/styles.rs",
        "components/combo-box/src/view.rs",
        "components/combo-box/src/motion.rs",
        "components/combo-box/src/spec.rs",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "combo-box check2 component-directory-standard section should reference `{needle}`.",
        );
    }
}

#[test]
fn combo_box_file_placement_discipline_is_strict_and_protocol_free() {
    combo_box_component_directory_standard_files_follow_contract_and_na_paths();

    assert!(
        !path_exists("src/combo_box/protocol.rs"),
        "combo-box file-placement discipline should not keep `src/combo_box/protocol.rs`."
    );

    let component_src_dir = resolve_source_path("src/combo_box/mod.rs")
        .and_then(|path| path.parent().map(Path::to_path_buf))
        .expect("combo-box src directory should be discoverable from mod.rs");
    let mut rust_files: Vec<String> = fs::read_dir(&component_src_dir)
        .unwrap_or_else(|e| panic!("read_dir failed for {component_src_dir:?}: {e}"))
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "rs"))
        .filter_map(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .map(str::to_string)
        })
        .collect();
    rust_files.sort();

    assert_eq!(
        rust_files,
        vec!["logic.rs", "mod.rs", "motion.rs", "styles.rs", "view.rs"],
        "combo-box src should keep strict file-placement discipline with only standard component files."
    );
}

#[test]
fn combo_box_component_files_check_script_covers_file_placement_discipline_contract() {
    let script_source = load_source("../../scripts/check-ui-components-component-files.sh");

    let needle = "cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_file_placement_discipline_is_strict_and_protocol_free";
    assert!(
        script_source.contains(needle),
        "component-files check script should enforce `{needle}`."
    );
}

#[test]
fn combo_box_check2_documents_file_placement_discipline_rules() {
    let source = load_source("../../components/combo-box/check2.md");

    for needle in [
        "- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。",
        "无 `protocol.rs/render.rs/spec.rs` 额外实现文件",
        "combo_box_file_placement_discipline_is_strict_and_protocol_free",
        "combo_box_component_files_check_script_covers_file_placement_discipline_contract",
        "scripts/check-ui-components-component-files.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "combo-box check2 file-placement discipline section should reference `{needle}`."
        );
    }
}

#[test]
fn combo_box_hyper_structure_builder_spec_is_not_applicable_for_simple_component() {
    let mod_source = load_source("src/combo_box/mod.rs");
    let readme_source = load_source("../../components/combo-box/src/README.md");
    let check2_source = load_source("../../components/combo-box/check2.md");

    assert!(
        !path_exists("src/combo_box/spec.rs"),
        "combo-box should not add `spec.rs` unless there is a stable external schema contract."
    );
    assert!(
        path_exists("src/button/spec.rs"),
        "button should remain the canonical complex component that carries `spec.rs`."
    );

    for forbidden in ["mod spec", "pub mod spec", "spec::", "ComboBoxSpec"] {
        assert!(
            !mod_source.contains(forbidden),
            "combo-box module boundary should not expose spec module via `{forbidden}`.",
        );
    }

    for forbidden in ["Spec::new(", ".render()", "schema_version", "spec.rs"] {
        assert!(
            !readme_source.contains(forbidden),
            "combo-box docs should not force Hyper-Structure builder token `{forbidden}` for simple component scope.",
        );
    }

    assert!(
        check2_source.contains("N/A-by-design：`combo-box` 当前为简单组件装配"),
        "combo-box check2 should keep explicit no-spec-for-simple-component constraint.",
    );
}

#[test]
fn combo_box_component_files_check_script_covers_hyper_structure_builder_contract() {
    let script_source = load_source("../../scripts/check-ui-components-component-files.sh");

    let needle = "cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_hyper_structure_builder_spec_is_not_applicable_for_simple_component";
    assert!(
        script_source.contains(needle),
        "component-files check script should enforce `{needle}`."
    );
}

#[test]
fn combo_box_check2_marks_hyper_structure_builder_item_complete() {
    let check2_source = load_source("../../components/combo-box/check2.md");

    for needle in [
        "- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。",
        "N/A-by-design：`combo-box` 当前为简单组件装配",
        "combo_box_hyper_structure_builder_spec_is_not_applicable_for_simple_component",
        "combo_box_component_files_check_script_covers_hyper_structure_builder_contract",
        "scripts/check-ui-components-component-files.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "combo-box check2 should keep Hyper-Structure builder marker `{needle}`.",
        );
    }
}

#[test]
fn combo_box_context_compression_manifest_and_rbi_projection_are_present_and_current() {
    for required_file in [
        "src/combo_box/Component.toml",
        "src/combo_box/combo_box.rbi",
    ] {
        assert!(
            path_exists(required_file),
            "combo-box context-compression artifact should exist: `{required_file}`.",
        );
    }

    let manifest_source = load_source("src/combo_box/Component.toml");
    let rbi_source = load_source("src/combo_box/combo_box.rbi");
    let view_source = load_source("../../components/combo-box/src/view.rs");

    for needle in [
        "schema_version = \"1\"",
        "name = \"ComboBox\"",
        "crate = \"ui-combo-box\"",
        "name = \"id_base\"",
        "name = \"label\"",
        "name = \"items\"",
        "name = \"selected_index\"",
        "name = \"set_selected_index\"",
        "name = \"is_disabled\"",
        "name = \"disabled_indices\"",
        "name = \"is_required\"",
        "name = \"is_invalid\"",
        "name = \"aria_describedby\"",
        "name = \"description\"",
        "name = \"error\"",
        "name = \"placeholder\"",
        "name = \"empty_message\"",
        "name = \"toggle_button_aria_label\"",
        "name = \"is_open\"",
        "name = \"default_open\"",
        "name = \"on_open_change\"",
        "name = \"lang\"",
        "name = \"dir\"",
        "name = \"motion\"",
        "name = \"class_name\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "combo-box Component.toml should include context-compression marker `{needle}`.",
        );
    }

    for needle in [
        "pub use crate::motion::ComboBoxMotion;",
        "pub fn ComboBox(",
        "id_base: String",
        "label: String",
        "items: Vec<String>",
        "selected_index: leptos::prelude::ReadSignal<Option<usize>>",
        "set_selected_index: leptos::prelude::WriteSignal<Option<usize>>",
        "is_disabled: Option<bool>",
        "disabled_indices: Vec<usize>",
        "is_required: Option<leptos::prelude::Signal<bool>>",
        "is_invalid: Option<leptos::prelude::Signal<bool>>",
        "aria_describedby: leptos::prelude::Signal<Option<String>>",
        "description: Option<String>",
        "error: Option<String>",
        "placeholder: Option<String>",
        "empty_message: Option<String>",
        "toggle_button_aria_label: Option<String>",
        "is_open: Option<leptos::prelude::Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<leptos::prelude::Callback<bool>>",
        "lang: Option<String>",
        "dir: Option<ui_headless::A11yDirection>",
        "motion: crate::motion::ComboBoxMotion",
        "class_name: Option<String>",
        ") -> impl leptos::prelude::IntoView;",
    ] {
        assert!(
            rbi_source.contains(needle),
            "combo-box RBI projection should keep signature marker `{needle}`.",
        );
    }

    for needle in [
        "pub fn ComboBox(",
        "id_base: String,",
        "label: String,",
        "items: Vec<String>,",
        "selected_index: ReadSignal<Option<usize>>",
        "set_selected_index: WriteSignal<Option<usize>>",
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] is_open: Option<Signal<bool>>",
        "#[prop(optional)] default_open: Option<bool>",
        "#[prop(optional)] on_open_change: Option<Callback<bool>>",
        "#[prop(optional)] motion: ComboBoxMotion",
    ] {
        assert!(
            view_source.contains(needle),
            "combo-box view signature should include `{needle}` for manifest/rbi drift detection.",
        );
    }
}

#[test]
fn combo_box_component_files_check_script_covers_context_compression_manifest_contract() {
    let script_source = load_source("../../scripts/check-ui-components-component-files.sh");

    let needle = "cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_context_compression_manifest_and_rbi_projection_are_present_and_current";
    assert!(
        script_source.contains(needle),
        "component-files check script should enforce `{needle}`.",
    );
}

#[test]
fn combo_box_check2_marks_context_compression_manifest_and_rbi_contract_complete() {
    let source = load_source("../../components/combo-box/check2.md");

    assert!(
        source.contains("- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。"),
        "combo-box check2 should mark context-compression manifest/rbi gate complete.",
    );

    for needle in [
        "components/combo-box/src/Component.toml",
        "components/combo-box/src/combo_box.rbi",
        "combo_box_context_compression_manifest_and_rbi_projection_are_present_and_current",
        "combo_box_component_files_check_script_covers_context_compression_manifest_contract",
        "scripts/check-ui-components-component-files.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "combo-box check2 context-compression section should reference `{needle}`.",
        );
    }
}

#[test]
fn combo_box_check2_documents_agent_contract_schema_governance_rules() {
    let checklist_source = load_source("../../components/combo-box/check2.md");

    for required in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "关键交互组件必须输出稳定机器可读语义（至少 `data-*` + 状态来源标记；复杂组件建议补 `data-ui-schema`）。",
        "Agent 消费字段应来自类型化 schema 生成，不允许散落字符串拼接。",
        "契约字段需可追溯到组件状态轴与动作语义（intent/action/state/source）。",
        "配置到组件的渲染链路必须走白名单能力边界，禁止任意脚本注入。",
    ] {
        assert!(
            checklist_source.contains(required),
            "ComboBox checklist should keep Agent Contract governance rule `{required}`.",
        );
    }
}

#[test]
fn combo_box_agent_contract_is_schema_typed_and_machine_readable() {
    let view_source = load_source("../../components/combo-box/src/view.rs");
    let logic_source = load_source("../../components/combo-box/src/logic.rs");

    for needle in [
        "pub enum ComboBoxAgentSchemaVersion",
        "pub enum ComboBoxAgentIntent",
        "pub enum ComboBoxAgentAction",
        "pub enum ComboBoxAgentStateAxis",
        "pub enum ComboBoxAgentSourceAxis",
        "pub struct ComboBoxAgentCapabilities",
        "pub struct ComboBoxAgentContractInput",
        "pub struct ComboBoxAgentContract",
        "pub fn resolve_agent_contract(input: ComboBoxAgentContractInput) -> ComboBoxAgentContract",
    ] {
        assert!(
            logic_source.contains(needle),
            "ComboBox agent contract typing should include `{needle}`."
        );
    }

    for needle in [
        "let agent_contract = Memo::new(move |_| {",
        "logic::resolve_agent_contract(logic::ComboBoxAgentContractInput {",
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-ui-schema-version=move || agent_contract.get().schema_version.as_str()",
        "data-ui-intent=move || agent_contract.get().intent.as_str()",
        "data-ui-action=move || agent_contract.get().action.as_str()",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "data-ui-source=move || agent_contract.get().source.as_str()",
        "data-ui-capability-filter=move || {",
        "data-ui-capability-select=move || {",
        "data-ui-capability-open=move || {",
    ] {
        assert!(
            view_source.contains(needle),
            "ComboBox view should mount schemaized agent contract field `{needle}`."
        );
    }

    for forbidden in [
        "data-ui-schema=format!(",
        "data-ui-intent=format!(",
        "data-ui-action=format!(",
        "data-ui-state=format!(",
        "data-ui-source=format!(",
        "format!(\"data-ui-",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "ComboBox Agent Contract should avoid free-form string splicing token `{forbidden}`."
        );
    }
}

#[test]
fn combo_box_agent_contract_render_path_is_whitelist_safe_and_script_injection_free() {
    let mod_source = load_source("../../components/combo-box/src/mod.rs");
    let logic_source = load_source("../../components/combo-box/src/logic.rs");
    let view_source = load_source("../../components/combo-box/src/view.rs");
    let styles_source = load_source("../../components/combo-box/src/styles.rs");
    let motion_source = load_source("../../components/combo-box/src/motion.rs");
    let combined =
        format!("{mod_source}\n{logic_source}\n{view_source}\n{styles_source}\n{motion_source}");

    for forbidden in [
        "<script",
        "javascript:",
        "onerror=",
        "onload=",
        "inner_html",
        "set_inner_html",
        "dangerously_set_inner_html",
    ] {
        assert!(
            !combined.contains(forbidden),
            "ComboBox Agent Contract render path should stay whitelist-safe without `{forbidden}`."
        );
    }
}

#[test]
fn combo_box_contract_hygiene_script_covers_agent_contract_schema_guards() {
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_agent_contract_is_schema_typed_and_machine_readable",
        "cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
        "cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_check2_documents_agent_contract_schema_governance_rules",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene check script should enforce `{needle}`."
        );
    }
}

#[test]
fn combo_box_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let section = combo_box_docs_section(&source);

    for needle in [
        "title=\"ComboBox\"",
        "slug=\"combo-box\"",
        "description=\"Combobox with input + listbox + popover, baseline-style root attrs, and baseline-level panel/highlight motion.\"",
        "title=\"Hello World (Uncontrolled)\"",
        "title=\"展示：多场景对比\"",
        "title=\"Workbench（展示 + Config + Code + CSS Test）\"",
        "title=\"Streaming/Snapshot Display\"",
        "data-slot=\"combo-box-state-matrix\"",
        "data-slot=\"combo-box-streaming-snapshot\"",
        "data-slot=\"combo-box-source-first\"",
        "code_imports=combo_box_code_imports.clone()",
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
        "id_base=\"docs-combo-box-hello\".to_string()",
        "\"hello selected: \"",
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
        "id_base=\"docs-combo-box-snapshot\".to_string()",
        "id_base=\"docs-combo-box-streaming\".to_string()",
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
fn combo_box_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let docs_section = combo_box_docs_section(&docs_source);
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let check2_source = load_source("../../components/combo-box/check2.md");

    for needle in [
        "title=\"Hello World (Uncontrolled)\"",
        "title=\"展示：多场景对比\"",
        "title=\"Streaming/Snapshot Display\"",
        "data-slot=\"combo-box-state-matrix\"",
        "data-slot=\"combo-box-streaming-snapshot\"",
        "data-slot=\"combo-box-source-first\"",
        "is_open=controlled_open",
        "on_open_change=on_open_change",
        "data-ui-streaming=\"optional\"",
        "data-ui-fallback=\"snapshot\"",
        "data-ui-output-state=\"snapshot\"",
        "data-ui-output-state=\"streaming\"",
        "<AiSpace mode=snapshot_mode output_status=verified_output>",
        "<AiSpace mode=streaming_mode output_status=draft_output>",
        "code_imports=combo_box_code_imports.clone()",
        "use leptos::prelude::*;\\nuse ui_components::ComboBox;",
        "<Snippet",
        "copyable=true",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
    ] {
        assert!(
            docs_section.contains(needle),
            "combo-box docs-as-product section should include `{needle}`."
        );
    }

    for needle in [
        "const DEFAULT_PLAYGROUND_IMPORTS: &str = \"use leptos::prelude::*;\\nuse ui_components::*;\";",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "let missing_imports = missing_import_lines(&raw, &imports);",
        "<CodeBlock code=resolved_code.get() />",
    ] {
        assert!(
            playground_source.contains(needle),
            "playground copy-ready pipeline should keep marker `{needle}`."
        );
    }

    for needle in [
        "- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。",
        "combo_box_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
        "scripts/check-ui-components-dx.sh",
        "compose_copy_ready_code",
    ] {
        assert!(
            check2_source.contains(needle),
            "combo_box/check2.md docs-as-product evidence should include `{needle}`."
        );
    }
}

#[test]
fn combo_box_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let docs_section = combo_box_docs_section(&docs_source);
    let view_source = load_source("../../components/combo-box/src/view.rs");
    let logic_source = load_source("../../components/combo-box/src/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/combo_box.rs");
    let check2_source = load_source("../../components/combo-box/check2.md");

    for needle in [
        "#[prop(optional)] is_open: Option<Signal<bool>>",
        "#[prop(optional)] default_open: Option<bool>",
        "#[prop(optional)] on_open_change: Option<Callback<bool>>",
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] disabled_indices: Vec<usize>",
        "#[prop(optional, into)] placeholder: Option<String>",
        "pub fn normalize_open_state(input: OpenStateInput) -> OpenState",
        "pub fn resolve_placeholder(placeholder: Option<String>) -> String",
        "pub const DEFAULT_PLACEHOLDER: &str = \"Select…\";",
        "pub const DEFAULT_EMPTY_MESSAGE: &str = \"No options\";",
    ] {
        assert!(
            view_source.contains(needle)
                || logic_source.contains(needle)
                || primitive_source.contains(needle),
            "ComboBox API/default contract should keep marker `{needle}` for docs sync."
        );
    }

    for needle in [
        "title=\"Hello World (Uncontrolled)\"",
        "title=\"展示：多场景对比\"",
        "data-slot=\"combo-box-state-matrix\"",
        "<code>\"open mode\"</code>",
        "<code>\"disabled\"</code>",
        "<code>\"item set\"</code>",
        "<code>\"validation\"</code>",
        "<code>\"selection\"</code>",
        "is_open=controlled_open",
        "on_open_change=on_open_change",
        "is_disabled=true",
        "disabled_indices=vec![4]",
        "placeholder=\"No options\".to_string()",
        "description=\"Open state is externally controlled\".to_string()",
    ] {
        assert!(
            docs_section.contains(needle),
            "combo-box docs should keep synced example/matrix marker `{needle}`."
        );
    }

    for needle in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "combo_box_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
        "scripts/check-ui-components-dx.sh",
        "normalize_open_state",
        "DEFAULT_PLACEHOLDER",
    ] {
        assert!(
            check2_source.contains(needle),
            "combo_box/check2.md should keep docs-sync evidence marker `{needle}`."
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
fn combo_box_documentation_as_product_keeps_beginner_path_before_advanced_sections() {
    let readme_source = load_source("../../components/combo-box/src/README.md");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let docs_section = combo_box_docs_section(&docs_source);
    let check2_source = load_source("../../components/combo-box/check2.md");

    assert!(
        path_exists("../../components/combo-box/src/README.md"),
        "ComboBox should keep a discoverable component README entry."
    );

    for needle in [
        "## Quick Start (Hello World)",
        "先跑默认路径，不需要先理解分层细节。",
        "## 常见用法",
        "受控 open：`is_open + on_open_change`",
        "非受控 open：`default_open`",
        "## Architecture Layers",
        "## API (Table)",
    ] {
        assert!(
            readme_source.contains(needle),
            "ComboBox README should include documentation-as-product marker `{needle}`."
        );
    }

    let quick_start_pos = readme_source
        .find("## Quick Start (Hello World)")
        .expect("README should include quick-start section.");
    let architecture_pos = readme_source
        .find("## Architecture Layers")
        .expect("README should include architecture section.");
    let api_pos = readme_source
        .find("## API (Table)")
        .expect("README should include API section.");
    assert!(
        quick_start_pos < architecture_pos && quick_start_pos < api_pos,
        "README should keep beginner quick-start path before advanced architecture/API sections."
    );

    for needle in [
        "title=\"Hello World (Uncontrolled)\"",
        "title=\"展示：多场景对比\"",
        "title=\"Workbench（展示 + Config + Code + CSS Test）\"",
    ] {
        assert!(
            docs_section.contains(needle),
            "docs page should include beginner+advanced progression marker `{needle}`."
        );
    }

    for needle in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "combo_box_documentation_as_product_keeps_beginner_path_before_advanced_sections",
        "components/combo-box/src/README.md",
        "Quick Start (Hello World)",
        "scripts/check-ui-components-dx.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "combo_box/check2.md should keep documentation-as-product evidence marker `{needle}`."
        );
    }
}

#[test]
fn combo_box_check2_documents_interactive_playground_rules() {
    let checklist_source = load_source("../../components/combo-box/check2.md");

    for required in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "Playground 至少支持基础 props 调整、状态切换、交互反馈观察。",
        "对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。",
        "Playground 作为验收面，需可重复复现关键交互路径。",
        "AI Spec 子条款对 `ComboBox` 为 N/A",
        "combo_box_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "combo_box_interactive_playground_reuses_repeatable_semantic_e2e_flow",
    ] {
        assert!(
            checklist_source.contains(required),
            "ComboBox checklist should keep interactive-playground contract marker `{required}`."
        );
    }
}

#[test]
fn combo_box_docs_app_provides_interactive_playground_for_props_state_and_preview() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let docs_section = combo_box_docs_section(&docs_source);
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");

    for needle in [
        "pub(super) fn combo_box() -> AnyView",
        "title=\"Workbench（展示 + Config + Code + CSS Test）\"",
        "data-slot=\"combo-box-workbench-controls\"",
        "\" Invalid\"",
        "\" Disabled root\"",
        "\" Controlled open\"",
        "\" Persist selected index (optional)\"",
        "data-slot=\"combo-box-workbench\"",
        "data-slot=\"combo-box-workbench-canvas\"",
        "test_config_signal=workbench_actual_config",
        "test_css_source=workbench_test_css",
        "code_signal=workbench_code",
        "\"open: \"",
        "\" · selected: \"",
        "\" · persist selected: \"",
    ] {
        assert!(
            docs_section.contains(needle),
            "ComboBox docs should provide interactive-playground marker `{needle}`."
        );
    }

    for needle in [
        "let section_class = \"docs-card playground\";",
        "<div class=\"playground__preview\" data-playground-scope=scope_id.clone()>",
        "<div class=\"playground__preview-stage\">{children()}</div>",
        "<aside class=\"playground__panel playground__controls\" data-slot=\"playground-controls\">",
    ] {
        assert!(
            playground_source.contains(needle),
            "docs-app Playground should keep interactive preview marker `{needle}`."
        );
    }
}

#[test]
fn combo_box_interactive_playground_reuses_repeatable_semantic_e2e_flow() {
    let e2e_source = load_source("../../e2e/tests/docs_app_combo_box_contract.spec.mjs");

    for needle in [
        "docs-app combo-box flow is repeatable with semantic ready/settled breakpoints",
        "await page.goto(\"/#/components/combo-box\");",
        "body:not(:has(#boot))",
        "[data-component=\"combo-box\"]",
        "[data-slot=\"combo-box-showcase\"]",
        "[data-slot=\"combo-box\"][data-controlled=\"true\"]",
        "[data-slot=\"combo-box-input\"][role=\"combobox\"]",
        "await trigger.click();",
        "toHaveAttribute(\"data-state\", \"open\")",
        "await page.keyboard.press(\"Escape\")",
        "toHaveAttribute(\"data-state\", \"closed\")",
        "await controlledInput.fill(\"Ru\");",
        "toHaveAttribute(\"data-ui-action\", \"filter-query\")",
        "await page.reload();",
    ] {
        assert!(
            e2e_source.contains(needle),
            "ComboBox interactive playground should keep repeatable semantic e2e marker `{needle}`."
        );
    }
}

#[test]
fn combo_box_check2_documents_source_first_copy_paste_ready_rules() {
    let checklist_source = load_source("../../components/combo-box/check2.md");

    for required in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
        "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
        "文档代码与当前实现必须同步，防止示例漂移。",
        "combo_box_docs_are_source_first_copy_paste_ready_with_imports_copy_button_and_sync",
    ] {
        assert!(
            checklist_source.contains(required),
            "ComboBox checklist should keep source-first copy-paste-ready marker `{required}`."
        );
    }
}

#[test]
fn combo_box_docs_are_source_first_copy_paste_ready_with_imports_copy_button_and_sync() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let docs_section = combo_box_docs_section(&docs_source);
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let view_source = load_source("../../components/combo-box/src/view.rs");
    let logic_source = load_source("../../components/combo-box/src/logic.rs");
    let check2_source = load_source("../../components/combo-box/check2.md");

    for needle in [
        "data-slot=\"combo-box-source-first\"",
        "<Snippet",
        "label=\"Copy starter\".to_string()",
        "copyable=true",
        "use leptos::prelude::*;\\nuse ui_components::ComboBox;",
        "<code>\"components/combo-box/src/mod.rs\"</code>",
        "<code>\"components/combo-box/src/logic.rs\"</code>",
        "<code>\"components/combo-box/src/view.rs\"</code>",
        "<code>\"components/combo-box/src/styles.rs\"</code>",
        "<code>\"components/combo-box/src/motion.rs\"</code>",
        "<code>\"component-combo_box\"</code>",
        "<code>\"inject-css\"</code>",
    ] {
        assert!(
            docs_section.contains(needle),
            "ComboBox source-first docs section should include `{needle}`."
        );
    }

    for needle in [
        "const DEFAULT_PLAYGROUND_IMPORTS: &str = \"use leptos::prelude::*;\\nuse ui_components::*;\";",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "let missing_imports = missing_import_lines(&raw, &imports);",
        "return compose_copy_ready_code(&dynamic_code.get(), &code_imports.get_value());",
        "<CodeBlock code=resolved_code.get() />",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground copy-ready pipeline should include `{needle}`."
        );
    }

    for needle in [
        "id_base: Option<String>",
        "label: Option<String>",
        "items: Vec<String>",
        "selected_index: RwSignal<Option<usize>>",
        "set_selected_index: Callback<Option<usize>>",
    ] {
        assert!(
            view_source.contains(needle),
            "ComboBox view API should expose source-first snippet prop `{needle}`."
        );
    }

    for needle in [
        "pub const DEFAULT_PLACEHOLDER: &str = \"Select…\";",
        "pub const DEFAULT_EMPTY_MESSAGE: &str = \"No options\";",
    ] {
        assert!(
            logic_source.contains(needle),
            "ComboBox logic defaults should keep source-first snippet sync anchor `{needle}`."
        );
    }

    for needle in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "combo_box_docs_are_source_first_copy_paste_ready_with_imports_copy_button_and_sync",
        "scripts/check-ui-components-dx.sh",
        "compose_copy_ready_code",
    ] {
        assert!(
            check2_source.contains(needle),
            "combo_box/check2.md should keep source-first evidence marker `{needle}`."
        );
    }
}

#[test]
fn combo_box_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes() {
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let docs_index_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let view_source = load_source("../../components/combo-box/src/view.rs");
    let logic_source = load_source("../../components/combo-box/src/logic.rs");

    for needle in [
        "### ComboBox 同步记录（2026-02-20）",
        "参数模型同步：`ComboBox` 参数主轴保持 `items + selected_index + set_selected_index`，并维持受控/非受控 open 轴 `is_open + on_open_change + default_open`。",
        "docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!(\"ComboBox\", \"combo-box\", \"Collections\", collections::combo_box)` 暴露入口；`#/components/combo-box` 可索引访问。",
        "示例矩阵同步：`apps/docs-app/src/pages/components/pages/collections.rs::combo_box()`",
        "Source-first / Copy-Paste Ready：ComboBox Playground 代码继续通过 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports，并在文档中显式给出源码落点与 feature 前提，避免复制即报错。",
        "研究文档补充判定：本轮为 ComboBox 参数语义与文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。",
        "HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。",
    ] {
        assert!(
            strategy_source.contains(needle),
            "ComboBox HeroUI strategy sync record should include `{needle}`."
        );
    }

    for needle in [
        "component_doc!(\"ComboBox\", \"combo-box\", \"Collections\", collections::combo_box)",
        "\"ComboBox\"",
        "\"combo-box\"",
        "collections::combo_box",
    ] {
        assert!(
            docs_index_source.contains(needle),
            "ComboBox docs route should stay indexable via `{needle}`."
        );
    }

    for needle in [
        "pub(super) fn combo_box() -> AnyView",
        "title=\"ComboBox\"",
        "slug=\"combo-box\"",
        "title=\"Hello World (Uncontrolled)\"",
        "title=\"Workbench（展示 + Config + Code + CSS Test）\"",
    ] {
        assert!(
            docs_page_source.contains(needle),
            "ComboBox docs entry should keep marker `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional)] is_open: Option<Signal<bool>>",
        "#[prop(optional)] default_open: Option<bool>",
        "#[prop(optional)] on_open_change: Option<Callback<bool>>",
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] disabled_indices: Vec<usize>",
        "#[prop(optional, into)] placeholder: Option<String>",
        "pub fn normalize_open_state(input: OpenStateInput) -> OpenState",
        "pub fn normalize_accessibility_state(input: AccessibilityStateInput) -> AccessibilityState",
    ] {
        assert!(
            view_source.contains(needle) || logic_source.contains(needle),
            "ComboBox parameter model marker `{needle}` should remain in implementation."
        );
    }
}

#[test]
fn combo_box_check2_marks_heroui_strategy_and_component_docs_sync_complete() {
    let check2_source = load_source("../../components/combo-box/check2.md");

    for needle in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。",
        "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
        "“仅代码更新无文档更新”在接口变更场景下直接判不通过。",
        "combo_box_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes",
        "scripts/check-ui-components-contract-hygiene.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "ComboBox checklist should keep HeroUI/docs sync evidence `{needle}`."
        );
    }
}

#[test]
fn combo_box_contract_hygiene_script_covers_heroui_strategy_doc_sync_contract() {
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes",
        "cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_check2_marks_heroui_strategy_and_component_docs_sync_complete",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene check script should enforce `{needle}`."
        );
    }
}

#[test]
fn combo_box_dx_playground_supports_css_hot_reload_without_wasm_rebuild() {
    let source = load_source("../../apps/docs-app/src/playground.rs");

    for needle in [
        "let (show_settings_panel, set_show_settings_panel) = signal(false);",
        "let (show_code_panel, set_show_code_panel) = signal(false);",
        "let (show_test_panel, set_show_test_panel) = signal(false);",
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
        "placeholder=\"/* Original CSS is loaded. Edit directly, or use :scope for local targeting. */\"",
        "\"Show settings\"",
        "\"Show code\"",
        "\"Show test\"",
        "\"Restore original CSS\"",
        "data-slot=\"playground-test\"",
    ] {
        assert!(
            source.contains(needle),
            "Playground should keep CSS hot-reload contract marker `{needle}`."
        );
    }
}

#[test]
fn combo_box_dx_workbench_supports_optional_state_persistence_and_isolated_canvas() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let section = combo_box_docs_section(&source);

    for needle in [
        "COMBO_BOX_WORKBENCH_STORAGE_KEY",
        "fn load_combo_box_workbench_selected() -> Option<usize>",
        "fn save_combo_box_workbench_selected(selected_index: usize)",
        "fn clear_combo_box_workbench_selected()",
        "let persisted_combo_box_workbench_selected = load_combo_box_workbench_selected();",
        "let (workbench_persist_state, set_workbench_persist_state) =",
        "save_combo_box_workbench_selected(selected_index);",
        "clear_combo_box_workbench_selected();",
        "title=\"Workbench（展示 + Config + Code + CSS Test）\"",
        "test_css_source=workbench_test_css",
        "test_config_signal=workbench_actual_config",
        "\" Persist selected index (optional)\"",
        "\" · persist selected: \"",
        "data-slot=\"combo-box-workbench-controls\"",
        "data-slot=\"combo-box-workbench\"",
        "data-slot=\"combo-box-workbench-canvas\"",
    ] {
        assert!(
            source.contains(needle) || section.contains(needle),
            "combo-box workbench should keep DX marker `{needle}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            source.contains(needle),
            "combo-box workbench persistence should keep platform guard `{needle}`."
        );
    }
}

#[test]
fn combo_box_dx_check_script_covers_hot_reload_and_workbench_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for needle in [
        "cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_dx_playground_supports_css_hot_reload_without_wasm_rebuild",
        "cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_dx_workbench_supports_optional_state_persistence_and_isolated_canvas",
        "cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_interactive_playground_reuses_repeatable_semantic_e2e_flow",
        "cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_docs_are_source_first_copy_paste_ready_with_imports_copy_button_and_sync",
        "cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
        "cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
        "cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_documentation_as_product_keeps_beginner_path_before_advanced_sections",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce `{needle}`."
        );
    }
}

#[test]
fn combo_box_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope() {
    let cargo_source = load_source("Cargo.toml");
    let mod_source = load_source("../../components/combo-box/src/mod.rs");
    let logic_source = load_source("../../components/combo-box/src/logic.rs");
    let view_source = load_source("../../components/combo-box/src/view.rs");
    let styles_source = load_source("../../components/combo-box/src/styles.rs");
    let motion_source = load_source("../../components/combo-box/src/motion.rs");
    let checklist_source = load_source("../../components/combo-box/check2.md");

    let spec_path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../components/combo-box/src/spec.rs");
    assert!(
        !spec_path.exists(),
        "ComboBox should keep spec/schema boundary as N/A for simple component scope."
    );
    assert!(
        cargo_source.contains(
            "component-combo_box = [\n    \"component-active_highlight\",\n    \"component-popover\",\n    \"dep:ui-combo-box\",\n]"
        ),
        "combo_box feature should stay lightweight without serde/spec dependency fan-out."
    );
    assert!(
        !cargo_source.contains("component-combo_box = [\"dep:serde\"")
            && !cargo_source.contains("component-combo_box = [\"dep:serde_json\""),
        "combo_box should not opt into serde/spec migration dependencies without explicit schema contract."
    );

    let combined =
        format!("{mod_source}\n{logic_source}\n{view_source}\n{styles_source}\n{motion_source}");
    for forbidden in [
        "serde::",
        "serde_json::",
        "Serialize",
        "Deserialize",
        "schema_version",
        "from_json(",
        "to_json_result(",
        "SchemaError",
        "spec::",
    ] {
        assert!(
            !combined.contains(forbidden),
            "combo_box engineering contract should keep serde/spec path as N/A and avoid `{forbidden}`."
        );
    }

    for required in [
        "- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "若组件涉及 spec/config 输入，序列化与错误输出应走统一结构化路径。",
        "关键流程埋点语义应与全库 tracing 约定一致，避免组件各说各话。",
        "异步边界不得把具体 runtime 类型暴露到组件公共接口。",
    ] {
        assert!(
            checklist_source.contains(required),
            "combo_box checklist should keep engineering governance rule `{required}`."
        );
    }
}

#[test]
fn combo_box_version_deprecation_migration_is_na_without_major_breaking_upgrade() {
    let manifest_source = load_source("../../components/combo-box/src/Component.toml");
    let rbi_source = load_source("../../components/combo-box/src/combo_box.rbi");
    let mod_source = load_source("../../components/combo-box/src/mod.rs");
    let logic_source = load_source("../../components/combo-box/src/logic.rs");
    let view_source = load_source("../../components/combo-box/src/view.rs");
    let styles_source = load_source("../../components/combo-box/src/styles.rs");
    let motion_source = load_source("../../components/combo-box/src/motion.rs");
    let check2_source = load_source("../../components/combo-box/check2.md");

    for needle in [
        "schema_version = \"1\"",
        "name = \"ComboBox\"",
        "crate = \"ui-combo-box\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "ComboBox manifest should keep stable v1 schema marker `{needle}`."
        );
    }

    for needle in [
        "pub fn ComboBox(",
        "is_open: Option<leptos::prelude::Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<leptos::prelude::Callback<bool>>",
        "is_disabled: Option<bool>",
    ] {
        assert!(
            rbi_source.contains(needle),
            "ComboBox RBI should keep stable public API marker `{needle}`."
        );
    }

    let combined =
        format!("{mod_source}\n{logic_source}\n{view_source}\n{styles_source}\n{motion_source}");
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
            "ComboBox should not introduce major-version migration marker `{forbidden}` in current scope."
        );
    }

    for needle in [
        "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。",
        "N/A：`ComboBox` 本次变更不包含跨大版本 API 破坏升级",
        "schema_version = \"1\"",
        "migrate_v1_to_v2",
    ] {
        assert!(
            check2_source.contains(needle),
            "combo_box/check2.md should keep version-migration governance marker `{needle}`."
        );
    }
}

#[test]
fn combo_box_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events() {
    let cargo_source = load_source("Cargo.toml");
    let button_view_source = load_source("src/button/view.rs");
    let combined = [
        load_source("../../components/combo-box/src/mod.rs"),
        load_source("../../components/combo-box/src/logic.rs"),
        load_source("../../components/combo-box/src/view.rs"),
        load_source("../../components/combo-box/src/styles.rs"),
        load_source("../../components/combo-box/src/motion.rs"),
    ]
    .join("\n");

    for required in [
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "target: \"ui_components::button::state_change\"",
    ] {
        assert!(
            cargo_source.contains(required) || button_view_source.contains(required),
            "engineering baseline should keep canonical tracing contract marker `{required}`."
        );
    }

    assert!(
        !cargo_source.contains("combo-box-wasm-debug")
            && !cargo_source.contains("combo_box-wasm-debug"),
        "combo_box should not define component-local tracing feature when no local debug event/replay contract exists."
    );
    assert!(
        combined.contains("overlay_open::use_controllable_open_state_traced(")
            && combined.contains("\"combo-box\""),
        "combo_box should reuse shared traced controllable-state hook instead of custom tracing events."
    );

    for forbidden in [
        "tracing::span!(",
        "tracing::event!(",
        "#[tracing::instrument]",
        "target: \"ui_components::combo_box::",
        "const COMBO_BOX_TRACE_TARGET",
    ] {
        assert!(
            !combined.contains(forbidden),
            "combo_box should avoid ad-hoc tracing semantic drift token `{forbidden}`."
        );
    }
}

#[test]
fn combo_box_engineering_contract_avoids_runtime_leaks_in_public_api_surface() {
    let mod_source = load_source("../../components/combo-box/src/mod.rs");
    let logic_source = load_source("../../components/combo-box/src/logic.rs");
    let view_source = load_source("../../components/combo-box/src/view.rs");
    let styles_source = load_source("../../components/combo-box/src/styles.rs");
    let motion_source = load_source("../../components/combo-box/src/motion.rs");
    let readme_source = load_source("../../components/combo-box/src/README.md");

    let sources = [
        &mod_source,
        &logic_source,
        &view_source,
        &styles_source,
        &motion_source,
        &readme_source,
    ];
    for source in sources {
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
                "combo_box engineering contract should not leak runtime marker `{forbidden}`."
            );
        }
    }

    assert!(
        !mod_source.contains("web_sys"),
        "combo_box public module boundary should not leak web_sys types."
    );
}

#[test]
fn combo_box_engineering_check_script_covers_serde_tracing_and_runtime_boundaries() {
    let script_source = load_source("../../scripts/check-ui-components-engineering.sh");

    for needle in [
        "cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope",
        "cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_version_deprecation_migration_is_na_without_major_breaking_upgrade",
        "cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events",
        "cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_engineering_contract_avoids_runtime_leaks_in_public_api_surface",
    ] {
        assert!(
            script_source.contains(needle),
            "engineering check script should enforce `{needle}`."
        );
    }
}

#[test]
fn combo_box_performance_governance_budget_is_defined_and_blocking() {
    let check2_source = load_source("../../components/combo-box/check2.md");
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_combo_box_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let perf_probe_source = load_source("../../apps/docs-app/src/perf_probe.rs");
    let coverage_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let script_source = load_source("../../scripts/check-ui-components-performance.sh");
    let view_source = load_source("../../components/combo-box/src/view.rs");

    for needle in [
        "- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。",
        "渲染次数预算为 `1`",
        "render_count",
        "若当前测试框架暂不支持精确渲染计数",
        "等价证据",
        "combo_box_performance_governance_budget_is_defined_and_blocking",
    ] {
        assert!(
            check2_source.contains(needle),
            "ComboBox checklist should keep performance governance marker `{needle}`."
        );
    }

    for needle in ["\"ComboBox\",", "\"combo-box\",", "collections::combo_box"] {
        assert!(
            pages_source.contains(needle),
            "ComboBox should stay in docs coverage traversal via `{needle}`."
        );
    }

    for needle in ["title=\"ComboBox\"", "slug=\"combo-box\"", "<ComponentPage"] {
        assert!(
            docs_combo_box_page_source.contains(needle),
            "ComboBox docs page should mount through ComponentPage contract `{needle}`."
        );
    }

    for needle in [
        "\"combo-box\" => UiPerfBudget {",
        "max_mount_ms: 38.0,",
        "max_update_ms: Some(13.0),",
        "max_heap_kb: Some(768.0),",
        "let perf_budget = component_page_perf_budget(slug);",
        "<UiPerfProbe name=perf_name budget=perf_budget>",
    ] {
        assert!(
            shell_source.contains(needle),
            "docs shell should keep ComboBox perf budget/probe wiring via `{needle}`."
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
    ] {
        assert!(
            perf_probe_source.contains(needle),
            "UiPerfProbe should expose stable performance marker `{needle}`."
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
            "docs coverage e2e should keep blocking perf assertion `{needle}`."
        );
    }

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "performance governance should keep explicit render_count follow-up marker `{needle}`."
        );
    }

    let script_needle = "cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_performance_governance_budget_is_defined_and_blocking";
    assert!(
        script_source.contains(script_needle),
        "performance gate script should include `{script_needle}`."
    );

    for needle in [
        "data-state=move || {",
        "data-open=move || is_open.get().then_some(\"true\")",
        "data-label-source=state.label_source_attr",
        "data-placeholder-source=state.placeholder_source_attr",
        "data-id-source=state.id_source_attr",
        "data-class-source=state.class_source_attr",
        "data-motion-source=state.motion_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "ComboBox view should expose attribution marker `{needle}` for perf triage."
        );
    }
}

#[test]
fn combo_box_view_macro_complexity_is_split_into_semantic_subrenders() {
    let view_source = load_source("../../components/combo-box/src/view.rs");
    let check2_source = load_source("../../components/combo-box/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-view-macro.sh");

    assert!(
        view_source.contains("view! {"),
        "ComboBox should keep explicit render blocks in view.rs.",
    );

    let view_macro_count = view_source.matches("view! {").count();
    assert!(
        view_macro_count <= 5,
        "ComboBox view macro expansion should stay bounded after panel split; expected <= 5, found {view_macro_count}.",
    );
    assert!(
        view_source.lines().count() <= 520,
        "ComboBox view.rs should stay bounded; split semantic subrenders further if this grows.",
    );

    for needle in [
        "#[component]\nfn ComboBoxPanel(",
        "#[component]\npub fn ComboBox(",
        "<ComboBoxPanel",
        "data-slot=\"combo-box-panel\"",
        "data-slot=\"combo-box-listbox\"",
        "data-slot=\"combo-box-option\"",
        "data-slot=\"combo-box-empty\"",
    ] {
        assert!(
            view_source.contains(needle),
            "ComboBox view macro split contract should include `{needle}`."
        );
    }

    assert!(
        check2_source.contains("- [x] `view!` 宏复杂度受控："),
        "combo-box check2 should mark view-macro complexity item complete.",
    );
    assert!(
        check2_source.contains("combo_box_view_macro_complexity_is_split_into_semantic_subrenders"),
        "combo-box check2 should reference macro complexity regression test name.",
    );

    let script_needle = "cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_view_macro_complexity_is_split_into_semantic_subrenders";
    assert!(
        script_source.contains(script_needle),
        "view macro gate script should include `{script_needle}`.",
    );
}

#[test]
fn combo_box_view_functional_split_prefers_plain_functions_over_extra_local_components() {
    let view_source = load_source("../../components/combo-box/src/view.rs");
    let check2_source = load_source("../../components/combo-box/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-view-macro.sh");

    assert_eq!(
        view_source.matches("#[component]").count(),
        2,
        "ComboBox should keep only two component boundaries: public root + panel shell.",
    );

    for needle in [
        "fn render_description_slot(description_id: String, description: String) -> impl IntoView",
        "fn render_error_slot(error_id: String, error: String, invalid: Signal<bool>) -> impl IntoView",
        "render_description_slot(text_field.description.id.clone(), description)",
        "render_error_slot(text_field.error.id.clone(), error, invalid)",
    ] {
        assert!(
            view_source.contains(needle),
            "ComboBox view should keep function-first split marker `{needle}`."
        );
    }

    assert!(
        !view_source.contains(
            "let description_id = text_field.description.id.clone();\n                view! {"
        ),
        "ComboBox should avoid inline description subrender macro blocks once helper exists.",
    );
    assert!(
        !view_source.contains("let error_id = text_field.error.id.clone();\n                let error_id = StoredValue::new(error_id);\n                let error = StoredValue::new(error);\n                view! {"),
        "ComboBox should avoid inline error subrender macro blocks once helper exists.",
    );

    assert!(
        check2_source.contains("- [x] 函数式拆分优先："),
        "combo-box check2 should mark function-first split item complete.",
    );
    assert!(
        check2_source.contains(
            "combo_box_view_functional_split_prefers_plain_functions_over_extra_local_components"
        ),
        "combo-box check2 should reference function-first split regression test name.",
    );

    let script_needle = "cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_view_functional_split_prefers_plain_functions_over_extra_local_components";
    assert!(
        script_source.contains(script_needle),
        "view macro gate script should include `{script_needle}`.",
    );
}

#[test]
fn combo_box_static_fragments_are_constantized_with_stable_semantics() {
    let view_source = load_source("../../components/combo-box/src/view.rs");
    let check2_source = load_source("../../components/combo-box/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-view-macro.sh");

    for needle in [
        "const SLOT_COMBO_BOX: &str = \"combo-box\";",
        "const SLOT_COMBO_BOX_PANEL: &str = \"combo-box-panel\";",
        "const SLOT_COMBO_BOX_OPTION: &str = \"combo-box-option\";",
        "const SLOT_COMBO_BOX_EMPTY: &str = \"combo-box-empty\";",
        "const CLASS_COMBO_BOX_PANEL: &str = \"ui-combo-box__panel\";",
        "const CLASS_COMBO_BOX_OPTION: &str = \"ui-combo-box__option\";",
        "const CLASS_ACTIVE_HIGHLIGHT: &str = \"ui-active-highlight\";",
        "const TRIGGER_GLYPH: &str = \"▾\";",
        "data-slot=SLOT_COMBO_BOX",
        "data-slot=SLOT_COMBO_BOX_PANEL",
        "data-slot=SLOT_COMBO_BOX_OPTION",
        "data-slot=SLOT_COMBO_BOX_EMPTY",
        "class=CLASS_COMBO_BOX_PANEL",
        "class=CLASS_COMBO_BOX_OPTION",
        "class=CLASS_ACTIVE_HIGHLIGHT",
        "{TRIGGER_GLYPH}",
    ] {
        assert!(
            view_source.contains(needle),
            "ComboBox should keep static-fragment constantization marker `{needle}`."
        );
    }

    for needle in [
        "role=aria.listbox.role",
        "aria-label=move || toggle_button_aria_label.get_value()",
        "data-slot=SLOT_COMBO_BOX_TRIGGER",
    ] {
        assert!(
            view_source.contains(needle),
            "ComboBox static constantization must keep a11y/semantics marker `{needle}` stable."
        );
    }

    assert!(
        check2_source.contains("- [x] 静态片段常量化："),
        "combo-box check2 should mark static-fragment constantization item complete.",
    );
    assert!(
        check2_source.contains("combo_box_static_fragments_are_constantized_with_stable_semantics"),
        "combo-box check2 should reference static-fragment regression test name.",
    );

    let script_needle = "cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_static_fragments_are_constantized_with_stable_semantics";
    assert!(
        script_source.contains(script_needle),
        "view macro gate script should include `{script_needle}`.",
    );
}

#[test]
fn combo_box_inner_html_usage_is_forbidden_in_component_and_docs_examples() {
    let mod_source = load_source("../../components/combo-box/src/mod.rs");
    let logic_source = load_source("../../components/combo-box/src/logic.rs");
    let styles_source = load_source("../../components/combo-box/src/styles.rs");
    let view_source = load_source("../../components/combo-box/src/view.rs");
    let motion_source = load_source("../../components/combo-box/src/motion.rs");
    let readme_source = load_source("../../components/combo-box/src/README.md");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let checklist_source = load_source("../../components/combo-box/check2.md");

    for forbidden in [
        "inner_html=",
        "set_inner_html(",
        "dangerously_set_inner_html",
        "markdown_to_html(",
        "format!(\"<",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !readme_source.contains(forbidden)
                && !docs_source.contains(forbidden),
            "ComboBox should not use html injection path `{forbidden}` in component/docs paths.",
        );
    }

    for required in [
        "- [x] `inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。",
        "仅允许编译期常量或明确白名单内容进入 `inner_html`。",
        "严禁直接或间接注入用户输入、远端返回或未清洗模板字符串。",
        "使用 `inner_html` 的节点必须补语义测试与安全回归说明。",
    ] {
        assert!(
            checklist_source.contains(required),
            "ComboBox checklist should keep inner_html safety governance rule `{required}`.",
        );
    }
}

#[test]
fn combo_box_inner_html_check_script_covers_security_contract() {
    let script_source = load_source("../../scripts/check-ui-components-inner-html.sh");

    let needle = "cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_inner_html_usage_is_forbidden_in_component_and_docs_examples";
    assert!(
        script_source.contains(needle),
        "inner-html check script should enforce `{needle}`."
    );
}

#[test]
fn combo_box_wasm_debug_contract_reuses_global_debug_trace_and_keeps_feature_isolated() {
    let cargo_source = load_source("Cargo.toml");
    let button_view_source = load_source("src/button/view.rs");
    let docs_app_source = load_source("../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let trace_source = load_source("../ui-headless/src/trace.rs");
    let view_source = load_source("../../components/combo-box/src/view.rs");
    let logic_source = load_source("../../components/combo-box/src/logic.rs");
    let motion_source = load_source("../../components/combo-box/src/motion.rs");
    let check2_source = load_source("../../components/combo-box/check2.md");

    assert!(
        cargo_source.contains("button-wasm-debug = [\"component-button\", \"dep:tracing\"]"),
        "wasm debug capability should stay feature-gated via `button-wasm-debug`."
    );

    let all_components_start = cargo_source
        .find("all-components = [")
        .expect("all-components feature list should exist");
    let all_components_end = cargo_source[all_components_start..]
        .find("\n\ncomponent-accordion")
        .map(|offset| all_components_start + offset)
        .expect("all-components list should end before component feature declarations");
    let all_components_block = &cargo_source[all_components_start..all_components_end];
    assert!(
        !all_components_block.contains("button-wasm-debug"),
        "wasm debug feature must not be pulled into all-components production path."
    );

    for needle in [
        "data-debug-source=source.clone()",
        "data-debug-before=before_attr",
        "data-debug-after=after_attr",
        "data-debug-timestamp-ms=format!(\"{:.0}\", event.timestamp_ms)",
        "data-slot=\"button-debug-replay\"",
        "request_replay.run(event.source)",
    ] {
        assert!(
            button_view_source.contains(needle),
            "shared button wasm debug path should keep trace/replay marker `{needle}`."
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
            "docs debug visual entry should keep `{needle}`."
        );
    }

    for needle in [
        "events.push(event);",
        ".into_iter()",
        ".take(40)",
        "let ts_ms = event.ts_ms;",
        "UiTraceEventKind::Note",
        "UiTraceEventKind::Inspect",
    ] {
        assert!(
            trace_source.contains(needle) || debug_overlay_source.contains(needle),
            "global trace timeline/replay evidence should keep marker `{needle}`."
        );
    }

    for needle in [
        "data-state=move || {",
        "data-controlled=state.is_controlled.then_some(\"true\")",
        "data-uncontrolled=state.is_uncontrolled.then_some(\"true\")",
        "data-label-source=state.label_source_attr",
        "data-placeholder-source=state.placeholder_source_attr",
        "data-id-source=state.id_source_attr",
        "data-class-source=state.class_source_attr",
        "data-motion-source=state.motion_source_attr",
        "let key_result = aria.handlers.on_input_key_down.run(key);",
        "on:pointermove=move |_| on_option_pointer_move.run(filtered_index)",
        "on:click=move |_| on_option_click.run(filtered_index)",
        "aria.handlers.toggle.run(())",
    ] {
        assert!(
            view_source.contains(needle),
            "ComboBox should keep machine-readable state/source/interaction marker `{needle}` for debug attribution."
        );
    }

    for forbidden in [
        "combo-box-wasm-debug",
        "wasm_debug",
        "render_debug_panel(",
        "data-debug-source",
        "request_replay.run(",
        "trace.emit(",
        "#[prop(optional)] debug",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "ComboBox should not duplicate shared wasm debug runtime token `{forbidden}`."
        );
    }

    for needle in [
        "WASM 调试要求：关键状态可追踪",
        "开发模式下至少能追踪关键状态变更来源与前后值",
        "关键交互链路应支持最小可复现记录",
        "调试开关默认不进入生产包体与公共 API",
    ] {
        assert!(
            check2_source.contains(needle),
            "ComboBox checklist should keep wasm-debug governance contract marker `{needle}`."
        );
    }
}

#[test]
fn combo_box_wasm_debug_check_script_covers_shared_contract() {
    let script_source = load_source("../../scripts/check-ui-components-wasm-debug.sh");

    let needle = "cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_wasm_debug_contract_reuses_global_debug_trace_and_keeps_feature_isolated";
    assert!(
        script_source.contains(needle),
        "wasm debug check script should enforce `{needle}`."
    );
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
fn combo_box_check2_documents_streaming_definition_is_llm_output_only_with_two_modes() {
    let check2_source = load_source("../../components/combo-box/check2.md");

    for needle in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
    ] {
        assert!(
            check2_source.contains(needle),
            "combo_box/check2.md should keep two-mode LLM streaming definition marker `{needle}`."
        );
    }
}

#[test]
fn combo_box_streaming_check_script_covers_two_mode_definition_guard() {
    let script_source = load_source("../../scripts/check-ui-components-streaming.sh");

    let needle = "cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_check2_documents_streaming_definition_is_llm_output_only_with_two_modes";
    assert!(
        script_source.contains(needle),
        "streaming check script should enforce `{needle}`."
    );
}

#[test]
fn combo_box_check2_documents_snapshot_as_default_baseline_capability() {
    let check2_source = load_source("../../components/combo-box/check2.md");

    for needle in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
    ] {
        assert!(
            check2_source.contains(needle),
            "combo_box/check2.md should keep snapshot-baseline marker `{needle}`."
        );
    }
}

#[test]
fn combo_box_snapshot_baseline_consumes_complete_result_and_renders_stably() {
    let view_source = load_source("../../components/combo-box/src/view.rs");
    let logic_source = load_source("../../components/combo-box/src/logic.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let docs_section = combo_box_docs_section(&docs_source);

    for marker in [
        "pub fn ComboBox(",
        "selected_index: ReadSignal<Option<usize>>",
        "set_selected_index: WriteSignal<Option<usize>>",
        "data-state=move || {",
        "data-controlled=state.is_controlled.then_some(\"true\")",
        "data-uncontrolled=state.is_uncontrolled.then_some(\"true\")",
        "data-id-source=state.id_source_attr",
        "data-placeholder-source=state.placeholder_source_attr",
    ] {
        assert!(
            view_source.contains(marker),
            "ComboBox snapshot baseline should keep stable complete-result render marker `{marker}`."
        );
    }

    for marker in [
        "pub struct OpenStateInput",
        "pub struct NormalizedOpenState",
        "pub fn normalize_open_state(input: OpenStateInput) -> NormalizedOpenState",
        "pub struct RootStateInput",
        "pub struct RootState",
        "pub fn normalize_root_state(input: RootStateInput) -> RootState",
    ] {
        assert!(
            logic_source.contains(marker),
            "ComboBox logic should keep normalized complete-result marker `{marker}`."
        );
    }

    for marker in [
        "title=\"ComboBox\"",
        "slug=\"combo-box\"",
        "<ComboBox",
        "selected_index=selected",
        "set_selected_index=set_selected",
    ] {
        assert!(
            docs_section.contains(marker),
            "ComboBox docs should keep snapshot-ready baseline usage marker `{marker}`."
        );
    }
}

#[test]
fn combo_box_streaming_check_script_covers_snapshot_baseline_contract() {
    let script_source = load_source("../../scripts/check-ui-components-streaming.sh");

    for needle in [
        "cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_check2_documents_snapshot_as_default_baseline_capability",
        "cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_snapshot_baseline_consumes_complete_result_and_renders_stably",
    ] {
        assert!(
            script_source.contains(needle),
            "streaming check script should enforce `{needle}`."
        );
    }
}

#[test]
fn combo_box_check2_documents_streaming_required_optional_classification_rules() {
    let check2_source = load_source("../../components/combo-box/check2.md");

    for needle in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
        "归类为 `Streaming Optional`",
        "data-ui-stream-support=\\\"unsupported\\\"",
        "data-ui-stream-fallback=\\\"snapshot\\\"",
        "data-ui-output-status=\\\"verified\\\"",
    ] {
        assert!(
            check2_source.contains(needle),
            "combo_box/check2.md should keep streaming required/optional marker `{needle}`."
        );
    }
}

#[test]
fn combo_box_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous() {
    let view_source = load_source("../../components/combo-box/src/view.rs");

    for needle in [
        "role=aria.input.role",
        "aria-controls=move || aria.input.aria_controls.get()",
        "aria-expanded=move || aria.input.aria_expanded.get()",
        "aria-activedescendant=move || aria.input.aria_activedescendant.get()",
        "aria-describedby=move || text_field.input.aria_describedby.get()",
        "aria-invalid=move || text_field.input.aria_invalid.get()",
        "aria-required=move || text_field.input.aria_required.get()",
        "data-ui-stream-support=move || agent_contract.get().stream_support.as_str()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
        "data-ui-stream-mode=\"snapshot\"",
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
    ] {
        assert!(
            view_source.contains(needle),
            "ComboBox should keep continuous aria/data semantics marker `{needle}` in optional-streaming scope."
        );
    }
}

#[test]
fn combo_box_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer() {
    let view_source = load_source("../../components/combo-box/src/view.rs");
    let logic_source = load_source("../../components/combo-box/src/logic.rs");
    let combined = format!("{view_source}\n{logic_source}");

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
            "ComboBox should keep validation/retry/resilience policy in upper layer; component must not include `{forbidden}`."
        );
    }
}

#[test]
fn combo_box_streaming_check_script_covers_required_optional_scope_contract() {
    let script_source = load_source("../../scripts/check-ui-components-streaming.sh");

    for needle in [
        "cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_check2_documents_streaming_required_optional_classification_rules",
        "cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
    ] {
        assert!(
            script_source.contains(needle),
            "streaming check script should enforce `{needle}`."
        );
    }
}

#[test]
fn combo_box_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources() {
    let mod_source = load_source("../../components/combo-box/src/mod.rs");
    let logic_source = load_source("../../components/combo-box/src/logic.rs");
    let styles_source = load_source("../../components/combo-box/src/styles.rs");
    let view_source = load_source("../../components/combo-box/src/view.rs");
    let motion_source = load_source("../../components/combo-box/src/motion.rs");
    let combined =
        format!("{mod_source}\n{logic_source}\n{styles_source}\n{view_source}\n{motion_source}");

    for forbidden in [".unwrap(", ".expect(", "let _ ="] {
        assert!(
            !combined.contains(forbidden),
            "ComboBox non-test source should forbid rust-hygiene anti-pattern `{forbidden}`."
        );
    }
}

#[test]
fn combo_box_rust_hygiene_string_clone_hotspots_converge_to_cow_or_static_borrow() {
    let logic_source = load_source("../../components/combo-box/src/logic.rs");
    let view_source = load_source("../../components/combo-box/src/view.rs");

    for required in [
        "use std::borrow::Cow;",
        "let mut classes: Vec<Cow<'static, str>> = vec![Cow::Borrowed(\"ui-combo-box\")];",
        "classes.push(Cow::Borrowed(\"ui-combo-box--custom-class\"));",
        ".map(Cow::into_owned)",
    ] {
        assert!(
            logic_source.contains(required),
            "ComboBox logic should keep Cow-based string hotspot mitigation marker `{required}`."
        );
    }

    for forbidden in [
        "\"ui-combo-box\".to_string()",
        "ui_state_primitives::combo_box::DEFAULT_ID_BASE.to_string()",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "ComboBox string hotspot contract should avoid `{forbidden}`."
        );
    }
}

#[test]
fn combo_box_check2_marks_rust_hygiene_contract_complete() {
    let check2_source = load_source("../../components/combo-box/check2.md");

    for needle in [
        "- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。",
        "Vec<Cow<'static, str>>",
        "./scripts/check-rust-hygiene.sh",
        "combo_box_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "combo_box_rust_hygiene_string_clone_hotspots_converge_to_cow_or_static_borrow",
    ] {
        assert!(
            check2_source.contains(needle),
            "combo_box/check2.md should keep rust-hygiene evidence marker `{needle}`."
        );
    }
}

#[test]
fn combo_box_rust_hygiene_script_enforces_global_contract() {
    let script_source = load_source("../../scripts/check-rust-hygiene.sh");

    for needle in [
        "forbidden unwrap/expect in non-test code",
        "forbidden let _ = in non-test code",
        "string clone hotspots (prefer Cow<'static, str>)",
        "failed: enforce no unwrap/expect, no let _ = swallowing, and no string clone churn",
    ] {
        assert!(
            script_source.contains(needle),
            "rust-hygiene script should enforce `{needle}`."
        );
    }
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
fn combo_box_component_local_semantics_test_file_exists() {
    let source = load_source("../../components/combo-box/src/mod.rs");
    let semantics = load_source("../../components/combo-box/test/semantics.rs");

    for needle in [
        "#[path = \"../test/semantics.rs\"]",
        "mod semantics_tests;",
        "fn combo_box_public_api_exports_are_minimal_and_dom_agnostic()",
        "fn combo_box_layering_is_split_across_logic_view_styles_and_motion()",
    ] {
        assert!(
            source.contains(needle) || semantics.contains(needle),
            "combo-box should keep local semantics test landing via `{needle}`."
        );
    }
}

#[test]
fn combo_box_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks()
 {
    let view_source = load_source("../../components/combo-box/src/view.rs");
    let local_semantics_source = load_source("../../components/combo-box/test/semantics.rs");
    let semantics_source = load_source("tests/combo_box_semantics.rs");
    let check2_source = load_source("../../components/combo-box/check2.md");
    let perf_script_source = load_source("../../scripts/check-ui-components-performance.sh");

    for needle in [
        "aria-controls=move || aria.input.aria_controls.get()",
        "role=aria.input.role",
        "role=move || option_attrs.run(filtered_index).role",
        "data-state=move ||",
        "data-open=move ||",
        "data-label-source=state.label_source_attr",
        "data-description-source=state.description_source_attr",
        "data-error-source=state.error_source_attr",
        "data-placeholder-source=state.placeholder_source_attr",
        "data-id-source=state.id_source_attr",
        "data-class-source=state.class_source_attr",
        "data-motion-source=state.motion_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "combo-box semantic tests should keep aria/data/role/source marker `{needle}`."
        );
    }

    for needle in [
        "fn combo_box_public_api_exports_are_minimal_and_dom_agnostic()",
        "fn combo_box_layering_is_split_across_logic_view_styles_and_motion()",
        "fn combo_box_component_layer_does_not_reimplement_primitives_or_headless_contracts()",
    ] {
        assert!(
            local_semantics_source.contains(needle),
            "combo-box should keep local *_semantics.rs coverage marker `{needle}`."
        );
    }

    for needle in [
        "fn combo_box_passes_lang_dir_and_headless_aria_controls_contract()",
        "fn combo_box_emits_baseline_style_state_data_attributes()",
        "fn combo_box_component_local_semantics_test_file_exists()",
        "fn combo_box_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()",
    ] {
        assert!(
            semantics_source.contains(needle),
            "combo-box semantic suite should keep contract-focused assertion `{needle}`."
        );
    }

    for forbidden in ["assert_snapshot!", "insta::assert"] {
        assert!(
            !local_semantics_source.contains(forbidden) && !semantics_source.contains(forbidden),
            "combo-box semantic contract should avoid snapshot-only assertion marker `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks";
    assert!(
        perf_script_source.contains(script_needle),
        "performance gate script should include semantic-priority command `{script_needle}`."
    );

    for needle in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "components/combo-box/test/semantics.rs",
        "combo_box_passes_lang_dir_and_headless_aria_controls_contract",
        "combo_box_emits_baseline_style_state_data_attributes",
        "combo_box_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks",
        "scripts/check-ui-components-performance.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "combo_box/check2.md should keep semantic-test-priority evidence marker `{needle}`."
        );
    }
}

#[test]
fn combo_box_check2_documents_semantic_e2e_selector_and_ready_wait_contract() {
    let check2_source = load_source("../../components/combo-box/check2.md");

    for needle in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "e2e/tests/docs_app_combo_box_contract.spec.mjs",
        "body:not(:has(#boot))",
        "data-slot=\"combo-box\"",
        "ready/settled",
        "combo_box_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
        "combo_box_e2e_flow_covers_ready_and_settled_semantic_breakpoints",
        "scripts/check-ui-components-e2e-combo-box.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "combo_box/check2.md should keep e2e stability marker `{needle}`."
        );
    }
}

#[test]
fn combo_box_e2e_selector_contract_uses_semantic_markers_and_settled_waits() {
    let e2e_source = load_source("../../e2e/tests/docs_app_combo_box_contract.spec.mjs");

    for needle in [
        "/#/components/combo-box",
        "body:not(:has(#boot))",
        "[data-component=\"combo-box\"]",
        "[data-slot=\"combo-box-showcase\"]",
        "[data-slot=\"combo-box\"][data-controlled=\"true\"]",
        "[data-slot=\"combo-box-input\"][role=\"combobox\"]",
        "[data-slot=\"combo-box-streaming-snapshot\"] [data-ui-output-state=\"snapshot\"]",
        "[data-slot=\"combo-box-streaming-snapshot\"] [data-ui-output-state=\"streaming\"]",
        "toHaveAttribute(\"data-state\", \"closed\")",
        "toHaveAttribute(\"data-ui-stream-fallback\", \"snapshot\")",
        "toHaveAttribute(\"aria-expanded\", \"false\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "ComboBox e2e semantic-selector contract should include `{needle}`."
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "sleep(",
        "getByText(",
        "locator(\"text=",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "ComboBox e2e selector contract should avoid unstable token `{forbidden}`."
        );
    }
}

#[test]
fn combo_box_e2e_flow_covers_ready_and_settled_semantic_breakpoints() {
    let e2e_source = load_source("../../e2e/tests/docs_app_combo_box_contract.spec.mjs");

    for needle in [
        "docs-app combo-box flow is repeatable with semantic ready/settled breakpoints",
        "await trigger.click();",
        "toHaveAttribute(\"data-state\", \"open\")",
        "toHaveAttribute(\"aria-expanded\", \"true\")",
        "toHaveAttribute(\"data-ui-action\", \"navigate-options\")",
        "await page.keyboard.press(\"Escape\")",
        "toHaveAttribute(\"data-state\", \"closed\")",
        "toHaveAttribute(\"aria-expanded\", \"false\")",
        "toHaveCount(0)",
        "await controlledInput.fill(\"Ru\");",
        "toHaveAttribute(\"data-typed\", \"true\")",
        "toHaveAttribute(\"data-ui-action\", \"filter-query\")",
        "await page.reload();",
    ] {
        assert!(
            e2e_source.contains(needle),
            "ComboBox e2e flow should keep semantic ready/settled marker `{needle}`."
        );
    }
}

#[test]
fn combo_box_e2e_regression_set_covers_repeatable_overlay_focus_keyboard_paths() {
    let e2e_source = load_source("../../e2e/tests/docs_app_combo_box_contract.spec.mjs");
    let check2_source = load_source("../../components/combo-box/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-e2e-combo-box.sh");

    for needle in [
        "docs-app combo-box flow is repeatable with semantic ready/settled breakpoints",
        "await trigger.click();",
        "[data-slot=\"combo-box-panel\"]",
        "[data-slot=\"combo-box-listbox\"]",
        "await page.keyboard.press(\"Escape\")",
        "await controlledInput.focus();",
        "await controlledInput.fill(\"Ru\");",
        "await page.reload();",
        "toHaveCount(0)",
    ] {
        assert!(
            e2e_source.contains(needle),
            "ComboBox e2e regression set should include high-risk flow marker `{needle}`."
        );
    }

    for forbidden in ["waitForTimeout(", "setTimeout(", "sleep("] {
        assert!(
            !e2e_source.contains(forbidden),
            "ComboBox e2e repeatable regression flow should avoid unstable wait token `{forbidden}`."
        );
    }

    for needle in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "e2e/tests/docs_app_combo_box_contract.spec.mjs",
        "open -> interaction -> Escape close",
        "overlay/focus/keyboard",
        "combo_box_e2e_regression_set_covers_repeatable_overlay_focus_keyboard_paths",
        "scripts/check-ui-components-e2e-combo-box.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "combo_box/check2.md should keep repeatable e2e regression evidence marker `{needle}`."
        );
    }

    let script_needle = "cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_e2e_regression_set_covers_repeatable_overlay_focus_keyboard_paths";
    assert!(
        script_source.contains(script_needle),
        "combo-box e2e check script should include `{script_needle}`."
    );
}

#[test]
fn combo_box_e2e_check_script_covers_selector_and_ready_wait_contract() {
    let script_source = load_source("../../scripts/check-ui-components-e2e-combo-box.sh");

    for needle in [
        "combo_box_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
        "combo_box_e2e_flow_covers_ready_and_settled_semantic_breakpoints",
        "combo_box_e2e_regression_set_covers_repeatable_overlay_focus_keyboard_paths",
        "--features component-combo_box,inject-css",
    ] {
        assert!(
            script_source.contains(needle),
            "combo-box e2e check script should include `{needle}`."
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

#[test]
fn combo_box_tree_shaking_feature_pruning_is_gated_in_lib_and_css() {
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");

    let combo_export_idx = lib_source
        .find("pub use ui_combo_box as combo_box;")
        .expect("ui-components lib.rs should expose combo_box module.");
    let lib_prefix_start = combo_export_idx.saturating_sub(96);
    let lib_prefix = &lib_source[lib_prefix_start..combo_export_idx];
    assert!(
        lib_prefix.contains("#[cfg(feature = \"component-combo_box\")]"),
        "combo_box export in lib.rs must stay behind `component-combo_box` feature gate."
    );

    let combo_css_idx = css_source
        .find("out.push_str(crate::combo_box::styles::CSS);")
        .expect("ui-components css.rs should aggregate combo_box CSS when feature is enabled.");
    let css_prefix_start = combo_css_idx.saturating_sub(96);
    let css_prefix = &css_source[css_prefix_start..combo_css_idx];
    assert!(
        css_prefix.contains("#[cfg(feature = \"component-combo_box\")]"),
        "combo_box CSS aggregation in css.rs must stay behind `component-combo_box` feature gate."
    );

    for needle in [
        "pub fn push_components_css(out: &mut String) {",
        "#[cfg(not(feature = \"inject-css\"))]",
        "pub fn push_components_css(_out: &mut String) {}",
    ] {
        assert!(
            css_source.contains(needle),
            "css aggregation entry should preserve tree-shaking guard marker `{needle}`."
        );
    }
}

#[test]
fn combo_box_check2_marks_tree_shaking_feature_pruning_contract_complete() {
    let check2_source = load_source("../../components/combo-box/check2.md");

    assert!(
        check2_source.contains("- [x] Tree Shaking & 特性剪裁："),
        "combo_box/check2.md should mark tree-shaking feature-pruning item complete."
    );

    for needle in [
        "component-combo_box",
        "#[cfg(feature = \"component-combo_box\")] pub use ui_combo_box as combo_box;",
        "out.push_str(crate::combo_box::styles::CSS);",
        "cargo tree -e features -i ui-components -p ui-components --no-default-features --features component-combo_box,inject-css",
        "cargo tree -e features -i ui-components -p web-demo",
        "combo_box_tree_shaking_feature_pruning_is_gated_in_lib_and_css",
    ] {
        assert!(
            check2_source.contains(needle),
            "combo_box/check2.md tree-shaking feature-pruning evidence should reference `{needle}`."
        );
    }
}

#[test]
fn combo_box_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()
 {
    let view_source = load_source("../../components/combo-box/src/view.rs");
    let check2_source = load_source("../../components/combo-box/check2.md");
    let perf_script_source = load_source("../../scripts/check-ui-components-performance.sh");
    let semantics_source = load_source("tests/combo_box_semantics.rs");
    let todo_source = load_source("../../docs/plan/TODO.md");

    for needle in [
        "aria-controls=move || aria.input.aria_controls.get()",
        "role=move || option_attrs.run(filtered_index).role",
        "aria-selected=move || option_attrs.run(filtered_index).aria_selected",
        "aria-disabled=move || option_attrs.run(filtered_index).aria_disabled",
        "data-state=move ||",
        "data-label-source=state.label_source_attr",
        "data-focused=move || focus_ring.is_focused.get().then_some(\"true\")",
        "data-focus-visible=move || focus_ring.is_focus_visible.get().then_some(\"true\")",
        "class:ui-combo-box--focus-visible=move || focus_ring.is_focus_visible.get()",
        "on:focus=on_focus",
        "on:blur=on_blur",
    ] {
        assert!(
            view_source.contains(needle),
            "ComboBox view should keep semantics/focus contract marker `{needle}`."
        );
    }

    let perf_gate_needle = "cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_performance_governance_budget_is_defined_and_blocking";
    assert!(
        perf_script_source.contains(perf_gate_needle),
        "performance gate script should include `{perf_gate_needle}`."
    );

    assert!(
        semantics_source
            .contains("fn combo_box_performance_governance_budget_is_defined_and_blocking()"),
        "combo_box semantics suite should keep a dedicated blocking performance governance test."
    );

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "performance governance follow-up tracking should include `{needle}`."
        );
    }

    for needle in [
        "- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。",
        "combo_box_passes_lang_dir_and_headless_aria_controls_contract",
        "combo_box_emits_baseline_style_state_data_attributes",
        "combo_box_performance_governance_budget_is_defined_and_blocking",
        "`render_count` 自动化回归仍在仓库统一 follow-up",
    ] {
        assert!(
            check2_source.contains(needle),
            "combo_box/check2.md should keep semantics+performance evidence marker `{needle}`."
        );
    }
}
