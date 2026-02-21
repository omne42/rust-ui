use super::*;

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
fn resolve_is_axes_prefer_is_prefix_and_fall_back_to_legacy_alias() {
    assert!(resolve_is_disabled(Some(true), Some(false)));
    assert!(!resolve_is_disabled(Some(false), Some(true)));
    assert!(resolve_is_disabled(None, Some(true)));
    assert!(!resolve_is_disabled(None, None));

    assert!(resolve_is_invalid(Some(true), Some(false)));
    assert!(!resolve_is_invalid(Some(false), Some(true)));
    assert!(resolve_is_invalid(None, Some(true)));
    assert!(!resolve_is_invalid(None, None));

    assert_eq!(
        resolve_disabled_source(Some(true), Some(false)).as_data_attr(),
        "is-prop"
    );
    assert_eq!(
        resolve_disabled_source(None, Some(true)).as_data_attr(),
        "legacy-prop"
    );
    assert_eq!(
        resolve_disabled_source(None, None).as_data_attr(),
        "default"
    );

    assert_eq!(
        resolve_invalid_source(Some(true), Some(false)).as_data_attr(),
        "is-prop"
    );
    assert_eq!(
        resolve_invalid_source(None, Some(true)).as_data_attr(),
        "legacy-prop"
    );
    assert_eq!(resolve_invalid_source(None, None).as_data_attr(), "default");
}

#[test]
fn resolve_content_centralizes_group_defaults_and_priority() {
    let content = resolve_content(FieldGroupContentInput {
        id_base: None,
        label: Some("  Profile  ".to_string()),
        description: Some(" ".to_string()),
        aria_label: None,
        lang: Some("  zh-CN  ".to_string()),
        class_name: Some("  docs-group  ".to_string()),
    });

    assert_eq!(content.id_base, "ui-field-group");
    assert_eq!(content.label_text, "Profile");
    assert_eq!(content.description_text, "");
    assert!(content.has_label);
    assert!(!content.has_description);
    assert_eq!(content.aria_label, DEFAULT_ARIA_LABEL);
    assert!(!content.has_custom_aria_label);
    assert_eq!(content.lang.as_deref(), Some("zh-CN"));
    assert_eq!(content.class_name.as_deref(), Some("docs-group"));
    assert!(content.has_custom_class_name);
}

#[test]
fn resolve_content_prefers_trimmed_custom_values() {
    let content = resolve_content(FieldGroupContentInput {
        id_base: Some("  docs-group  ".to_string()),
        label: Some(" ".to_string()),
        description: Some("  grouped controls  ".to_string()),
        aria_label: Some("  Controls  ".to_string()),
        lang: None,
        class_name: None,
    });

    assert_eq!(content.id_base, "docs-group");
    assert_eq!(content.label_text, "");
    assert_eq!(content.description_text, "grouped controls");
    assert!(!content.has_label);
    assert!(content.has_description);
    assert_eq!(content.aria_label, "Controls");
    assert!(content.has_custom_aria_label);
    assert_eq!(content.class_name, None);
    assert!(!content.has_custom_class_name);
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
