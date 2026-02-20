use super::*;

#[test]
fn class_and_attr_contracts_are_stable() {
    assert_eq!(
        AspectRatioPreset::Square.class_name(),
        "ui-aspect-ratio--ratio-square"
    );
    assert_eq!(AspectRatioPreset::UltraWide.as_attr(), "ultra-wide");
    assert_eq!(
        AspectRatioRadius::Md.class_name(),
        "ui-aspect-ratio--radius-md"
    );
    assert_eq!(AspectRatioRadius::Full.as_attr(), "full");
}

#[test]
fn normalize_helpers_trim_and_fallback() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("  \n\t".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  docs-aspect-ratio  ".to_string())),
        Some("docs-aspect-ratio".to_string())
    );

    let (custom_label, is_custom) = normalize_aria_label(Some("  Featured  ".to_string()));
    assert_eq!(custom_label, "Featured");
    assert!(is_custom);

    let (fallback_label, is_custom) = normalize_aria_label(Some(" ".to_string()));
    assert_eq!(fallback_label, DEFAULT_ARIA_LABEL);
    assert!(!is_custom);
}

#[test]
fn resolve_state_tracks_sources_and_priority_state() {
    let state = resolve_state(AspectRatioStateInput {
        ratio: AspectRatioPreset::Portrait,
        radius: AspectRatioRadius::Lg,
        bordered: true,
        fill: true,
        has_custom_aria_label: true,
        has_custom_class_name: false,
    });

    assert_eq!(state.ratio_attr, "portrait");
    assert_eq!(state.radius_attr, "lg");
    assert!(state.is_bordered);
    assert!(state.is_fill);
    assert_eq!(state.data_state_attr, "media");
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "default");
}
