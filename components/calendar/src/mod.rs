#[cfg(all(feature = "web", feature = "ssr"))]
compile_error!("features `web` and `ssr` are mutually exclusive for `ui-calendar`");

mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use logic::{CalendarFirstWeekday, CalendarGridCell, CalendarTone, DEFAULT_ARIA_LABEL};
pub use motion::CalendarMotion;
pub use view::Calendar;

#[cfg(test)]
#[path = "../test/semantics.rs"]
mod semantics_tests;
