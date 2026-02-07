mod logic;
pub mod styles;
mod view;

pub use logic::{DEFAULT_ARIA_LABEL, DateRangePickerTone};
pub use view::DateRangePicker;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DateRangePickerStateInput {
    pub tone: DateRangePickerTone,
    pub disabled: bool,
    pub has_start_value: bool,
    pub has_end_value: bool,
    pub is_invalid_range: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DateRangePickerState {
    pub tone: DateRangePickerTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub is_disabled: bool,
    pub has_start_value: bool,
    pub has_end_value: bool,
    pub has_full_value: bool,
    pub is_partial: bool,
    pub is_invalid_range: bool,
    pub data_state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}
