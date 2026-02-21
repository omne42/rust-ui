pub use crate::asset::AssetVariant as CoachmarkAssetVariant;
pub use crate::contextual_help::ContextualHelpVariant as CoachmarkVariant;

mod logic;
mod motion;
pub mod styles;
mod view;

pub use motion::CoachmarkMotion;
pub use view::Coachmark;

#[cfg(all(test, not(feature = "component-coachmark")))]
#[path = "../test/semantics.rs"]
mod semantics_tests;
