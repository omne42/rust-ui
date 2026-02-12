use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn field_group_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/field_group/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "FieldGroup internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn field_group_uses_logic_state_model() {
    let mod_source = load_source("src/field_group/mod.rs");
    let logic_source = load_source("src/field_group/logic.rs");
    let view_source = load_source("src/field_group/view.rs");

    for needle in [
        "pub struct FieldGroupStateInput",
        "pub struct FieldGroupState",
    ] {
        assert!(
            mod_source.contains(needle),
            "FieldGroup module should include `{needle}` state contracts."
        );
    }

    for needle in [
        "pub fn normalize_optional_text(",
        "pub fn normalize_id_base(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "FieldGroup logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_id_base(id_base)",
        "logic::normalize_optional_text(label)",
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(FieldGroupStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "FieldGroup view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn field_group_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/field_group/view.rs");

    for attr in [
        "data-slot=\"field-group\"",
        "data-orientation=move || state.get().orientation_attr",
        "data-density=move || state.get().density_attr",
        "data-state=move || state.get().state_attr",
        "data-label=move || state.get().label_attr",
        "data-description=move || state.get().description_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-slot=\"field-group-label\"",
        "data-slot=\"field-group-content\"",
        "data-slot=\"field-group-description\"",
    ] {
        assert!(
            source.contains(attr),
            "FieldGroup should expose `{attr}` for Spectrum-style state inspection and styling."
        );
    }
}

#[test]
fn field_group_aria_contracts_are_preserved() {
    let source = load_source("src/field_group/view.rs");

    for needle in [
        "aria-label=move || aria_label_value.get()",
        "aria-labelledby=move || aria_labelledby.get()",
        "aria-describedby=move || aria_describedby.get()",
    ] {
        assert!(
            source.contains(needle),
            "FieldGroup should preserve accessibility contracts (`{needle}`)."
        );
    }
}

#[test]
fn field_group_styles_include_state_marker_contracts() {
    let source = load_source("src/field_group/styles.rs");

    for selector in [
        ".ui-field-group--density-compact",
        ".ui-field-group[data-density=\"comfortable\"]",
        ".ui-field-group--orientation-horizontal .ui-field-group__content",
        ".ui-field-group[data-orientation=\"vertical\"] .ui-field-group__content",
        ".ui-field-group--invalid",
        ".ui-field-group[data-disabled=\"true\"]",
        ".ui-field-group--custom-class",
    ] {
        assert!(
            source.contains(selector),
            "FieldGroup styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn field_group_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_groups.rs");

    for needle in [
        "pub(super) fn field_group() -> AnyView",
        "title=\"FieldGroup\"",
        "slug=\"field-group\"",
        "description=\"Spectrum/HeroUI-compatible field clustering primitive with centralized orientation/density/aria/class-state contracts and stable slot + data markers.\"",
        "<Playground title=\"Vertical + Label + Description\" code=base_code>",
        "<Playground title=\"Horizontal + Compact + Invalid + Disabled\" code=states_code>",
        "<FieldGroup",
    ] {
        assert!(
            source.contains(needle),
            "forms_groups docs page should include `{needle}` for field_group primary playground coverage.",
        );
    }
}

#[test]
fn field_group_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_groups.rs");

    for needle in [
        "title=\"Vertical + Label + Description\"",
        "id_base=\"docs-field-group-account\".to_string()",
        "label=\"Account details\".to_string()",
        "description=\"Group related fields to keep form scanning predictable.\".to_string()",
        "placeholder=\"Ada Lovelace\"",
        "placeholder=\"ada@example.com\"",
        "title=\"Horizontal + Compact + Invalid + Disabled\"",
        "id_base=\"docs-field-group-billing\".to_string()",
        "orientation=FieldGroupOrientation::Horizontal",
        "density=FieldGroupDensity::Compact",
        "invalid=true",
        "disabled=true",
        "class_name=\"docs-field-group-custom\".to_string()",
        "aria_label=\"Billing field cluster\".to_string()",
        "error_message=\"VAT ID is required\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "field_group docs playgrounds should contain `{needle}`.",
        );
    }
}
