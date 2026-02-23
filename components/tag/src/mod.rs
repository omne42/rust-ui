#[cfg(feature = "tag-group")]
pub mod group;
mod logic;
pub mod styles;
mod view;

pub use logic::{
    DEFAULT_REMOVE_ARIA_LABEL, TagInteractivityMode, TagSize, TagState, TagStateInput, TagVariant,
};
pub use view::Tag;
