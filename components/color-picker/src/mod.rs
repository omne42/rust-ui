pub(crate) mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use logic::{ColorPickerState, ColorPickerStateInput, DEFAULT_ARIA_LABEL, DEFAULT_LABEL};
pub use motion::ColorPickerMotion;
pub use view::ColorPicker;

#[cfg(test)]
#[path = "../test/semantics.rs"]
mod semantics_tests;
