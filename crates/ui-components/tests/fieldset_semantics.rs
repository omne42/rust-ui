use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn fieldset_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/fieldset/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Fieldset internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn fieldset_uses_logic_state_model() {
    let logic_source = load_source("src/fieldset/logic.rs");
    let view_source = load_source("src/fieldset/view.rs");

    for needle in [
        "pub enum FieldsetOrientation",
        "pub enum FieldsetTone",
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(",
        "pub fn normalize_error_message(",
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
            "Fieldset logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_aria_label(aria_label)",
        "logic::normalize_error_message(error_message, invalid)",
        "logic::resolve_state(FieldsetStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "Fieldset view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn fieldset_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/fieldset/view.rs");

    for attr in [
        "data-slot=\"fieldset\"",
        "data-orientation=move || state.get().orientation_attr",
        "data-tone=move || state.get().tone_attr",
        "data-state=move || state.get().data_state_attr",
        "data-message-kind=move || state.get().message_kind_attr",
        "data-required=move || state.get().is_required.then_some(\"true\")",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-invalid=move || state.get().is_invalid.then_some(\"true\")",
        "data-has-legend=move || state.get().has_legend.then_some(\"true\")",
        "data-has-description=move || state.get().has_description.then_some(\"true\")",
        "data-has-error=move || state.get().has_error_message.then_some(\"true\")",
        "data-has-actions=move || state.get().has_actions.then_some(\"true\")",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-error-source=move || state.get().error_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
        "data-slot=\"fieldset-field-group\"",
        "data-slot=\"fieldset-actions\"",
    ] {
        assert!(
            source.contains(attr),
            "Fieldset should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn fieldset_styles_include_state_markers() {
    let source = load_source("src/fieldset/styles.rs");

    for selector in [
        ".ui-fieldset--orientation-vertical",
        ".ui-fieldset[data-orientation=\"horizontal\"]",
        ".ui-fieldset--tone-default",
        ".ui-fieldset[data-tone=\"muted\"]",
        ".ui-fieldset--required .ui-fieldset__legend",
        ".ui-fieldset[data-required=\"true\"] .ui-fieldset__legend",
        ".ui-fieldset--disabled",
        ".ui-fieldset[data-disabled=\"true\"]",
        ".ui-fieldset--invalid .ui-fieldset__group",
        ".ui-fieldset[data-invalid=\"true\"] .ui-fieldset__group",
        ".ui-fieldset--custom-class",
        ".ui-fieldset[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Fieldset styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn fieldset_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "pub(super) fn fieldset() -> AnyView",
        "title=\"Fieldset\"",
        "slug=\"fieldset\"",
        "description=\"Spectrum/HeroUI-style fieldset primitive with centralized orientation/tone/validation/message/action-state modeling and stable data contracts.\"",
        "<Playground title=\"Legend + Description\" code_signal=default_code>",
        "<Playground title=\"Horizontal + Invalid + Actions\" code_signal=invalid_code>",
        "<Fieldset",
        "orientation=FieldsetOrientation::Horizontal",
        "tone=FieldsetTone::Muted",
        "invalid=true",
    ] {
        assert!(
            source.contains(needle),
            "forms_extra docs page should include `{needle}` for fieldset primary coverage.",
        );
    }
}

#[test]
fn fieldset_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "legend=\"Notification channels\".to_string()",
        "description=\"Pick every channel you want to receive release updates from.\".to_string()",
        "required=true",
        "aria_label=\"Notification channel group\".to_string()",
        "<span>\"Email\"</span>",
        "<span>\"SMS\"</span>",
        "<span>\"Push\"</span>",
        "orientation=FieldsetOrientation::Horizontal",
        "tone=FieldsetTone::Muted",
        "invalid=true",
        "error_message=\"Pick at least one channel\".to_string()",
        "class_name=\"docs-fieldset-custom\".to_string()",
        "variant=ui_components::ButtonVariant::Secondary",
        "size=ui_components::ButtonSize::S",
        "\"Manage channels\"",
    ] {
        assert!(
            source.contains(needle),
            "fieldset docs playgrounds should contain `{needle}`.",
        );
    }
}
