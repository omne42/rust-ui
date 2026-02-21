pub(crate) mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use logic::{ColorThumbState, ColorThumbStateInput, DEFAULT_ARIA_LABEL, DEFAULT_COLOR};
pub use motion::ColorThumbMotion;
pub use view::ColorThumb;

#[cfg(test)]
#[path = "../test/semantics.rs"]
mod semantics_tests;
