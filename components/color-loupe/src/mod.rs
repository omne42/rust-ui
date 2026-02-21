pub(crate) mod logic;
pub mod styles;
mod view;

pub use logic::{
    ColorLoupeOutputState, ColorLoupeState, ColorLoupeStateInput, DEFAULT_ARIA_LABEL, DEFAULT_COLOR,
};
pub use view::ColorLoupe;

#[cfg(test)]
#[path = "../test/semantics.rs"]
mod semantics_tests;
