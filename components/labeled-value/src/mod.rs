mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use logic::{
    DEFAULT_ARIA_LABEL, DEFAULT_LABEL_TEXT, DEFAULT_VALUE_TEXT, LabeledValueOrientation,
    LabeledValueState, LabeledValueStateInput, LabeledValueTone,
};
pub use motion::LabeledValueMotion;
pub use view::LabeledValue;
