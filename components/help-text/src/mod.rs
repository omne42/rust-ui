pub(crate) mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use logic::A11yDirection;
pub use logic::{
    DEFAULT_ARIA_LABEL, DEFAULT_ERROR_MESSAGE, HelpTextDataState, HelpTextErrorSourceAttr,
    HelpTextMessageKind, HelpTextSourceAttr, HelpTextState, HelpTextStateInput, HelpTextTone,
};
pub use motion::HelpTextMotion;
pub use view::HelpText;

#[cfg(test)]
#[path = "../test/semantics.rs"]
mod semantics_tests;
