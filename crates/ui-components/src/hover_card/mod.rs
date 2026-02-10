mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use motion::HoverCardMotion;
pub use view::HoverCard;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HoverCardSlot {
    Root,
    Trigger,
    Panel,
}

impl HoverCardSlot {
    pub fn as_attr(self) -> &'static str {
        match self {
            HoverCardSlot::Root => "hover-card",
            HoverCardSlot::Trigger => "hover-card-trigger",
            HoverCardSlot::Panel => "hover-card-panel",
        }
    }

    pub fn base_class(self) -> &'static str {
        match self {
            HoverCardSlot::Root => "ui-hover-card",
            HoverCardSlot::Trigger => "ui-hover-card__trigger",
            HoverCardSlot::Panel => "ui-hover-card__panel",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HoverCardPartStateInput {
    pub slot: HoverCardSlot,
    pub open: bool,
    pub disabled: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_custom_delays: bool,
    pub has_custom_id: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HoverCardPartState {
    pub slot: HoverCardSlot,
    pub slot_attr: &'static str,
    pub base_class: &'static str,
    pub state_attr: &'static str,
    pub is_open: bool,
    pub is_disabled: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_custom_delays: bool,
    pub has_custom_id: bool,
    pub class_source_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub delay_source_attr: &'static str,
    pub id_source_attr: &'static str,
}
