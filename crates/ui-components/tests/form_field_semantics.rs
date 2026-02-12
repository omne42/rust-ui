use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn form_field_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/form_field/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "FormField internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn form_field_uses_logic_state_model() {
    let mod_source = load_source("src/form_field/mod.rs");
    let logic_source = load_source("src/form_field/logic.rs");
    let view_source = load_source("src/form_field/view.rs");

    for needle in [
        "pub struct FormFieldStateInput",
        "pub struct FormFieldState",
    ] {
        assert!(
            mod_source.contains(needle),
            "FormField module should include `{needle}` state contracts."
        );
    }

    for needle in [
        "pub fn normalize_optional_text(",
        "pub fn normalize_id_base(",
        "pub fn normalize_label(",
        "pub fn normalize_aria_label(",
        "pub fn normalize_error_message(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "FormField logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_id_base(id_base)",
        "logic::normalize_label(label)",
        "logic::normalize_optional_text(description)",
        "logic::normalize_error_message(error_message, invalid)",
        "logic::resolve_state(FormFieldStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "FormField view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn form_field_composes_switch_and_checkbox_indicators() {
    let source = load_source("src/form_field/view.rs");

    for needle in [
        "FormFieldIndicatorVariant::Switch",
        "FormFieldIndicatorVariant::Checkbox",
        "<Switch",
        "<Checkbox",
        "on_change=on_selected_change",
        "checked=selected",
        "set_checked=set_selected",
    ] {
        assert!(
            source.contains(needle),
            "FormField should compose indicator controls with stable contracts (`{needle}`)."
        );
    }
}

#[test]
fn form_field_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/form_field/view.rs");

    for attr in [
        "data-slot=\"form-field\"",
        "data-state=move || state.get().state_attr",
        "data-tone=move || state.get().tone_attr",
        "data-indicator-variant=move || state.get().indicator_variant_attr",
        "data-indicator-placement=move || state.get().indicator_placement_attr",
        "data-message-kind=move || state.get().message_kind_attr",
        "data-label-source=move || state.get().label_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-error-source=move || state.get().error_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-slot=\"form-field-content\"",
        "data-slot=\"form-field-indicator\"",
        "data-slot=\"form-field-label\"",
        "data-slot=\"form-field-description\"",
        "data-slot=\"form-field-error\"",
    ] {
        assert!(
            source.contains(attr),
            "FormField should expose `{attr}` for Spectrum-style state inspection and styling."
        );
    }
}

#[test]
fn form_field_styles_include_state_marker_contracts() {
    let source = load_source("src/form_field/styles.rs");

    for selector in [
        ".ui-form-field--placement-end",
        ".ui-form-field[data-indicator-placement=\"start\"]",
        ".ui-form-field--tone-quiet",
        ".ui-form-field[data-tone=\"default\"]",
        ".ui-form-field--invalid .ui-form-field__label",
        ".ui-form-field[data-disabled=\"true\"]",
        ".ui-form-field__control.ui-switch .ui-switch__label",
        ".ui-form-field--custom-class",
    ] {
        assert!(
            source.contains(selector),
            "FormField styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn form_field_docs_page_covers_primary_playgrounds() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_groups_extra.rs");

    for needle in [
        "pub(super) fn form_field() -> AnyView",
        "title=\"FormField\"",
        "slug=\"form-field\"",
        "description=\"Spectrum/HeroUI-style form field primitive that composes switch/checkbox indicators with centralized tone/placement/message state derivation and stable slot/data-state markers.\"",
        "<Playground title=\"Switch Indicator + Description\" code=code>",
        "<Playground title=\"Checkbox Indicator + Quiet + Invalid/Disabled\" code=states_code>",
        "<FormField",
    ] {
        assert!(
            source.contains(needle),
            "forms_groups_extra form_field docs page should include `{needle}` for primary playground coverage.",
        );
    }
}

#[test]
fn form_field_docs_playgrounds_lock_state_matrix_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_groups_extra.rs");

    for needle in [
        "title=\"Switch Indicator + Description\"",
        "id_base=\"docs-form-field-marketing\".to_string()",
        "label=\"Subscribe to product updates\".to_string()",
        "description=\"Receive release notes and occasional best-practice tips.\".to_string()",
        "indicator_placement=FormFieldIndicatorPlacement::Start",
        "title=\"Checkbox Indicator + Quiet + Invalid/Disabled\"",
        "id_base=\"docs-form-field-tos\".to_string()",
        "indicator_variant=FormFieldIndicatorVariant::Checkbox",
        "indicator_placement=FormFieldIndicatorPlacement::End",
        "tone=FormFieldTone::Quiet",
        "invalid=true",
        "error_message=\"Please accept terms to continue.\".to_string()",
        "class_name=\"docs-form-field-custom\".to_string()",
        "id_base=\"docs-form-field-read-only\".to_string()",
        "disabled=true",
        "aria_label=\"Maintenance alerts (read only)\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "form_field docs playgrounds should contain `{needle}`.",
        );
    }
}
