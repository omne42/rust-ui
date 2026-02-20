use super::*;

#[test]
fn no_shadow_when_not_scrollable() {
    assert_eq!(
        compute_scroll_shadow_edges(0.0, 100.0, 80.0),
        ScrollShadowEdges::default()
    );
}

#[test]
fn top_shadow_when_scrolled_down() {
    let edges = compute_scroll_shadow_edges(10.0, 100.0, 200.0);
    assert!(edges.top);
    assert!(edges.bottom);
}

#[test]
fn bottom_shadow_disappears_at_end() {
    let edges = compute_scroll_shadow_edges(100.0, 100.0, 200.0);
    assert!(edges.top);
    assert!(!edges.bottom);
}

#[test]
fn edge_state_and_scrollable_mapping_is_stable() {
    assert_eq!(
        resolve_edge_state(false, false),
        ScrollShadowEdgeState::None
    );
    assert_eq!(resolve_edge_state(true, false), ScrollShadowEdgeState::Top);
    assert_eq!(
        resolve_edge_state(false, true),
        ScrollShadowEdgeState::Bottom
    );
    assert_eq!(resolve_edge_state(true, true), ScrollShadowEdgeState::Both);

    assert_eq!(edge_state_attr(false, false), "none");
    assert_eq!(edge_state_attr(true, false), "top");
    assert_eq!(edge_state_attr(false, true), "bottom");
    assert_eq!(edge_state_attr(true, true), "both");

    assert!(!is_scrollable(false, false));
    assert!(is_scrollable(true, false));
    assert!(is_scrollable(false, true));
}

#[test]
fn resolve_semantic_state_maps_inputs_to_closed_marker_set() {
    let state_none = resolve_semantic_state(ScrollShadowSemanticInput {
        edge_state: ScrollShadowEdgeState::None,
    });
    assert_eq!(state_none.edge_state, ScrollShadowEdgeState::None);
    assert_eq!(state_none.edge_state_attr, "none");
    assert!(!state_none.is_scrollable);
    assert_eq!(state_none.scrollable_attr, None);
    assert_eq!(state_none.shadow_top_attr, None);
    assert_eq!(state_none.shadow_bottom_attr, None);

    let state_both = resolve_semantic_state(ScrollShadowSemanticInput {
        edge_state: ScrollShadowEdgeState::Both,
    });
    assert_eq!(state_both.edge_state, ScrollShadowEdgeState::Both);
    assert_eq!(state_both.edge_state_attr, "both");
    assert!(state_both.is_scrollable);
    assert_eq!(state_both.scrollable_attr, Some("true"));
    assert_eq!(state_both.shadow_top_attr, Some("true"));
    assert_eq!(state_both.shadow_bottom_attr, Some("true"));
}

#[test]
fn normalize_max_height_filters_invalid_values() {
    assert_eq!(normalize_max_height(None), None);
    assert_eq!(normalize_max_height(Some(0)), None);
    assert_eq!(normalize_max_height(Some(160)), Some(160));
}

#[test]
fn resolve_state_tracks_max_height_and_custom_class_flags() {
    let state = resolve_state(ScrollShadowStateInput {
        max_height_px: Some(180),
        has_custom_class_name: true,
    });

    assert_eq!(state.max_height_px, 180);
    assert!(state.has_custom_max_height);
    assert_eq!(state.max_height_attr, "custom");
    assert!(state.has_custom_class_name);
}

#[test]
fn resolve_state_uses_single_default_max_height_source() {
    let missing = resolve_state(ScrollShadowStateInput {
        max_height_px: None,
        has_custom_class_name: false,
    });
    let zero = resolve_state(ScrollShadowStateInput {
        max_height_px: Some(0),
        has_custom_class_name: false,
    });

    assert_eq!(missing.max_height_px, DEFAULT_MAX_HEIGHT_PX);
    assert_eq!(zero.max_height_px, DEFAULT_MAX_HEIGHT_PX);
    assert_eq!(missing.max_height_attr, "default");
    assert_eq!(zero.max_height_attr, "default");
    assert!(!missing.has_custom_max_height);
    assert!(!zero.has_custom_max_height);
}
