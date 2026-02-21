use crate::ActionBarStrings;
use std::borrow::Cow;

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
        Cow::Borrowed("ui-action-bar"),
        Cow::Borrowed(state.position_class),
        Cow::Borrowed(state.phase_class),
        Cow::Borrowed(state.selection_class),
    ];

    if state.has_clear_action {
        classes.push(Cow::Borrowed("ui-action-bar--clearable"));
    }

    if state.has_custom_label {
        classes.push(Cow::Borrowed("ui-action-bar--label-custom"));
    }

    if state.selection_source_attr == "custom" {
        classes.push(Cow::Borrowed("ui-action-bar--selection-custom"));
    }

    if state.clear_label_source_attr == "custom" {
        classes.push(Cow::Borrowed("ui-action-bar--clear-label-custom"));
    }

    if state.motion_source_attr == "custom" {
        classes.push(Cow::Borrowed("ui-action-bar--motion-custom"));
    }

    if state.has_custom_class_name {
        classes.push(Cow::Borrowed("ui-action-bar--custom-class"));
        if let Some(base_class_name) = base_class_name {
            classes.push(Cow::Owned(base_class_name));
        }
    }

    let mut out = String::new();
    for (index, class_name) in classes.iter().enumerate() {
        if index > 0 {
            out.push(' ');
        }
        out.push_str(class_name.as_ref());
    }
    out
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
