pub const DEFAULT_ARIA_LABEL: &str = "Header";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum HeaderTone {
    #[default]
    Default,
    Strong,
}

impl HeaderTone {
    pub fn class_name(self) -> &'static str {
        match self {
            HeaderTone::Default => "ui-header--tone-default",
            HeaderTone::Strong => "ui-header--tone-strong",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            HeaderTone::Default => "default",
            HeaderTone::Strong => "strong",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HeaderStateInput {
    pub tone: HeaderTone,
    pub bordered: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HeaderState {
    pub tone: HeaderTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub is_bordered: bool,
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

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.into(), false)
}

pub fn resolve_state(input: HeaderStateInput) -> HeaderState {
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

    let data_state_attr = if input.bordered && input.tone == HeaderTone::Strong {
        "strong-bordered"
    } else if input.bordered {
        "bordered"
    } else if input.tone == HeaderTone::Strong {
        "strong"
    } else {
        "default"
    };

    HeaderState {
        tone: input.tone,
        tone_class: input.tone.class_name(),
        tone_attr: input.tone.as_attr(),
        is_bordered: input.bordered,
        data_state_attr,
        aria_source_attr,
        class_source_attr,
        has_custom_class_name: input.has_custom_class_name,
    }
}

#[cfg(test)]
#[path = "test/header.rs"]
mod tests;
