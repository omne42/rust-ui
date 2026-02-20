use crate::heading::{HeadingState, HeadingStateInput};

pub const DEFAULT_ARIA_LABEL: &str = "Heading";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum HeadingLevel {
    H1,
    H2,
    #[default]
    H3,
    H4,
    H5,
    H6,
}

impl HeadingLevel {
    pub fn class_name(self) -> &'static str {
        match self {
            HeadingLevel::H1 => "ui-heading--level-1",
            HeadingLevel::H2 => "ui-heading--level-2",
            HeadingLevel::H3 => "ui-heading--level-3",
            HeadingLevel::H4 => "ui-heading--level-4",
            HeadingLevel::H5 => "ui-heading--level-5",
            HeadingLevel::H6 => "ui-heading--level-6",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            HeadingLevel::H1 => "1",
            HeadingLevel::H2 => "2",
            HeadingLevel::H3 => "3",
            HeadingLevel::H4 => "4",
            HeadingLevel::H5 => "5",
            HeadingLevel::H6 => "6",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum HeadingTone {
    #[default]
    Default,
    Strong,
    Muted,
}

impl HeadingTone {
    pub fn class_name(self) -> &'static str {
        match self {
            HeadingTone::Default => "ui-heading--tone-default",
            HeadingTone::Strong => "ui-heading--tone-strong",
            HeadingTone::Muted => "ui-heading--tone-muted",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            HeadingTone::Default => "default",
            HeadingTone::Strong => "strong",
            HeadingTone::Muted => "muted",
        }
    }
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

pub fn resolve_state(input: HeadingStateInput) -> HeadingState {
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

    let data_state_attr = if input.truncate {
        "truncate"
    } else if input.tone == HeadingTone::Strong {
        "strong"
    } else if input.tone == HeadingTone::Muted {
        "muted"
    } else {
        "default"
    };

    HeadingState {
        level: input.level,
        level_class: input.level.class_name(),
        level_attr: input.level.as_attr(),
        tone: input.tone,
        tone_class: input.tone.class_name(),
        tone_attr: input.tone.as_attr(),
        is_truncated: input.truncate,
        data_state_attr,
        aria_source_attr,
        class_source_attr,
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: HeadingState) -> String {
    let mut classes = vec![
        "ui-heading".to_string(),
        state.level_class.into(),
        state.tone_class.into(),
    ];

    if state.is_truncated {
        classes.push("ui-heading--truncate".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-heading--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "test/logic.rs"]
mod tests;
