mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use motion::PopoverMotion;
pub use view::Popover;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PopoverSlot {
    Root,
    Panel,
}

impl PopoverSlot {
    pub fn as_attr(self) -> &'static str {
        match self {
            PopoverSlot::Root => "popover",
            PopoverSlot::Panel => "popover-panel",
        }
    }

    pub fn base_class(self) -> &'static str {
        match self {
            PopoverSlot::Root => "ui-popover",
            PopoverSlot::Panel => "ui-popover__panel",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PopoverPartStateInput {
    pub slot: PopoverSlot,
    pub open: bool,
    pub is_modal: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_custom_placement: bool,
    pub has_on_exit_complete: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PopoverPartState {
    pub slot: PopoverSlot,
    pub slot_attr: &'static str,
    pub base_class: &'static str,
    pub state_attr: &'static str,
    pub is_open: bool,
    pub is_modal: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_custom_placement: bool,
    pub has_on_exit_complete: bool,
    pub modal_attr: &'static str,
    pub class_source_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub placement_source_attr: &'static str,
    pub modal_source_attr: &'static str,
    pub exit_source_attr: &'static str,
}
