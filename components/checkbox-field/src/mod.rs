mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use logic::{
    CheckboxFieldIndicatorPlacement, CheckboxFieldState, CheckboxFieldStateInput,
    CheckboxFieldStatus, CheckboxFieldTone, DEFAULT_ARIA_LABEL, DEFAULT_LABEL,
};
pub use motion::CheckboxFieldMotion;
pub use view::CheckboxField;

#[cfg(test)]
#[path = "../test/semantics.rs"]
mod semantics_tests;
