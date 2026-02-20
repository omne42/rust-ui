use super::*;

#[test]
fn vertical_sets_aria_orientation() {
    assert_eq!(
        SeparatorOrientation::Vertical.aria_orientation(),
        Some("vertical")
    );
    assert_eq!(SeparatorOrientation::Horizontal.aria_orientation(), None);

    assert_eq!(SeparatorOrientation::Vertical.as_str(), "vertical");
    assert_eq!(SeparatorOrientation::Horizontal.as_str(), "horizontal");
}

#[test]
fn element_type_mapping_is_stable() {
    assert_eq!(SeparatorElementType::Div.as_attr(), "div");
    assert_eq!(SeparatorElementType::Hr.as_attr(), "hr");

    assert_eq!(
        SeparatorElementType::Div.class_name(),
        "ui-separator--element-div"
    );
    assert_eq!(
        SeparatorElementType::Hr.class_name(),
        "ui-separator--element-hr"
    );
}

#[test]
fn normalize_optional_text_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("  \n\t".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  docs-separator  ".to_string())),
        Some("docs-separator".to_string())
    );
}

#[test]
fn resolve_state_preserves_fields_and_flags() {
    let state = resolve_state(SeparatorStateInput {
        orientation: SeparatorOrientation::Vertical,
        element_type: SeparatorElementType::Hr,
        decorative: true,
        has_custom_class_name: true,
    });

    assert_eq!(state.orientation, SeparatorOrientation::Vertical);
    assert_eq!(state.orientation_class, "ui-separator--vertical");
    assert_eq!(state.orientation_attr, "vertical");
    assert_eq!(state.aria_orientation, Some("vertical"));

    assert_eq!(state.element_type, SeparatorElementType::Hr);
    assert_eq!(state.element_class, "ui-separator--element-hr");
    assert_eq!(state.element_attr, "hr");

    assert!(state.is_decorative);
    assert!(!state.is_semantic);
    assert_eq!(state.state_attr, "decorative");
    assert_eq!(state.state_source_attr, "props-static");
    assert_eq!(state.ui_schema_attr, SEPARATOR_UI_SCHEMA);
    assert_eq!(state.intent_attr, "separate-content");
    assert_eq!(state.action_attr, "none");
    assert_eq!(state.output_mode_attr, "snapshot");
    assert_eq!(state.streaming_fallback_attr, "snapshot");
    assert_eq!(state.output_status_attr, "verified");
    assert!(state.has_custom_class_name);
}

#[test]
fn compose_class_name_includes_state_markers() {
    let class_name = compose_class_name(
        Some("custom".to_string()),
        resolve_state(SeparatorStateInput {
            orientation: SeparatorOrientation::Horizontal,
            element_type: SeparatorElementType::Div,
            decorative: false,
            has_custom_class_name: true,
        }),
    );

    for token in [
        "ui-separator",
        "ui-separator--horizontal",
        "ui-separator--element-div",
        "ui-separator--semantic",
        "custom",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}
