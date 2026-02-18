#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ScrollShadowEdges {
    pub top: bool,
    pub bottom: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScrollShadowEdgeState {
    None,
    Top,
    Bottom,
    Both,
}

impl ScrollShadowEdgeState {
    pub fn as_str(self) -> &'static str {
        match self {
            ScrollShadowEdgeState::None => "none",
            ScrollShadowEdgeState::Top => "top",
            ScrollShadowEdgeState::Bottom => "bottom",
            ScrollShadowEdgeState::Both => "both",
        }
    }

    pub const fn has_top(self) -> bool {
        matches!(
            self,
            ScrollShadowEdgeState::Top | ScrollShadowEdgeState::Both
        )
    }

    pub const fn has_bottom(self) -> bool {
        matches!(
            self,
            ScrollShadowEdgeState::Bottom | ScrollShadowEdgeState::Both
        )
    }

    pub const fn is_scrollable(self) -> bool {
        !matches!(self, ScrollShadowEdgeState::None)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScrollShadowStateInput {
    pub max_height_px: Option<u32>,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScrollShadowState {
    pub max_height_px: u32,
    pub has_custom_max_height: bool,
    pub max_height_attr: &'static str,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScrollShadowSemanticInput {
    pub edge_state: ScrollShadowEdgeState,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScrollShadowSemanticState {
    pub edge_state: ScrollShadowEdgeState,
    pub edge_state_attr: &'static str,
    pub is_scrollable: bool,
    pub scrollable_attr: Option<&'static str>,
    pub shadow_top_attr: Option<&'static str>,
    pub shadow_bottom_attr: Option<&'static str>,
}

pub const DEFAULT_MAX_HEIGHT_PX: u32 = 192;

pub fn normalize_max_height(max_height_px: Option<u32>) -> Option<u32> {
    max_height_px.filter(|value| *value > 0)
}

pub fn resolve_state(input: ScrollShadowStateInput) -> ScrollShadowState {
    let custom_max_height_px = normalize_max_height(input.max_height_px);
    let has_custom_max_height = custom_max_height_px.is_some();
    let max_height_px = custom_max_height_px.unwrap_or(DEFAULT_MAX_HEIGHT_PX);

    ScrollShadowState {
        max_height_px,
        has_custom_max_height,
        max_height_attr: if has_custom_max_height {
            "custom"
        } else {
            "default"
        },
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn resolve_edge_state(shadow_top: bool, shadow_bottom: bool) -> ScrollShadowEdgeState {
    match (shadow_top, shadow_bottom) {
        (false, false) => ScrollShadowEdgeState::None,
        (true, false) => ScrollShadowEdgeState::Top,
        (false, true) => ScrollShadowEdgeState::Bottom,
        (true, true) => ScrollShadowEdgeState::Both,
    }
}

pub fn edge_state_attr(shadow_top: bool, shadow_bottom: bool) -> &'static str {
    resolve_edge_state(shadow_top, shadow_bottom).as_str()
}

pub fn is_scrollable(shadow_top: bool, shadow_bottom: bool) -> bool {
    shadow_top || shadow_bottom
}

pub fn resolve_semantic_state(input: ScrollShadowSemanticInput) -> ScrollShadowSemanticState {
    let edge_state = input.edge_state;
    let has_top = edge_state.has_top();
    let has_bottom = edge_state.has_bottom();
    let is_scrollable_from_enum = edge_state.is_scrollable();
    let is_scrollable = is_scrollable(has_top, has_bottom);
    debug_assert_eq!(is_scrollable_from_enum, is_scrollable);

    ScrollShadowSemanticState {
        edge_state,
        edge_state_attr: edge_state_attr(has_top, has_bottom),
        is_scrollable,
        scrollable_attr: is_scrollable.then_some("true"),
        shadow_top_attr: has_top.then_some("true"),
        shadow_bottom_attr: has_bottom.then_some("true"),
    }
}

pub fn compute_scroll_shadow_edges(
    scroll_top: f64,
    client_height: f64,
    scroll_height: f64,
) -> ScrollShadowEdges {
    if client_height <= 0.0 || scroll_height <= client_height {
        return ScrollShadowEdges::default();
    }

    let scroll_top = scroll_top.max(0.0);
    let max_scroll = (scroll_height - client_height).max(0.0);
    let epsilon = 0.5;

    ScrollShadowEdges {
        top: scroll_top > epsilon,
        bottom: scroll_top < max_scroll - epsilon,
    }
}

#[cfg(test)]
mod tests {
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
}
