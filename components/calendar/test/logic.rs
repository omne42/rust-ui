use super::*;

#[test]
fn state_primitives_are_reexported_from_ui_state_primitives() {
    assert_eq!(normalize_month(0), 1);
    assert_eq!(
        normalize_aria_label(Some("  Calendar picker  ".to_string())),
        ("Calendar picker".to_string(), true)
    );
    assert_eq!(
        normalize_aria_label(None),
        (
            ui_state_primitives::calendar::DEFAULT_ARIA_LABEL.into(),
            false
        )
    );
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

#[test]
fn resolve_agent_contract_uses_selection_state_as_machine_contract() {
    let selected = resolve_agent_contract(resolve_state(CalendarStateInput {
        year: 2026,
        month: 1,
        tone: CalendarTone::Default,
        first_weekday: CalendarFirstWeekday::Sunday,
        show_outside_days: true,
        selected_day: Some(6),
        has_custom_aria_label: false,
        has_custom_class_name: false,
    }));

    assert_eq!(selected.schema_attr, "ui.calendar");
    assert_eq!(selected.intent_attr, "date-selection");
    assert_eq!(selected.action.as_attr(), "select-day");
    assert_eq!(selected.state.as_attr(), "selected");
    assert_eq!(selected.source.as_attr(), "props-selected-day");
    assert_eq!(selected.stream_support.as_attr(), "unsupported");
    assert_eq!(selected.stream_fallback.as_attr(), "snapshot");
    assert_eq!(selected.output_status.as_attr(), "verified");
}
