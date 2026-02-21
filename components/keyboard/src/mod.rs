mod logic;
pub mod styles;
mod view;

pub use logic::{DEFAULT_ARIA_LABEL, KeyboardTone};
pub use view::Keyboard;

#[cfg(test)]
#[path = "../test/semantics.rs"]
mod semantics_tests;
