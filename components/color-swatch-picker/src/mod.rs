pub(crate) mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use logic::{
    ColorSwatchPickerItem, ColorSwatchPickerState, ColorSwatchPickerStateInput, DEFAULT_ARIA_LABEL,
};
pub use motion::ColorSwatchPickerMotion;
pub use view::ColorSwatchPicker;

#[cfg(all(test, not(feature = "component-color_swatch_picker")))]
#[path = "../test/semantics.rs"]
mod semantics_tests;
