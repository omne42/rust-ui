#[cfg(feature = "component-switch_group")]
pub mod group;
mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use motion::SwitchMotion;
pub use view::Switch;
