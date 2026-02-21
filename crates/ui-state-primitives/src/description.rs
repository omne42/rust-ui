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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DescriptionStateInput {
    pub tone: DescriptionTone,
    pub disabled: bool,
    pub truncate: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DescriptionState {
    pub tone: DescriptionTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub is_disabled: bool,
    pub is_truncated: bool,
    pub data_state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
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

#[cfg(test)]
#[path = "test/description.rs"]
mod tests;
