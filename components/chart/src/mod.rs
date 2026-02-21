mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use logic::{ChartKind, ChartPoint, DEFAULT_ARIA_LABEL, DEFAULT_ID_BASE};
pub use motion::ChartMotion;
pub use view::Chart;

#[cfg(test)]
#[path = "../test/semantics.rs"]
mod semantics_tests;
