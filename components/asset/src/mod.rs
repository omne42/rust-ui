pub use ui_state_primitives::asset::AssetVariant;
pub use ui_state_primitives::thumbnail::ThumbnailSize as AssetSize;

mod logic;
pub mod motion;
pub mod protocol;
pub mod styles;
mod view;

pub use motion::AssetMotion;
pub use view::Asset;

#[cfg(test)]
#[path = "../test/semantics.rs"]
mod semantics_tests;
