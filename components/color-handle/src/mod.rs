pub(crate) mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use logic::{ColorHandleState, ColorHandleStateInput, DEFAULT_ARIA_LABEL};
pub use motion::ColorHandleMotion;
pub use view::ColorHandle;

#[cfg(test)]
#[path = "../test/semantics.rs"]
mod semantics_tests;
