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
fn layer_kind_helpers_cover_all_variants() {
    assert_eq!(OverlaysLayerKind::Stack.class_name(), "ui-overlays--stack");
    assert_eq!(OverlaysLayerKind::Modal.class_name(), "ui-overlays--modal");
    assert_eq!(
        OverlaysLayerKind::NonModal.class_name(),
        "ui-overlays--non-modal"
    );

    assert_eq!(OverlaysLayerKind::Stack.as_attr(), "stack");
    assert_eq!(OverlaysLayerKind::Modal.as_attr(), "modal");
    assert_eq!(OverlaysLayerKind::NonModal.as_attr(), "non-modal");
}
