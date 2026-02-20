use super::*;
use crate::text_input::date_field::DateFieldStateInput;

#[test]
fn tone_class_names_and_attrs_are_stable() {
    assert_eq!(
        DateFieldTone::Default.class_name(),
        "ui-date-field--tone-default"
    );
    assert_eq!(
        DateFieldTone::Quiet.class_name(),
        "ui-date-field--tone-quiet"
    );
    assert_eq!(
        DateFieldTone::Strong.class_name(),
        "ui-date-field--tone-strong"
    );

    assert_eq!(DateFieldTone::Default.as_attr(), "default");
    assert_eq!(DateFieldTone::Quiet.as_attr(), "quiet");
    assert_eq!(DateFieldTone::Strong.as_attr(), "strong");
}

#[test]
fn normalize_and_parse_date_values_are_consistent() {
    let value = normalize_date_value(Some(" 2026-2-9 ".to_string()));
    assert_eq!(value, Some("2026-02-09".to_string()));

    let parsed = parse_date_value("2026-02-09");
    assert_eq!(parsed, Some((2026, 2, 9)));

    assert_eq!(normalize_date_value(Some("bad-input".to_string())), None);
}

#[test]
fn update_helpers_keep_other_segments_stable() {
    let value = update_year_from_input(Some("2026-07-14".to_string()), "2025");
    assert_eq!(value, Some("2025-07-14".to_string()));

    let value = update_month_from_input(value, "2");
    assert_eq!(value, Some("2025-02-14".to_string()));

    let value = update_day_from_input(value, "30");
    assert_eq!(value, Some("2025-02-28".to_string()));
}

#[test]
fn resolve_state_tracks_sources_and_value() {
    let state = resolve_state(DateFieldStateInput {
        tone: DateFieldTone::Strong,
        disabled: false,
        has_value: true,
        has_custom_label: true,
        has_custom_placeholder: false,
        has_custom_aria_label: true,
        has_custom_class_name: false,
    });

    assert_eq!(state.tone_attr, "strong");
    assert_eq!(state.data_state_attr, "value");
    assert_eq!(state.label_source_attr, "custom");
    assert_eq!(state.placeholder_source_attr, "default");
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "default");
}

#[test]
fn compose_class_name_includes_state_markers() {
    let class_name = compose_class_name(
        Some("docs-date-field".to_string()),
        resolve_state(DateFieldStateInput {
            tone: DateFieldTone::Quiet,
            disabled: true,
            has_value: false,
            has_custom_label: false,
            has_custom_placeholder: false,
            has_custom_aria_label: false,
            has_custom_class_name: true,
        }),
    );

    for token in [
        "ui-date-field",
        "ui-date-field--tone-quiet",
        "ui-date-field--disabled",
        "ui-date-field--custom-class",
        "docs-date-field",
    ] {
        assert!(class_name.contains(token), "class should include `{token}`");
    }
}
