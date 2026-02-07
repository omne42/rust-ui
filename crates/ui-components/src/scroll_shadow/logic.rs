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
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScrollShadowStateInput {
    pub max_height_px: Option<u32>,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScrollShadowState {
    pub max_height_px: Option<u32>,
    pub has_custom_max_height: bool,
    pub max_height_attr: &'static str,
    pub has_custom_class_name: bool,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_max_height(max_height_px: Option<u32>) -> Option<u32> {
    max_height_px.filter(|value| *value > 0)
}

pub fn resolve_state(input: ScrollShadowStateInput) -> ScrollShadowState {
    let max_height_px = normalize_max_height(input.max_height_px);
    let has_custom_max_height = max_height_px.is_some();

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

pub fn compose_class_name(base_class_name: Option<String>, state: ScrollShadowState) -> String {
    let mut classes = vec!["ui-scroll-shadow".to_string()];

    if state.has_custom_max_height {
        classes.push("ui-scroll-shadow--max-height-custom".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-scroll-shadow--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
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
    fn normalize_helpers_filter_empty_inputs() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("  \n\t".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  docs-scroll-shadow  ".to_string())),
            Some("docs-scroll-shadow".to_string())
        );

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

        assert_eq!(state.max_height_px, Some(180));
        assert!(state.has_custom_max_height);
        assert_eq!(state.max_height_attr, "custom");
        assert!(state.has_custom_class_name);
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("custom".to_string()),
            resolve_state(ScrollShadowStateInput {
                max_height_px: Some(200),
                has_custom_class_name: true,
            }),
        );

        for token in [
            "ui-scroll-shadow",
            "ui-scroll-shadow--max-height-custom",
            "ui-scroll-shadow--custom-class",
            "custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
