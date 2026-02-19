use crate::action_bar::ActionBarStrings;

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

    if state.has_custom_selection_text {
        classes.push("ui-action-bar--selection-custom".to_string());
    }

    if state.has_custom_clear_label {
        classes.push("ui-action-bar--clear-label-custom".to_string());
    }

    if state.has_custom_motion {
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
mod tests {
    use super::*;

    #[test]
    fn normalize_optional_text_trims_and_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("\n \t".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some(" action-bar ".to_string())),
            Some("action-bar".to_string())
        );
    }

    #[test]
    fn normalize_labels_use_defaults_when_empty() {
        let (label, custom) =
            normalize_aria_label(Some("  Batch actions  ".to_string()), DEFAULT_ARIA_LABEL);
        assert_eq!(label, "Batch actions");
        assert!(custom);

        let (label, custom) = normalize_aria_label(Some(" \n ".to_string()), DEFAULT_ARIA_LABEL);
        assert_eq!(label, DEFAULT_ARIA_LABEL);
        assert!(!custom);

        let (clear_label, custom) = normalize_clear_label(None, DEFAULT_CLEAR_LABEL);
        assert_eq!(clear_label, DEFAULT_CLEAR_LABEL);
        assert!(!custom);
    }

    #[test]
    fn resolve_selection_text_supports_default_and_custom_paths() {
        let strings = ActionBarStrings::default();
        assert_eq!(
            resolve_selection_text(0, None, &strings),
            DEFAULT_SELECTION_EMPTY_LABEL
        );
        assert_eq!(
            resolve_selection_text(1, None, &strings),
            DEFAULT_SELECTION_SINGLE_LABEL
        );
        assert_eq!(
            resolve_selection_text(3, None, &strings),
            "3 items selected"
        );

        assert_eq!(
            resolve_selection_text(24, Some("24 rows selected".to_string()), &strings),
            "24 rows selected"
        );
    }

    #[test]
    fn normalize_default_selected_count_falls_back_to_zero() {
        assert_eq!(normalize_default_selected_count(None), 0);
        assert_eq!(normalize_default_selected_count(Some(7)), 7);
    }

    #[test]
    fn resolve_view_state_maps_typed_input_into_state_primitive_input() {
        let state = resolve_view_state(ActionBarViewStateInput {
            selected_count: 2,
            position: ActionBarPosition::Top,
            is_force_visible: true,
            is_controlled_selected_count: true,
            has_default_selected_count: true,
            has_selected_count_change_handler: true,
            has_clear_action: true,
            has_custom_label: false,
            has_custom_class_name: true,
            has_custom_selection_text: false,
            has_custom_clear_label: true,
            has_custom_motion: false,
        });

        assert!(state.is_visible);
        assert_eq!(state.position_attr, "top");
        assert_eq!(state.selection_attr, "multiple");
        assert_eq!(state.class_source_attr, "custom");
        assert_eq!(state.clear_label_source_attr, "custom");
    }

    #[test]
    fn resolve_state_tracks_phase_position_and_source_markers() {
        let state = resolve_state(ActionBarStateInput {
            selected_count: 2,
            position: ActionBarPosition::Top,
            force_visible: false,
            is_controlled_selected_count: true,
            has_default_selected_count: true,
            has_selected_count_change_handler: true,
            has_clear_action: true,
            has_custom_label: true,
            has_custom_class_name: false,
            has_custom_selection_text: true,
            has_custom_clear_label: false,
            has_custom_motion: true,
        });

        assert!(state.is_visible);
        assert!(state.is_top);
        assert!(!state.is_bottom);
        assert_eq!(state.phase_attr, "visible");
        assert_eq!(state.position_attr, "top");
        assert_eq!(state.selection_attr, "multiple");
        assert_eq!(state.control_mode_attr, "controlled");
        assert_eq!(state.selected_count_source_attr, "external");
        assert_eq!(state.default_selected_count_source_attr, "provided");
        assert_eq!(state.selected_count_change_source_attr, "provided");
        assert_eq!(state.clear_action_source_attr, "provided");
        assert_eq!(state.label_source_attr, "custom");
        assert_eq!(state.class_source_attr, "default");
        assert_eq!(state.selection_source_attr, "custom");
        assert_eq!(state.clear_label_source_attr, "default");
        assert_eq!(state.motion_source_attr, "custom");
    }

    #[test]
    fn compose_class_name_includes_state_and_custom_markers() {
        let state = resolve_state(ActionBarStateInput {
            selected_count: 0,
            position: ActionBarPosition::Bottom,
            force_visible: false,
            is_controlled_selected_count: false,
            has_default_selected_count: false,
            has_selected_count_change_handler: false,
            has_clear_action: false,
            has_custom_label: false,
            has_custom_class_name: true,
            has_custom_selection_text: false,
            has_custom_clear_label: true,
            has_custom_motion: true,
        });

        let class_name = compose_class_name(Some("docs-action-bar".to_string()), state);
        assert!(class_name.contains("ui-action-bar"));
        assert!(class_name.contains("ui-action-bar--state-hidden"));
        assert!(class_name.contains("ui-action-bar--position-bottom"));
        assert!(class_name.contains("ui-action-bar--selection-empty"));
        assert!(class_name.contains("ui-action-bar--custom-class"));
        assert!(class_name.contains("ui-action-bar--clear-label-custom"));
        assert!(class_name.contains("ui-action-bar--motion-custom"));
        assert!(class_name.contains("docs-action-bar"));
    }
}
