mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use motion::TrayMotion;
pub use ui_state_primitives::tray::{TrayPartState, TrayPartStateInput, TraySlot};
pub use view::Tray;
