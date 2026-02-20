use super::*;

#[test]
fn tone_class_names_and_attrs_are_stable() {
    assert_eq!(
        CalendarTone::Default.class_name(),
        "ui-calendar--tone-default"
    );
    assert_eq!(CalendarTone::Quiet.class_name(), "ui-calendar--tone-quiet");
    assert_eq!(
        CalendarTone::Strong.class_name(),
        "ui-calendar--tone-strong"
    );

    assert_eq!(CalendarTone::Default.as_attr(), "default");
    assert_eq!(CalendarTone::Quiet.as_attr(), "quiet");
    assert_eq!(CalendarTone::Strong.as_attr(), "strong");
}

#[test]
fn date_helpers_cover_leap_year_and_month_bounds() {
    assert!(is_leap_year(2024));
    assert!(!is_leap_year(2023));
    assert_eq!(days_in_month(2024, 2), 29);
    assert_eq!(days_in_month(2023, 2), 28);
    assert_eq!(normalize_month(0), 1);
    assert_eq!(normalize_month(15), 12);
}

#[test]
fn weekday_index_matches_known_calendar_day() {
    assert_eq!(weekday_index(2026, 1, 1, CalendarFirstWeekday::Sunday), 4);
    assert_eq!(weekday_index(2026, 1, 1, CalendarFirstWeekday::Monday), 3);
}

#[test]
fn month_grid_has_fixed_cell_count_and_selection_marker() {
    let grid = build_month_grid(2026, 1, CalendarFirstWeekday::Sunday, true, Some(6));
    assert_eq!(grid.len(), 42);
    assert!(
        grid.iter()
            .any(|cell| cell.is_selected && cell.day == Some(6))
    );
    assert!(
        grid.iter()
            .any(|cell| !cell.in_current_month && cell.day.is_some())
    );
}

#[test]
fn resolve_state_tracks_source_and_selection_state() {
    let state = resolve_state(CalendarStateInput {
        year: 2026,
        month: 1,
        tone: CalendarTone::Strong,
        first_weekday: CalendarFirstWeekday::Monday,
        show_outside_days: true,
        selected_day: Some(4),
        has_custom_aria_label: true,
        has_custom_class_name: false,
    });

    assert_eq!(state.tone_attr, "strong");
    assert_eq!(state.first_weekday_attr, "monday");
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "default");
    assert_eq!(state.data_state_attr, "selected");
    assert_eq!(state.selected_day, Some(4));
}
