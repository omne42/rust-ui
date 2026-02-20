use super::*;

#[test]
fn state_primitives_are_reexported_from_ui_state_primitives() {
    assert_eq!(normalize_month(0), 1);
    assert_eq!(normalize_month(17), 12);
    assert_eq!(ui_logic_calendar::date_picker::days_in_month(2024, 2), 29);
    assert_eq!(ui_logic_calendar::date_picker::days_in_month(2023, 2), 28);
    assert_eq!(normalize_selected_day(Some(31), 2026, 4), None);
    assert_eq!(normalize_selected_day(Some(30), 2026, 4), Some(30));
    assert_eq!(
        normalize_aria_label(Some("  Ship date ".to_string())),
        ("Ship date".to_string(), true)
    );
    assert_eq!(
        normalize_aria_label(None),
        (
            ui_logic_calendar::date_picker::DEFAULT_ARIA_LABEL.into(),
            false
        )
    );
}

#[test]
fn compose_class_name_includes_state_markers() {
    let class_name = compose_class_name(
        Some("docs-date-picker".to_string()),
        resolve_state(DatePickerStateInput {
            year: 2026,
            month: 1,
            selected_day: None,
            tone: DatePickerTone::Quiet,
            disabled: true,
            open: false,
            has_custom_placeholder: false,
            has_custom_aria_label: false,
            has_custom_class_name: true,
            has_custom_motion: true,
        }),
    );

    for token in [
        "ui-date-picker",
        "ui-date-picker--tone-quiet",
        "ui-date-picker--closed",
        "ui-date-picker--disabled",
        "ui-date-picker--empty",
        "ui-date-picker--custom-class",
        "ui-date-picker--custom-motion",
        "docs-date-picker",
    ] {
        assert!(class_name.contains(token), "class should include `{token}`");
    }
}
