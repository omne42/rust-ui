use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn date_picker_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/date_picker/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "DatePicker internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn date_picker_uses_logic_state_model() {
    let logic_source = load_source("src/date_picker/logic.rs");
    let view_source = load_source("src/date_picker/view.rs");

    for needle in [
        "pub enum DatePickerTone",
        "pub struct DatePickerIds",
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(",
        "pub fn normalize_placeholder(",
        "pub fn normalize_month(",
        "pub fn normalize_selected_day(",
        "pub fn resolve_ids(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "placeholder_source_attr",
        "aria_source_attr",
        "class_source_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "DatePicker logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "overlay_open::use_controllable_open_state(open, default_open, on_open_change)",
        "overlay_open::use_controllable_state(",
        "logic::normalize_placeholder(placeholder)",
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_ids(&id_base)",
        "logic::resolve_state(DatePickerStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
        "motion: DatePickerMotion",
        "motion=motion.popover",
    ] {
        assert!(
            view_source.contains(needle),
            "DatePicker view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn date_picker_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/date_picker/view.rs");

    for attr in [
        "data-slot=\"date-picker\"",
        "data-tone=move || state.get().tone_attr",
        "data-state=move || state.get().data_state_attr",
        "data-open=move || state.get().is_open.then_some(\"true\")",
        "data-closed=move || state.get().is_closed.then_some(\"true\")",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-has-value=move || state.get().has_value.then_some(\"true\")",
        "data-selected-day=move || state.get().selected_day.map(|day| day.to_string())",
        "data-placeholder-source=move || state.get().placeholder_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=if motion == DatePickerMotion::default()",
        "data-custom-motion=(motion != DatePickerMotion::default()).then_some(\"true\")",
        "data-slot=\"date-picker-trigger\"",
        "data-slot=\"date-picker-panel\"",
        "class_name=\"ui-date-picker__trigger\".to_string()",
        "class_name=\"ui-date-picker__calendar\".to_string()",
        "role=\"group\"",
    ] {
        assert!(
            source.contains(attr),
            "DatePicker should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn date_picker_styles_include_tone_open_value_and_source_markers() {
    let source = load_source("src/date_picker/styles.rs");

    for selector in [
        ".ui-date-picker--tone-default",
        ".ui-date-picker[data-tone=\"default\"]",
        ".ui-date-picker--tone-quiet",
        ".ui-date-picker--tone-strong",
        ".ui-date-picker--open",
        ".ui-date-picker[data-open=\"true\"]",
        ".ui-date-picker--disabled",
        ".ui-date-picker[data-disabled=\"true\"]",
        ".ui-date-picker--has-value",
        ".ui-date-picker[data-has-value=\"true\"]",
        ".ui-date-picker--custom-class",
        ".ui-date-picker[data-custom-class=\"true\"]",
        ".ui-date-picker[data-motion-source=\"custom\"]",
        ".ui-date-picker[data-custom-motion=\"true\"]",
        ".ui-date-picker__trigger",
        ".ui-date-picker__panel",
        ".ui-date-picker__calendar",
    ] {
        assert!(
            source.contains(selector),
            "DatePicker styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn date_picker_exposes_motion_contract_and_internal_module() {
    let mod_source = load_source("src/date_picker/mod.rs");
    let motion_source = load_source("src/date_picker/motion.rs");

    for needle in [
        "pub mod motion;",
        "pub use motion::DatePickerMotion;",
        "pub struct DatePickerMotion",
        "pub popover: PopoverMotion",
    ] {
        assert!(
            mod_source.contains(needle) || motion_source.contains(needle),
            "DatePicker motion contract should include `{needle}` for HeroUI-style spring customization."
        );
    }
}

#[test]
fn date_picker_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("src/date_picker/motion.rs");
    let view_source = load_source("src/date_picker/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: DatePickerMotion) -> DatePickerMotion",
        "popover: crate::popover::motion::sanitize_motion(motion.popover)",
        "fn sanitize_motion_delegates_to_popover_contract()",
    ] {
        assert!(
            motion_source.contains(needle),
            "DatePicker motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    assert!(
        view_source.contains("let motion = crate::date_picker::motion::sanitize_motion(motion);"),
        "DatePicker view should sanitize motion before forwarding to Popover.",
    );
}

#[test]
fn date_picker_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "pub(super) fn date_picker() -> AnyView",
        "title=\"DatePicker\"",
        "slug=\"date-picker\"",
        "description=\"Date picker trigger + popover calendar with centralized open/value/source state contracts and HeroUI-grade popover motion handoff.\"",
        "<Playground title=\"Default + Outside Days\" code=code>",
        "<Playground title=\"Monday First + Strong Tone\" code=states_code>",
        "<DatePicker",
        "tone=DatePickerTone::Strong",
        "first_weekday=CalendarFirstWeekday::Monday",
        "show_outside_days=false",
    ] {
        assert!(
            source.contains(needle),
            "forms_extra docs page should include `{needle}` for date_picker primary coverage.",
        );
    }
}

#[test]
fn date_picker_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "id_base=\"docs-date-picker-release\".to_string()",
        "year=2026",
        "month=3",
        "default_selected_day=12",
        "tone=DatePickerTone::Default",
        "first_weekday=CalendarFirstWeekday::Sunday",
        "show_outside_days=true",
        "id_base=\"docs-date-picker-ship\".to_string()",
        "month=4",
        "default_selected_day=21",
        "tone=DatePickerTone::Strong",
        "first_weekday=CalendarFirstWeekday::Monday",
        "show_outside_days=false",
        "placeholder=\"Pick ship date\".to_string()",
        "class_name=\"docs-date-picker-custom\".to_string()",
        "motion=DatePickerMotion {",
        "popover: PopoverMotion {",
        "initial_scale: 0.95",
        "offset_y_px: 10.0",
    ] {
        assert!(
            source.contains(needle),
            "date_picker docs playgrounds should contain `{needle}`.",
        );
    }
}
