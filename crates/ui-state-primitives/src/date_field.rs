pub const DEFAULT_ARIA_LABEL: &str = "Date field";
pub const DEFAULT_LABEL: &str = "Date";
pub const DEFAULT_PLACEHOLDER: &str = "yyyy-mm-dd";
pub const DEFAULT_YEAR_ARIA_LABEL: &str = "Year";
pub const DEFAULT_MONTH_ARIA_LABEL: &str = "Month";
pub const DEFAULT_DAY_ARIA_LABEL: &str = "Day";
pub const DEFAULT_CLEAR_LABEL: &str = "Clear";
pub const DEFAULT_CLEAR_ARIA_LABEL: &str = "Clear date";

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

fn normalize_text_with_fallback(value: Option<String>, fallback: &str) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (fallback.trim().into(), false)
}

pub fn normalize_label(value: Option<String>, fallback: &str) -> (String, bool) {
    normalize_text_with_fallback(value, fallback)
}

pub fn normalize_placeholder(value: Option<String>, fallback: &str) -> (String, bool) {
    normalize_text_with_fallback(value, fallback)
}

pub fn normalize_aria_label(value: Option<String>, fallback: &str) -> (String, bool) {
    normalize_text_with_fallback(value, fallback)
}

pub fn normalize_year_aria_label(value: Option<String>, fallback: &str) -> (String, bool) {
    normalize_text_with_fallback(value, fallback)
}

pub fn normalize_month_aria_label(value: Option<String>, fallback: &str) -> (String, bool) {
    normalize_text_with_fallback(value, fallback)
}

pub fn normalize_day_aria_label(value: Option<String>, fallback: &str) -> (String, bool) {
    normalize_text_with_fallback(value, fallback)
}

pub fn normalize_clear_label(value: Option<String>, fallback: &str) -> (String, bool) {
    normalize_text_with_fallback(value, fallback)
}

pub fn normalize_clear_aria_label(value: Option<String>, fallback: &str) -> (String, bool) {
    normalize_text_with_fallback(value, fallback)
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

#[cfg(test)]
#[path = "test/date_field.rs"]
mod tests;
