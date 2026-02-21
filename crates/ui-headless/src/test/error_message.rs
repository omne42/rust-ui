use super::*;
use ui_state_primitives::error_message::{ErrorMessageStateInput, ErrorMessageTone, resolve_state};

#[test]
fn use_error_message_maps_live_region_locale_and_state_attrs() {
    let state = resolve_state(ErrorMessageStateInput {
        tone: ErrorMessageTone::Neutral,
        disabled: true,
        truncate: true,
        has_custom_message: true,
        has_custom_aria_label: true,
        has_custom_class_name: true,
    });

    let contract = use_error_message(ErrorMessageOptions {
        state,
        aria_label: "Email validation error".to_string(),
        lang: Some("  zh-CN ".to_string()),
        dir: Some(A11yDirection::Rtl),
    });

    assert_eq!(contract.attrs.role, "alert");
    assert_eq!(contract.attrs.aria_live, "assertive");
    assert_eq!(contract.attrs.aria_label, "Email validation error");
    assert_eq!(contract.attrs.aria_disabled, Some("true"));
    assert_eq!(contract.attrs.lang.as_deref(), Some("zh-CN"));
    assert_eq!(contract.attrs.dir, Some("rtl"));
    assert_eq!(contract.attrs.data_tone, "neutral");
    assert_eq!(contract.attrs.data_state, "disabled");
    assert_eq!(contract.attrs.data_disabled, Some("true"));
    assert_eq!(contract.attrs.data_truncate, Some("true"));
    assert_eq!(contract.attrs.data_message_source, "custom");
    assert_eq!(contract.attrs.data_aria_source, "custom");
    assert_eq!(contract.attrs.data_custom_class, Some("true"));
    assert_eq!(contract.attrs.data_class_source, "custom");
    assert_eq!(
        contract.attrs.data_ui_schema,
        "ui.error-message.agent-contract.v1"
    );
    assert_eq!(contract.attrs.data_ui_schema_version, "1");
    assert_eq!(contract.attrs.data_ui_intent, "form-validation-feedback");
    assert_eq!(contract.attrs.data_ui_action, "read-only");
    assert_eq!(contract.attrs.data_ui_stream_support, "optional");
    assert_eq!(contract.attrs.data_ui_stream_mode, "snapshot");
    assert_eq!(contract.attrs.data_ui_stream_fallback, "snapshot");
    assert_eq!(contract.attrs.data_ui_output_status, "draft");
    assert_eq!(contract.attrs.data_stream_mode, "snapshot");
    assert_eq!(contract.attrs.data_stream_fallback, "snapshot");
    assert_eq!(contract.attrs.data_output_status, "draft");
    assert_eq!(contract.state.ui_output_mode, "snapshot");
    assert_eq!(contract.state.ui_action, "read-only");
    assert_eq!(contract.state.ui_output_status, "draft");
}

#[test]
fn use_error_message_preserves_default_state_without_optional_attrs() {
    let state = resolve_state(ErrorMessageStateInput {
        tone: ErrorMessageTone::Auto,
        disabled: false,
        truncate: false,
        has_custom_message: false,
        has_custom_aria_label: false,
        has_custom_class_name: false,
    });

    let contract = use_error_message(ErrorMessageOptions {
        state,
        aria_label: "ErrorMessage".to_string(),
        lang: None,
        dir: None,
    });

    assert_eq!(contract.attrs.role, "alert");
    assert_eq!(contract.attrs.aria_live, "assertive");
    assert_eq!(contract.attrs.aria_disabled, None);
    assert_eq!(contract.attrs.lang, None);
    assert_eq!(contract.attrs.dir, None);
    assert_eq!(contract.attrs.data_tone, "negative");
    assert_eq!(contract.attrs.data_state, "default");
    assert_eq!(contract.attrs.data_disabled, None);
    assert_eq!(contract.attrs.data_truncate, None);
    assert_eq!(contract.attrs.data_message_source, "default");
    assert_eq!(contract.attrs.data_aria_source, "default");
    assert_eq!(contract.attrs.data_custom_class, None);
    assert_eq!(contract.attrs.data_class_source, "default");
    assert_eq!(
        contract.attrs.data_ui_schema,
        "ui.error-message.agent-contract.v1"
    );
    assert_eq!(contract.attrs.data_ui_schema_version, "1");
    assert_eq!(contract.attrs.data_ui_intent, "form-validation-feedback");
    assert_eq!(contract.attrs.data_ui_action, "announce-error");
    assert_eq!(contract.attrs.data_ui_stream_support, "optional");
    assert_eq!(contract.attrs.data_ui_stream_mode, "snapshot");
    assert_eq!(contract.attrs.data_ui_stream_fallback, "snapshot");
    assert_eq!(contract.attrs.data_ui_output_status, "verified");
    assert_eq!(contract.attrs.data_stream_mode, "snapshot");
    assert_eq!(contract.attrs.data_stream_fallback, "snapshot");
    assert_eq!(contract.attrs.data_output_status, "verified");
    assert_eq!(contract.state.ui_output_mode, "snapshot");
    assert_eq!(contract.state.ui_action, "announce-error");
    assert_eq!(contract.state.ui_output_status, "verified");
}
