#[cfg(feature = "component-input_group")]
pub mod group;
mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use logic::{InputLabelPlacement, InputSize, InputVariant};
pub use motion::InputMotion;
pub use view::Input;
