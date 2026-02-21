mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use logic::{
    DEFAULT_REQUIRED_INDICATOR, DEFAULT_TEXT, LegendState, LegendStateInput, LegendTone,
};
pub use motion::LegendMotion;
pub use view::Legend;

#[cfg(test)]
#[path = "../test/semantics.rs"]
mod semantics_tests;
