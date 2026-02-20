use super::*;
use crate::FieldErrorStateInput;

#[test]
fn tone_contract_is_stable() {
    assert_eq!(
        FieldErrorTone::Auto.class_name(),
        "ui-field-error--tone-auto"
    );
    assert_eq!(
        FieldErrorTone::Neutral.class_name(),
        "ui-field-error--tone-neutral"
    );
    assert_eq!(
        FieldErrorTone::Negative.class_name(),
        "ui-field-error--tone-negative"
    );

    assert_eq!(FieldErrorTone::Auto.as_attr(), "auto");
    assert_eq!(FieldErrorTone::Neutral.as_attr(), "neutral");
    assert_eq!(FieldErrorTone::Negative.as_attr(), "negative");
}

#[test]
fn normalize_helpers_trim_and_fallback() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  bad email  ".to_string())),
        Some("bad email".to_string())
    );

    let (label, custom_label) = normalize_aria_label(Some("  Email error  ".to_string()));
    assert_eq!(label, "Email error");
    assert!(custom_label);

    let (label, custom_label) = normalize_aria_label(None);
    assert_eq!(label, DEFAULT_ARIA_LABEL);
    assert!(!custom_label);

    let (message, custom_message) = normalize_message(Some("  Required  ".to_string()), true);
    assert_eq!(message, Some("Required".to_string()));
    assert!(custom_message);

    let (message, custom_message) = normalize_message(None, true);
    assert_eq!(message, Some(DEFAULT_MESSAGE.into()));
    assert!(!custom_message);

    let (message, custom_message) = normalize_message(Some("ignored".to_string()), false);
    assert_eq!(message, None);
    assert!(!custom_message);
}

#[test]
fn resolve_state_tracks_visibility_and_sources() {
    let state = resolve_state(FieldErrorStateInput {
        tone: FieldErrorTone::Auto,
        visible: true,
        disabled: false,
        show_icon: true,
        has_message: true,
        has_custom_aria_label: true,
        has_custom_message: false,
        has_custom_class_name: false,
    });

    assert!(state.is_visible);
    assert_eq!(state.tone_attr, "negative");
    assert_eq!(state.data_state_attr, "visible");
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.message_source_attr, "default");
    assert_eq!(state.class_source_attr, "default");
    assert!(state.show_icon);
}

#[test]
fn compose_class_name_includes_state_markers() {
    let class_name = compose_class_name(
        Some("docs-field-error".to_string()),
        resolve_state(FieldErrorStateInput {
            tone: FieldErrorTone::Negative,
            visible: true,
            disabled: true,
            show_icon: true,
            has_message: true,
            has_custom_aria_label: false,
            has_custom_message: true,
            has_custom_class_name: true,
        }),
    );

    for token in [
        "ui-field-error",
        "ui-field-error--tone-negative",
        "ui-field-error--visible",
        "ui-field-error--disabled",
        "ui-field-error--with-icon",
        "ui-field-error--custom-class",
        "docs-field-error",
    ] {
        assert!(
            class_name.contains(token),
            "composed class should contain `{token}`"
        );
    }
}
