mod logic;
pub mod motion;
pub mod styles;
mod view;

use super::{ButtonLoadingPlacement, ButtonMotion, ButtonSize, ButtonType};

pub type ActionButtonSize = ButtonSize;
pub type ActionButtonLoadingPlacement = ButtonLoadingPlacement;
pub type ActionButtonMotion = ButtonMotion;
pub type ActionButtonType = ButtonType;

#[cfg(feature = "component-action_button_group")]
pub use logic::{ActionButtonGroupDensity, ActionButtonGroupOrientation};
#[cfg(feature = "component-action_group")]
pub use logic::{ActionGroupSelectionMode, ActionGroupTone, DEFAULT_ARIA_LABEL};

#[cfg(feature = "component-action_group")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionGroupItem {
    pub id: String,
    pub label: String,
    pub disabled: bool,
}

#[cfg(feature = "component-action_group")]
impl ActionGroupItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            disabled: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

#[cfg(feature = "component-action_group")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActionGroupStateInput {
    pub tone: ActionGroupTone,
    pub selection_mode: ActionGroupSelectionMode,
    pub is_disabled: bool,
    pub is_selection_controlled: bool,
    pub item_count: usize,
    pub selected_count: usize,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[cfg(feature = "component-action_group")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActionGroupState {
    pub tone: ActionGroupTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub selection_mode: ActionGroupSelectionMode,
    pub selection_mode_class: &'static str,
    pub selection_mode_attr: &'static str,
    pub is_disabled: bool,
    pub item_count: usize,
    pub selected_count: usize,
    pub has_selection: bool,
    pub is_empty: bool,
    pub data_state_attr: &'static str,
    pub selection_source_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}

#[cfg(feature = "component-action_button_group")]
pub use motion::ActionButtonGroupMotion;

pub use view::ActionButton;
#[cfg(feature = "component-action_button_group")]
pub use view::ActionButtonGroup;
#[cfg(feature = "component-action_group")]
pub use view::ActionGroup;
