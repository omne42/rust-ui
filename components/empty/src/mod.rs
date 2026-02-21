mod logic;
pub mod styles;
mod view;

pub use logic::{EmptyMediaVariant, EmptyPartState, EmptyPartStateInput, EmptySlot};
pub use view::{Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyTitle};

#[cfg(test)]
#[path = "../test/semantics.rs"]
mod semantics_tests;
