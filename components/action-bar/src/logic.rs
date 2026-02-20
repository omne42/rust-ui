use crate::ActionBarStrings;

pub use ui_state_primitives::action_bar::{
    ActionBarPhase, ActionBarPosition, ActionBarSelectionKind, ActionBarState, ActionBarStateInput,
    normalize_aria_label, normalize_clear_label, normalize_optional_text, normalize_selection_text,
    resolve_state,
};

pub const DEFAULT_ARIA_LABEL: &str = "Actions";
pub const DEFAULT_CLEAR_LABEL: &str = "Clear selection";
pub const DEFAULT_SELECTION_EMPTY_LABEL: &str = "No items selected";
pub const DEFAULT_SELECTION_SINGLE_LABEL: &str = "1 item selected";
pub const DEFAULT_SELECTION_MULTIPLE_SUFFIX: &str = "items selected";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActionBarViewStateInput {
    pub selected_count: usize,
    pub position: ActionBarPosition,
    pub is_force_visible: bool,
    pub is_controlled_selected_count: bool,
    pub has_default_selected_count: bool,
    pub has_selected_count_change_handler: bool,
    pub has_clear_action: bool,
    pub has_custom_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_selection_text: bool,
    pub has_custom_clear_label: bool,
    pub has_custom_motion: bool,
}

pub fn normalize_default_selected_count(value: Option<usize>) -> usize {
    value.unwrap_or_default()
}

pub fn resolve_view_state(input: ActionBarViewStateInput) -> ActionBarState {
    resolve_state(ActionBarStateInput {
        selected_count: input.selected_count,
        position: input.position,
        force_visible: input.is_force_visible,
        is_controlled_selected_count: input.is_controlled_selected_count,
        has_default_selected_count: input.has_default_selected_count,
        has_selected_count_change_handler: input.has_selected_count_change_handler,
        has_clear_action: input.has_clear_action,
        has_custom_label: input.has_custom_label,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_selection_text: input.has_custom_selection_text,
        has_custom_clear_label: input.has_custom_clear_label,
        has_custom_motion: input.has_custom_motion,
    })
}

pub fn resolve_selection_text(
    selected_count: usize,
    custom_text: Option<String>,
    strings: &ActionBarStrings,
) -> String {
    if let Some(custom_text) = custom_text {
        return custom_text;
    }

    strings.selection_label(selected_count)
}

pub fn compose_class_name(base_class_name: Option<String>, state: ActionBarState) -> String {
    let mut classes = vec![
        "ui-action-bar".to_string(),
        state.position_class.into(),
        state.phase_class.into(),
        state.selection_class.into(),
    ];

    if state.has_clear_action {
        classes.push("ui-action-bar--clearable".to_string());
    }

    if state.has_custom_label {
        classes.push("ui-action-bar--label-custom".to_string());
    }

    if state.selection_source_attr == "custom" {
        classes.push("ui-action-bar--selection-custom".to_string());
    }

    if state.clear_label_source_attr == "custom" {
        classes.push("ui-action-bar--clear-label-custom".to_string());
    }

    if state.motion_source_attr == "custom" {
        classes.push("ui-action-bar--motion-custom".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-action-bar--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
