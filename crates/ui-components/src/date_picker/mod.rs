mod i18n;
mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use i18n::DatePickerStrings;
pub use logic::{DEFAULT_ARIA_LABEL, DEFAULT_PLACEHOLDER, DatePickerIds, DatePickerTone};
pub use motion::DatePickerMotion;
pub use view::DatePicker;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DatePickerStateInput {
    pub year: i32,
    pub month: u8,
    pub selected_day: Option<u8>,
    pub tone: DatePickerTone,
    pub disabled: bool,
    pub open: bool,
    pub has_custom_placeholder: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DatePickerState {
    pub year: i32,
    pub month: u8,
    pub selected_day: Option<u8>,
    pub tone: DatePickerTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub is_open: bool,
    pub is_closed: bool,
    pub is_disabled: bool,
    pub has_value: bool,
    pub is_empty: bool,
    pub data_state_attr: &'static str,
    pub placeholder_source_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
}
