use super::*;

#[test]
fn resolve_view_source_state_tracks_prop_vs_default_sources() {
    let defaults = resolve_view_source_state(SkeletonViewInput {
        variant: None,
        is_shimmer: None,
        has_custom_class_name: false,
    });
    assert_eq!(defaults.variant_source_attr, "default");
    assert_eq!(defaults.shimmer_source_attr, "default");

    let props = resolve_view_source_state(SkeletonViewInput {
        variant: Some(SkeletonVariant::Circle),
        is_shimmer: Some(false),
        has_custom_class_name: true,
    });
    assert_eq!(props.variant_source_attr, "prop");
    assert_eq!(props.shimmer_source_attr, "prop");
}

#[test]
fn normalize_state_input_applies_single_default_source() {
    let state_input = normalize_state_input(SkeletonViewInput {
        variant: None,
        is_shimmer: None,
        has_custom_class_name: false,
    });

    assert_eq!(state_input.variant, SkeletonVariant::default());
    assert!(state_input.is_shimmer);
    assert!(!state_input.has_custom_class_name);
}

#[test]
fn normalize_state_input_prefers_explicit_values() {
    let state_input = normalize_state_input(SkeletonViewInput {
        variant: Some(SkeletonVariant::Circle),
        is_shimmer: Some(false),
        has_custom_class_name: true,
    });

    assert_eq!(state_input.variant, SkeletonVariant::Circle);
    assert!(!state_input.is_shimmer);
    assert!(state_input.has_custom_class_name);
}
