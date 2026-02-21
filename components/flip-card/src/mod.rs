pub mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use motion::FlipCardMotion;
pub use ui_state_primitives::flip_card::{
    DEFAULT_DISABLED, DEFAULT_FLIPPED, DEFAULT_HOVER_FLIP, FlipCardFlipMode, FlipCardPartState,
    FlipCardPartStateInput, FlipCardSlot,
};
pub use view::FlipCard;

#[cfg(test)]
#[path = "../test/mod.rs"]
mod tests;
