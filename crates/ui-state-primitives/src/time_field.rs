pub use crate::button::normalize_optional_text;

pub const DEFAULT_ARIA_LABEL: &str = "Time field";
pub const DEFAULT_LABEL: &str = "Time";
pub const DEFAULT_PLACEHOLDER: &str = "hh:mm";
pub const DEFAULT_HOUR_ARIA_LABEL: &str = "Hour";
pub const DEFAULT_MINUTE_ARIA_LABEL: &str = "Minute";
pub const DEFAULT_CLEAR_LABEL: &str = "Clear";
pub const DEFAULT_CLEAR_ARIA_LABEL: &str = "Clear time";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TimeFieldTone {
    #[default]
    Default,
    Quiet,
    Strong,
}

impl TimeFieldTone {
    pub fn class_name(self) -> &'static str {
        match self {
            TimeFieldTone::Default => "ui-time-field--tone-default",
            TimeFieldTone::Quiet => "ui-time-field--tone-quiet",
            TimeFieldTone::Strong => "ui-time-field--tone-strong",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            TimeFieldTone::Default => "default",
            TimeFieldTone::Quiet => "quiet",
            TimeFieldTone::Strong => "strong",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TimeFieldIds {
    pub root_id: String,
    pub label_id: String,
    pub hour_id: String,
    pub minute_id: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimeFieldStateInput {
    pub tone: TimeFieldTone,
    pub disabled: bool,
    pub is_controlled: bool,
    pub has_default_value: bool,
    pub has_value_change_handler: bool,
    pub has_value: bool,
    pub minute_step: u8,
    pub has_custom_label: bool,
    pub has_custom_placeholder: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimeFieldState {
    pub tone: TimeFieldTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub is_disabled: bool,
    pub is_controlled: bool,
    pub is_uncontrolled: bool,
    pub has_value: bool,
    pub is_empty: bool,
    pub minute_step: u8,
    pub data_state_attr: &'static str,
    pub label_source_attr: &'static str,
    pub placeholder_source_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
    pub motion_source_attr: &'static str,
    pub control_mode_attr: &'static str,
    pub value_source_attr: &'static str,
    pub default_value_source_attr: &'static str,
    pub value_change_source_attr: &'static str,
}

pub fn normalize_label(value: Option<String>, fallback: &str) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (fallback.to_string(), false)
}

pub fn normalize_placeholder(value: Option<String>, fallback: &str) -> (String, bool) {
    if let Some(placeholder) = normalize_optional_text(value) {
        return (placeholder, true);
    }

    (fallback.to_string(), false)
}

pub fn normalize_aria_label(value: Option<String>, fallback: &str) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (fallback.to_string(), false)
}

pub fn normalize_hour_aria_label(value: Option<String>, fallback: &str) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (fallback.to_string(), false)
}

pub fn normalize_minute_aria_label(value: Option<String>, fallback: &str) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (fallback.to_string(), false)
}

pub fn normalize_clear_label(value: Option<String>, fallback: &str) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (fallback.to_string(), false)
}

pub fn normalize_clear_aria_label(value: Option<String>, fallback: &str) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (fallback.to_string(), false)
}

pub fn resolve_ids(id_base: &str) -> TimeFieldIds {
    let base = normalize_optional_text(Some(id_base.to_string()))
        .unwrap_or_else(|| "ui-time-field".to_string());

    TimeFieldIds {
        root_id: base.clone(),
        label_id: format!("{base}-label"),
        hour_id: format!("{base}-hour"),
        minute_id: format!("{base}-minute"),
    }
}

pub fn normalize_minute_step(minute_step: u8) -> u8 {
    minute_step.clamp(1, 30)
}

pub fn normalize_hour(hour: u8) -> u8 {
    hour.min(23)
}

pub fn normalize_minute(minute: u8, minute_step: u8) -> u8 {
    let minute = minute.min(59);
    let minute_step = normalize_minute_step(minute_step);
    if minute_step <= 1 {
        return minute;
    }

    ((minute / minute_step) * minute_step).min(59)
}

pub fn format_time_value(hour: u8, minute: u8, minute_step: u8) -> String {
    format!(
        "{:02}:{:02}",
        normalize_hour(hour),
        normalize_minute(minute, minute_step)
    )
}

pub fn parse_time_value(value: &str, minute_step: u8) -> Option<(u8, u8)> {
    let trimmed = value.trim();
    let (hour_raw, minute_raw) = trimmed.split_once(':')?;
    let hour = hour_raw.trim().parse::<u8>().ok()?;
    let minute = minute_raw.trim().parse::<u8>().ok()?;

    if hour > 23 || minute > 59 {
        return None;
    }

    Some((normalize_hour(hour), normalize_minute(minute, minute_step)))
}

pub fn normalize_time_value(value: Option<String>, minute_step: u8) -> Option<String> {
    normalize_optional_text(value).and_then(|value| {
        parse_time_value(&value, minute_step)
            .map(|(hour, minute)| format_time_value(hour, minute, minute_step))
    })
}

pub fn resolve_time_parts(value: Option<String>, minute_step: u8) -> (u8, u8, bool) {
    if let Some(value) = normalize_time_value(value, minute_step)
        && let Some((hour, minute)) = parse_time_value(&value, minute_step)
    {
        return (hour, minute, true);
    }

    (0, 0, false)
}

pub fn resolve_input_placeholders(placeholder: &str) -> (String, String) {
    if let Some((hour, minute)) = placeholder.split_once(':') {
        let hour = hour.trim();
        let minute = minute.trim();

        if !hour.is_empty() && !minute.is_empty() {
            return (hour.to_string(), minute.to_string());
        }
    }

    ("hh".to_string(), "mm".to_string())
}

pub fn update_hour_from_input(
    current_value: Option<String>,
    hour_input: &str,
    minute_step: u8,
) -> Option<String> {
    let current_value = normalize_time_value(current_value, minute_step);
    let trimmed = hour_input.trim();
    if trimmed.is_empty() {
        return current_value;
    }

    let Ok(hour) = trimmed.parse::<u8>() else {
        return current_value;
    };

    let (_, minute, has_value) = resolve_time_parts(current_value.clone(), minute_step);
    let minute = if has_value { minute } else { 0 };

    Some(format_time_value(hour, minute, minute_step))
}

pub fn update_minute_from_input(
    current_value: Option<String>,
    minute_input: &str,
    minute_step: u8,
) -> Option<String> {
    let current_value = normalize_time_value(current_value, minute_step);
    let trimmed = minute_input.trim();
    if trimmed.is_empty() {
        return current_value;
    }

    let Ok(minute) = trimmed.parse::<u8>() else {
        return current_value;
    };

    let (hour, _, has_value) = resolve_time_parts(current_value.clone(), minute_step);
    let hour = if has_value { hour } else { 0 };

    Some(format_time_value(hour, minute, minute_step))
}

pub fn resolve_state(input: TimeFieldStateInput) -> TimeFieldState {
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
    let motion_source_attr = if input.has_custom_motion {
        "custom"
    } else {
        "default"
    };
    let control_mode_attr = if input.is_controlled {
        "controlled"
    } else {
        "uncontrolled"
    };
    let value_source_attr = if input.is_controlled {
        "external"
    } else {
        "default"
    };
    let default_value_source_attr = if input.has_default_value {
        "provided"
    } else {
        "implicit"
    };
    let value_change_source_attr = if input.has_value_change_handler {
        "provided"
    } else {
        "none"
    };

    let is_empty = !input.has_value;

    let data_state_attr = if input.disabled {
        "disabled"
    } else if input.has_value {
        "value"
    } else {
        "empty"
    };

    TimeFieldState {
        tone: input.tone,
        tone_class: input.tone.class_name(),
        tone_attr: input.tone.as_attr(),
        is_disabled: input.disabled,
        is_controlled: input.is_controlled,
        is_uncontrolled: !input.is_controlled,
        has_value: input.has_value,
        is_empty,
        minute_step: input.minute_step,
        data_state_attr,
        label_source_attr,
        placeholder_source_attr,
        aria_source_attr,
        class_source_attr,
        has_custom_class_name: input.has_custom_class_name,
        motion_source_attr,
        control_mode_attr,
        value_source_attr,
        default_value_source_attr,
        value_change_source_attr,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: TimeFieldState) -> String {
    let mut classes = vec!["ui-time-field".to_string(), state.tone_class.to_string()];

    if state.is_disabled {
        classes.push("ui-time-field--disabled".to_string());
    }
    if state.has_value {
        classes.push("ui-time-field--has-value".to_string());
    }
    if state.has_custom_class_name {
        classes.push("ui-time-field--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tone_class_names_and_attrs_are_stable() {
        assert_eq!(
            TimeFieldTone::Default.class_name(),
            "ui-time-field--tone-default"
        );
        assert_eq!(
            TimeFieldTone::Quiet.class_name(),
            "ui-time-field--tone-quiet"
        );
        assert_eq!(
            TimeFieldTone::Strong.class_name(),
            "ui-time-field--tone-strong"
        );

        assert_eq!(TimeFieldTone::Default.as_attr(), "default");
        assert_eq!(TimeFieldTone::Quiet.as_attr(), "quiet");
        assert_eq!(TimeFieldTone::Strong.as_attr(), "strong");
    }

    #[test]
    fn normalize_time_value_formats_zero_padded_step_aware_values() {
        let value = normalize_time_value(Some(" 9:17 ".to_string()), 5);
        assert_eq!(value, Some("09:15".to_string()));

        let invalid = normalize_time_value(Some("not-a-time".to_string()), 15);
        assert_eq!(invalid, None);
    }

    #[test]
    fn normalize_a11y_and_clear_labels_use_defaults_for_blank_values() {
        assert_eq!(
            normalize_hour_aria_label(Some("  ".to_string()), DEFAULT_HOUR_ARIA_LABEL),
            (DEFAULT_HOUR_ARIA_LABEL.to_string(), false)
        );
        assert_eq!(
            normalize_minute_aria_label(None, DEFAULT_MINUTE_ARIA_LABEL),
            (DEFAULT_MINUTE_ARIA_LABEL.to_string(), false)
        );
        assert_eq!(
            normalize_clear_label(Some("  Clear now  ".to_string()), DEFAULT_CLEAR_LABEL),
            ("Clear now".to_string(), true)
        );
        assert_eq!(
            normalize_clear_aria_label(None, DEFAULT_CLEAR_ARIA_LABEL),
            (DEFAULT_CLEAR_ARIA_LABEL.to_string(), false)
        );
    }

    #[test]
    fn update_helpers_keep_other_segment_stable() {
        let value = update_hour_from_input(Some("06:45".to_string()), "9", 15);
        assert_eq!(value, Some("09:45".to_string()));

        let value = update_minute_from_input(value, "14", 5);
        assert_eq!(value, Some("09:10".to_string()));
    }

    #[test]
    fn resolve_state_tracks_sources_and_value_state() {
        let state = resolve_state(TimeFieldStateInput {
            tone: TimeFieldTone::Strong,
            disabled: false,
            is_controlled: true,
            has_default_value: true,
            has_value_change_handler: true,
            has_value: true,
            minute_step: 15,
            has_custom_label: true,
            has_custom_placeholder: false,
            has_custom_aria_label: true,
            has_custom_class_name: false,
            has_custom_motion: true,
        });

        assert_eq!(state.tone_attr, "strong");
        assert_eq!(state.data_state_attr, "value");
        assert_eq!(state.label_source_attr, "custom");
        assert_eq!(state.placeholder_source_attr, "default");
        assert_eq!(state.aria_source_attr, "custom");
        assert_eq!(state.class_source_attr, "default");
        assert_eq!(state.motion_source_attr, "custom");
        assert!(state.is_controlled);
        assert!(!state.is_uncontrolled);
        assert_eq!(state.control_mode_attr, "controlled");
        assert_eq!(state.value_source_attr, "external");
        assert_eq!(state.default_value_source_attr, "provided");
        assert_eq!(state.value_change_source_attr, "provided");
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("docs-time-field".to_string()),
            resolve_state(TimeFieldStateInput {
                tone: TimeFieldTone::Quiet,
                disabled: true,
                is_controlled: false,
                has_default_value: false,
                has_value_change_handler: false,
                has_value: false,
                minute_step: 10,
                has_custom_label: false,
                has_custom_placeholder: false,
                has_custom_aria_label: false,
                has_custom_class_name: true,
                has_custom_motion: false,
            }),
        );

        for token in [
            "ui-time-field",
            "ui-time-field--tone-quiet",
            "ui-time-field--disabled",
            "ui-time-field--custom-class",
            "docs-time-field",
        ] {
            assert!(
                class_name.contains(token),
                "class should include `{token}`, got `{class_name}`"
            );
        }
    }
}
