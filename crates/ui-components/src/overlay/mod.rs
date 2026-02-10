mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use motion::OverlayMotion;
pub use view::Overlay;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OverlaySlot {
    Root,
    Backdrop,
    Panel,
}

impl OverlaySlot {
    pub fn as_attr(self) -> &'static str {
        match self {
            OverlaySlot::Root => "overlay",
            OverlaySlot::Backdrop => "overlay-backdrop",
            OverlaySlot::Panel => "overlay-panel",
        }
    }

    pub fn base_class(self) -> &'static str {
        match self {
            OverlaySlot::Root => "ui-overlay",
            OverlaySlot::Backdrop => "ui-overlay__backdrop",
            OverlaySlot::Panel => "ui-overlay__panel",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OverlayPartStateInput {
    pub slot: OverlaySlot,
    pub open: bool,
    pub is_dismissable: bool,
    pub is_keyboard_dismiss_disabled: bool,
    pub has_custom_role: bool,
    pub has_custom_aria_labelledby: bool,
    pub has_custom_aria_describedby: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_on_exit_complete: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OverlayPartState {
    pub slot: OverlaySlot,
    pub slot_attr: &'static str,
    pub base_class: &'static str,
    pub state_attr: &'static str,
    pub is_open: bool,
    pub is_dismissable: bool,
    pub is_keyboard_dismiss_disabled: bool,
    pub has_custom_role: bool,
    pub has_custom_aria_labelledby: bool,
    pub has_custom_aria_describedby: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_on_exit_complete: bool,
    pub dismiss_attr: &'static str,
    pub keyboard_dismiss_attr: &'static str,
    pub role_source_attr: &'static str,
    pub aria_labelledby_source_attr: &'static str,
    pub aria_describedby_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub dismiss_source_attr: &'static str,
    pub keyboard_dismiss_source_attr: &'static str,
    pub exit_source_attr: &'static str,
}
