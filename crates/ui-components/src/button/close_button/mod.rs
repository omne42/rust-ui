mod logic;
pub mod styles;
mod view;

pub use logic::{CloseButtonSize, CloseButtonVariant, DEFAULT_ARIA_LABEL};
pub use view::CloseButton;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CloseButtonStateInput {
    pub variant: CloseButtonVariant,
    pub size: CloseButtonSize,
    pub disabled: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_press_handler: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CloseButtonState {
    pub variant: CloseButtonVariant,
    pub size: CloseButtonSize,
    pub variant_class: &'static str,
    pub size_class: &'static str,
    pub variant_attr: &'static str,
    pub size_attr: &'static str,
    pub is_disabled: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_press_handler: bool,
    pub data_state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
}
