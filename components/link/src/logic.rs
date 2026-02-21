pub use ui_state_primitives::link::{
    LinkState, LinkStateInput, normalize_href, normalize_is_disabled, normalize_optional_text,
    resolve_rel, resolve_state, resolve_target_kind,
};
#[cfg(test)]
pub use ui_state_primitives::link::{LinkTargetKind, LinkVisualState};

pub fn compose_class_name(base_class_name: Option<String>, state: LinkState) -> String {
    let mut classes = vec![
        "ui-link".to_string(),
        state.state.as_class().to_string(),
        state.rel_source.as_class().to_string(),
    ];

    if state.opens_new_context {
        classes.push("ui-link--external".to_string());
    }

    if state.has_aria_label {
        classes.push("ui-link--with-aria-label".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-link--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
