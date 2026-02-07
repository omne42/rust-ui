use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn input_group_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/input_group/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "InputGroup internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn input_group_uses_logic_state_model() {
    let logic_source = load_source("src/input_group/logic.rs");
    let view_source = load_source("src/input_group/view.rs");

    for needle in [
        "pub enum InputGroupPhase",
        "pub enum InputGroupAttachment",
        "pub struct InputGroupStateInput",
        "pub struct InputGroupState",
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "label_source_attr",
        "class_source_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "InputGroup logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_optional_text(class_name)",
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(InputGroupStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "InputGroup view should derive root state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn input_group_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/input_group/view.rs");

    for attr in [
        "data-slot=\"input-group\"",
        "data-state=move || state.get().phase_attr",
        "data-attachment=move || state.get().attachment_attr",
        "data-enabled=move || state.get().is_enabled.then_some(\"true\")",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-invalid=move || state.get().is_invalid.then_some(\"true\")",
        "data-attached=move || state.get().is_attached.then_some(\"true\")",
        "data-detached=move || state.get().is_detached.then_some(\"true\")",
        "data-has-start=move || state.get().has_start_content.then_some(\"true\")",
        "data-has-end=move || state.get().has_end_content.then_some(\"true\")",
        "data-label-source=move || state.get().label_source_attr",
        "data-custom-label=move || state.get().has_custom_label.then_some(\"true\")",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
        "role=\"group\"",
    ] {
        assert!(
            source.contains(attr),
            "InputGroup should set `{attr}` to support Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn input_group_styles_define_attachment_and_state_contracts() {
    let source = load_source("src/input_group/styles.rs");

    for selector in [
        ".ui-input-group--invalid .ui-input-group__control",
        ".ui-input-group[data-invalid=\"true\"] .ui-input-group__control",
        ".ui-input-group--state-disabled",
        ".ui-input-group[data-state=\"disabled\"]",
        ".ui-input-group[data-disabled=\"true\"]",
        ".ui-input-group--detached .ui-input-group__control",
        ".ui-input-group[data-attachment=\"detached\"] .ui-input-group__control",
        ".ui-input-group--label-custom",
        ".ui-input-group[data-label-source=\"custom\"]",
        ".ui-input-group--custom-class",
        ".ui-input-group[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "InputGroup styles should include `{selector}` as stable state-marker contracts."
        );
    }
}
