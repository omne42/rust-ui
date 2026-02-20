use ui_logic_calendar::date_range_picker as date_range_picker_state;

pub use ui_logic_calendar::date_range_picker::{
    DEFAULT_ARIA_LABEL, DateRangePickerState, DateRangePickerStateInput, DateRangePickerTone,
};

pub const DEFAULT_START_LABEL: &str = "Start";
pub const DEFAULT_END_LABEL: &str = "End";
pub const DEFAULT_START_PLACEHOLDER: &str = "Start date";
pub const DEFAULT_END_PLACEHOLDER: &str = "End date";
pub const DEFAULT_INVALID_RANGE_MESSAGE: &str = "End date must be on or after start date.";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DateRangePickerTextInput {
    pub start_label: Option<String>,
    pub end_label: Option<String>,
    pub start_placeholder: Option<String>,
    pub end_placeholder: Option<String>,
    pub start_aria_label: Option<String>,
    pub end_aria_label: Option<String>,
    pub invalid_range_message: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DateRangePickerTextState {
    pub start_label: String,
    pub end_label: String,
    pub start_placeholder: String,
    pub end_placeholder: String,
    pub start_aria_label: String,
    pub end_aria_label: String,
    pub invalid_range_message: String,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    date_range_picker_state::normalize_optional_text(value)
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    date_range_picker_state::normalize_aria_label(value)
}

pub fn normalize_month(month: u8) -> u8 {
    date_range_picker_state::normalize_month(month)
}

pub fn days_in_month(year: i32, month: u8) -> u8 {
    date_range_picker_state::days_in_month(year, month)
}

pub fn normalize_day(day: Option<u8>, year: i32, month: u8) -> Option<u8> {
    day.and_then(|day| {
        let max_day = days_in_month(year, month);
        (1..=max_day).contains(&day).then_some(day)
    })
}

pub fn is_range_invalid(start: Option<(i32, u8, u8)>, end: Option<(i32, u8, u8)>) -> bool {
    date_range_picker_state::is_range_invalid(start, end)
}

pub fn resolve_state(input: DateRangePickerStateInput) -> DateRangePickerState {
    date_range_picker_state::resolve_state(input)
}

pub fn compose_class_name(base_class_name: Option<String>, state: DateRangePickerState) -> String {
    date_range_picker_state::compose_class_name(base_class_name, state)
}

pub fn resolve_text_state(input: DateRangePickerTextInput) -> DateRangePickerTextState {
    let start_label =
        normalize_optional_text(input.start_label).unwrap_or_else(|| DEFAULT_START_LABEL.into());
    let end_label =
        normalize_optional_text(input.end_label).unwrap_or_else(|| DEFAULT_END_LABEL.into());

    let start_placeholder = normalize_optional_text(input.start_placeholder)
        .unwrap_or_else(|| DEFAULT_START_PLACEHOLDER.into());
    let end_placeholder = normalize_optional_text(input.end_placeholder)
        .unwrap_or_else(|| DEFAULT_END_PLACEHOLDER.into());

    let start_aria_label = normalize_optional_text(input.start_aria_label)
        .unwrap_or_else(|| start_placeholder.clone());
    let end_aria_label =
        normalize_optional_text(input.end_aria_label).unwrap_or_else(|| end_placeholder.clone());

    let invalid_range_message = normalize_optional_text(input.invalid_range_message)
        .unwrap_or_else(|| DEFAULT_INVALID_RANGE_MESSAGE.into());

    DateRangePickerTextState {
        start_label,
        end_label,
        start_placeholder,
        end_placeholder,
        start_aria_label,
        end_aria_label,
        invalid_range_message,
    }
}

#[cfg(test)]
#[path = "../../test/date_range_picker/logic.rs"]
mod tests;
