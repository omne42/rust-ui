#[cfg(feature = "component-avatar_group")]
pub mod group;
mod logic;
pub mod styles;
mod view;

pub use logic::AvatarSize;
pub use view::Avatar;
