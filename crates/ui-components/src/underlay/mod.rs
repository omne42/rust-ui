mod logic;
pub mod styles;
mod view;

pub use logic::{DEFAULT_DISABLED, DEFAULT_OPEN, DEFAULT_TRANSPARENT};
pub use view::Underlay;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnderlaySlot {
    Root,
}

impl UnderlaySlot {
    pub fn as_attr(self) -> &'static str {
        match self {
            UnderlaySlot::Root => "underlay",
        }
    }

    pub fn base_class(self) -> &'static str {
        match self {
            UnderlaySlot::Root => "ui-underlay",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnderlayPartStateInput {
    pub slot: UnderlaySlot,
    pub open: bool,
    pub transparent: bool,
    pub disabled: bool,
    pub has_on_close: bool,
    pub has_custom_transparent: bool,
    pub has_custom_disabled: bool,
    pub has_custom_close_handler: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnderlayPartState {
    pub slot: UnderlaySlot,
    pub slot_attr: &'static str,
    pub base_class: &'static str,
    pub state_attr: &'static str,
    pub tone_attr: &'static str,
    pub close_mode_attr: &'static str,
    pub open_attr: Option<&'static str>,
    pub transparent_attr: Option<&'static str>,
    pub disabled_attr: Option<&'static str>,
    pub interactive_attr: Option<&'static str>,
    pub is_open: bool,
    pub is_transparent: bool,
    pub is_disabled: bool,
    pub is_interactive: bool,
    pub has_custom_transparent: bool,
    pub has_custom_disabled: bool,
    pub has_custom_close_handler: bool,
    pub has_custom_class_name: bool,
    pub transparent_source_attr: &'static str,
    pub disabled_source_attr: &'static str,
    pub close_source_attr: &'static str,
    pub class_source_attr: &'static str,
}
