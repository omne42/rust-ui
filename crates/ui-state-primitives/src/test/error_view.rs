use super::*;

#[test]
fn tone_contract_is_stable() {
    assert_eq!(
        ErrorViewTone::Negative.class_name(),
        "ui-error-view--tone-negative"
    );
    assert_eq!(
        ErrorViewTone::Neutral.class_name(),
        "ui-error-view--tone-neutral"
    );
    assert_eq!(ErrorViewTone::Negative.as_attr(), "negative");
    assert_eq!(ErrorViewTone::Neutral.as_attr(), "neutral");
}

#[test]
fn normalize_helpers_trim_and_fallback() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("\n\t  ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  bad input  ".to_string())),
        Some("bad input".to_string())
    );

    let (message, custom_message) = normalize_message(Some("  Email invalid  ".to_string()));
    assert_eq!(message, "Email invalid");
    assert!(custom_message);

    let (message, custom_message) = normalize_message(None);
    assert_eq!(message, DEFAULT_MESSAGE);
    assert!(!custom_message);

    let (label, custom_label) = normalize_aria_label(Some("  field error  ".to_string()));
    assert_eq!(label, "field error");
    assert!(custom_label);

    let (label, custom_label) = normalize_aria_label(None);
    assert_eq!(label, DEFAULT_ARIA_LABEL);
    assert!(!custom_label);
}
