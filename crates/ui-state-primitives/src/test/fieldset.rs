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
    assert_eq!(state.message_kind_attr, "error");
    assert_eq!(state.data_state_attr, "invalid");
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.error_source_attr, "default");
    assert_eq!(state.class_source_attr, "custom");
    assert!(state.has_actions);
}
