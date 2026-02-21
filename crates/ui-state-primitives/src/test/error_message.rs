use super::*;

#[test]
fn tone_and_element_contracts_are_stable() {
    assert_eq!(
        ErrorMessageTone::Auto.class_name(),
        "ui-error-message--tone-auto"
    );
    assert_eq!(
        ErrorMessageTone::Neutral.class_name(),
        "ui-error-message--tone-neutral"
    );
    assert_eq!(
        ErrorMessageTone::Negative.class_name(),
        "ui-error-message--tone-negative"
    );

    assert_eq!(ErrorMessageTone::Auto.as_attr(), "auto");
    assert_eq!(ErrorMessageTone::Neutral.as_attr(), "neutral");
    assert_eq!(ErrorMessageTone::Negative.as_attr(), "negative");
    assert_eq!(
        ErrorMessageElement::default(),
        ErrorMessageElement::Paragraph
    );
}

#[test]
fn normalize_helpers_trim_and_fallback() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  bad input  ".to_string())),
        Some("bad input".to_string())
    );

    let (message, custom_message) = normalize_message(Some("  Required  ".to_string()));
    assert_eq!(message, "Required");
    assert!(custom_message);

    let (message, custom_message) = normalize_message(None);
    assert_eq!(message, DEFAULT_MESSAGE);
    assert!(!custom_message);

    let (label, custom_label) = normalize_aria_label(Some("  Email error  ".to_string()));
    assert_eq!(label, "Email error");
    assert!(custom_label);

    let (label, custom_label) = normalize_aria_label(None);
    assert_eq!(label, DEFAULT_ARIA_LABEL);
    assert!(!custom_label);
}

#[test]
fn resolve_state_tracks_sources_and_priority() {
    let state = resolve_state(ErrorMessageStateInput {
        tone: ErrorMessageTone::Auto,
        disabled: false,
        truncate: true,
        has_custom_message: true,
        has_custom_aria_label: false,
        has_custom_class_name: true,
    });

    assert_eq!(state.tone_attr, "negative");
    assert_eq!(state.data_state_attr, "truncate");
    assert_eq!(state.message_source_attr, "custom");
    assert_eq!(state.aria_source_attr, "default");
    assert_eq!(state.class_source_attr, "custom");
}

#[test]
fn status_resolution_normalizes_invalid_bool_combinations() {
    assert_eq!(
        resolve_status(ErrorMessageStateFlags {
            disabled: false,
            truncate: false,
        }),
        ErrorMessageStatus::Default
    );
    assert_eq!(
        resolve_status(ErrorMessageStateFlags {
            disabled: false,
            truncate: true,
        }),
        ErrorMessageStatus::Truncate
    );
    assert_eq!(
        resolve_status(ErrorMessageStateFlags {
            disabled: true,
            truncate: false,
        }),
        ErrorMessageStatus::Disabled
    );
    assert_eq!(
        resolve_status(ErrorMessageStateFlags {
            disabled: true,
            truncate: true,
        }),
        ErrorMessageStatus::Disabled
    );

    assert_eq!(
        status_to_primitive_flags(ErrorMessageStatus::Default),
        ErrorMessageStateFlags {
            disabled: false,
            truncate: false,
        }
    );
    assert_eq!(
        status_to_primitive_flags(ErrorMessageStatus::Truncate),
        ErrorMessageStateFlags {
            disabled: false,
            truncate: true,
        }
    );
    assert_eq!(
        status_to_primitive_flags(ErrorMessageStatus::Disabled),
        ErrorMessageStateFlags {
            disabled: true,
            truncate: false,
        }
    );
}

#[test]
fn resolve_model_derives_state_from_typed_status_axis() {
    let model = resolve_model(ErrorMessageModelInput {
        tone: ErrorMessageTone::Auto,
        is_disabled: Some(false),
        disabled: Some(true),
        is_truncated: None,
        truncate: Some(true),
        text: Some("custom message".to_string()),
        aria_label: None,
        class_name: Some("   ".to_string()),
    });

    assert_eq!(model.text, "custom message");
    assert_eq!(model.aria_label, DEFAULT_ARIA_LABEL);
    assert_eq!(model.class_name, None);
    assert_eq!(model.status, ErrorMessageStatus::Truncate);
    assert_eq!(model.state.data_state_attr, "truncate");
    assert_eq!(model.state.message_source_attr, "custom");
    assert_eq!(model.state.aria_source_attr, "default");
    assert_eq!(model.state.class_source_attr, "default");
}
