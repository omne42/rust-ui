use super::*;

#[test]
fn normalize_defaults_are_explicit_and_stable() {
    assert_eq!(
        normalize_orientation(None),
        SeparatorOrientation::Horizontal
    );
    assert_eq!(
        normalize_orientation(Some(SeparatorOrientation::Vertical)),
        SeparatorOrientation::Vertical
    );

    assert!(!normalize_is_decorative(None));
    assert!(normalize_is_decorative(Some(true)));

    assert_eq!(normalize_element_type(None), SeparatorElementType::Div);
    assert_eq!(
        normalize_element_type(Some(SeparatorElementType::Hr)),
        SeparatorElementType::Hr
    );
}

#[test]
fn normalize_props_centralizes_state_input_and_class_source() {
    let normalized = normalize_props(SeparatorNormalizeInput {
        orientation: Some(SeparatorOrientation::Vertical),
        is_decorative: None,
        element_type: None,
        class_name: Some(" docs-separator ".to_string()),
    });

    assert_eq!(
        normalized.state_input.orientation,
        SeparatorOrientation::Vertical
    );
    assert_eq!(
        normalized.state_input.element_type,
        SeparatorElementType::Div
    );
    assert!(!normalized.state_input.decorative);
    assert!(normalized.state_input.has_custom_class_name);
    assert_eq!(normalized.class_name, Some("docs-separator".to_string()));
}
