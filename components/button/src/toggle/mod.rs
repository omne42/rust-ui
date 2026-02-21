mod logic;
pub mod styles;
mod view;

pub use super::toggle_button::ToggleButtonMotion as ToggleMotion;
pub use super::toggle_button::ToggleButtonSize as ToggleSize;
pub use super::toggle_button::ToggleButtonVariant as ToggleVariant;
#[cfg(feature = "component-toggle_group")]
pub use logic::{DEFAULT_ARIA_LABEL, ToggleGroupOrientation, ToggleGroupSelectionMode};
#[cfg(feature = "component-toggle_group")]
pub use ui_state_primitives::toggle_button::{
    ToggleGroupItem, ToggleGroupState, ToggleGroupStateInput,
};
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
