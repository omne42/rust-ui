pub use crate::asset::AssetVariant as CoachmarkAssetVariant;
pub use crate::contextual_help::ContextualHelpVariant as CoachmarkVariant;

mod logic;
mod motion;
pub mod styles;
mod view;

pub use motion::CoachmarkMotion;
pub use view::Coachmark;

#[cfg(test)]
#[path = "../test/semantics.rs"]
mod semantics_tests;
