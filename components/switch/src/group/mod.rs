mod logic;
pub mod styles;
mod view;

pub use logic::{
    DEFAULT_ARIA_LABEL, DEFAULT_ERROR_MESSAGE, DEFAULT_LABEL, SwitchGroupOrientation,
    SwitchGroupState, SwitchGroupStateInput, SwitchGroupTone,
};
pub use view::SwitchGroup;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SwitchGroupIds {
    pub root_id: String,
    pub label_id: String,
    pub description_id: String,
    pub error_id: String,
}
