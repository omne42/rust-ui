pub const DEFAULT_ARIA_LABEL: &str = "Date picker";
pub const DEFAULT_PLACEHOLDER: &str = "Select date";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DatePickerTone {
    #[default]
    Default,
    Quiet,
    Strong,
}

impl DatePickerTone {
    pub fn class_name(self) -> &'static str {
        match self {
            DatePickerTone::Default => "ui-date-picker--tone-default",
            DatePickerTone::Quiet => "ui-date-picker--tone-quiet",
            DatePickerTone::Strong => "ui-date-picker--tone-strong",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            DatePickerTone::Default => "default",
            DatePickerTone::Quiet => "quiet",
            DatePickerTone::Strong => "strong",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DatePickerIds {
    pub trigger_id: String,
    pub panel_id: String,
    pub calendar_id: String,
}

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

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.into(), false)
}

pub fn normalize_placeholder(value: Option<String>) -> (String, bool) {
    if let Some(placeholder) = normalize_optional_text(value) {
        return (placeholder, true);
    }

    (DEFAULT_PLACEHOLDER.into(), false)
}

pub fn normalize_month(month: u8) -> u8 {
    month.clamp(1, 12)
}

pub fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

pub fn days_in_month(year: i32, month: u8) -> u8 {
    match normalize_month(month) {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 31,
    }
}

pub fn normalize_selected_day(selected_day: Option<u8>, year: i32, month: u8) -> Option<u8> {
    selected_day.and_then(|day| {
        let max_day = days_in_month(year, month);
        (1..=max_day).contains(&day).then_some(day)
    })
}

pub fn month_name(month: u8) -> &'static str {
    match normalize_month(month) {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "January",
    }
}

pub fn resolve_trigger_label(
    year: i32,
    month: u8,
    selected_day: Option<u8>,
    placeholder: &str,
) -> String {
    match selected_day {
        Some(day) => format!("{} {:02}, {}", month_name(month), day, year),
        None => placeholder.into(),
    }
}

pub fn resolve_ids(id_base: &str) -> DatePickerIds {
    DatePickerIds {
        trigger_id: format!("{id_base}-trigger"),
        panel_id: format!("{id_base}-panel"),
        calendar_id: format!("{id_base}-calendar"),
    }
}

pub fn resolve_state(input: DatePickerStateInput) -> DatePickerState {
    let normalized_month = normalize_month(input.month);
    let selected_day = normalize_selected_day(input.selected_day, input.year, normalized_month);

    let placeholder_source_attr = if input.has_custom_placeholder {
        "custom"
    } else {
        "default"
    };
    let aria_source_attr = if input.has_custom_aria_label {
        "custom"
    } else {
        "default"
    };
    let class_source_attr = if input.has_custom_class_name {
        "custom"
    } else {
        "default"
    };
    let motion_source_attr = if input.has_custom_motion {
        "custom"
    } else {
        "default"
    };

    let has_value = selected_day.is_some();

    let data_state_attr = if input.disabled {
        "disabled"
    } else if input.open {
        "open"
    } else if has_value {
        "value"
    } else {
        "empty"
    };

    DatePickerState {
        year: input.year,
        month: normalized_month,
        selected_day,
        tone: input.tone,
        tone_class: input.tone.class_name(),
        tone_attr: input.tone.as_attr(),
        is_open: input.open,
        is_closed: !input.open,
        is_disabled: input.disabled,
        has_value,
        is_empty: !has_value,
        data_state_attr,
        placeholder_source_attr,
        aria_source_attr,
        class_source_attr,
        motion_source_attr,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_motion: input.has_custom_motion,
    }
}

#[cfg(test)]
#[path = "test/date_picker.rs"]
mod tests;
