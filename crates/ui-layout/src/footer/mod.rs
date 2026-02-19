mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use logic::{DEFAULT_ARIA_LABEL, FooterState, FooterStateInput, FooterTone};
pub use motion::FooterMotion;
pub use view::Footer;
