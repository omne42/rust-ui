pub(crate) mod logic;
pub mod styles;
mod view;

pub use logic::{
    ColorLoupeOutputState, ColorLoupeState, ColorLoupeStateInput, DEFAULT_ARIA_LABEL, DEFAULT_COLOR,
};
pub use view::ColorLoupe;

#[cfg(all(test, not(feature = "component-color_loupe")))]
#[path = "../test/semantics.rs"]
mod semantics_tests;
