pub(crate) mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use logic::{ColorPickerState, ColorPickerStateInput, DEFAULT_ARIA_LABEL, DEFAULT_LABEL};
pub use motion::ColorPickerMotion;
pub use view::ColorPicker;

#[cfg(all(test, not(feature = "component-color_picker")))]
#[path = "../test/semantics.rs"]
mod semantics_tests;
