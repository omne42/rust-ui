use crate::date_range_picker::{DateRangePickerState, DateRangePickerStateInput};

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

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.to_string(), false)
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
    let mut classes = vec![
        "ui-date-range-picker".to_string(),
        state.tone_class.to_string(),
    ];

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
mod tests {
    use super::*;
    use crate::date_range_picker::DateRangePickerStateInput;

    #[test]
    fn tone_class_names_and_attrs_are_stable() {
        assert_eq!(
            DateRangePickerTone::Default.class_name(),
            "ui-date-range-picker--tone-default"
        );
        assert_eq!(
            DateRangePickerTone::Quiet.class_name(),
            "ui-date-range-picker--tone-quiet"
        );
        assert_eq!(
            DateRangePickerTone::Strong.class_name(),
            "ui-date-range-picker--tone-strong"
        );

        assert_eq!(DateRangePickerTone::Default.as_attr(), "default");
        assert_eq!(DateRangePickerTone::Quiet.as_attr(), "quiet");
        assert_eq!(DateRangePickerTone::Strong.as_attr(), "strong");
    }

    #[test]
    fn day_normalization_and_range_order_are_stable() {
        assert_eq!(normalize_month(0), 1);
        assert_eq!(normalize_month(22), 12);
        assert_eq!(normalize_day(Some(31), 2026, 4), None);
        assert_eq!(normalize_day(Some(30), 2026, 4), Some(30));

        assert!(is_range_invalid(Some((2026, 4, 20)), Some((2026, 4, 12))));
        assert!(!is_range_invalid(Some((2026, 4, 12)), Some((2026, 4, 20))));
    }

    #[test]
    fn resolve_state_tracks_value_shape_and_invalidity() {
        let state = resolve_state(DateRangePickerStateInput {
            tone: DateRangePickerTone::Strong,
            disabled: false,
            has_start_value: true,
            has_end_value: true,
            is_invalid_range: false,
            has_custom_aria_label: true,
            has_custom_class_name: false,
        });

        assert_eq!(state.data_state_attr, "value");
        assert_eq!(state.aria_source_attr, "custom");
        assert_eq!(state.class_source_attr, "default");
        assert!(state.has_full_value);
        assert!(!state.is_partial);
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("docs-date-range".to_string()),
            resolve_state(DateRangePickerStateInput {
                tone: DateRangePickerTone::Quiet,
                disabled: true,
                has_start_value: true,
                has_end_value: false,
                is_invalid_range: false,
                has_custom_aria_label: false,
                has_custom_class_name: true,
            }),
        );

        for token in [
            "ui-date-range-picker",
            "ui-date-range-picker--tone-quiet",
            "ui-date-range-picker--disabled",
            "ui-date-range-picker--has-start",
            "ui-date-range-picker--partial",
            "ui-date-range-picker--custom-class",
            "docs-date-range",
        ] {
            assert!(class_name.contains(token), "class should include `{token}`");
        }
    }
}
