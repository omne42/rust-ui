pub const DEFAULT_ARIA_LABEL: &str = "Label";
pub const DEFAULT_REQUIRED_INDICATOR: &str = "*";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum LabelEmphasis {
    #[default]
    Default,
    Subtle,
    Strong,
}

impl LabelEmphasis {
    pub fn class_name(self) -> &'static str {
        match self {
            LabelEmphasis::Default => "ui-label--emphasis-default",
            LabelEmphasis::Subtle => "ui-label--emphasis-subtle",
            LabelEmphasis::Strong => "ui-label--emphasis-strong",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            LabelEmphasis::Default => "default",
            LabelEmphasis::Subtle => "subtle",
            LabelEmphasis::Strong => "strong",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LabelStateInput {
    pub emphasis: LabelEmphasis,
    pub required: bool,
    pub disabled: bool,
    pub has_for_id: bool,
    pub has_custom_label: bool,
    pub has_custom_indicator: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LabelState {
    pub emphasis: LabelEmphasis,
    pub emphasis_class: &'static str,
    pub emphasis_attr: &'static str,
    pub is_required: bool,
    pub is_optional: bool,
    pub is_disabled: bool,
    pub is_enabled: bool,
    pub has_for_id: bool,
    pub has_custom_label: bool,
    pub has_custom_indicator: bool,
    pub has_custom_class_name: bool,
    pub label_source_attr: &'static str,
    pub indicator_source_attr: &'static str,
    pub class_source_attr: &'static str,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_label_text(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.into(), false)
}

pub fn normalize_required_indicator(value: Option<String>) -> (String, bool) {
    if let Some(indicator) = normalize_optional_text(value) {
        return (indicator, true);
    }

    (DEFAULT_REQUIRED_INDICATOR.into(), false)
}

pub fn resolve_state(input: LabelStateInput) -> LabelState {
    let label_source_attr = if input.has_custom_label {
        "custom"
    } else {
        "default"
    };
    let indicator_source_attr = if input.has_custom_indicator {
        "custom"
    } else {
        "default"
    };
    let class_source_attr = if input.has_custom_class_name {
        "custom"
    } else {
        "default"
    };

    LabelState {
        emphasis: input.emphasis,
        emphasis_class: input.emphasis.class_name(),
        emphasis_attr: input.emphasis.as_attr(),
        is_required: input.required,
        is_optional: !input.required,
        is_disabled: input.disabled,
        is_enabled: !input.disabled,
        has_for_id: input.has_for_id,
        has_custom_label: input.has_custom_label,
        has_custom_indicator: input.has_custom_indicator,
        has_custom_class_name: input.has_custom_class_name,
        label_source_attr,
        indicator_source_attr,
        class_source_attr,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: LabelState) -> String {
    let mut classes = vec!["ui-label".to_string(), state.emphasis_class.into()];

    if state.is_required {
        classes.push("ui-label--required".to_string());
    }
    if state.is_disabled {
        classes.push("ui-label--disabled".to_string());
    }
    if state.has_for_id {
        classes.push("ui-label--for".to_string());
    }
    if state.has_custom_label {
        classes.push("ui-label--text-custom".to_string());
    }
    if state.has_custom_indicator {
        classes.push("ui-label--indicator-custom".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-label--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "test/label.rs"]
mod tests;
