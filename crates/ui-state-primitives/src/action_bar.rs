pub use crate::button::normalize_optional_text;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ActionBarPosition {
    Top,
    #[default]
    Bottom,
}

impl ActionBarPosition {
    pub fn class_name(self) -> &'static str {
        match self {
            ActionBarPosition::Top => "ui-action-bar--position-top",
            ActionBarPosition::Bottom => "ui-action-bar--position-bottom",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            ActionBarPosition::Top => "top",
            ActionBarPosition::Bottom => "bottom",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActionBarPhase {
    Visible,
    Hidden,
}

impl ActionBarPhase {
    pub fn class_name(self) -> &'static str {
        match self {
            ActionBarPhase::Visible => "ui-action-bar--state-visible",
            ActionBarPhase::Hidden => "ui-action-bar--state-hidden",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            ActionBarPhase::Visible => "visible",
            ActionBarPhase::Hidden => "hidden",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActionBarSelectionKind {
    Empty,
    Single,
    Multiple,
}

impl ActionBarSelectionKind {
    pub fn class_name(self) -> &'static str {
        match self {
            ActionBarSelectionKind::Empty => "ui-action-bar--selection-empty",
            ActionBarSelectionKind::Single => "ui-action-bar--selection-single",
            ActionBarSelectionKind::Multiple => "ui-action-bar--selection-multiple",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            ActionBarSelectionKind::Empty => "empty",
            ActionBarSelectionKind::Single => "single",
            ActionBarSelectionKind::Multiple => "multiple",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActionBarStateInput {
    pub selected_count: usize,
    pub position: ActionBarPosition,
    pub force_visible: bool,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActionBarState {
    pub position: ActionBarPosition,
    pub position_class: &'static str,
    pub position_attr: &'static str,
    pub phase: ActionBarPhase,
    pub phase_class: &'static str,
    pub phase_attr: &'static str,
    pub selection_kind: ActionBarSelectionKind,
    pub selection_class: &'static str,
    pub selection_attr: &'static str,
    pub selected_count: usize,
    pub is_visible: bool,
    pub is_hidden: bool,
    pub is_top: bool,
    pub is_bottom: bool,
    pub is_controlled_selected_count: bool,
    pub is_uncontrolled_selected_count: bool,
    pub has_clear_action: bool,
    pub has_custom_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_selection_text: bool,
    pub has_custom_clear_label: bool,
    pub has_custom_motion: bool,
    pub control_mode_attr: &'static str,
    pub selected_count_source_attr: &'static str,
    pub default_selected_count_source_attr: &'static str,
    pub selected_count_change_source_attr: &'static str,
    pub clear_action_source_attr: &'static str,
    pub label_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub selection_source_attr: &'static str,
    pub clear_label_source_attr: &'static str,
    pub motion_source_attr: &'static str,
}

pub fn normalize_aria_label(value: Option<String>, fallback: &str) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (fallback.into(), false)
}

pub fn normalize_clear_label(value: Option<String>, fallback: &str) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (fallback.into(), false)
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

    let control_mode_attr = if input.is_controlled_selected_count {
        "controlled"
    } else {
        "uncontrolled"
    };
    let selected_count_source_attr = if input.is_controlled_selected_count {
        "external"
    } else {
        "default"
    };
    let default_selected_count_source_attr = if input.has_default_selected_count {
        "provided"
    } else {
        "implicit"
    };
    let selected_count_change_source_attr = if input.has_selected_count_change_handler {
        "provided"
    } else {
        "none"
    };
    let clear_action_source_attr = if input.has_clear_action {
        "provided"
    } else {
        "none"
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
        is_controlled_selected_count: input.is_controlled_selected_count,
        is_uncontrolled_selected_count: !input.is_controlled_selected_count,
        has_clear_action: input.has_clear_action,
        has_custom_label: input.has_custom_label,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_selection_text: input.has_custom_selection_text,
        has_custom_clear_label: input.has_custom_clear_label,
        has_custom_motion: input.has_custom_motion,
        control_mode_attr,
        selected_count_source_attr,
        default_selected_count_source_attr,
        selected_count_change_source_attr,
        clear_action_source_attr,
        label_source_attr,
        class_source_attr,
        selection_source_attr,
        clear_label_source_attr,
        motion_source_attr,
    }
}

#[cfg(test)]
#[path = "test/action_bar.rs"]
mod tests;
