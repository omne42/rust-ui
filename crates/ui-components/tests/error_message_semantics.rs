use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn error_message_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/error_message/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ErrorMessage internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn error_message_uses_logic_state_model() {
    let logic_source = load_source("src/error_message/logic.rs");
    let view_source = load_source("src/error_message/view.rs");

    for needle in [
        "pub enum ErrorMessageTone",
        "pub enum ErrorMessageElement",
        "pub fn normalize_optional_text(",
        "pub fn normalize_message(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_effective_tone(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "message_source_attr",
        "aria_source_attr",
        "class_source_attr",
        "data_state_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "ErrorMessage logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_message(Some(text))",
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(ErrorMessageStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "ErrorMessage view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn error_message_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/error_message/view.rs");

    for attr in [
        "data-slot=\"error-message\"",
        "slot=\"errorMessage\"",
        "data-tone=move || state.get().tone_attr",
        "data-state=move || state.get().data_state_attr",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-truncate=move || state.get().is_truncated.then_some(\"true\")",
        "data-message-source=move || state.get().message_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            source.contains(attr),
            "ErrorMessage should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn error_message_styles_include_tone_state_and_markers() {
    let source = load_source("src/error_message/styles.rs");

    for selector in [
        ".ui-error-message--tone-auto",
        ".ui-error-message[data-tone=\"auto\"]",
        ".ui-error-message--tone-negative",
        ".ui-error-message[data-tone=\"negative\"]",
        ".ui-error-message--tone-neutral",
        ".ui-error-message[data-tone=\"neutral\"]",
        ".ui-error-message--disabled",
        ".ui-error-message[data-disabled=\"true\"]",
        ".ui-error-message--truncate",
        ".ui-error-message[data-truncate=\"true\"]",
        ".ui-error-message--custom-class",
        ".ui-error-message[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "ErrorMessage styles should include `{selector}` as stable state-marker contracts."
        );
    }
}
