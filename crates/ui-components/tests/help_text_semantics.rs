use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn help_text_does_not_expose_logic_or_render_modules() {
    let source = load_source("src/help_text/mod.rs");

    for needle in ["pub mod logic", "pub mod render"] {
        assert!(
            !source.contains(needle),
            "HelpText internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn help_text_uses_logic_state_model() {
    let logic_source = load_source("src/help_text/logic.rs");
    let render_source = load_source("src/help_text/render.rs");

    for needle in [
        "pub enum HelpTextTone",
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(",
        "pub fn normalize_error_message(",
        "pub fn resolve_effective_tone(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "aria_source_attr",
        "error_source_attr",
        "class_source_attr",
        "message_kind_attr",
        "data_state_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "HelpText logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_aria_label(aria_label)",
        "logic::normalize_error_message(error_message, invalid)",
        "logic::resolve_state(HelpTextStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            render_source.contains(needle),
            "HelpText render should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn help_text_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/help_text/render.rs");

    for attr in [
        "data-slot=\"help-text\"",
        "data-tone=move || state.get().tone_attr",
        "data-state=move || state.get().data_state_attr",
        "data-message-kind=move || state.get().message_kind_attr",
        "data-invalid=move || state.get().is_invalid.then_some(\"true\")",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-show-error-icon=move || state.get().show_error_icon.then_some(\"true\")",
        "data-has-description=move || state.get().has_description.then_some(\"true\")",
        "data-has-error=move || state.get().has_error_message.then_some(\"true\")",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-error-source=move || state.get().error_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            source.contains(attr),
            "HelpText should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn help_text_styles_include_state_markers() {
    let source = load_source("src/help_text/styles.rs");

    for selector in [
        ".ui-help-text--tone-auto",
        ".ui-help-text[data-tone=\"negative\"]",
        ".ui-help-text--invalid",
        ".ui-help-text[data-invalid=\"true\"]",
        ".ui-help-text--disabled",
        ".ui-help-text[data-disabled=\"true\"]",
        ".ui-help-text__icon",
        ".ui-help-text__text",
        ".ui-help-text--custom-class",
        ".ui-help-text[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "HelpText styles should include `{selector}` as stable state-marker contracts."
        );
    }
}
