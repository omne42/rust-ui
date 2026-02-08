use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn field_button_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/field_button/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "FieldButton internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn field_button_uses_logic_state_model() {
    let logic_source = load_source("src/field_button/logic.rs");
    let view_source = load_source("src/field_button/view.rs");

    for needle in [
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "aria_source_attr",
        "class_source_attr",
        "data_state_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "FieldButton logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(FieldButtonStateInput {",
        "logic::compose_class_name(class_name, state)",
    ] {
        assert!(
            view_source.contains(needle),
            "FieldButton view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn field_button_uses_headless_hooks() {
    let source = load_source("src/field_button/view.rs");

    for needle in ["use_button", "use_focus_ring", "use_hover"] {
        assert!(
            source.contains(needle),
            "FieldButton should use headless `{needle}` hooks for consistent modality semantics."
        );
    }
}

#[test]
fn field_button_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/field_button/view.rs");

    for attr in [
        "data-slot=\"field-button\"",
        "data-state=state.data_state_attr",
        "data-quiet=state.is_quiet.then_some(\"true\")",
        "data-invalid=state.is_invalid.then_some(\"true\")",
        "data-disabled=state.is_disabled.then_some(\"true\")",
        "data-active=move || (is_active || aria.is_pressed.get()).then_some(\"true\")",
        "data-hovered=move || hover.is_hovered.get().then_some(\"true\")",
        "data-pressed=move || aria.is_pressed.get().then_some(\"true\")",
        "data-has-handler=state.has_custom_press_handler.then_some(\"true\")",
        "data-active-mode=state.active_mode_attr",
        "data-quiet-mode=state.quiet_attr",
        "data-invalid-mode=state.invalid_attr",
        "data-disabled-mode=state.disabled_attr",
        "data-aria-source=state.aria_source_attr",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-class-source=state.class_source_attr",
    ] {
        assert!(
            source.contains(attr),
            "FieldButton should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn field_button_styles_include_quiet_invalid_and_active_markers() {
    let source = load_source("src/field_button/styles.rs");

    for selector in [
        ".ui-field-button--quiet",
        ".ui-field-button[data-quiet=\"true\"]",
        ".ui-field-button--invalid",
        ".ui-field-button[data-invalid=\"true\"]",
        ".ui-field-button.is-hovered",
        ".ui-field-button[data-hovered=\"true\"]",
        ".ui-field-button.is-active",
        ".ui-field-button[data-active=\"true\"]",
        ".ui-field-button[data-pressed=\"true\"]",
        ".ui-field-button--disabled",
        ".ui-field-button[data-disabled=\"true\"]",
        ".ui-field-button--focus-visible",
        ".ui-field-button--custom-class",
        ".ui-field-button[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "FieldButton styles should include `{selector}` as stable state-marker contracts."
        );
    }
}
