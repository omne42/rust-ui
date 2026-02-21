mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use logic::ContextualHelpVariant;
pub use motion::ContextualHelpMotion;
pub use view::ContextualHelp;

#[cfg(all(test, not(feature = "component-contextual_help")))]
#[path = "../test/semantics.rs"]
mod semantics_tests;
