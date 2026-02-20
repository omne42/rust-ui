use super::*;

#[test]
fn heading_level_and_tone_contracts_are_stable() {
    assert_eq!(HeadingLevel::H1.class_name(), "ui-heading--level-1");
    assert_eq!(HeadingLevel::H6.class_name(), "ui-heading--level-6");
    assert_eq!(HeadingLevel::H3.as_attr(), "3");

    assert_eq!(
        HeadingTone::Default.class_name(),
        "ui-heading--tone-default"
    );
    assert_eq!(HeadingTone::Strong.class_name(), "ui-heading--tone-strong");
    assert_eq!(HeadingTone::Muted.class_name(), "ui-heading--tone-muted");
    assert_eq!(HeadingTone::Muted.as_attr(), "muted");
}

#[test]
fn normalize_optional_text_trims_and_drops_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("\n\t  ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  docs-heading  ".to_string())),
        Some("docs-heading".to_string())
    );
}

#[test]
fn normalize_aria_label_uses_fallback_when_missing() {
    let (label, custom) = normalize_aria_label(Some("  Dialog Title  ".to_string()));
    assert_eq!(label, "Dialog Title");
    assert!(custom);

    let (label, custom) = normalize_aria_label(Some("  ".to_string()));
    assert_eq!(label, DEFAULT_ARIA_LABEL);
    assert!(!custom);
}

#[test]
fn resolve_state_tracks_level_tone_and_sources() {
    let state = resolve_state(HeadingStateInput {
        level: HeadingLevel::H4,
        tone: HeadingTone::Strong,
        truncate: true,
        has_custom_aria_label: true,
        has_custom_class_name: false,
    });

    assert_eq!(state.level_attr, "4");
    assert_eq!(state.tone_attr, "strong");
    assert!(state.is_truncated);
    assert_eq!(state.data_state_attr, "truncate");
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "default");
}

#[test]
fn compose_class_name_includes_custom_marker_and_user_class() {
    let state = resolve_state(HeadingStateInput {
        level: HeadingLevel::H2,
        tone: HeadingTone::Muted,
        truncate: false,
        has_custom_aria_label: false,
        has_custom_class_name: true,
    });

    let class_name = compose_class_name(Some("docs-heading-custom".to_string()), state);

    for token in [
        "ui-heading",
        "ui-heading--level-2",
        "ui-heading--tone-muted",
        "ui-heading--custom-class",
        "docs-heading-custom",
    ] {
        assert!(
            class_name.contains(token),
            "composed class should include `{token}`"
        );
    }
}
