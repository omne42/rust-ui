mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use logic::{ClearButtonVariant, DEFAULT_ARIA_LABEL};
pub use motion::ClearButtonMotion;
pub use view::ClearButton;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClearButtonStateInput {
    pub variant: ClearButtonVariant,
    pub inset: bool,
    pub disabled: bool,
    pub prevent_focus: bool,
    pub exclude_from_tab_order: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_press_handler: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClearButtonState {
    pub variant: ClearButtonVariant,
    pub variant_class: &'static str,
    pub variant_attr: &'static str,
    pub is_inset: bool,
    pub is_disabled: bool,
    pub prevent_focus: bool,
    pub exclude_from_tab_order: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_press_handler: bool,
    pub data_state_attr: &'static str,
    pub focus_mode_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
}
