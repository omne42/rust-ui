use super::*;

#[test]
fn variant_mappings_are_stable() {
    assert_eq!(
        SkeletonVariant::Rect.class_name(),
        "ui-skeleton--variant-rect"
    );
    assert_eq!(
        SkeletonVariant::Circle.class_name(),
        "ui-skeleton--variant-circle"
    );

    assert_eq!(SkeletonVariant::Rect.as_str(), "rect");
    assert_eq!(SkeletonVariant::Circle.as_str(), "circle");
}

#[test]
fn normalize_optional_text_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("  \n\t".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  docs-skeleton  ".to_string())),
        Some("docs-skeleton".to_string())
    );
}

#[test]
fn resolve_state_tracks_variant_and_animation_flags() {
    let state = resolve_state(SkeletonStateInput {
        variant: SkeletonVariant::Circle,
        is_shimmer: false,
        has_custom_class_name: true,
    });

    assert_eq!(state.variant, SkeletonVariant::Circle);
    assert_eq!(state.variant_class, "ui-skeleton--variant-circle");
    assert_eq!(state.variant_attr, "circle");
    assert_eq!(state.state_attr, "still");
    assert!(!state.has_shimmer);
    assert!(state.is_still);
    assert!(state.has_custom_class_name);
}

#[test]
fn compose_class_name_includes_state_markers() {
    let class_name = compose_class_name(
        Some("custom".to_string()),
        resolve_state(SkeletonStateInput {
            variant: SkeletonVariant::Rect,
            is_shimmer: true,
            has_custom_class_name: true,
        }),
    );

    for token in [
        "ui-skeleton",
        "ui-skeleton--variant-rect",
        "ui-skeleton--shimmer",
        "custom",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}
