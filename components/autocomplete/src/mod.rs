mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use motion::AutocompleteMotion;
pub use view::Autocomplete;

#[cfg(test)]
#[path = "../test/semantics.rs"]
mod semantics_tests;
