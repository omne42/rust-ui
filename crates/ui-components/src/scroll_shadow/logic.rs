#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ScrollShadowEdges {
    pub top: bool,
    pub bottom: bool,
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
}
