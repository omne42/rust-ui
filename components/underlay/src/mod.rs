mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use motion::UnderlayMotion;
pub use ui_state_primitives::underlay::{
    DEFAULT_DISABLED, DEFAULT_OPEN, DEFAULT_TRANSPARENT, UnderlayPartState, UnderlayPartStateInput,
    UnderlaySlot,
};
pub use view::Underlay;
