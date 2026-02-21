pub(crate) mod logic;
pub mod styles;
mod view;

pub use logic::{DEFAULT_ARIA_LABEL, DEFAULT_REQUIRED_INDICATOR, DEFAULT_TEXT, FieldLabelTone};
pub use view::FieldLabel;

#[cfg(test)]
#[path = "../test/semantics.rs"]
mod semantics_tests;
