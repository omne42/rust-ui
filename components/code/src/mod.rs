mod logic;
pub mod styles;
mod view;

pub use logic::CodeVariant;
pub use view::Code;

#[cfg(test)]
#[path = "../test/semantics.rs"]
mod semantics_tests;
