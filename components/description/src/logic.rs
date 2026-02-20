use crate::{DescriptionState, DescriptionStateInput};

pub const DEFAULT_ARIA_LABEL: &str = "Description";
pub const DEFAULT_TEXT: &str = "—";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DescriptionTone {
    #[default]
    Default,
    Muted,
    Negative,
}

impl DescriptionTone {
    pub fn class_name(self) -> &'static str {
        match self {
            DescriptionTone::Default => "ui-description--tone-default",
            DescriptionTone::Muted => "ui-description--tone-muted",
            DescriptionTone::Negative => "ui-description--tone-negative",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            DescriptionTone::Default => "default",
            DescriptionTone::Muted => "muted",
            DescriptionTone::Negative => "negative",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DescriptionElement {
    Span,
    #[default]
    Paragraph,
    Div,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_content(value: Option<String>) -> String {
    normalize_optional_text(value).unwrap_or_else(|| DEFAULT_TEXT.into())
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.into(), false)
}

pub fn resolve_state(input: DescriptionStateInput) -> DescriptionState {
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
    } else if input.truncate {
        "truncate"
    } else {
        "default"
    };

    DescriptionState {
        tone: input.tone,
        tone_class: input.tone.class_name(),
        tone_attr: input.tone.as_attr(),
        is_disabled: input.disabled,
        is_truncated: input.truncate,
        data_state_attr,
        aria_source_attr,
        class_source_attr,
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: DescriptionState) -> String {
    let mut classes = vec!["ui-description".to_string(), state.tone_class.into()];

    if state.is_disabled {
        classes.push("ui-description--disabled".to_string());
    }

    if state.is_truncated {
        classes.push("ui-description--truncate".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-description--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
