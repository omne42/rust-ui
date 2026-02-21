use super::*;

#[test]
fn logic_consumes_state_primitives_contract() {
    let state = resolve_state(AssetStateInput {
        variant: AssetVariant::Folder,
        size: ThumbnailSize::Size700,
        selected: false,
        focused: true,
        has_custom_label: false,
        has_custom_class_name: true,
        has_custom_content: false,
    });

    assert_eq!(state.variant_attr, "folder");
    assert_eq!(state.size_attr, "700");
    assert_eq!(state.data_state_attr, "focused");
    assert_eq!(state.label_source_attr, "default");
    assert_eq!(state.class_source_attr, "custom");
    assert_eq!(state.content_source_attr, "builtin-icon");
}

#[test]
fn logic_keeps_component_level_class_wiring_thin() {
    let class_name = compose_class_name(
        Some("docs-asset".to_string()),
        resolve_state(AssetStateInput {
            variant: AssetVariant::Custom,
            size: ThumbnailSize::Size600,
            selected: true,
            focused: false,
            has_custom_label: true,
            has_custom_class_name: true,
            has_custom_content: true,
        }),
    );

    for token in [
        "ui-asset",
        "ui-asset--variant-custom",
        "ui-asset--size-600",
        "ui-asset--selected",
        "ui-asset--custom-content",
        "ui-asset--custom-class",
        "docs-asset",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`",
        );
    }
}

#[test]
fn logic_resolves_default_priority_in_single_entrypoint() {
    let resolved = resolve_view_state(AssetResolvedInput {
        variant: AssetVariant::File,
        size: ThumbnailSize::Size600,
        is_selected: false,
        is_focused: false,
        label: Some("   ".to_string()),
        class_name: Some("   ".to_string()),
        has_children: false,
    });

    assert_eq!(resolved.label, DEFAULT_FILE_LABEL);
    assert_eq!(resolved.state.label_source_attr, "default");
    assert_eq!(resolved.state.class_source_attr, "default");
    assert_eq!(resolved.state.content_source_attr, "builtin-icon");
    assert!(resolved.class_name.contains("ui-asset--variant-file"));
    assert!(!resolved.class_name.contains("ui-asset--custom-class"));
}
