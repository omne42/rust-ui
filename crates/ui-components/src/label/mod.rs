mod logic;
mod motion;
pub mod styles;
mod view;

pub use logic::{
    DEFAULT_ARIA_LABEL, DEFAULT_REQUIRED_INDICATOR, LabelEmphasis, LabelState, LabelStateInput,
};
pub use motion::LabelMotion;
pub use view::Label;
