mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use logic::SheetPlacement;
pub use motion::SheetMotion;
pub use view::Sheet;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SheetSlot {
    Root,
    Backdrop,
    Panel,
}

impl SheetSlot {
    pub fn as_attr(self) -> &'static str {
        match self {
            SheetSlot::Root => "sheet",
            SheetSlot::Backdrop => "sheet-backdrop",
            SheetSlot::Panel => "sheet-panel",
        }
    }

    pub fn base_class(self) -> &'static str {
        match self {
            SheetSlot::Root => "ui-sheet",
            SheetSlot::Backdrop => "ui-sheet__backdrop",
            SheetSlot::Panel => "ui-sheet__panel",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SheetPartStateInput {
    pub slot: SheetSlot,
    pub open: bool,
    pub placement: SheetPlacement,
    pub is_dismissable: bool,
    pub is_keyboard_dismiss_disabled: bool,
    pub has_custom_motion: bool,
    pub has_custom_aria_labelledby: bool,
    pub has_custom_aria_describedby: bool,
    pub has_on_exit_complete: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SheetPartState {
    pub slot: SheetSlot,
    pub slot_attr: &'static str,
    pub base_class: &'static str,
    pub state_attr: &'static str,
    pub placement_attr: &'static str,
    pub placement_class: &'static str,
    pub is_open: bool,
    pub is_dismissable: bool,
    pub is_keyboard_dismiss_disabled: bool,
    pub has_custom_motion: bool,
    pub has_custom_placement: bool,
    pub has_custom_aria_labelledby: bool,
    pub has_custom_aria_describedby: bool,
    pub has_on_exit_complete: bool,
    pub dismiss_attr: &'static str,
    pub keyboard_dismiss_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub placement_source_attr: &'static str,
    pub dismiss_source_attr: &'static str,
    pub keyboard_dismiss_source_attr: &'static str,
    pub aria_labelledby_source_attr: &'static str,
    pub aria_describedby_source_attr: &'static str,
    pub exit_source_attr: &'static str,
}
