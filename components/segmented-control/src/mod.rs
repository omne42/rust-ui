mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use logic::{SegmentedControlOrientation, SegmentedControlSize};
pub use motion::SegmentedControlMotion;
pub use view::{SegmentedControl, SegmentedControlItem, SegmentedControlItemSpec};
