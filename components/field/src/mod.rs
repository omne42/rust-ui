#[cfg(feature = "field-group")]
pub mod group;
pub(crate) mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use motion::FieldMotion;
pub use ui_headless::A11yDirection;
pub use ui_state_primitives::field::{
    DEFAULT_ARIA_LABEL, DEFAULT_ERROR_MESSAGE, FieldOrientation, FieldState, FieldStateInput,
    FieldTone,
};
pub use view::Field;

#[cfg(test)]
#[path = "../test/semantics.rs"]
mod semantics;
