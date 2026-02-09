use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn field_label_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/field_label/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "FieldLabel internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn field_label_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/field_label/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::FieldLabel;"),
        "field_label module should export `FieldLabel`."
    );
    assert!(
        crate_source.contains("pub use field_label::{FieldLabel, FieldLabelTone};"),
        "crate root should re-export FieldLabel contract."
    );
}

#[test]
fn field_label_uses_logic_state_model() {
    let logic_source = load_source("src/field_label/logic.rs");
    let view_source = load_source("src/field_label/view.rs");

    for needle in [
        "pub enum FieldLabelTone",
        "pub fn normalize_optional_text(",
        "pub fn normalize_text(",
        "pub fn normalize_required_indicator(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "text_source_attr",
        "indicator_source_attr",
        "aria_source_attr",
        "class_source_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "FieldLabel logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_text(text)",
        "logic::normalize_required_indicator(required_indicator)",
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(FieldLabelStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "FieldLabel view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn field_label_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/field_label/view.rs");

    for attr in [
        "data-slot=\"field-label\"",
        "data-tone=move || state.get().tone_attr",
        "data-state=move || if state.get().is_required { \"required\" } else { \"optional\" }",
        "data-required=move || state.get().is_required.then_some(\"true\")",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-has-for=move || state.get().has_for_id.then_some(\"true\")",
        "data-text-source=move || state.get().text_source_attr",
        "data-indicator-source=move || state.get().indicator_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
        "data-slot=\"field-label-text\"",
        "data-slot=\"field-label-required\"",
    ] {
        assert!(
            source.contains(attr),
            "FieldLabel should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn field_label_styles_include_tone_state_and_markers() {
    let source = load_source("src/field_label/styles.rs");

    for selector in [
        ".ui-field-label {",
        ".ui-field-label--tone-default",
        ".ui-field-label[data-tone=\"default\"]",
        ".ui-field-label--tone-muted",
        ".ui-field-label[data-tone=\"muted\"]",
        ".ui-field-label--tone-strong",
        ".ui-field-label[data-tone=\"strong\"]",
        ".ui-field-label--required",
        ".ui-field-label[data-required=\"true\"]",
        ".ui-field-label--disabled",
        ".ui-field-label[data-disabled=\"true\"]",
        ".ui-field-label--for",
        ".ui-field-label[data-has-for=\"true\"]",
        ".ui-field-label--text-custom",
        ".ui-field-label[data-text-source=\"custom\"]",
        ".ui-field-label--indicator-custom",
        ".ui-field-label[data-indicator-source=\"custom\"]",
        ".ui-field-label--aria-custom",
        ".ui-field-label[data-aria-source=\"custom\"]",
        ".ui-field-label--custom-class",
        ".ui-field-label[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "FieldLabel styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn field_label_docs_page_exists_in_forms_extra() {
    let forms_extra =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_extra_field_label.rs");

    for needle in [
        "pub(super) fn field_label() -> AnyView",
        "title=\"FieldLabel\"",
        "slug=\"field-label\"",
        "<FieldLabel",
    ] {
        assert!(
            forms_extra.contains(needle),
            "forms_extra docs page should contain `{needle}`."
        );
    }
}
