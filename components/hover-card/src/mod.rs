mod logic;
mod motion;
pub mod styles;
mod view;

pub use motion::HoverCardMotion;
pub use ui_state_primitives::hover_card::{
    HoverCardPartState, HoverCardPartStateInput, HoverCardSlot,
};
pub use view::HoverCard;

#[cfg(test)]
#[path = "../test/semantics.rs"]
mod semantics_tests;
