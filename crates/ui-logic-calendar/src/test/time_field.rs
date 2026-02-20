use super::*;

#[test]
fn tone_class_names_and_attrs_are_stable() {
    assert_eq!(
        TimeFieldTone::Default.class_name(),
        "ui-time-field--tone-default"
    );
    assert_eq!(
        TimeFieldTone::Quiet.class_name(),
        "ui-time-field--tone-quiet"
    );
    assert_eq!(
        TimeFieldTone::Strong.class_name(),
        "ui-time-field--tone-strong"
    );

    assert_eq!(TimeFieldTone::Default.as_attr(), "default");
    assert_eq!(TimeFieldTone::Quiet.as_attr(), "quiet");
    assert_eq!(TimeFieldTone::Strong.as_attr(), "strong");
}

#[test]
fn normalize_time_value_formats_zero_padded_step_aware_values() {
    let value = normalize_time_value(Some(" 9:17 ".to_string()), 5);
    assert_eq!(value, Some("09:15".to_string()));

    let invalid = normalize_time_value(Some("not-a-time".to_string()), 15);
    assert_eq!(invalid, None);
}

#[test]
fn normalize_a11y_and_clear_labels_use_defaults_for_blank_values() {
    assert_eq!(
        normalize_hour_aria_label(Some("  ".to_string()), DEFAULT_HOUR_ARIA_LABEL),
        (DEFAULT_HOUR_ARIA_LABEL.into(), false)
    );
    assert_eq!(
        normalize_minute_aria_label(None, DEFAULT_MINUTE_ARIA_LABEL),
        (DEFAULT_MINUTE_ARIA_LABEL.into(), false)
    );
    assert_eq!(
        normalize_clear_label(Some("  Clear now  ".to_string()), DEFAULT_CLEAR_LABEL),
        ("Clear now".to_string(), true)
    );
    assert_eq!(
        normalize_clear_aria_label(None, DEFAULT_CLEAR_ARIA_LABEL),
        (DEFAULT_CLEAR_ARIA_LABEL.into(), false)
    );
}

#[test]
fn update_helpers_keep_other_segment_stable() {
    let value = update_hour_from_input(Some("06:45".to_string()), "9", 15);
    assert_eq!(value, Some("09:45".to_string()));

    let value = update_minute_from_input(value, "14", 5);
    assert_eq!(value, Some("09:10".to_string()));
}

#[test]
fn resolve_state_tracks_sources_and_value_state() {
    let state = resolve_state(TimeFieldStateInput {
        tone: TimeFieldTone::Strong,
        disabled: false,
        is_controlled: true,
        has_default_value: true,
        has_value_change_handler: true,
        has_value: true,
        minute_step: 15,
        has_custom_label: true,
        has_custom_placeholder: false,
        has_custom_aria_label: true,
        has_custom_class_name: false,
        has_custom_motion: true,
    });

    assert_eq!(state.tone_attr, "strong");
    assert_eq!(state.data_state_attr, "value");
    assert_eq!(state.label_source_attr, "custom");
    assert_eq!(state.placeholder_source_attr, "default");
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "default");
    assert_eq!(state.motion_source_attr, "custom");
    assert!(state.is_controlled);
    assert!(!state.is_uncontrolled);
    assert_eq!(state.control_mode_attr, "controlled");
    assert_eq!(state.value_source_attr, "external");
    assert_eq!(state.default_value_source_attr, "provided");
    assert_eq!(state.value_change_source_attr, "provided");
}

#[test]
fn compose_class_name_includes_state_markers() {
    let class_name = compose_class_name(
        Some("docs-time-field".to_string()),
        resolve_state(TimeFieldStateInput {
            tone: TimeFieldTone::Quiet,
            disabled: true,
            is_controlled: false,
            has_default_value: false,
            has_value_change_handler: false,
            has_value: false,
            minute_step: 10,
            has_custom_label: false,
            has_custom_placeholder: false,
            has_custom_aria_label: false,
            has_custom_class_name: true,
            has_custom_motion: false,
        }),
    );

    for token in [
        "ui-time-field",
        "ui-time-field--tone-quiet",
        "ui-time-field--disabled",
        "ui-time-field--custom-class",
        "docs-time-field",
    ] {
        assert!(
            class_name.contains(token),
            "class should include `{token}`, got `{class_name}`"
        );
    }
}
