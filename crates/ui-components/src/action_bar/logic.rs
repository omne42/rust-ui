use crate::action_bar::ActionBarStrings;
use crate::action_bar::{
    ActionBarPhase, ActionBarPosition, ActionBarSelectionKind, ActionBarState, ActionBarStateInput,
};

pub const DEFAULT_ARIA_LABEL: &str = "Actions";
pub const DEFAULT_CLEAR_LABEL: &str = "Clear selection";
pub const DEFAULT_SELECTION_EMPTY_LABEL: &str = "No items selected";
pub const DEFAULT_SELECTION_SINGLE_LABEL: &str = "1 item selected";
pub const DEFAULT_SELECTION_MULTIPLE_SUFFIX: &str = "items selected";

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_aria_label(value: Option<String>, fallback: &str) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (fallback.to_string(), false)
}

pub fn normalize_clear_label(value: Option<String>, fallback: &str) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (fallback.to_string(), false)
}

pub fn normalize_selection_text(value: Option<String>) -> (Option<String>, bool) {
    let custom = normalize_optional_text(value);
    let has_custom = custom.is_some();
    (custom, has_custom)
}

pub fn resolve_selection_kind(selected_count: usize) -> ActionBarSelectionKind {
    match selected_count {
        0 => ActionBarSelectionKind::Empty,
        1 => ActionBarSelectionKind::Single,
        _ => ActionBarSelectionKind::Multiple,
    }
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

pub fn resolve_state(input: ActionBarStateInput) -> ActionBarState {
    let phase = if input.force_visible || input.selected_count > 0 {
        ActionBarPhase::Visible
    } else {
        ActionBarPhase::Hidden
    };

    let selection_kind = resolve_selection_kind(input.selected_count);

    let label_source_attr = if input.has_custom_label {
        "custom"
    } else {
        "default"
    };
    let class_source_attr = if input.has_custom_class_name {
        "custom"
    } else {
        "default"
    };
    let selection_source_attr = if input.has_custom_selection_text {
        "custom"
    } else {
        "default"
    };
    let clear_label_source_attr = if input.has_custom_clear_label {
        "custom"
    } else {
        "default"
    };
    let motion_source_attr = if input.has_custom_motion {
        "custom"
    } else {
        "default"
    };

    ActionBarState {
        position: input.position,
        position_class: input.position.class_name(),
        position_attr: input.position.as_attr(),
        phase,
        phase_class: phase.class_name(),
        phase_attr: phase.as_attr(),
        selection_kind,
        selection_class: selection_kind.class_name(),
        selection_attr: selection_kind.as_attr(),
        selected_count: input.selected_count,
        is_visible: matches!(phase, ActionBarPhase::Visible),
        is_hidden: matches!(phase, ActionBarPhase::Hidden),
        is_top: matches!(input.position, ActionBarPosition::Top),
        is_bottom: matches!(input.position, ActionBarPosition::Bottom),
        has_clear_action: input.has_clear_action,
        has_custom_label: input.has_custom_label,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_selection_text: input.has_custom_selection_text,
        has_custom_clear_label: input.has_custom_clear_label,
        has_custom_motion: input.has_custom_motion,
        label_source_attr,
        class_source_attr,
        selection_source_attr,
        clear_label_source_attr,
        motion_source_attr,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: ActionBarState) -> String {
    let mut classes = vec![
        "ui-action-bar".to_string(),
        state.position_class.to_string(),
        state.phase_class.to_string(),
        state.selection_class.to_string(),
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
    fn resolve_state_tracks_phase_position_and_source_markers() {
        let state = resolve_state(ActionBarStateInput {
            selected_count: 2,
            position: ActionBarPosition::Top,
            force_visible: false,
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
