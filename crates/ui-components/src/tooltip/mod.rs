mod logic;
mod motion;
pub mod styles;
mod view;

pub use motion::TooltipMotion;
pub use view::Tooltip;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TooltipSlot {
    Root,
    Panel,
}

impl TooltipSlot {
    pub fn as_attr(self) -> &'static str {
        match self {
            TooltipSlot::Root => "tooltip",
            TooltipSlot::Panel => "tooltip-panel",
        }
    }

    pub fn base_class(self) -> &'static str {
        match self {
            TooltipSlot::Root => "ui-tooltip",
            TooltipSlot::Panel => "ui-tooltip__panel",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TooltipPartStateInput {
    pub slot: TooltipSlot,
    pub open: bool,
    pub disabled: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_custom_delays: bool,
    pub has_custom_trigger_mode: bool,
    pub has_custom_press_behavior: bool,
    pub has_custom_id: bool,
    pub trigger_attr: &'static str,
    pub press_behavior_attr: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TooltipPartState {
    pub slot: TooltipSlot,
    pub slot_attr: &'static str,
    pub base_class: &'static str,
    pub state_attr: &'static str,
    pub is_open: bool,
    pub is_disabled: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_custom_delays: bool,
    pub has_custom_trigger_mode: bool,
    pub has_custom_press_behavior: bool,
    pub has_custom_id: bool,
    pub trigger_attr: &'static str,
    pub press_behavior_attr: &'static str,
    pub class_source_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub delay_source_attr: &'static str,
    pub trigger_source_attr: &'static str,
    pub press_source_attr: &'static str,
    pub id_source_attr: &'static str,
}
