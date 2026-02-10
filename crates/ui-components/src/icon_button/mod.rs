mod logic;
pub mod styles;
mod view;

pub use logic::DEFAULT_ARIA_LABEL;
pub use view::IconButton;

use crate::button::ButtonSize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IconButtonStateInput {
    pub disabled: bool,
    pub size: ButtonSize,
    pub has_custom_press_handler: bool,
    pub has_explicit_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IconButtonState {
    pub is_disabled: bool,
    pub is_enabled: bool,
    pub uses_icon_size: bool,
    pub uses_custom_size: bool,
    pub has_custom_press_handler: bool,
    pub has_explicit_aria_label: bool,
    pub has_fallback_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub state_attr: &'static str,
    pub size_mode_attr: &'static str,
    pub handler_source_attr: &'static str,
    pub label_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub motion_source_attr: &'static str,
}
