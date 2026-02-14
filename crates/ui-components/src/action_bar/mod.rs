mod i18n;
mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use i18n::ActionBarStrings;
pub use logic::{
    DEFAULT_ARIA_LABEL, DEFAULT_CLEAR_LABEL, DEFAULT_SELECTION_EMPTY_LABEL,
    DEFAULT_SELECTION_MULTIPLE_SUFFIX, DEFAULT_SELECTION_SINGLE_LABEL,
};
pub use motion::ActionBarMotion;
pub use view::ActionBar;

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
    pub has_clear_action: bool,
    pub has_custom_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_selection_text: bool,
    pub has_custom_clear_label: bool,
    pub has_custom_motion: bool,
    pub label_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub selection_source_attr: &'static str,
    pub clear_label_source_attr: &'static str,
    pub motion_source_attr: &'static str,
}
