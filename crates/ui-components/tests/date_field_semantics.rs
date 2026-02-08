use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn date_field_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/date_field/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "DateField internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn date_field_uses_logic_state_model() {
    let logic_source = load_source("src/date_field/logic.rs");
    let view_source = load_source("src/date_field/view.rs");

    for needle in [
        "pub enum DateFieldTone",
        "pub struct DateFieldIds",
        "pub fn normalize_optional_text(",
        "pub fn normalize_label(",
        "pub fn normalize_placeholder(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_ids(",
        "pub fn normalize_year(",
        "pub fn normalize_month(",
        "pub fn days_in_month(",
        "pub fn normalize_day(",
        "pub fn parse_date_value(",
        "pub fn normalize_date_value(",
        "pub fn resolve_date_parts(",
        "pub fn update_year_from_input(",
        "pub fn update_month_from_input(",
        "pub fn update_day_from_input(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "label_source_attr",
        "placeholder_source_attr",
        "aria_source_attr",
        "class_source_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "DateField logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "overlay_open::use_controllable_state(value, Some(default_value), on_value_change)",
        "logic::normalize_label(label)",
        "logic::normalize_placeholder(placeholder)",
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_ids(&id_base)",
        "logic::resolve_state(DateFieldStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "DateField view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn date_field_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/date_field/view.rs");

    for attr in [
        "data-slot=\"date-field\"",
        "data-tone=move || state.get().tone_attr",
        "data-state=move || state.get().data_state_attr",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-has-value=move || state.get().has_value.then_some(\"true\")",
        "data-label-source=move || state.get().label_source_attr",
        "data-placeholder-source=move || state.get().placeholder_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
        "data-slot=\"date-field-label\"",
        "data-slot=\"date-field-control\"",
        "data-slot=\"date-field-year\"",
        "data-slot=\"date-field-month\"",
        "data-slot=\"date-field-day\"",
        "data-slot=\"date-field-separator\"",
        "data-slot=\"date-field-clear\"",
        "role=\"group\"",
    ] {
        assert!(
            source.contains(attr),
            "DateField should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn date_field_styles_include_tone_value_and_source_markers() {
    let source = load_source("src/date_field/styles.rs");

    for selector in [
        ".ui-date-field--tone-default",
        ".ui-date-field[data-tone=\"default\"]",
        ".ui-date-field--tone-quiet",
        ".ui-date-field--tone-strong",
        ".ui-date-field--disabled",
        ".ui-date-field[data-disabled=\"true\"]",
        ".ui-date-field--has-value",
        ".ui-date-field[data-has-value=\"true\"] .ui-date-field__control",
        ".ui-date-field--custom-class",
        ".ui-date-field[data-custom-class=\"true\"]",
        ".ui-date-field__control",
        ".ui-date-field__input",
        ".ui-date-field__clear",
    ] {
        assert!(
            source.contains(selector),
            "DateField styles should include `{selector}` as stable state-marker contracts."
        );
    }
}
