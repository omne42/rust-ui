pub(crate) mod logic;
pub mod styles;
mod view;

pub use logic::{
    A11yDirection, DEFAULT_ARIA_LABEL, DEFAULT_TEXT, DescriptionElement, DescriptionState,
    DescriptionStateInput, DescriptionTone,
};
pub use view::Description;

#[cfg(test)]
#[path = "../test/semantics.rs"]
mod semantics_tests;
