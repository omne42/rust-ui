use super::*;

#[test]
fn normalize_href_trims_and_rejects_blank_values() {
    assert_eq!(
        normalize_href(" https://example.com/docs ".to_string()),
        Some("https://example.com/docs".to_string())
    );
    assert_eq!(normalize_href("   ".to_string()), None);
}

#[test]
fn normalize_optional_text_trims_and_rejects_blank_values() {
    assert_eq!(
        normalize_optional_text(Some(" external ".to_string())),
        Some("external".to_string())
    );
    assert_eq!(normalize_optional_text(Some("  ".to_string())), None);
    assert_eq!(normalize_optional_text(None), None);
}

#[test]
fn resolve_rel_adds_security_tokens_for_blank_targets() {
    assert_eq!(
        resolve_rel(Some("_blank"), Some("noopener custom".to_string())),
        Some("custom noopener noreferrer".to_string())
    );
    assert_eq!(
        resolve_rel(Some("_self"), Some("  sponsored   sponsored  ".to_string())),
        Some("sponsored".to_string())
    );
    assert_eq!(resolve_rel(None, None), None);
}

#[test]
fn resolve_state_tracks_enablement_target_and_metadata() {
    let enabled_state = resolve_state(
        false,
        Some("https://example.com"),
        Some("_blank"),
        true,
        true,
        true,
    );

    assert!(enabled_state.is_enabled);
    assert!(!enabled_state.is_disabled);
    assert!(enabled_state.has_href);
    assert_eq!(enabled_state.target_kind, "blank");
    assert!(enabled_state.opens_new_context);
    assert!(enabled_state.has_explicit_rel);
    assert!(enabled_state.has_aria_label);
    assert!(enabled_state.has_custom_class_name);

    let disabled_state = resolve_state(false, None, None, false, false, false);
    assert!(disabled_state.is_enabled);
    assert!(!disabled_state.has_href);
    assert_eq!(disabled_state.target_kind, "self");
    assert!(!disabled_state.opens_new_context);
    assert!(!disabled_state.has_explicit_rel);
}

#[test]
fn compose_class_name_includes_state_tokens() {
    let class_name = compose_class_name(
        ButtonVariant::Secondary,
        ButtonSize::Lg,
        Some("custom".to_string()),
        resolve_state(
            true,
            Some("https://example.com"),
            Some("_blank"),
            false,
            false,
            true,
        ),
    );

    for token in [
        "ui-link-button",
        "ui-button",
        "ui-link-button--disabled",
        "ui-link-button--external",
        "custom",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}
