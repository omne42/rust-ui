mod logic;
pub mod styles;
mod view;

pub use logic::{AvatarGroupItemFields, AvatarGroupNormalizedInput, AvatarSize};
pub use view::{AvatarGroup, AvatarGroupItem};

#[cfg(test)]
#[path = "../test/semantics.rs"]
mod semantics_tests;
