pub mod logic;
pub mod styles;
mod view;

pub use logic::{DEFAULT_ARIA_LABEL, NativeSelectSize, NativeSelectState};
pub use ui_state_primitives::native_select::{
    NativeSelectOption, NativeSelectOptionResolved, NativeSelectStateInput,
};
pub use view::NativeSelect;

#[cfg(test)]
#[path = "../test/semantics.rs"]
mod semantics_tests;
