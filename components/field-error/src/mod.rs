pub(crate) mod logic;
pub mod styles;
mod view;

pub use logic::{DEFAULT_ARIA_LABEL, DEFAULT_MESSAGE, FieldErrorTone};
pub use view::FieldError;

#[cfg(test)]
#[path = "../test/semantics.rs"]
mod semantics_tests;
