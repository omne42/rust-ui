mod logic;
pub mod styles;
mod view;

pub use logic::AvatarSize;
pub use view::Avatar;

#[cfg(test)]
#[path = "../test/semantics.rs"]
mod semantics_tests;
