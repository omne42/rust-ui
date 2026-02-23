use crate::text_input::date_field::{DateFieldState, DateFieldStateInput};

pub use ui_state_primitives::date_field::{
    DEFAULT_ARIA_LABEL, DEFAULT_CLEAR_ARIA_LABEL, DEFAULT_CLEAR_LABEL, DEFAULT_DAY_ARIA_LABEL,
    DEFAULT_LABEL, DEFAULT_MONTH_ARIA_LABEL, DEFAULT_PLACEHOLDER, DEFAULT_YEAR_ARIA_LABEL,
    DateFieldIds, normalize_aria_label, normalize_clear_aria_label, normalize_clear_label,
    normalize_date_value, normalize_day_aria_label, normalize_label, normalize_month_aria_label,
    normalize_optional_text, normalize_placeholder, normalize_year_aria_label, resolve_date_parts,
    resolve_ids, resolve_input_placeholders, update_day_from_input, update_month_from_input,
    update_year_from_input,
};

#[cfg(test)]
pub use ui_state_primitives::date_field::parse_date_value;

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DateFieldDataState {
    Disabled,
    Value,
    Empty,
}

impl DateFieldDataState {
    pub fn from_flags(is_disabled: bool, has_value: bool) -> Self {
        if is_disabled {
            Self::Disabled
        } else if has_value {
            Self::Value
        } else {
            Self::Empty
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::Value => "value",
            Self::Empty => "empty",
        }
    }

    pub fn is_disabled(self) -> bool {
        matches!(self, Self::Disabled)
    }

    pub fn has_value(self) -> bool {
        matches!(self, Self::Value)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DateFieldControlMode {
    Controlled,
    Uncontrolled,
}

impl DateFieldControlMode {
    pub fn from_is_controlled(is_controlled: bool) -> Self {
        if is_controlled {
            Self::Controlled
        } else {
            Self::Uncontrolled
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Controlled => "controlled",
            Self::Uncontrolled => "uncontrolled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DateFieldValueSource {
    External,
    Default,
    Internal,
}

impl DateFieldValueSource {
    pub fn from_control_mode(mode: DateFieldControlMode, has_default_value: bool) -> Self {
        if matches!(mode, DateFieldControlMode::Controlled) {
            Self::External
        } else if has_default_value {
            Self::Default
        } else {
            Self::Internal
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            Self::External => "external",
            Self::Default => "default",
            Self::Internal => "internal",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DateFieldValueChangeSource {
    ExternalHandler,
    InternalOnly,
}

impl DateFieldValueChangeSource {
    pub fn from_has_handler(has_handler: bool) -> Self {
        if has_handler {
            Self::ExternalHandler
        } else {
            Self::InternalOnly
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            Self::ExternalHandler => "external-handler",
            Self::InternalOnly => "internal-only",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DateFieldInteractionSource {
    Programmatic,
    YearInput,
    MonthInput,
    DayInput,
    ClearButton,
}

impl DateFieldInteractionSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Programmatic => "programmatic",
            Self::YearInput => "year-input",
            Self::MonthInput => "month-input",
            Self::DayInput => "day-input",
            Self::ClearButton => "clear-button",
        }
    }
}

pub fn resolve_is_disabled(is_disabled: Option<bool>, disabled: Option<bool>) -> bool {
    is_disabled.or(disabled).unwrap_or(false)
}

pub fn resolve_default_value(default_value: Option<String>) -> Option<String> {
    normalize_date_value(default_value)
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

    let data_state = DateFieldDataState::from_flags(input.disabled, input.has_value);
    let data_state_attr = data_state.as_attr();

    DateFieldState {
        tone: input.tone,
        tone_class: input.tone.class_name(),
        tone_attr: input.tone.as_attr(),
        data_state,
        is_disabled: data_state.is_disabled(),
        has_value: data_state.has_value(),
        is_empty: !data_state.has_value(),
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
