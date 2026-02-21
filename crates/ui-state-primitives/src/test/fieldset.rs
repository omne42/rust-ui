use super::*;

#[test]
fn orientation_and_tone_contracts_are_stable() {
    assert_eq!(
        FieldsetOrientation::Vertical.class_name(),
        "ui-fieldset--orientation-vertical"
    );
    assert_eq!(
        FieldsetOrientation::Horizontal.class_name(),
        "ui-fieldset--orientation-horizontal"
    );
    assert_eq!(FieldsetOrientation::Vertical.as_attr(), "vertical");
    assert_eq!(FieldsetOrientation::Horizontal.as_attr(), "horizontal");

    assert_eq!(
        FieldsetTone::Default.class_name(),
        "ui-fieldset--tone-default"
    );
    assert_eq!(FieldsetTone::Muted.class_name(), "ui-fieldset--tone-muted");
    assert_eq!(FieldsetTone::Default.as_attr(), "default");
    assert_eq!(FieldsetTone::Muted.as_attr(), "muted");
}

#[test]
fn normalize_helpers_trim_and_fallback() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("   \n\t ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  Billing details  ".to_string())),
        Some("Billing details".to_string())
    );

    let (label, custom) = normalize_aria_label(Some("  Payment group  ".to_string()));
    assert_eq!(label, "Payment group");
    assert!(custom);

    let (label, custom) = normalize_aria_label(None);
    assert_eq!(label, DEFAULT_ARIA_LABEL);
    assert!(!custom);
}

#[test]
fn error_message_normalization_respects_invalid_state() {
    let (message, custom) = normalize_error_message(Some("  Missing value  ".to_string()), true);
    assert_eq!(message, Some("Missing value".to_string()));
    assert!(custom);

    let (message, custom) = normalize_error_message(None, true);
    assert_eq!(message, Some(DEFAULT_ERROR_MESSAGE.into()));
    assert!(!custom);

    let (message, custom) = normalize_error_message(Some("Ignored".to_string()), false);
    assert_eq!(message, None);
    assert!(!custom);
}

#[test]
fn resolve_state_tracks_sources_and_priorities() {
    let state = resolve_state(FieldsetStateInput {
        orientation: FieldsetOrientation::Horizontal,
        tone: FieldsetTone::Muted,
        required: true,
        disabled: false,
        invalid: true,
        has_legend: true,
        has_description: false,
        has_error_message: true,
        has_actions: true,
        has_custom_aria_label: true,
        has_custom_error_message: false,
        has_custom_class_name: true,
    });

    assert_eq!(state.orientation_attr, "horizontal");
    assert_eq!(state.tone_attr, "muted");
    assert_eq!(state.message_kind, FieldsetMessageKind::Error);
    assert_eq!(state.data_state, FieldsetDataState::Invalid);
    assert_eq!(state.message_kind_attr, "error");
    assert_eq!(state.data_state_attr, "invalid");
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.error_source_attr, "default");
    assert_eq!(state.class_source_attr, "custom");
    assert!(state.has_actions);
}

#[test]
fn normalize_boolean_axis_prefers_controlled_value_source() {
    let axis = normalize_boolean_axis(FieldsetBooleanAxisInput {
        value: Some(true),
        default_value: Some(false),
        has_on_change: true,
        value_source_attr: "is_required",
        default_source_attr: "default_is_required",
        change_source_attr: "on_is_required_change",
    });

    assert_eq!(axis.controlled_value, Some(true));
    assert!(!axis.initial_value);
    assert_eq!(axis.value_source_attr, "is_required");
    assert_eq!(axis.control_mode_attr, "controlled");
    assert_eq!(axis.change_source_attr, "on_is_required_change");
}

#[test]
fn normalize_boolean_axis_uses_default_source_for_uncontrolled_axis() {
    let axis = normalize_boolean_axis(FieldsetBooleanAxisInput {
        value: None,
        default_value: Some(true),
        has_on_change: false,
        value_source_attr: "is_disabled",
        default_source_attr: "default_is_disabled",
        change_source_attr: "on_is_disabled_change",
    });

    assert_eq!(axis.controlled_value, None);
    assert!(axis.initial_value);
    assert_eq!(axis.value_source_attr, "default_is_disabled");
    assert_eq!(axis.control_mode_attr, "uncontrolled");
    assert_eq!(axis.change_source_attr, "none");
}

#[test]
fn normalize_boolean_axis_falls_back_to_builtin_default_false() {
    let axis = normalize_boolean_axis(FieldsetBooleanAxisInput {
        value: None,
        default_value: None,
        has_on_change: false,
        value_source_attr: "is_invalid",
        default_source_attr: "default_is_invalid",
        change_source_attr: "on_is_invalid_change",
    });

    assert_eq!(axis.controlled_value, None);
    assert!(!axis.initial_value);
    assert_eq!(axis.value_source_attr, "default");
    assert_eq!(axis.control_mode_attr, "uncontrolled");
    assert_eq!(axis.change_source_attr, "none");
}
