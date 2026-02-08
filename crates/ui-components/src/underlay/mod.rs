mod logic;
pub mod styles;
mod view;

pub use view::Underlay;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnderlayStateInput {
    pub open: bool,
    pub transparent: bool,
    pub disabled: bool,
    pub has_on_close: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnderlayState {
    pub is_open: bool,
    pub is_transparent: bool,
    pub is_disabled: bool,
    pub is_interactive: bool,
    pub data_state_attr: &'static str,
    pub tone_attr: &'static str,
    pub close_mode_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}
