use super::*;

#[test]
fn resolve_label_uses_defaults_by_variant() {
    assert_eq!(resolve_label(None, AssetVariant::File), DEFAULT_FILE_LABEL);
    assert_eq!(
        resolve_label(Some("  ".to_string()), AssetVariant::Folder),
        DEFAULT_FOLDER_LABEL
    );
    assert_eq!(
        resolve_label(None, AssetVariant::Custom),
        DEFAULT_CUSTOM_LABEL
    );
    assert_eq!(
        resolve_label(
            Some("  Featured Artwork  ".to_string()),
            AssetVariant::Custom
        ),
        "Featured Artwork"
    );
}

#[test]
fn resolve_state_tracks_sources_and_state_markers() {
    let state = resolve_state(AssetStateInput {
        variant: AssetVariant::Custom,
        size: ThumbnailSize::Size700,
        selected: true,
        focused: false,
        has_custom_label: true,
        has_custom_class_name: true,
        has_custom_content: true,
    });

    assert_eq!(state.variant_attr, "custom");
    assert_eq!(state.size_attr, "700");
    assert_eq!(state.data_state_attr, "selected");
    assert_eq!(state.label_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");
    assert_eq!(state.content_source_attr, "custom-slot");
}

#[test]
fn compose_class_name_exposes_state_markers() {
    let state = resolve_state(AssetStateInput {
        variant: AssetVariant::Folder,
        size: ThumbnailSize::Size600,
        selected: false,
        focused: true,
        has_custom_label: false,
        has_custom_class_name: true,
        has_custom_content: false,
    });

    let class_name = compose_class_name(Some("docs-asset".to_string()), state);
    for token in [
        "ui-asset",
        "ui-asset--variant-folder",
        "ui-asset--size-600",
        "ui-asset--focused",
        "ui-asset--custom-class",
        "docs-asset",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}
