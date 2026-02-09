mod logic;
pub mod styles;
mod view;

pub use crate::active_highlight::ActiveHighlightMotion as ChartMotion;
pub use logic::{ChartKind, ChartPoint};
pub use view::Chart;

pub const DEFAULT_ID_BASE: &str = "ui-chart";
pub const DEFAULT_ARIA_LABEL: &str = "Chart";
