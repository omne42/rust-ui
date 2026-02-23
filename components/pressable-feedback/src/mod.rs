mod logic;
mod motion;
pub mod styles;
mod view;

pub use logic::{DEFAULT_ARIA_LABEL, PressableFeedbackEffect, PressableFeedbackTone};
pub use motion::PressableFeedbackMotion;
pub use view::PressableFeedback;

#[cfg(test)]
#[path = "../test/semantics.rs"]
mod semantics_tests;
