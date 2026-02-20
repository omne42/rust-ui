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
fn normalize_is_disabled_uses_is_prefixed_or_default() {
    assert_eq!(
        normalize_is_disabled(Some(true)),
        (true, LinkDisabledSource::IsProp)
    );
    assert_eq!(
        normalize_is_disabled(None),
        (false, LinkDisabledSource::Default)
    );
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
    let enabled_state = resolve_state(LinkStateInput {
        is_disabled: false,
        has_href: true,
        target: Some("_blank"),
        has_explicit_rel: true,
        has_aria_label: true,
        has_custom_class_name: true,
    });

    assert!(enabled_state.is_enabled);
    assert!(!enabled_state.is_disabled);
    assert!(enabled_state.has_href);
    assert_eq!(enabled_state.target_kind, "blank");
    assert!(enabled_state.opens_new_context);
    assert!(enabled_state.has_explicit_rel);
    assert!(enabled_state.has_aria_label);
    assert!(enabled_state.has_custom_class_name);
    assert_eq!(enabled_state.state_attr, "enabled");
    assert_eq!(enabled_state.rel_source_attr, "provided");

    let missing_state = resolve_state(LinkStateInput {
        is_disabled: false,
        has_href: false,
        target: None,
        has_explicit_rel: false,
        has_aria_label: false,
        has_custom_class_name: false,
    });
    assert_eq!(missing_state.state_attr, "missing-href");
    assert_eq!(missing_state.target_kind, "self");
    assert_eq!(missing_state.rel_source_attr, "auto");
}

#[test]
fn compose_class_name_includes_state_tokens() {
    let class_name = compose_class_name(
        Some("custom".to_string()),
        resolve_state(LinkStateInput {
            is_disabled: false,
            has_href: true,
            target: Some("_blank"),
            has_explicit_rel: false,
            has_aria_label: true,
            has_custom_class_name: true,
        }),
    );

    for token in [
        "ui-link",
        "ui-link--enabled",
        "ui-link--rel-auto",
        "ui-link--external",
        "ui-link--with-aria-label",
        "ui-link--custom-class",
        "custom",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}
