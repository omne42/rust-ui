use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn calendar_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/calendar/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Calendar internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn calendar_uses_logic_state_model() {
    let logic_source = load_source("src/calendar/logic.rs");
    let view_source = load_source("src/calendar/view.rs");

    for needle in [
        "pub enum CalendarTone",
        "pub enum CalendarFirstWeekday",
        "pub struct CalendarGridCell",
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(",
        "pub fn normalize_month(",
        "pub fn normalize_selected_day(",
        "pub fn weekday_index(",
        "pub fn build_month_grid(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "aria_source_attr",
        "class_source_attr",
        "data_state_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "Calendar logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_month(month)",
        "logic::normalize_selected_day(selected_day, year, normalized_month)",
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(CalendarStateInput {",
        "logic::compose_class_name(class_name, state)",
        "logic::build_month_grid(",
    ] {
        assert!(
            view_source.contains(needle),
            "Calendar view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn calendar_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/calendar/view.rs");

    for attr in [
        "data-slot=\"calendar\"",
        "data-tone=state.tone_attr",
        "data-first-weekday=state.first_weekday_attr",
        "data-state=state.data_state_attr",
        "data-show-outside-days=state.show_outside_days.then_some(\"true\")",
        "data-selected-day=state.selected_day.map(|day| day.to_string())",
        "data-year=state.year.to_string()",
        "data-month=state.month.to_string()",
        "data-aria-source=state.aria_source_attr",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-class-source=state.class_source_attr",
        "data-slot=\"calendar-header\"",
        "data-slot=\"calendar-title\"",
        "data-slot=\"calendar-weekdays\"",
        "data-slot=\"calendar-weekday\"",
        "data-slot=\"calendar-grid\"",
        "data-slot=\"calendar-day\"",
        "data-slot=\"calendar-day-empty\"",
        "role=\"group\"",
    ] {
        assert!(
            source.contains(attr),
            "Calendar should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn calendar_styles_include_tone_weekday_and_selection_markers() {
    let source = load_source("src/calendar/styles.rs");

    for selector in [
        ".ui-calendar--tone-default",
        ".ui-calendar[data-tone=\"default\"]",
        ".ui-calendar--tone-quiet",
        ".ui-calendar--tone-strong",
        ".ui-calendar--weekday-sunday",
        ".ui-calendar[data-first-weekday=\"sunday\"]",
        ".ui-calendar--weekday-monday",
        ".ui-calendar[data-first-weekday=\"monday\"]",
        ".ui-calendar--outside-days",
        ".ui-calendar[data-show-outside-days=\"true\"]",
        ".ui-calendar--has-selection",
        ".ui-calendar[data-state=\"selected\"]",
        ".ui-calendar--custom-class",
        ".ui-calendar[data-custom-class=\"true\"]",
        ".ui-calendar__day--selected",
        ".ui-calendar__day[data-selected=\"true\"]",
        ".ui-calendar__day--outside",
        ".ui-calendar__day[data-month-source=\"outside\"]",
    ] {
        assert!(
            source.contains(selector),
            "Calendar styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn calendar_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "pub(super) fn calendar() -> AnyView",
        "title=\"Calendar\"",
        "slug=\"calendar\"",
        "title=\"Default + Outside Days\"",
        "title=\"Monday First + Strong Tone\"",
    ] {
        assert!(
            source.contains(needle),
            "calendar docs page should contain `{needle}`.",
        );
    }
}

#[test]
fn calendar_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "<Playground title=\"Default + Outside Days\" code=code>",
        "year=2026",
        "month=1",
        "selected_day=Some(6)",
        "tone=CalendarTone::Default",
        "first_weekday=CalendarFirstWeekday::Sunday",
        "show_outside_days=true",
        "<Playground title=\"Monday First + Strong Tone\" code=states_code>",
        "month=2",
        "selected_day=Some(14)",
        "tone=CalendarTone::Strong",
        "first_weekday=CalendarFirstWeekday::Monday",
        "show_outside_days=false",
        "class_name=\"docs-calendar-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "calendar docs playground should contain `{needle}`.",
        );
    }
}
