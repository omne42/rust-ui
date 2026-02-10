mod logic;
pub mod styles;
mod view;

pub use logic::DEFAULT_ARIA_LABEL;
pub use view::PickerButton;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PickerButtonStateInput {
    pub quiet: bool,
    pub invalid: bool,
    pub disabled: bool,
    pub forced_active: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_press_handler: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PickerButtonState {
    pub is_quiet: bool,
    pub is_invalid: bool,
    pub is_disabled: bool,
    pub is_forced_active: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_press_handler: bool,
    pub quiet_attr: &'static str,
    pub invalid_attr: &'static str,
    pub disabled_attr: &'static str,
    pub active_mode_attr: &'static str,
    pub data_state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub handler_source_attr: &'static str,
}
