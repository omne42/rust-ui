use super::*;

#[test]
fn class_and_attr_contracts_are_stable() {
    assert_eq!(
        ViewBackground::Default.class_name(),
        "ui-view--background-default"
    );
    assert_eq!(ViewBackground::Accent.as_attr(), "accent");
    assert_eq!(ViewBorder::Strong.class_name(), "ui-view--border-strong");
    assert_eq!(ViewPadding::Md.as_attr(), "md");
    assert_eq!(ViewRadius::Lg.class_name(), "ui-view--radius-lg");
    assert_eq!(ViewShadow::Sm.as_attr(), "sm");
    assert_eq!(
        ViewElement::Section.class_name(),
        "ui-view--element-section"
    );
}

#[test]
fn normalize_optional_text_trims_and_drops_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("\n  \t".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  docs-view  ".to_string())),
        Some("docs-view".to_string())
    );
}

#[test]
fn normalize_aria_label_uses_fallback_when_missing() {
    let (custom_label, is_custom) = normalize_aria_label(Some("  Region  ".to_string()));
    assert_eq!(custom_label, "Region");
    assert!(is_custom);

    let (fallback_label, is_custom) = normalize_aria_label(Some("  ".to_string()));
    assert_eq!(fallback_label, DEFAULT_ARIA_LABEL);
    assert!(!is_custom);
}

#[test]
fn resolve_state_tracks_sources_and_priority_state() {
    let state = resolve_state(ViewStateInput {
        background: ViewBackground::Subtle,
        border: ViewBorder::Strong,
        padding: ViewPadding::Lg,
        radius: ViewRadius::Md,
        shadow: ViewShadow::Sm,
        element: ViewElement::Section,
        fluid: true,
        has_custom_aria_label: true,
        has_custom_class_name: false,
    });

    assert_eq!(state.background_attr, "subtle");
    assert_eq!(state.border_attr, "strong");
    assert_eq!(state.padding_attr, "lg");
    assert_eq!(state.radius_attr, "md");
    assert_eq!(state.shadow_attr, "sm");
    assert_eq!(state.element_attr, "section");
    assert_eq!(state.data_state_attr, "fluid");
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "default");
}

#[test]
fn compose_class_name_includes_custom_marker_and_user_class() {
    let state = resolve_state(ViewStateInput {
        background: ViewBackground::Accent,
        border: ViewBorder::Subtle,
        padding: ViewPadding::Md,
        radius: ViewRadius::Lg,
        shadow: ViewShadow::Md,
        element: ViewElement::Span,
        fluid: false,
        has_custom_aria_label: false,
        has_custom_class_name: true,
    });

    let class_name = compose_class_name(Some("docs-view-custom".to_string()), state);

    for token in [
        "ui-view",
        "ui-view--background-accent",
        "ui-view--border-subtle",
        "ui-view--padding-md",
        "ui-view--radius-lg",
        "ui-view--shadow-md",
        "ui-view--element-span",
        "ui-view--custom-class",
        "docs-view-custom",
    ] {
        assert!(
            class_name.contains(token),
            "composed class should include `{token}`"
        );
    }
}
