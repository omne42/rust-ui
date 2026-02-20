use crate::content::{ContentState, ContentStateInput};

pub const DEFAULT_ARIA_LABEL: &str = "Content";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ContentTone {
    #[default]
    Default,
    Muted,
}

impl ContentTone {
    pub fn class_name(self) -> &'static str {
        match self {
            ContentTone::Default => "ui-content--tone-default",
            ContentTone::Muted => "ui-content--tone-muted",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            ContentTone::Default => "default",
            ContentTone::Muted => "muted",
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

pub fn resolve_state(input: ContentStateInput) -> ContentState {
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

    let data_state_attr = if input.padded && input.tone == ContentTone::Muted {
        "muted-padded"
    } else if input.padded {
        "padded"
    } else if input.tone == ContentTone::Muted {
        "muted"
    } else {
        "default"
    };

    ContentState {
        tone: input.tone,
        tone_class: input.tone.class_name(),
        tone_attr: input.tone.as_attr(),
        is_padded: input.padded,
        data_state_attr,
        aria_source_attr,
        class_source_attr,
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: ContentState) -> String {
    let mut classes = vec!["ui-content".to_string(), state.tone_class.into()];

    if state.is_padded {
        classes.push("ui-content--padded".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-content--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "test/logic.rs"]
mod tests;
