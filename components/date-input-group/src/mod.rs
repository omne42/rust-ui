pub(crate) mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use logic::{DEFAULT_ARIA_LABEL, DateInputGroupVariant};
pub use motion::DateInputGroupMotion;
pub use view::DateInputGroup;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DateInputGroupStateInput {
    pub variant: DateInputGroupVariant,
    pub full_width: bool,
    pub disabled: bool,
    pub invalid: bool,
    pub segmented: bool,
    pub has_prefix: bool,
    pub has_suffix: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DateInputGroupState {
    pub variant: DateInputGroupVariant,
    pub variant_class: &'static str,
    pub variant_attr: &'static str,
    pub width_class: &'static str,
    pub width_attr: &'static str,
    pub is_full_width: bool,
    pub is_disabled: bool,
    pub is_invalid: bool,
    pub is_segmented: bool,
    pub has_prefix: bool,
    pub has_suffix: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub data_state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
}
