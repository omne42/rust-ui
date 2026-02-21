pub(crate) mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use motion::ColorSliderMotion;
pub use ui_state_primitives::color_slider::{
    ColorSliderChannel, ColorSliderState, ColorSliderStateInput, DEFAULT_ARIA_LABEL,
};
pub use view::ColorSlider;

#[cfg(test)]
#[path = "../test/semantics.rs"]
mod semantics_tests;
