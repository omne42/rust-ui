mod logic;
pub mod styles;
mod view;

pub use logic::DEFAULT_ARIA_LABEL;
pub use view::CircularProgress;

#[cfg(test)]
#[path = "../test/semantics.rs"]
mod semantics_tests;
