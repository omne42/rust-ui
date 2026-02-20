use super::*;

#[test]
fn variant_mappings_are_stable() {
    assert_eq!(
        CardVariant::Default.class_name(),
        "ui-card--variant-default"
    );
    assert_eq!(CardVariant::Muted.class_name(), "ui-card--variant-muted");
    assert_eq!(
        CardVariant::Outline.class_name(),
        "ui-card--variant-outline"
    );

    assert_eq!(CardVariant::Default.as_str(), "default");
    assert_eq!(CardVariant::Muted.as_str(), "muted");
    assert_eq!(CardVariant::Outline.as_str(), "outline");
}

#[test]
fn normalize_optional_text_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("\n\t".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  docs-card  ".to_string())),
        Some("docs-card".to_string())
    );
}

#[test]
fn resolve_state_tracks_variant_padding_and_custom_class() {
    let state = resolve_state(CardStateInput {
        variant: CardVariant::Outline,
        padded: false,
        has_custom_class_name: true,
    });

    assert_eq!(state.variant, CardVariant::Outline);
    assert_eq!(state.variant_class, "ui-card--variant-outline");
    assert_eq!(state.variant_attr, "outline");
    assert!(!state.is_padded);
    assert!(state.is_flush);
    assert!(state.has_custom_class_name);
}

#[test]
fn compose_class_name_includes_state_markers() {
    let class_name = compose_class_name(
        Some("custom".to_string()),
        resolve_state(CardStateInput {
            variant: CardVariant::Muted,
            padded: true,
            has_custom_class_name: true,
        }),
    );

    for token in [
        "ui-card",
        "ui-card--variant-muted",
        "ui-card--padded",
        "custom",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}
