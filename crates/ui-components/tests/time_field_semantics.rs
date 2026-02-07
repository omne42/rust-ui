use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn time_field_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/time_field/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "TimeField internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn time_field_uses_logic_state_model() {
    let logic_source = load_source("src/time_field/logic.rs");
    let view_source = load_source("src/time_field/view.rs");

    for needle in [
        "pub enum TimeFieldTone",
        "pub struct TimeFieldIds",
        "pub fn normalize_optional_text(",
        "pub fn normalize_label(",
        "pub fn normalize_placeholder(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_ids(",
        "pub fn normalize_minute_step(",
        "pub fn parse_time_value(",
        "pub fn normalize_time_value(",
        "pub fn resolve_time_parts(",
        "pub fn update_hour_from_input(",
        "pub fn update_minute_from_input(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "label_source_attr",
        "placeholder_source_attr",
        "aria_source_attr",
        "class_source_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "TimeField logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "overlay_open::use_controllable_state(value, Some(default_value), on_value_change)",
        "logic::normalize_label(label)",
        "logic::normalize_placeholder(placeholder)",
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_ids(&id_base)",
        "logic::resolve_state(TimeFieldStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "TimeField view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn time_field_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/time_field/view.rs");

    for attr in [
        "data-slot=\"time-field\"",
        "data-tone=move || state.get().tone_attr",
        "data-state=move || state.get().data_state_attr",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-has-value=move || state.get().has_value.then_some(\"true\")",
        "data-minute-step=move || state.get().minute_step.to_string()",
        "data-label-source=move || state.get().label_source_attr",
        "data-placeholder-source=move || state.get().placeholder_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
        "data-slot=\"time-field-label\"",
        "data-slot=\"time-field-control\"",
        "data-slot=\"time-field-hour\"",
        "data-slot=\"time-field-separator\"",
        "data-slot=\"time-field-minute\"",
        "data-slot=\"time-field-clear\"",
        "role=\"group\"",
    ] {
        assert!(
            source.contains(attr),
            "TimeField should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn time_field_styles_include_tone_value_and_source_markers() {
    let source = load_source("src/time_field/styles.rs");

    for selector in [
        ".ui-time-field--tone-default",
        ".ui-time-field[data-tone=\"default\"]",
        ".ui-time-field--tone-quiet",
        ".ui-time-field--tone-strong",
        ".ui-time-field--disabled",
        ".ui-time-field[data-disabled=\"true\"]",
        ".ui-time-field--has-value",
        ".ui-time-field[data-has-value=\"true\"] .ui-time-field__control",
        ".ui-time-field--custom-class",
        ".ui-time-field[data-custom-class=\"true\"]",
        ".ui-time-field__control",
        ".ui-time-field__input",
        ".ui-time-field__clear",
    ] {
        assert!(
            source.contains(selector),
            "TimeField styles should include `{selector}` as stable state-marker contracts."
        );
    }
}
