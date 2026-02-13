mod logic;
mod motion;
pub mod styles;
mod view;

pub use logic::ResizableOrientation;
pub use motion::ResizableMotion;
pub use view::Resizable;

pub const DEFAULT_ARIA_LABEL: &str = "Resizable panels";
pub const DEFAULT_SPLIT_PERCENT: f64 = 50.0;
pub const DEFAULT_MIN_SPLIT_PERCENT: f64 = 10.0;
pub const DEFAULT_MAX_SPLIT_PERCENT: f64 = 90.0;
