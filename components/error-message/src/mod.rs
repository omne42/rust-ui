mod logic;
mod motion;
pub mod styles;
mod view;

pub use logic::{DEFAULT_ARIA_LABEL, DEFAULT_MESSAGE, ErrorMessageElement, ErrorMessageTone};
pub use motion::ErrorMessageMotion;
pub use view::ErrorMessage;

#[cfg(test)]
#[path = "../test/semantics.rs"]
mod semantics_tests;
