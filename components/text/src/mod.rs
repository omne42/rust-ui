pub(crate) mod logic;
pub mod styles;
mod view;

pub use logic::{DEFAULT_TEXT, TextAlign, TextDirection, TextElement, TextTone, TextWeight};
pub use view::Text;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextStateInput {
    pub tone: TextTone,
    pub align: TextAlign,
    pub weight: TextWeight,
    pub disabled: bool,
    pub truncate: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub slot_kind_attr: &'static str,
    pub has_named_slot: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextState {
    pub tone: TextTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub align: TextAlign,
    pub align_class: &'static str,
    pub align_attr: &'static str,
    pub weight: TextWeight,
    pub weight_class: &'static str,
    pub weight_attr: &'static str,
    pub is_disabled: bool,
    pub is_truncated: bool,
    pub data_state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
    pub slot_kind_attr: &'static str,
    pub has_named_slot: bool,
}

#[cfg(test)]
#[path = "test/semantics.rs"]
mod semantics_tests;
