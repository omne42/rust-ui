use super::*;

#[test]
fn compose_class_name_includes_markers() {
    let class_name = compose_class_name(
        Some("docs-error-message".to_string()),
        resolve_state(ErrorMessageStateInput {
            tone: ErrorMessageTone::Negative,
            disabled: true,
            truncate: true,
            has_custom_message: false,
            has_custom_aria_label: false,
            has_custom_class_name: true,
        }),
    );

    for token in [
        "ui-error-message",
        "ui-error-message--tone-negative",
        "ui-error-message--disabled",
        "ui-error-message--truncate",
        "ui-error-message--custom-class",
        "docs-error-message",
    ] {
        assert!(
            class_name.contains(token),
            "composed class should include `{token}`"
        );
    }
}

#[test]
fn normalize_state_flags_centralizes_default_priority() {
    let defaults = normalize_state_flags(ErrorMessageStateFlagsInput::default());
    assert!(
        !defaults.disabled && !defaults.truncate,
        "state flags should default to false when no explicit props are provided"
    );

    let alias_only = normalize_state_flags(ErrorMessageStateFlagsInput {
        disabled: Some(true),
        truncate: Some(true),
        ..ErrorMessageStateFlagsInput::default()
    });
    assert!(
        alias_only.disabled && alias_only.truncate,
        "legacy alias props should remain supported during migration"
    );

    let canonical_wins = normalize_state_flags(ErrorMessageStateFlagsInput {
        is_disabled: Some(false),
        disabled: Some(true),
        is_truncated: Some(false),
        truncate: Some(true),
    });
    assert!(
        !canonical_wins.disabled && !canonical_wins.truncate,
        "canonical is_* props must take precedence over legacy alias props"
    );
}

#[test]
fn resolve_status_models_mutually_exclusive_state_axis() {
    assert_eq!(
        resolve_status(ErrorMessageStateFlags {
            disabled: false,
            truncate: false,
        }),
        ErrorMessageStatus::Default,
        "all flags false should map to default status"
    );
    assert_eq!(
        resolve_status(ErrorMessageStateFlags {
            disabled: false,
            truncate: true,
        }),
        ErrorMessageStatus::Truncate,
        "truncate-only should map to truncate status"
    );
    assert_eq!(
        resolve_status(ErrorMessageStateFlags {
            disabled: true,
            truncate: false,
        }),
        ErrorMessageStatus::Disabled,
        "disabled-only should map to disabled status"
    );
    assert_eq!(
        resolve_status(ErrorMessageStateFlags {
            disabled: true,
            truncate: true,
        }),
        ErrorMessageStatus::Disabled,
        "invalid bool combo should be normalized into disabled status"
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
fn resolve_model_centralizes_input_normalization_and_state_derivation() {
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
