use super::*;

#[test]
fn size_contracts_are_stable() {
    assert_eq!(ThumbnailSize::Size50.class_name(), "ui-thumbnail--size-50");
    assert_eq!(ThumbnailSize::Size500.as_attr(), "500");
    assert_eq!(ThumbnailSize::Size1000.as_attr(), "1000");
}

#[test]
fn sanitize_background_rejects_invalid_content() {
    assert_eq!(
        sanitize_background(Some("  #ff0000  ".to_string())),
        Some("#ff0000".to_string())
    );
    assert_eq!(
        sanitize_background(Some("javascript:alert(1)".to_string())),
        None
    );
    assert_eq!(sanitize_background(Some(" ".to_string())), None);
}

#[test]
fn resolve_state_tracks_priority_and_flags() {
    let selected = resolve_state(ThumbnailStateInput {
        size: ThumbnailSize::Size600,
        cover: true,
        layer: true,
        selected: true,
        focused: true,
        has_background: true,
        has_custom_class_name: true,
    });
    assert_eq!(selected.data_state, ThumbnailDataState::Selected);
    assert_eq!(selected.data_state.as_attr(), "selected");
    assert_eq!(selected.size_class, "ui-thumbnail--size-600");
    assert_eq!(selected.size_attr, "600");
    assert!(selected.cover);
    assert!(selected.layer);
    assert!(selected.selected);
    assert!(selected.focused);
    assert!(selected.has_background);
    assert!(selected.has_custom_class_name);

    let focused = resolve_state(ThumbnailStateInput {
        size: ThumbnailSize::Size500,
        cover: false,
        layer: true,
        selected: false,
        focused: true,
        has_background: false,
        has_custom_class_name: false,
    });
    assert_eq!(focused.data_state, ThumbnailDataState::Focused);
    assert_eq!(focused.data_state.as_attr(), "focused");

    let layer = resolve_state(ThumbnailStateInput {
        size: ThumbnailSize::Size500,
        cover: false,
        layer: true,
        selected: false,
        focused: false,
        has_background: false,
        has_custom_class_name: false,
    });
    assert_eq!(layer.data_state, ThumbnailDataState::Layer);
    assert_eq!(layer.data_state.as_attr(), "layer");

    let default_state = resolve_state(ThumbnailStateInput {
        size: ThumbnailSize::Size500,
        cover: false,
        layer: false,
        selected: false,
        focused: false,
        has_background: false,
        has_custom_class_name: false,
    });
    assert_eq!(default_state.data_state, ThumbnailDataState::Default);
    assert_eq!(default_state.data_state.as_attr(), "default");
}
