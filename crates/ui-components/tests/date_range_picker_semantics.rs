use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn date_range_picker_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/date_range_picker/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "DateRangePicker internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn date_range_picker_uses_logic_state_model() {
    let logic_source = load_source("src/date_range_picker/logic.rs");
    let view_source = load_source("src/date_range_picker/view.rs");

    for needle in [
        "pub enum DateRangePickerTone",
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(",
        "pub fn normalize_month(",
        "pub fn days_in_month(",
        "pub fn normalize_day(",
        "pub fn is_range_invalid(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "aria_source_attr",
        "class_source_attr",
        "data_state_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "DateRangePicker logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "overlay_open::use_controllable_state(",
        "logic::normalize_aria_label(aria_label)",
        "logic::is_range_invalid(",
        "logic::resolve_state(DateRangePickerStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
        "<DatePicker",
    ] {
        assert!(
            view_source.contains(needle),
            "DateRangePicker view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn date_range_picker_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/date_range_picker/view.rs");

    for attr in [
        "data-slot=\"date-range-picker\"",
        "data-tone=move || state.get().tone_attr",
        "data-state=move || state.get().data_state_attr",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-has-start-value=move || state.get().has_start_value.then_some(\"true\")",
        "data-has-end-value=move || state.get().has_end_value.then_some(\"true\")",
        "data-has-full-value=move || state.get().has_full_value.then_some(\"true\")",
        "data-partial=move || state.get().is_partial.then_some(\"true\")",
        "data-invalid-range=move || state.get().is_invalid_range.then_some(\"true\")",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
        "data-slot=\"date-range-picker-fields\"",
        "data-slot=\"date-range-picker-start\"",
        "data-slot=\"date-range-picker-end\"",
        "data-slot=\"date-range-picker-start-label\"",
        "data-slot=\"date-range-picker-end-label\"",
        "data-slot=\"date-range-picker-hint\"",
        "role=\"group\"",
    ] {
        assert!(
            source.contains(attr),
            "DateRangePicker should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn date_range_picker_styles_include_tone_partial_invalid_and_source_markers() {
    let source = load_source("src/date_range_picker/styles.rs");

    for selector in [
        ".ui-date-range-picker--tone-default",
        ".ui-date-range-picker[data-tone=\"default\"]",
        ".ui-date-range-picker--tone-quiet",
        ".ui-date-range-picker--tone-strong",
        ".ui-date-range-picker--disabled",
        ".ui-date-range-picker[data-disabled=\"true\"]",
        ".ui-date-range-picker--partial",
        ".ui-date-range-picker[data-state=\"partial\"]",
        ".ui-date-range-picker--has-full-value",
        ".ui-date-range-picker[data-has-full-value=\"true\"]",
        ".ui-date-range-picker--invalid-range",
        ".ui-date-range-picker[data-invalid-range=\"true\"]",
        ".ui-date-range-picker--custom-class",
        ".ui-date-range-picker[data-custom-class=\"true\"]",
        ".ui-date-range-picker__fields",
        ".ui-date-range-picker__picker",
        ".ui-date-range-picker__hint",
    ] {
        assert!(
            source.contains(selector),
            "DateRangePicker styles should include `{selector}` as stable state-marker contracts."
        );
    }
}
