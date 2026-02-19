mod logic;
pub mod styles;
mod view;

pub use logic::AvatarSize;
#[cfg(feature = "component-avatar_group")]
pub use logic::{AvatarGroupItemFields, AvatarGroupNormalizedInput};
pub use view::Avatar;
#[cfg(feature = "component-avatar_group")]
pub use view::{AvatarGroup, AvatarGroupItem};
