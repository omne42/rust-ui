pub(crate) mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use logic::{DEFAULT_ARIA_LABEL, DateInputGroupVariant};
pub use motion::DateInputGroupMotion;
pub use ui_state_primitives::date_input_group::{DateInputGroupState, DateInputGroupStateInput};
pub use view::DateInputGroup;

#[cfg(test)]
#[path = "../test/semantics.rs"]
mod semantics_tests;
