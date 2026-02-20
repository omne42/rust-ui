use super::*;

#[test]
fn tone_class_names_and_attrs_are_stable() {
    assert_eq!(
        DateRangePickerTone::Default.class_name(),
        "ui-date-range-picker--tone-default"
    );
    assert_eq!(
        DateRangePickerTone::Quiet.class_name(),
        "ui-date-range-picker--tone-quiet"
    );
    assert_eq!(
        DateRangePickerTone::Strong.class_name(),
        "ui-date-range-picker--tone-strong"
    );

    assert_eq!(DateRangePickerTone::Default.as_attr(), "default");
    assert_eq!(DateRangePickerTone::Quiet.as_attr(), "quiet");
    assert_eq!(DateRangePickerTone::Strong.as_attr(), "strong");
}

#[test]
fn day_normalization_and_range_order_are_stable() {
    assert_eq!(normalize_month(0), 1);
    assert_eq!(normalize_month(22), 12);
    assert_eq!(normalize_day(Some(31), 2026, 4), None);
    assert_eq!(normalize_day(Some(30), 2026, 4), Some(30));

    assert!(is_range_invalid(Some((2026, 4, 20)), Some((2026, 4, 12))));
    assert!(!is_range_invalid(Some((2026, 4, 12)), Some((2026, 4, 20))));
}

#[test]
fn resolve_state_tracks_value_shape_and_invalidity() {
    let state = resolve_state(DateRangePickerStateInput {
        tone: DateRangePickerTone::Strong,
        disabled: false,
        has_start_value: true,
        has_end_value: true,
        is_invalid_range: false,
        has_custom_aria_label: true,
        has_custom_class_name: false,
    });

    assert_eq!(state.data_state_attr, "value");
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "default");
    assert!(state.has_full_value);
    assert!(!state.is_partial);
}

#[test]
fn compose_class_name_includes_state_markers() {
    let class_name = compose_class_name(
        Some("docs-date-range".to_string()),
        resolve_state(DateRangePickerStateInput {
            tone: DateRangePickerTone::Quiet,
            disabled: true,
            has_start_value: true,
            has_end_value: false,
            is_invalid_range: false,
            has_custom_aria_label: false,
            has_custom_class_name: true,
        }),
    );

    for token in [
        "ui-date-range-picker",
        "ui-date-range-picker--tone-quiet",
        "ui-date-range-picker--disabled",
        "ui-date-range-picker--has-start",
        "ui-date-range-picker--partial",
        "ui-date-range-picker--custom-class",
        "docs-date-range",
    ] {
        assert!(class_name.contains(token), "class should include `{token}`");
    }
}
