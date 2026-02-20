mod logic;
pub mod styles;
mod view;

pub use super::toggle_button::ToggleButtonMotion as ToggleMotion;
pub use super::toggle_button::ToggleButtonSize as ToggleSize;
pub use super::toggle_button::ToggleButtonVariant as ToggleVariant;
#[cfg(feature = "component-toggle_group")]
pub use logic::{DEFAULT_ARIA_LABEL, ToggleGroupOrientation, ToggleGroupSelectionMode};
pub use view::Toggle;
#[cfg(feature = "component-toggle_group")]
pub use view::ToggleGroup;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToggleStateInput {
    pub selected: bool,
    pub disabled: bool,
    pub hovered: bool,
    pub pressed_interaction: bool,
    pub focused: bool,
    pub focus_visible: bool,
    pub variant: ToggleVariant,
    pub size: ToggleSize,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_custom_aria_label: bool,
    pub has_on_pressed_change: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToggleState {
    pub is_selected: bool,
    pub is_disabled: bool,
    pub is_hovered: bool,
    pub is_pressed: bool,
    pub is_focused: bool,
    pub is_focus_visible: bool,
    pub variant: ToggleVariant,
    pub size: ToggleSize,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_custom_aria_label: bool,
    pub has_on_pressed_change: bool,
    pub state_attr: &'static str,
    pub interaction_attr: &'static str,
    pub variant_attr: &'static str,
    pub size_attr: &'static str,
    pub variant_source_attr: &'static str,
    pub size_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub handler_source_attr: &'static str,
}

#[cfg(feature = "component-toggle_group")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ToggleGroupItem {
    pub id: String,
    pub label: String,
    pub disabled: bool,
}

#[cfg(feature = "component-toggle_group")]
impl ToggleGroupItem {
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

#[cfg(feature = "component-toggle_group")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToggleGroupStateInput {
    pub orientation: ToggleGroupOrientation,
    pub selection_mode: ToggleGroupSelectionMode,
    pub disabled: bool,
    pub attached: bool,
    pub item_count: usize,
    pub selected_count: usize,
    pub disabled_item_count: usize,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[cfg(feature = "component-toggle_group")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToggleGroupState {
    pub orientation: ToggleGroupOrientation,
    pub orientation_class: &'static str,
    pub orientation_attr: &'static str,
    pub selection_mode: ToggleGroupSelectionMode,
    pub selection_mode_class: &'static str,
    pub selection_mode_attr: &'static str,
    pub is_disabled: bool,
    pub is_attached: bool,
    pub item_count: usize,
    pub selected_count: usize,
    pub disabled_item_count: usize,
    pub has_selection: bool,
    pub is_empty: bool,
    pub has_disabled_items: bool,
    pub data_state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}
