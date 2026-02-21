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

#[test]
fn selected_day_axis_normalization_and_press_updates_are_stable() {
    assert!(normalize_is_show_outside_days(Some(true), Some(false)));
    assert!(normalize_is_show_outside_days(None, Some(true)));
    assert!(!normalize_is_show_outside_days(None, None));

    let controlled = normalize_selected_day_axis(Some(10), Some(1), 2026, 2);
    assert_eq!(controlled.mode, CalendarSelectedDayMode::Controlled);
    assert_eq!(controlled.selected_day, Some(10));
    assert_eq!(controlled.source, CalendarSelectedDaySource::SelectedDay);
    assert_eq!(
        resolve_effective_selected_day(controlled, Some(4)),
        controlled.selected_day
    );

    let uncontrolled_default = normalize_selected_day_axis(None, Some(12), 2026, 2);
    assert_eq!(
        uncontrolled_default.mode,
        CalendarSelectedDayMode::Uncontrolled
    );
    assert_eq!(uncontrolled_default.selected_day, Some(12));
    assert_eq!(
        uncontrolled_default.source,
        CalendarSelectedDaySource::DefaultSelectedDay
    );
    assert_eq!(
        resolve_effective_selected_day(uncontrolled_default, Some(7)),
        Some(7)
    );

    let uncontrolled_implicit = normalize_selected_day_axis(None, None, 2026, 2);
    assert_eq!(
        uncontrolled_implicit.mode,
        CalendarSelectedDayMode::Uncontrolled
    );
    assert_eq!(uncontrolled_implicit.selected_day, None);
    assert_eq!(
        uncontrolled_implicit.source,
        CalendarSelectedDaySource::ImplicitDefault
    );

    let controlled_invalid = normalize_selected_day_axis(Some(99), Some(12), 2026, 2);
    assert_eq!(controlled_invalid.mode, CalendarSelectedDayMode::Controlled);
    assert_eq!(controlled_invalid.selected_day, None);
    assert_eq!(
        controlled_invalid.source,
        CalendarSelectedDaySource::SelectedDay
    );

    let uncontrolled_invalid_default = normalize_selected_day_axis(None, Some(99), 2026, 2);
    assert_eq!(
        uncontrolled_invalid_default.mode,
        CalendarSelectedDayMode::Uncontrolled
    );
    assert_eq!(uncontrolled_invalid_default.selected_day, None);
    assert_eq!(
        uncontrolled_invalid_default.source,
        CalendarSelectedDaySource::DefaultSelectedDay
    );

    let controlled_press =
        resolve_selected_day_press_update(CalendarSelectedDayMode::Controlled, 8);
    assert_eq!(controlled_press.next_uncontrolled_selected_day, None);
    assert_eq!(
        controlled_press.next_source,
        CalendarSelectedDaySource::SelectedDay
    );

    let uncontrolled_press =
        resolve_selected_day_press_update(CalendarSelectedDayMode::Uncontrolled, 8);
    assert_eq!(
        uncontrolled_press.next_uncontrolled_selected_day,
        Some(Some(8))
    );
    assert_eq!(
        uncontrolled_press.next_source,
        CalendarSelectedDaySource::Interaction
    );
}
