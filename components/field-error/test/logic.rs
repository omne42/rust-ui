use super::*;
use ui_state_primitives::field_error::{
    FieldErrorDataState, FieldErrorMessageSource, FieldErrorSource, FieldErrorStateInput,
};

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

    assert_eq!(
        to_error_message_tone(FieldErrorTone::Auto),
        ui_state_primitives::error_message::ErrorMessageTone::Auto
    );
    assert_eq!(
        to_error_message_tone(FieldErrorTone::Neutral),
        ui_state_primitives::error_message::ErrorMessageTone::Neutral
    );
    assert_eq!(
        to_error_message_tone(FieldErrorTone::Negative),
        ui_state_primitives::error_message::ErrorMessageTone::Negative
    );
}

#[test]
fn normalize_control_inputs_prefers_is_prefix_values() {
    let preferred =
        normalize_control_inputs(Some(true), false, Some(true), false, Some(true), false);
    assert!(preferred.visible);
    assert!(preferred.disabled);
    assert!(preferred.show_icon);

    let legacy_fallback = normalize_control_inputs(None, true, None, true, None, true);
    assert!(legacy_fallback.visible);
    assert!(legacy_fallback.disabled);
    assert!(legacy_fallback.show_icon);
}

#[test]
fn resolve_view_model_centralizes_input_normalization_and_state_derivation() {
    let model = resolve_view_model(FieldErrorLogicInput {
        tone: FieldErrorTone::Auto,
        is_visible: Some(true),
        visible: false,
        is_disabled: Some(true),
        disabled: false,
        is_icon_visible: Some(true),
        show_icon: false,
        message: Some("  Required value  ".to_string()),
        aria_label: Some("  Email Error  ".to_string()),
        class_name: Some("  docs-field-error  ".to_string()),
        default_message: None,
        default_aria_label: None,
    });

    assert_eq!(model.aria_label, "Email Error");
    assert_eq!(model.message, Some("Required value".to_string()));
    assert_eq!(model.class_name, Some("docs-field-error".to_string()));
    assert!(model.has_custom_aria_label);
    assert!(model.has_custom_message);
    assert!(model.has_custom_class_name);
    assert!(model.state.is_visible);
    assert!(model.state.is_disabled);
    assert!(model.state.show_icon);
    assert_eq!(model.state.data_state, FieldErrorDataState::Disabled);
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
    assert_eq!(state.data_state, FieldErrorDataState::Visible);
    assert_eq!(state.aria_source, FieldErrorSource::Custom);
    assert_eq!(state.message_source, FieldErrorMessageSource::Default);
    assert_eq!(state.class_source, FieldErrorSource::Default);
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

#[test]
fn resolve_headless_state_tracks_accessibility_semantic_sources() {
    let state = resolve_headless_state(FieldErrorTone::Negative, true, true, false, true);

    assert_eq!(state.tone_attr, "negative");
    assert_eq!(state.data_state_attr, "disabled");
    assert_eq!(state.message_source_attr, "custom");
    assert_eq!(state.aria_source_attr, "default");
    assert_eq!(state.class_source_attr, "custom");
}
