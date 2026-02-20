pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub const DEFAULT_ARIA_LABEL: &str = "Date range picker";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DateRangePickerTone {
    #[default]
    Default,
    Quiet,
    Strong,
}

impl DateRangePickerTone {
    pub fn class_name(self) -> &'static str {
        match self {
            DateRangePickerTone::Default => "ui-date-range-picker--tone-default",
            DateRangePickerTone::Quiet => "ui-date-range-picker--tone-quiet",
            DateRangePickerTone::Strong => "ui-date-range-picker--tone-strong",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            DateRangePickerTone::Default => "default",
            DateRangePickerTone::Quiet => "quiet",
            DateRangePickerTone::Strong => "strong",
        }
    }
}

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

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.into(), false)
}

pub fn normalize_month(month: u8) -> u8 {
    crate::calendar::normalize_month(month)
}

pub fn days_in_month(year: i32, month: u8) -> u8 {
    crate::calendar::days_in_month(year, month)
}

pub fn normalize_day(day: Option<u8>, year: i32, month: u8) -> Option<u8> {
    day.and_then(|day| {
        let max_day = days_in_month(year, month);
        (1..=max_day).contains(&day).then_some(day)
    })
}

fn date_rank(year: i32, month: u8, day: u8) -> i64 {
    (year as i64 * 372) + (normalize_month(month) as i64 * 31) + day as i64
}

pub fn is_range_invalid(start: Option<(i32, u8, u8)>, end: Option<(i32, u8, u8)>) -> bool {
    match (start, end) {
        (Some((start_year, start_month, start_day)), Some((end_year, end_month, end_day))) => {
            date_rank(start_year, start_month, start_day) > date_rank(end_year, end_month, end_day)
        }
        _ => false,
    }
}

pub fn resolve_state(input: DateRangePickerStateInput) -> DateRangePickerState {
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

    let has_full_value = input.has_start_value && input.has_end_value;
    let is_partial = input.has_start_value ^ input.has_end_value;

    let data_state_attr = if input.disabled {
        "disabled"
    } else if input.is_invalid_range {
        "invalid"
    } else if has_full_value {
        "value"
    } else if is_partial {
        "partial"
    } else {
        "empty"
    };

    DateRangePickerState {
        tone: input.tone,
        tone_class: input.tone.class_name(),
        tone_attr: input.tone.as_attr(),
        is_disabled: input.disabled,
        has_start_value: input.has_start_value,
        has_end_value: input.has_end_value,
        has_full_value,
        is_partial,
        is_invalid_range: input.is_invalid_range,
        data_state_attr,
        aria_source_attr,
        class_source_attr,
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: DateRangePickerState) -> String {
    let mut classes = vec!["ui-date-range-picker".to_string(), state.tone_class.into()];

    if state.is_disabled {
        classes.push("ui-date-range-picker--disabled".to_string());
    }
    if state.has_start_value {
        classes.push("ui-date-range-picker--has-start".to_string());
    }
    if state.has_end_value {
        classes.push("ui-date-range-picker--has-end".to_string());
    }
    if state.has_full_value {
        classes.push("ui-date-range-picker--has-full-value".to_string());
    }
    if state.is_partial {
        classes.push("ui-date-range-picker--partial".to_string());
    }
    if state.is_invalid_range {
        classes.push("ui-date-range-picker--invalid-range".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-date-range-picker--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "test/date_range_picker.rs"]
mod tests;
