use super::*;

#[test]
fn orientation_mappings_are_stable() {
    assert_eq!(
        DividerOrientation::Horizontal.class_name(),
        "ui-divider--horizontal"
    );
    assert_eq!(
        DividerOrientation::Vertical.class_name(),
        "ui-divider--vertical"
    );

    assert_eq!(DividerOrientation::Horizontal.as_str(), "horizontal");
    assert_eq!(DividerOrientation::Vertical.as_str(), "vertical");

    assert_eq!(DividerOrientation::Horizontal.aria_orientation(), None);
    assert_eq!(
        DividerOrientation::Vertical.aria_orientation(),
        Some("vertical")
    );
}

#[test]
fn normalize_optional_text_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("  \n\t".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  docs-divider  ".to_string())),
        Some("docs-divider".to_string())
    );
}

#[test]
fn resolve_state_tracks_orientation_flags() {
    let state = resolve_state(DividerStateInput {
        orientation: DividerOrientation::Vertical,
        has_custom_class_name: true,
    });

    assert_eq!(state.orientation, DividerOrientation::Vertical);
    assert_eq!(state.orientation_class, "ui-divider--vertical");
    assert_eq!(state.orientation_attr, "vertical");
    assert_eq!(state.aria_orientation, Some("vertical"));
    assert!(!state.is_horizontal);
    assert!(state.is_vertical);
    assert!(state.has_custom_class_name);
}
