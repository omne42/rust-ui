use super::*;

#[test]
fn tone_class_names_and_attrs_are_stable() {
    assert_eq!(
        DatePickerTone::Default.class_name(),
        "ui-date-picker--tone-default"
    );
    assert_eq!(
        DatePickerTone::Quiet.class_name(),
        "ui-date-picker--tone-quiet"
    );
    assert_eq!(
        DatePickerTone::Strong.class_name(),
        "ui-date-picker--tone-strong"
    );

    assert_eq!(DatePickerTone::Default.as_attr(), "default");
    assert_eq!(DatePickerTone::Quiet.as_attr(), "quiet");
    assert_eq!(DatePickerTone::Strong.as_attr(), "strong");
}

#[test]
fn normalize_text_and_placeholder_use_defaults() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("  \n\t ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some(" docs ".to_string())),
        Some("docs".to_string())
    );

    let (placeholder, is_custom) = normalize_placeholder(None);
    assert_eq!(placeholder, DEFAULT_PLACEHOLDER);
    assert!(!is_custom);
}

#[test]
fn date_helpers_clamp_and_validate_day() {
    assert_eq!(normalize_month(0), 1);
    assert_eq!(normalize_month(17), 12);
    assert_eq!(days_in_month(2024, 2), 29);
    assert_eq!(days_in_month(2023, 2), 28);
    assert_eq!(normalize_selected_day(Some(31), 2026, 4), None);
    assert_eq!(normalize_selected_day(Some(30), 2026, 4), Some(30));
}

#[test]
fn resolve_state_tracks_sources_and_open_value_flags() {
    let state = resolve_state(DatePickerStateInput {
        year: 2026,
        month: 1,
        selected_day: Some(6),
        tone: DatePickerTone::Strong,
        disabled: false,
        open: true,
        has_custom_placeholder: true,
        has_custom_aria_label: true,
        has_custom_class_name: false,
        has_custom_motion: true,
    });

    assert_eq!(state.month, 1);
    assert_eq!(state.selected_day, Some(6));
    assert_eq!(state.tone_attr, "strong");
    assert_eq!(state.data_state_attr, "open");
    assert_eq!(state.placeholder_source_attr, "custom");
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "default");
    assert_eq!(state.motion_source_attr, "custom");
    assert!(state.has_custom_motion);
}
