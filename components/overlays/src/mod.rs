mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use crate::modal::Modal;
pub use crate::overlay::{Overlay, OverlayMotion};
pub use crate::popover::{Popover, PopoverMotion};
pub use crate::tray::{Tray, TrayMotion};
pub use logic::{OverlaysRootState, OverlaysRootStateInput, resolve_root_state};
pub use motion::OverlaysMotion;
pub use view::OverlaysRoot;

#[cfg(test)]
#[path = "../test/semantics.rs"]
mod semantics_tests;
