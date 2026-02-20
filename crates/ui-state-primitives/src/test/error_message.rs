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
