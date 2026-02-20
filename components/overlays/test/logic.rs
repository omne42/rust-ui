use super::*;

#[test]
fn normalize_id_base_uses_default_for_empty_values() {
    assert_eq!(
        normalize_id_base(Some("  docs-overlays  ".to_string())),
        ("docs-overlays".to_string(), true)
    );
    assert_eq!(
        normalize_id_base(Some(" \n\t ".to_string())),
        (DEFAULT_ID_BASE.into(), false)
    );
    assert_eq!(normalize_id_base(None), (DEFAULT_ID_BASE.into(), false));
}

#[test]
fn resolve_root_state_tracks_state_and_sources() {
    let state = resolve_root_state(OverlaysRootStateInput {
        open: true,
        modal: true,
        has_custom_id_base: true,
        has_custom_class_name: false,
    });

    assert_eq!(state.layer_kind_attr, "modal");
    assert_eq!(state.data_state_attr, "open");
    assert_eq!(state.id_source_attr, "custom");
    assert_eq!(state.class_source_attr, "default");
    assert!(state.is_open);
    assert!(!state.is_closed);
}

#[test]
fn compose_root_class_name_includes_markers() {
    let class_name = compose_root_class_name(
        Some("docs-overlays".to_string()),
        resolve_root_state(OverlaysRootStateInput {
            open: false,
            modal: false,
            has_custom_id_base: true,
            has_custom_class_name: true,
        }),
    );

    for token in [
        "ui-overlays",
        "ui-overlays--non-modal",
        "ui-overlays--closed",
        "ui-overlays--custom-id",
        "ui-overlays--custom-class",
        "docs-overlays",
    ] {
        assert!(
            class_name.contains(token),
            "composed class should include `{token}`"
        );
    }
}
