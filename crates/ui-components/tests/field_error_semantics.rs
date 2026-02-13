use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn field_error_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/field_error/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "FieldError internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn field_error_uses_logic_state_model() {
    let logic_source = load_source("src/field_error/logic.rs");
    let view_source = load_source("src/field_error/view.rs");

    for needle in [
        "pub enum FieldErrorTone",
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(",
        "pub fn normalize_message(",
        "pub fn resolve_effective_tone(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "aria_source_attr",
        "message_source_attr",
        "class_source_attr",
        "data_state_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "FieldError logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_aria_label(aria_label)",
        "logic::normalize_message(message, visible)",
        "logic::resolve_state(FieldErrorStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "FieldError view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn field_error_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/field_error/view.rs");

    for attr in [
        "data-slot=\"field-error\"",
        "data-tone=move || state.get().tone_attr",
        "data-state=move || state.get().data_state_attr",
        "data-visible=move || state.get().is_visible.then_some(\"true\")",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-show-icon=move || state.get().show_icon.then_some(\"true\")",
        "data-has-message=move || state.get().has_message.then_some(\"true\")",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-message-source=move || state.get().message_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
        "data-slot=\"field-error-text\"",
    ] {
        assert!(
            source.contains(attr),
            "FieldError should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn field_error_styles_include_tone_state_and_markers() {
    let source = load_source("src/field_error/styles.rs");

    for selector in [
        ".ui-field-error--tone-auto",
        ".ui-field-error[data-tone=\"auto\"]",
        ".ui-field-error--tone-neutral",
        ".ui-field-error[data-tone=\"neutral\"]",
        ".ui-field-error--tone-negative",
        ".ui-field-error[data-tone=\"negative\"]",
        ".ui-field-error[data-state=\"hidden\"]",
        ".ui-field-error--disabled",
        ".ui-field-error[data-disabled=\"true\"]",
        ".ui-field-error--custom-class",
        ".ui-field-error[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "FieldError styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn field_error_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "pub(super) fn field_error() -> AnyView",
        "title=\"FieldError\"",
        "slug=\"field-error\"",
        "description=\"Spectrum/HeroUI-style field error primitive with centralized visibility/tone/message normalization and stable data contracts.\"",
        "<Playground title=\"Visible + Tone\" code_signal=default_code>",
        "<Playground title=\"Hidden + Disabled + Custom Class\" code_signal=hidden_code>",
        "<FieldError",
    ] {
        assert!(
            source.contains(needle),
            "forms_extra docs page should include `{needle}` for field_error primary playground coverage.",
        );
    }
}

#[test]
fn field_error_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "title=\"Visible + Tone\"",
        "visible=true",
        "message=\"Email is required\".to_string()",
        "aria_label=\"Email error\".to_string()",
        "tone=FieldErrorTone::Neutral",
        "message=\"Password should include at least one symbol\".to_string()",
        "tone=FieldErrorTone::Negative",
        "show_icon=true",
        "message=\"Two-factor code is invalid\".to_string()",
        "title=\"Hidden + Disabled + Custom Class\"",
        "visible=false",
        "message=\"This text should not render when hidden\".to_string()",
        "disabled=true",
        "class_name=\"docs-field-error-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "field_error docs playgrounds should contain `{needle}`.",
        );
    }
}
