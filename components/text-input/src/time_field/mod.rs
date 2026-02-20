mod i18n;
pub(crate) mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use i18n::TimeFieldStrings;
pub use logic::{
    DEFAULT_ARIA_LABEL, DEFAULT_CLEAR_ARIA_LABEL, DEFAULT_CLEAR_LABEL, DEFAULT_HOUR_ARIA_LABEL,
    DEFAULT_LABEL, DEFAULT_MINUTE_ARIA_LABEL, DEFAULT_PLACEHOLDER, TimeFieldIds, TimeFieldState,
    TimeFieldStateInput, TimeFieldTone,
};
pub use motion::TimeFieldMotion;
pub use view::TimeField;
