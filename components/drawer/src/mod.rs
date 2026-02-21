mod logic;
pub mod motion;
pub mod protocol;
pub mod styles;
mod view;

pub use logic::{DrawerPartState, DrawerPartStateInput, DrawerPlacement, DrawerSlot};
pub use motion::DrawerMotion;
pub use view::Drawer;

#[cfg(test)]
#[path = "../test/semantics.rs"]
mod semantics_tests;
