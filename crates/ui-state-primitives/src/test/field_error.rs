use super::*;

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
fn resolve_state_tracks_visibility_and_source_markers() {
    let visible = resolve_state(FieldErrorStateInput {
        tone: FieldErrorTone::Auto,
        visible: true,
        disabled: false,
        show_icon: true,
        has_message: true,
        has_custom_aria_label: true,
        has_custom_message: false,
        has_custom_class_name: false,
    });

    assert!(visible.is_visible);
    assert_eq!(visible.tone, FieldErrorTone::Negative);
    assert_eq!(visible.data_state, FieldErrorDataState::Visible);
    assert_eq!(visible.aria_source, FieldErrorSource::Custom);
    assert_eq!(visible.message_source, FieldErrorMessageSource::Default);
    assert_eq!(visible.class_source, FieldErrorSource::Default);

    let hidden = resolve_state(FieldErrorStateInput {
        tone: FieldErrorTone::Auto,
        visible: true,
        disabled: false,
        show_icon: true,
        has_message: false,
        has_custom_aria_label: false,
        has_custom_message: false,
        has_custom_class_name: true,
    });

    assert!(!hidden.is_visible);
    assert_eq!(hidden.data_state, FieldErrorDataState::Hidden);
    assert_eq!(hidden.message_source, FieldErrorMessageSource::None);
    assert_eq!(hidden.class_source, FieldErrorSource::Custom);
}
