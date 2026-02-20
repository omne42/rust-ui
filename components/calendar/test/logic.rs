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
            ui_logic_calendar::calendar::DEFAULT_ARIA_LABEL.into(),
            false
        )
    );
}

#[test]
fn compose_class_name_includes_state_markers() {
    let class_name = compose_class_name(
        Some("docs-calendar".to_string()),
        resolve_state(CalendarStateInput {
            year: 2026,
            month: 1,
            tone: CalendarTone::Quiet,
            first_weekday: CalendarFirstWeekday::Sunday,
            show_outside_days: true,
            selected_day: None,
            has_custom_aria_label: false,
            has_custom_class_name: true,
        }),
    );

    for token in [
        "ui-calendar",
        "ui-calendar--tone-quiet",
        "ui-calendar--weekday-sunday",
        "ui-calendar--outside-days",
        "ui-calendar--custom-class",
        "docs-calendar",
    ] {
        assert!(class_name.contains(token), "class should include `{token}`");
    }
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
