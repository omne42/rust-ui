use super::*;

#[test]
fn footer_tone_contract_is_stable() {
    assert_eq!(FooterTone::Default.class_name(), "ui-footer--tone-default");
    assert_eq!(FooterTone::Muted.class_name(), "ui-footer--tone-muted");

    assert_eq!(FooterTone::Default.as_attr(), "default");
    assert_eq!(FooterTone::Muted.as_attr(), "muted");
}

#[test]
fn normalize_optional_text_trims_and_drops_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("\n\t  ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  docs-footer  ".to_string())),
        Some("docs-footer".to_string())
    );
}

#[test]
fn normalize_aria_label_uses_fallback_when_missing() {
    let (label, custom) = normalize_aria_label(Some("  Dialog Footer  ".to_string()));
    assert_eq!(label, "Dialog Footer");
    assert!(custom);

    let (label, custom) = normalize_aria_label(Some("  ".to_string()));
    assert_eq!(label, DEFAULT_ARIA_LABEL);
    assert!(!custom);
}

#[test]
fn resolve_state_tracks_flags_and_sources() {
    let state = resolve_state(FooterStateInput {
        tone: FooterTone::Muted,
        bordered: true,
        has_custom_aria_label: true,
        has_custom_class_name: false,
    });

    assert_eq!(state.tone_attr, "muted");
    assert!(state.is_bordered);
    assert_eq!(state.data_state_attr, "muted-bordered");
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "default");
}

#[test]
fn compose_class_name_includes_custom_marker_and_user_class() {
    let state = resolve_state(FooterStateInput {
        tone: FooterTone::Default,
        bordered: true,
        has_custom_aria_label: false,
        has_custom_class_name: true,
    });

    let class_name = compose_class_name(Some("docs-footer-custom".to_string()), state);

    for token in [
        "ui-footer",
        "ui-footer--tone-default",
        "ui-footer--bordered",
        "ui-footer--custom-class",
        "docs-footer-custom",
    ] {
        assert!(
            class_name.contains(token),
            "composed class should include `{token}`"
        );
    }
}
