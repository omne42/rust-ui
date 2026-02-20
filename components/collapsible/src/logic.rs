pub use ui_state_primitives::collapsible::{
    CollapsibleState, DEFAULT_ID_BASE, DEFAULT_TITLE, normalize_id_base, normalize_optional_text,
    resolve_aria_label, resolve_state, resolve_title,
};

const _: &str = DEFAULT_ID_BASE;
const _: &str = DEFAULT_TITLE;

pub fn compose_class_name(class_name: Option<String>, state: CollapsibleState) -> String {
    let mut classes = vec![
        "ui-collapsible".to_string(),
        format!("ui-collapsible--state-{}", state.state_attr),
        format!("ui-collapsible--mode-{}", state.open_mode_attr),
    ];

    if state.has_custom_motion {
        classes.push("ui-collapsible--custom-motion".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-collapsible--custom-class".to_string());
        if let Some(class_name) = class_name {
            classes.push(class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
