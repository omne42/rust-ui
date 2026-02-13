use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn color_field_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/color_field/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ColorField internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn color_field_uses_logic_state_model() {
    let logic_source = load_source("src/color_field/logic.rs");
    let view_source = load_source("src/color_field/view.rs");

    for needle in [
        "pub const DEFAULT_LABEL",
        "pub const DEFAULT_PLACEHOLDER",
        "pub const DEFAULT_ARIA_LABEL",
        "pub fn normalize_label(",
        "pub fn normalize_placeholder(",
        "pub fn normalize_aria_label(",
        "pub fn sanitize_preview_color(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "ColorField logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "overlay_open::use_controllable_state(",
        "logic::sanitize_preview_color(value.get())",
        "logic::resolve_state(ColorFieldStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "ColorField view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn color_field_exposes_spectrum_style_data_markers() {
    let source = load_source("src/color_field/view.rs");

    for attr in [
        "data-slot=\"color-field\"",
        "data-state=move || state.get().data_state_attr",
        "data-valid=move || state.get().has_valid_value.then_some(\"true\")",
        "data-invalid=move ||",
        "data-has-preview=move || state.get().has_preview.then_some(\"true\")",
        "data-label-source=move || state.get().label_source_attr",
        "data-placeholder-source=move || state.get().placeholder_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-slot=\"color-field-preview\"",
        "data-slot=\"color-field-input\"",
        "data-slot=\"color-field-clear\"",
    ] {
        assert!(
            source.contains(attr),
            "ColorField should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn color_field_styles_include_valid_invalid_disabled_and_custom_contracts() {
    let source = load_source("src/color_field/styles.rs");

    for selector in [
        ".ui-color-field",
        ".ui-color-field__control",
        ".ui-color-field__preview",
        ".ui-color-field__input",
        ".ui-color-field[data-state=\"valid\"] .ui-color-field__input",
        ".ui-color-field[data-state=\"invalid\"] .ui-color-field__input",
        ".ui-color-field--disabled",
        ".ui-color-field[data-disabled=\"true\"]",
        ".ui-color-field--custom-class",
        ".ui-color-field[data-custom-class=\"true\"]",
        ".ui-color-field[data-class-source=\"custom\"]",
    ] {
        assert!(
            source.contains(selector),
            "ColorField styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn color_field_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    for needle in [
        "pub(super) fn color_field() -> AnyView",
        "title=\"ColorField\"",
        "slug=\"color-field\"",
        "title=\"Controlled Value\"",
        "title=\"Invalid + Disabled + Custom Class\"",
    ] {
        assert!(
            source.contains(needle),
            "color-field docs page should contain `{needle}`.",
        );
    }
}

#[test]
fn color_field_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    for needle in [
        "<Playground title=\"Controlled Value\" code_signal=basic_code>",
        "id_base=\"docs-color-field-basic\".to_string()",
        "label=\"Fill color\".to_string()",
        "value=value.into()",
        "on_value_change=on_value_change",
        "<Playground title=\"Invalid + Disabled + Custom Class\" code_signal=states_code>",
        "id_base=\"docs-color-field-invalid\".to_string()",
        "default_value=\"javascript:alert(1)\".to_string()",
        "class_name=\"docs-color-field-custom\".to_string()",
        "id_base=\"docs-color-field-disabled\".to_string()",
        "default_value=\"#0ea5e9\".to_string()",
        "disabled=true",
    ] {
        assert!(
            source.contains(needle),
            "color-field docs playground should contain `{needle}`.",
        );
    }
}
