use std::path::Path;

fn load_source(path: &str) -> &'static str {
    match path {
        "mod" => include_str!("../src/mod.rs"),
        "logic" => include_str!("../src/logic.rs"),
        "primitives" => include_str!("../../../crates/ui-state-primitives/src/color_field.rs"),
        "view" => include_str!("../src/view.rs"),
        "styles" => include_str!("../src/styles.rs"),
        _ => panic!("unsupported source path: {path}"),
    }
}

#[test]
fn color_field_ui_components_module_keeps_layered_exports() {
    let module = load_source("mod");

    for required in [
        "pub(crate) mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::{ColorFieldState, ColorFieldStateInput};",
        "pub use view::ColorField;",
    ] {
        assert!(
            module.contains(required),
            "color-field module should keep layered export contract `{required}`."
        );
    }
}

#[test]
fn color_field_logic_consumes_state_primitives_and_view_mounts_headless() {
    let logic = load_source("logic");
    let view = load_source("view");

    assert!(
        logic.contains("pub use ui_state_primitives::color_field::{"),
        "color-field logic should consume ui-state-primitives instead of redefining state machines."
    );

    for required in [
        "use ui_headless::{",
        "use_controllable_state(value, Some(default_value), on_value_change)",
        "let i18n = use_ui_i18n();",
        "let locale = locale_attrs(logic::normalize_optional_text(lang), dir);",
    ] {
        assert!(
            view.contains(required),
            "color-field view should mount ui-headless contract via `{required}`."
        );
    }
}

#[test]
fn color_field_api_naming_prefers_is_prefix_with_legacy_alias_bridge() {
    let logic = load_source("logic");
    let view = load_source("view");

    for required in [
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] disabled: Option<bool>",
        "#[prop(optional)] is_preview_visible: Option<bool>",
        "#[prop(optional)] show_preview: Option<bool>",
        "let is_disabled = logic::resolve_is_disabled(is_disabled, disabled);",
        "let is_preview_visible = logic::resolve_is_preview_visible(is_preview_visible, show_preview);",
    ] {
        assert!(
            view.contains(required),
            "color-field view should keep naming compatibility contract `{required}`."
        );
    }

    for required in [
        "pub fn resolve_is_disabled(is_disabled: Option<bool>, disabled: Option<bool>) -> bool",
        "pub fn resolve_is_preview_visible(",
    ] {
        assert!(
            logic.contains(required),
            "color-field logic should centralize naming alias resolution via `{required}`."
        );
    }
}

#[test]
fn color_field_controlled_uncontrolled_contract_is_complete() {
    let view = load_source("view");

    for required in [
        "#[prop(optional)] value: Option<Signal<Option<String>>>",
        "#[prop(optional)] default_value: Option<String>",
        "#[prop(optional)] on_value_change: Option<Callback<Option<String>>>",
        "let default_value = logic::normalize_color_value(default_value);",
        "prop:value=move || logic::resolve_input_value(value.get())",
        "let value_state = use_controllable_state(value, Some(default_value), on_value_change);",
        "let value = value_state.value;",
        "let request_value_change = value_state.request_change;",
    ] {
        assert!(
            view.contains(required),
            "color-field should keep controlled/uncontrolled triple contract marker `{required}`."
        );
    }

    for forbidden in [
        "let (value, set_value) = signal(",
        "set_value.set(",
        "set_value.update(",
        "prop:value=move || value.get().unwrap_or_default()",
    ] {
        assert!(
            !view.contains(forbidden),
            "color-field view must not build ad-hoc local value state: `{forbidden}`."
        );
    }
}

#[test]
fn color_field_public_surface_does_not_leak_platform_dom_types() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");

    for forbidden in ["web_sys::", "web-sys", "wasm_bindgen", "JsValue"] {
        assert!(
            !module.contains(forbidden),
            "color-field module public surface must not expose `{forbidden}`."
        );
        assert!(
            !logic.contains(forbidden),
            "color-field logic must not expose `{forbidden}`."
        );
        assert!(
            !view.contains(forbidden),
            "color-field view must not expose `{forbidden}` in public API surface."
        );
    }
}

#[test]
fn color_field_default_value_resolution_is_centralized_in_logic() {
    let logic = load_source("logic");
    let view = load_source("view");

    for required in [
        "pub fn resolve_is_disabled(is_disabled: Option<bool>, disabled: Option<bool>) -> bool",
        "pub fn resolve_is_preview_visible(",
        "pub fn resolve_input_value(value: Option<String>) -> String",
        "value.unwrap_or_default()",
    ] {
        assert!(
            logic.contains(required),
            "color-field logic should own default resolution marker `{required}`."
        );
    }

    assert!(
        !view.contains("unwrap_or_default()"),
        "color-field view should not perform default fallback directly."
    );
}

#[test]
fn color_field_state_normalization_is_centralized_in_logic_layer() {
    let logic = load_source("logic");
    let primitives = load_source("primitives");
    let view = load_source("view");

    for required in [
        "ColorFieldDerivedStateInput",
        "pub fn resolve_preview_color(value: Option<String>) -> Option<String>",
        "pub fn resolve_next_value(raw_value: String) -> Option<String>",
        "resolve_derived_state",
        "pub fn is_invalid_state(state: ColorFieldState) -> bool",
    ] {
        assert!(
            logic.contains(required) || primitives.contains(required),
            "color-field state normalization contract should stay in logic re-exports or primitives definitions: `{required}`."
        );
    }

    for required in [
        "logic::resolve_preview_color(value.get())",
        "logic::resolve_derived_state(logic::ColorFieldDerivedStateInput {",
        "let next = logic::resolve_next_value(event_target_value(&ev));",
        "data-invalid=move || logic::is_invalid_state(state.get()).then_some(\"true\")",
        "aria-invalid=move || logic::is_invalid_state(state.get()).then_some(\"true\")",
    ] {
        assert!(
            view.contains(required),
            "color-field view should consume centralized logic output via `{required}`."
        );
    }

    for forbidden in [
        "let has_value = raw_value.is_some();",
        "let has_valid_value = preview_color.get().is_some();",
        "(state.get().has_value && !state.get().has_valid_value).then_some(\"true\")",
    ] {
        assert!(
            !view.contains(forbidden),
            "color-field view should not reconstruct state machine details: `{forbidden}`."
        );
    }
}

#[test]
fn color_field_discrete_state_axis_is_type_safe_enum_contract() {
    let logic = load_source("logic");
    let primitives = load_source("primitives");
    let view = load_source("view");

    assert!(
        logic.contains("ColorFieldVisualState")
            || primitives.contains("pub enum ColorFieldVisualState"),
        "color-field logic should consume typed discrete state enum from primitives."
    );

    assert!(
        view.contains("data-state=move || state.get().visual_state.as_attr()"),
        "color-field view should map typed visual_state enum to data-state attr."
    );
}

#[test]
fn color_field_styles_are_token_first_and_test_layout_exists() {
    let styles = load_source("styles");

    assert!(
        styles.contains("pub const CSS: &str ="),
        "color-field styles should expose static CSS contract."
    );
    assert!(
        styles.contains("var(--ui-"),
        "color-field styles should consume ui-theme tokens."
    );

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let test_dir = manifest_dir.join("test");
    assert!(
        test_dir.join("protocol.rs").exists(),
        "color-field test directory should keep protocol tests in src-adjacent `test/`."
    );
    assert!(
        test_dir.join("semantics.rs").exists(),
        "color-field should provide `test/semantics.rs` for semantic contract coverage."
    );
}
