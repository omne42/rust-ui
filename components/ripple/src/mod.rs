mod logic;
mod motion;
pub mod styles;
mod view;

pub use motion::{RippleMotion, trigger_ripple, trigger_ripple_at};
pub use view::MotionRipple;
