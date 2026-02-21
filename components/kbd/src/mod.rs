mod logic;
pub mod styles;
mod view;

pub use logic::KbdSize;
pub use view::Kbd;

#[cfg(test)]
#[path = "../test/semantics.rs"]
mod semantics_tests;
