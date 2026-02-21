pub(crate) mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use logic::{
    DEFAULT_ARIA_LABEL, DEFAULT_ERROR_MESSAGE, FieldsetDataState, FieldsetMessageKind,
    FieldsetOrientation, FieldsetState, FieldsetStateInput, FieldsetTone,
};
pub use motion::FieldsetMotion;
pub use view::Fieldset;

#[cfg(test)]
#[path = "../test/semantics.rs"]
mod semantics;
