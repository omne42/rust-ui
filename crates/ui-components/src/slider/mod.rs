mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use ui_state_primitives::slider::{SliderPhase, SliderState, SliderStateInput};

pub use motion::SliderMotion;
pub use view::Slider;
