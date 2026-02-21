pub(crate) mod logic;
pub(crate) mod motion;
pub mod styles;
mod view;

pub use logic::{
    ColorWheelSource, ColorWheelState, ColorWheelStateInput, ColorWheelStatus,
    ColorWheelValueLabelMode, DEFAULT_ARIA_LABEL,
};
pub use motion::ColorWheelMotion;
pub use ui_headless::A11yDirection;
pub use view::ColorWheel;

#[cfg(test)]
#[path = "../test/semantics.rs"]
mod semantics_tests;
