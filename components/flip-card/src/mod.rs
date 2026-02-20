mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use logic::{DEFAULT_DISABLED, DEFAULT_FLIPPED, DEFAULT_HOVER_FLIP};
pub use motion::FlipCardMotion;
pub use view::FlipCard;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlipCardSlot {
    Root,
    Front,
    Back,
}

impl FlipCardSlot {
    pub fn as_attr(self) -> &'static str {
        match self {
            FlipCardSlot::Root => "flip-card",
            FlipCardSlot::Front => "flip-card-front",
            FlipCardSlot::Back => "flip-card-back",
        }
    }

    pub fn base_class(self) -> &'static str {
        match self {
            FlipCardSlot::Root => "ui-flip-card",
            FlipCardSlot::Front => "ui-flip-card__face ui-flip-card__front",
            FlipCardSlot::Back => "ui-flip-card__face ui-flip-card__back",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlipCardPartStateInput {
    pub slot: FlipCardSlot,
    pub disabled: bool,
    pub is_flipped: bool,
    pub flip_on_hover: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_custom_id: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlipCardPartState {
    pub slot: FlipCardSlot,
    pub slot_attr: &'static str,
    pub base_class: &'static str,
    pub state_attr: &'static str,
    pub visibility_attr: &'static str,
    pub is_disabled: bool,
    pub is_flipped: bool,
    pub flip_mode_attr: &'static str,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_custom_id: bool,
    pub class_source_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub id_source_attr: &'static str,
    pub flip_mode_source_attr: &'static str,
}

#[cfg(test)]
#[path = "../test/mod.rs"]
mod tests;
