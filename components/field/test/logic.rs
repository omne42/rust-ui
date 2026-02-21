use super::*;

#[test]
fn field_enums_map_to_stable_class_and_attr_contracts() {
    assert_eq!(
        FieldOrientation::Vertical.class_name(),
        "ui-field--orientation-vertical"
    );
    assert_eq!(FieldOrientation::Horizontal.as_attr(), "horizontal");

    assert_eq!(FieldTone::Default.class_name(), "ui-field--tone-default");
    assert_eq!(FieldTone::Muted.as_attr(), "muted");
}

#[test]
fn normalize_optional_text_trims_and_drops_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  docs-field  ".to_string())),
        Some("docs-field".to_string())
    );
}

#[test]
fn normalize_helpers_apply_expected_fallbacks() {
    let (aria_label, custom_aria_label) =
        normalize_aria_label(Some("  Profile Field  ".to_string()));
    assert_eq!(aria_label, "Profile Field");
    assert!(custom_aria_label);

    let (aria_label, custom_aria_label) = normalize_aria_label(None);
    assert_eq!(aria_label, DEFAULT_ARIA_LABEL);
    assert!(!custom_aria_label);

    let (error_message, custom_error_message) =
        normalize_error_message(Some("  Required value  ".to_string()), true);
    assert_eq!(error_message, Some("Required value".to_string()));
    assert!(custom_error_message);

    let (error_message, custom_error_message) = normalize_error_message(None, true);
    assert_eq!(error_message, Some(DEFAULT_ERROR_MESSAGE.into()));
    assert!(!custom_error_message);

    let (error_message, custom_error_message) =
        normalize_error_message(Some("ignored".to_string()), false);
    assert_eq!(error_message, None);
    assert!(!custom_error_message);
}

#[test]
fn resolve_is_axes_prefer_is_prefix_and_fall_back_to_legacy_alias() {
    assert!(resolve_is_required(Some(true), Some(false)));
    assert!(!resolve_is_required(Some(false), Some(true)));
    assert!(resolve_is_required(None, Some(true)));
    assert!(!resolve_is_required(None, None));

    assert!(resolve_is_disabled(Some(true), Some(false)));
    assert!(!resolve_is_disabled(Some(false), Some(true)));
    assert!(resolve_is_disabled(None, Some(true)));
    assert!(!resolve_is_disabled(None, None));

    assert!(resolve_is_invalid(Some(true), Some(false)));
    assert!(!resolve_is_invalid(Some(false), Some(true)));
    assert!(resolve_is_invalid(None, Some(true)));
    assert!(!resolve_is_invalid(None, None));

    assert_eq!(
        resolve_required_source(Some(true), Some(false)).as_data_attr(),
        "is-prop"
    );
    assert_eq!(
        resolve_required_source(None, Some(true)).as_data_attr(),
        "legacy-prop"
    );
    assert_eq!(
        resolve_required_source(None, None).as_data_attr(),
        "default"
    );

    assert_eq!(
        resolve_disabled_source(Some(true), Some(false)).as_data_attr(),
        "is-prop"
    );
    assert_eq!(
        resolve_disabled_source(None, Some(true)).as_data_attr(),
        "legacy-prop"
    );
    assert_eq!(
        resolve_disabled_source(None, None).as_data_attr(),
        "default"
    );

    assert_eq!(
        resolve_invalid_source(Some(true), Some(false)).as_data_attr(),
        "is-prop"
    );
    assert_eq!(
        resolve_invalid_source(None, Some(true)).as_data_attr(),
        "legacy-prop"
    );
    assert_eq!(resolve_invalid_source(None, None).as_data_attr(), "default");
}

#[test]
fn resolve_content_centralizes_defaults_and_priority() {
    let content = resolve_content(FieldContentInput {
        label: Some("  Email  ".to_string()),
        description: Some("   ".to_string()),
        error_message: None,
        aria_label: None,
        lang: Some("  en-US  ".to_string()),
        class_name: Some("  docs-field  ".to_string()),
        is_invalid: true,
    });

    assert_eq!(content.label_text, "Email");
    assert_eq!(content.description_text, "");
    assert_eq!(content.error_message_text, DEFAULT_ERROR_MESSAGE);
    assert!(content.has_label);
    assert!(!content.has_description);
    assert!(content.has_error_message);
    assert_eq!(content.aria_label, DEFAULT_ARIA_LABEL);
    assert!(!content.has_custom_aria_label);
    assert!(!content.has_custom_error_message);
    assert_eq!(content.lang.as_deref(), Some("en-US"));
    assert_eq!(content.class_name.as_deref(), Some("docs-field"));
    assert!(content.has_custom_class_name);
}

#[test]
fn resolve_content_ignores_error_message_when_not_invalid() {
    let content = resolve_content(FieldContentInput {
        label: None,
        description: Some("  helper  ".to_string()),
        error_message: Some("  should drop  ".to_string()),
        aria_label: Some("  Profile Field  ".to_string()),
        lang: None,
        class_name: Some(" ".to_string()),
        is_invalid: false,
    });

    assert_eq!(content.label_text, "");
    assert_eq!(content.description_text, "helper");
    assert_eq!(content.error_message_text, "");
    assert!(!content.has_label);
    assert!(content.has_description);
    assert!(!content.has_error_message);
    assert_eq!(content.aria_label, "Profile Field");
    assert!(content.has_custom_aria_label);
    assert!(!content.has_custom_error_message);
    assert_eq!(content.class_name, None);
    assert!(!content.has_custom_class_name);
}

#[test]
fn resolve_state_tracks_flags_sources_and_message_kind() {
    let state = resolve_state(FieldStateInput {
        orientation: FieldOrientation::Horizontal,
        tone: FieldTone::Muted,
        required: true,
        disabled: false,
        invalid: true,
        has_label: true,
        has_description: true,
        has_error_message: true,
        has_custom_aria_label: true,
        has_custom_error_message: false,
        has_custom_class_name: false,
    });

    assert_eq!(state.orientation_attr, "horizontal");
    assert_eq!(state.tone_attr, "muted");
    assert!(state.is_required);
    assert!(state.is_invalid);
    assert_eq!(state.message_kind_attr, "error");
    assert_eq!(state.error_source_attr, "default");
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "default");
    assert_eq!(state.data_state_attr, "invalid");
}

#[test]
fn compose_class_name_includes_state_and_custom_markers() {
    let state = resolve_state(FieldStateInput {
        orientation: FieldOrientation::Vertical,
        tone: FieldTone::Default,
        required: true,
        disabled: false,
        invalid: false,
        has_label: true,
        has_description: true,
        has_error_message: false,
        has_custom_aria_label: false,
        has_custom_error_message: false,
        has_custom_class_name: true,
    });

    let class_name = compose_class_name(Some("docs-field-custom".to_string()), state);

    for token in [
        "ui-field",
        "ui-field--orientation-vertical",
        "ui-field--tone-default",
        "ui-field--required",
        "ui-field--has-label",
        "ui-field--has-description",
        "ui-field--custom-class",
        "docs-field-custom",
    ] {
        assert!(
            class_name.contains(token),
            "composed class should include `{token}`"
        );
    }
}
