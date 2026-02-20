pub use ui_state_primitives::scroll_shadow::{
    ScrollShadowEdgeState, ScrollShadowEdges, ScrollShadowSemanticInput, ScrollShadowState,
    ScrollShadowStateInput, compute_scroll_shadow_edges, resolve_edge_state,
    resolve_semantic_state, resolve_state,
};

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
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

pub fn compose_inline_style(max_height_px: u32) -> String {
    format!("--ui-scroll-shadow-max-h: {max_height_px}px;")
}

#[cfg(test)]
#[path = "test/logic.rs"]
mod tests;
