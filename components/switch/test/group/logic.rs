use super::SwitchGroupStateInput;
use super::*;

#[test]
fn orientation_and_tone_contracts_are_stable() {
    assert_eq!(
        SwitchGroupOrientation::Vertical.class_name(),
        "ui-switch-group--orientation-vertical"
    );
    assert_eq!(
        SwitchGroupOrientation::Horizontal.class_name(),
        "ui-switch-group--orientation-horizontal"
    );

    assert_eq!(SwitchGroupOrientation::Vertical.as_attr(), "vertical");
    assert_eq!(SwitchGroupOrientation::Horizontal.as_attr(), "horizontal");

    assert_eq!(
        SwitchGroupTone::Default.class_name(),
        "ui-switch-group--tone-default"
    );
    assert_eq!(
        SwitchGroupTone::Muted.class_name(),
        "ui-switch-group--tone-muted"
    );

    assert_eq!(SwitchGroupTone::Default.as_attr(), "default");
    assert_eq!(SwitchGroupTone::Muted.as_attr(), "muted");
}

#[test]
fn resolve_ids_uses_trimmed_or_fallback_base() {
    assert_eq!(
        resolve_ids("  notifications  ".to_string()),
        SwitchGroupIds {
            root_id: "notifications".to_string(),
            label_id: "notifications-label".to_string(),
            description_id: "notifications-description".to_string(),
            error_id: "notifications-error".to_string(),
        }
    );

    assert_eq!(
        resolve_ids("   ".to_string()).root_id,
        "switch-group".to_string()
    );
}

#[test]
fn normalize_helpers_trim_and_fallback() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  digest  ".to_string())),
        Some("digest".to_string())
    );

    let (label, custom_label) = normalize_label(Some("  Alerts  ".to_string()));
    assert_eq!(label, "Alerts");
    assert!(custom_label);

    let (label, custom_label) = normalize_label(None);
    assert_eq!(label, DEFAULT_LABEL);
    assert!(!custom_label);

    assert_eq!(
        normalize_description(Some("  Switch group helper  ".to_string())),
        Some("Switch group helper".to_string())
    );

    let (aria, custom_aria) = normalize_aria_label(Some("  Channels  ".to_string()));
    assert_eq!(aria, "Channels");
    assert!(custom_aria);

    let (aria, custom_aria) = normalize_aria_label(None);
    assert_eq!(aria, DEFAULT_ARIA_LABEL);
    assert!(!custom_aria);

    let (error, custom_error) = normalize_error_message(Some("  Choose one  ".to_string()), true);
    assert_eq!(error, Some("Choose one".to_string()));
    assert!(custom_error);

    let (error, custom_error) = normalize_error_message(None, true);
    assert_eq!(error, Some(DEFAULT_ERROR_MESSAGE.into()));
    assert!(!custom_error);

    let (error, custom_error) = normalize_error_message(None, false);
    assert_eq!(error, None);
    assert!(!custom_error);
}

#[test]
fn resolve_state_tracks_markers_and_sources() {
    let state = resolve_state(SwitchGroupStateInput {
        orientation: SwitchGroupOrientation::Horizontal,
        tone: SwitchGroupTone::Muted,
        required: true,
        disabled: false,
        invalid: true,
        has_label: true,
        has_description: true,
        has_error_message: true,
        has_custom_label: true,
        has_custom_aria_label: false,
        has_custom_error_message: true,
        has_custom_class_name: true,
    });

    assert_eq!(state.orientation_attr, "horizontal");
    assert_eq!(state.tone_attr, "muted");
    assert!(state.is_required);
    assert!(state.is_invalid);
    assert!(state.has_messages);
    assert!(state.shows_error);
    assert_eq!(state.message_kind_attr, "error");
    assert_eq!(state.data_state_attr, "invalid");
    assert_eq!(state.label_source_attr, "custom");
    assert_eq!(state.aria_source_attr, "default");
    assert_eq!(state.error_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");
}

#[test]
fn compose_class_name_includes_state_markers() {
    let class_name = compose_class_name(
        Some("docs-switch-group".to_string()),
        resolve_state(SwitchGroupStateInput {
            orientation: SwitchGroupOrientation::Horizontal,
            tone: SwitchGroupTone::Muted,
            required: true,
            disabled: true,
            invalid: true,
            has_label: true,
            has_description: true,
            has_error_message: true,
            has_custom_label: true,
            has_custom_aria_label: false,
            has_custom_error_message: false,
            has_custom_class_name: true,
        }),
    );

    for token in [
        "ui-switch-group",
        "ui-switch-group--orientation-horizontal",
        "ui-switch-group--tone-muted",
        "ui-switch-group--required",
        "ui-switch-group--disabled",
        "ui-switch-group--invalid",
        "ui-switch-group--has-description",
        "ui-switch-group--has-error",
        "ui-switch-group--label-custom",
        "ui-switch-group--custom-class",
        "docs-switch-group",
    ] {
        assert!(
            class_name.contains(token),
            "composed class should include `{token}`"
        );
    }
}
