use crate::field_group::*;

#[test]
fn normalize_helpers_trim_and_fallback() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("  ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  Group  ".to_string())),
        Some("Group".to_string())
    );

    assert_eq!(normalize_id_base(None), "ui-field-group");
    assert_eq!(normalize_id_base(Some("  docs  ".to_string())), "docs");

    assert_eq!(
        normalize_aria_label(Some("  Controls  ".to_string())),
        ("Controls".to_string(), true)
    );
    assert_eq!(
        normalize_aria_label(None),
        (DEFAULT_ARIA_LABEL.into(), false)
    );
}

#[test]
fn resolve_state_tracks_semantic_markers() {
    let state = resolve_state(FieldGroupStateInput {
        orientation: FieldGroupOrientation::Horizontal,
        density: FieldGroupDensity::Compact,
        disabled: true,
        invalid: true,
        has_label: false,
        has_description: true,
        has_custom_aria_label: false,
        has_custom_class_name: true,
    });

    assert_eq!(
        state.orientation_class,
        "ui-field-group--orientation-horizontal"
    );
    assert_eq!(state.orientation_attr, "horizontal");
    assert_eq!(state.density_class, "ui-field-group--density-compact");
    assert_eq!(state.density_attr, "compact");
    assert_eq!(state.state_attr, "invalid-disabled");
    assert_eq!(state.label_attr, "absent");
    assert_eq!(state.description_attr, "present");
    assert_eq!(state.aria_source_attr, "default");
    assert_eq!(state.class_source_attr, "custom");
}

#[test]
fn compose_class_name_includes_state_classes() {
    let state = resolve_state(FieldGroupStateInput {
        orientation: FieldGroupOrientation::Vertical,
        density: FieldGroupDensity::Comfortable,
        disabled: false,
        invalid: true,
        has_label: true,
        has_description: false,
        has_custom_aria_label: true,
        has_custom_class_name: true,
    });

    let class_name = compose_class_name(Some("docs-field-group".to_string()), state);

    for expected in [
        "ui-field-group",
        "ui-field-group--orientation-vertical",
        "ui-field-group--density-comfortable",
        "ui-field-group--invalid",
        "ui-field-group--has-label",
        "ui-field-group--no-description",
        "ui-field-group--custom-class",
        "docs-field-group",
    ] {
        assert!(class_name.contains(expected));
    }
}
