mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use logic::{NumberFormatOptions, format_static_number};
pub use motion::SlidingNumberMotion;
pub use view::{SlidingNumber, StaticNumber};
