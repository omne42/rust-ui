mod logic;
mod motion;
pub mod styles;
mod view;

pub use logic::ResizableOrientation;
pub use motion::ResizableMotion;
pub use view::Resizable;

pub const DEFAULT_ARIA_LABEL: &str = ui_state_primitives::resizable::DEFAULT_ARIA_LABEL;
pub const DEFAULT_SPLIT_PERCENT: f64 = ui_state_primitives::resizable::DEFAULT_SPLIT_PERCENT;
pub const DEFAULT_MIN_SPLIT_PERCENT: f64 =
    ui_state_primitives::resizable::DEFAULT_MIN_SPLIT_PERCENT;
pub const DEFAULT_MAX_SPLIT_PERCENT: f64 =
    ui_state_primitives::resizable::DEFAULT_MAX_SPLIT_PERCENT;
