use ui_state_primitives::date_range_picker as date_range_picker_state;

pub use ui_state_primitives::date_range_picker::{
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
    let start_label = normalize_optional_text(input.start_label)
        .unwrap_or_else(|| DEFAULT_START_LABEL.to_string());
    let end_label =
        normalize_optional_text(input.end_label).unwrap_or_else(|| DEFAULT_END_LABEL.to_string());

    let start_placeholder = normalize_optional_text(input.start_placeholder)
        .unwrap_or_else(|| DEFAULT_START_PLACEHOLDER.to_string());
    let end_placeholder = normalize_optional_text(input.end_placeholder)
        .unwrap_or_else(|| DEFAULT_END_PLACEHOLDER.to_string());

    let start_aria_label = normalize_optional_text(input.start_aria_label)
        .unwrap_or_else(|| start_placeholder.clone());
    let end_aria_label =
        normalize_optional_text(input.end_aria_label).unwrap_or_else(|| end_placeholder.clone());

    let invalid_range_message = normalize_optional_text(input.invalid_range_message)
        .unwrap_or_else(|| DEFAULT_INVALID_RANGE_MESSAGE.to_string());

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
mod tests {
    use super::*;

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

    #[test]
    fn resolve_text_state_uses_single_logic_fallback_source() {
        let text = resolve_text_state(DateRangePickerTextInput {
            start_label: Some("  ".to_string()),
            end_label: None,
            start_placeholder: Some("Begin".to_string()),
            end_placeholder: None,
            start_aria_label: None,
            end_aria_label: Some("Finish".to_string()),
            invalid_range_message: None,
        });

        assert_eq!(text.start_label, DEFAULT_START_LABEL);
        assert_eq!(text.end_label, DEFAULT_END_LABEL);
        assert_eq!(text.start_placeholder, "Begin");
        assert_eq!(text.end_placeholder, DEFAULT_END_PLACEHOLDER);
        assert_eq!(text.start_aria_label, "Begin");
        assert_eq!(text.end_aria_label, "Finish");
        assert_eq!(text.invalid_range_message, DEFAULT_INVALID_RANGE_MESSAGE);
    }
}
