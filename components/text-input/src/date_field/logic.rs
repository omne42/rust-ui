use crate::text_input::date_field::{DateFieldState, DateFieldStateInput};

pub const DEFAULT_ARIA_LABEL: &str = "Date field";
pub const DEFAULT_LABEL: &str = "Date";
pub const DEFAULT_PLACEHOLDER: &str = "yyyy-mm-dd";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DateFieldTone {
    #[default]
    Default,
    Quiet,
    Strong,
}

impl DateFieldTone {
    pub fn class_name(self) -> &'static str {
        match self {
            DateFieldTone::Default => "ui-date-field--tone-default",
            DateFieldTone::Quiet => "ui-date-field--tone-quiet",
            DateFieldTone::Strong => "ui-date-field--tone-strong",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            DateFieldTone::Default => "default",
            DateFieldTone::Quiet => "quiet",
            DateFieldTone::Strong => "strong",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DateFieldIds {
    pub root_id: String,
    pub label_id: String,
    pub year_id: String,
    pub month_id: String,
    pub day_id: String,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_LABEL.into(), false)
}

pub fn normalize_placeholder(value: Option<String>) -> (String, bool) {
    if let Some(placeholder) = normalize_optional_text(value) {
        return (placeholder, true);
    }

    (DEFAULT_PLACEHOLDER.into(), false)
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.into(), false)
}

pub fn resolve_ids(id_base: &str) -> DateFieldIds {
    let base = normalize_optional_text(Some(id_base.into()))
        .unwrap_or_else(|| "ui-date-field".to_string());

    DateFieldIds {
        root_id: base.clone(),
        label_id: format!("{base}-label"),
        year_id: format!("{base}-year"),
        month_id: format!("{base}-month"),
        day_id: format!("{base}-day"),
    }
}

pub fn normalize_year(year: i32) -> i32 {
    year.clamp(1, 9999)
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

pub fn normalize_day(year: i32, month: u8, day: u8) -> u8 {
    let year = normalize_year(year);
    let month = normalize_month(month);
    day.clamp(1, days_in_month(year, month))
}

pub fn format_date_value(year: i32, month: u8, day: u8) -> String {
    let year = normalize_year(year);
    let month = normalize_month(month);
    let day = normalize_day(year, month, day);
    format!("{year:04}-{month:02}-{day:02}")
}

pub fn parse_date_value(value: &str) -> Option<(i32, u8, u8)> {
    let trimmed = value.trim();
    let (year_raw, rest) = trimmed.split_once('-')?;
    let (month_raw, day_raw) = rest.split_once('-')?;

    let year = year_raw.trim().parse::<i32>().ok()?;
    let month = month_raw.trim().parse::<u8>().ok()?;
    let day = day_raw.trim().parse::<u8>().ok()?;

    if !(1..=9999).contains(&year) || !(1..=12).contains(&month) {
        return None;
    }

    if !(1..=days_in_month(year, month)).contains(&day) {
        return None;
    }

    Some((year, month, day))
}

pub fn normalize_date_value(value: Option<String>) -> Option<String> {
    normalize_optional_text(value).and_then(|value| {
        parse_date_value(&value).map(|(year, month, day)| format_date_value(year, month, day))
    })
}

pub fn resolve_date_parts(value: Option<String>) -> (i32, u8, u8, bool) {
    if let Some(value) = normalize_date_value(value)
        && let Some((year, month, day)) = parse_date_value(&value)
    {
        return (year, month, day, true);
    }

    (1970, 1, 1, false)
}

pub fn resolve_input_placeholders(placeholder: &str) -> (String, String, String) {
    if let Some((year, rest)) = placeholder.split_once('-')
        && let Some((month, day)) = rest.split_once('-')
    {
        let year = year.trim();
        let month = month.trim();
        let day = day.trim();
        if !year.is_empty() && !month.is_empty() && !day.is_empty() {
            return (year.to_string(), month.to_string(), day.to_string());
        }
    }

    ("yyyy".to_string(), "mm".to_string(), "dd".to_string())
}

pub fn update_year_from_input(current_value: Option<String>, year_input: &str) -> Option<String> {
    let current_value = normalize_date_value(current_value);
    let trimmed = year_input.trim();
    if trimmed.is_empty() {
        return current_value;
    }

    let Ok(year) = trimmed.parse::<i32>() else {
        return current_value;
    };

    let (_, month, day, has_value) = resolve_date_parts(current_value.clone());
    let month = if has_value { month } else { 1 };
    let day = if has_value { day } else { 1 };

    Some(format_date_value(year, month, day))
}

pub fn update_month_from_input(current_value: Option<String>, month_input: &str) -> Option<String> {
    let current_value = normalize_date_value(current_value);
    let trimmed = month_input.trim();
    if trimmed.is_empty() {
        return current_value;
    }

    let Ok(month) = trimmed.parse::<u8>() else {
        return current_value;
    };

    let (year, _, day, has_value) = resolve_date_parts(current_value.clone());
    let year = if has_value { year } else { 1970 };
    let day = if has_value { day } else { 1 };

    Some(format_date_value(year, month, day))
}

pub fn update_day_from_input(current_value: Option<String>, day_input: &str) -> Option<String> {
    let current_value = normalize_date_value(current_value);
    let trimmed = day_input.trim();
    if trimmed.is_empty() {
        return current_value;
    }

    let Ok(day) = trimmed.parse::<u8>() else {
        return current_value;
    };

    let (year, month, _, has_value) = resolve_date_parts(current_value.clone());
    let year = if has_value { year } else { 1970 };
    let month = if has_value { month } else { 1 };

    Some(format_date_value(year, month, day))
}

pub fn resolve_state(input: DateFieldStateInput) -> DateFieldState {
    let label_source_attr = if input.has_custom_label {
        "custom"
    } else {
        "default"
    };
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

    let data_state_attr = if input.disabled {
        "disabled"
    } else if input.has_value {
        "value"
    } else {
        "empty"
    };

    DateFieldState {
        tone: input.tone,
        tone_class: input.tone.class_name(),
        tone_attr: input.tone.as_attr(),
        is_disabled: input.disabled,
        has_value: input.has_value,
        is_empty: !input.has_value,
        data_state_attr,
        label_source_attr,
        placeholder_source_attr,
        aria_source_attr,
        class_source_attr,
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: DateFieldState) -> String {
    let mut classes = vec!["ui-date-field".to_string(), state.tone_class.into()];

    if state.is_disabled {
        classes.push("ui-date-field--disabled".to_string());
    }
    if state.has_value {
        classes.push("ui-date-field--has-value".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-date-field--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../../test/date_field/logic.rs"]
mod tests;
