mod logic;
mod motion;
pub mod styles;
mod view;

pub use logic::{CheckboxSize, CheckboxVariant};
pub use motion::CheckboxMotion;
pub use view::Checkbox;

#[cfg(test)]
#[path = "../test/semantics.rs"]
mod semantics_tests;
