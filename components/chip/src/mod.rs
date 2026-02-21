mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use logic::{ChipSize, ChipVariant};
pub use motion::ChipMotion;
pub use view::Chip;

#[cfg(test)]
#[path = "../test/semantics.rs"]
mod semantics_tests;
