use crate::{KeyboardState, KeyboardStateInput};

pub const DEFAULT_ARIA_LABEL: &str = "Keyboard";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum KeyboardTone {
    #[default]
    Default,
    Muted,
}

impl KeyboardTone {
    pub fn class_name(self) -> &'static str {
        match self {
            KeyboardTone::Default => "ui-keyboard--tone-default",
            KeyboardTone::Muted => "ui-keyboard--tone-muted",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            KeyboardTone::Default => "default",
            KeyboardTone::Muted => "muted",
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

pub fn resolve_state(input: KeyboardStateInput) -> KeyboardState {
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

    let data_state_attr = if input.compact {
        "compact"
    } else if input.tone == KeyboardTone::Muted {
        "muted"
    } else {
        "default"
    };

    KeyboardState {
        tone: input.tone,
        tone_class: input.tone.class_name(),
        tone_attr: input.tone.as_attr(),
        is_compact: input.compact,
        data_state_attr,
        aria_source_attr,
        class_source_attr,
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: KeyboardState) -> String {
    let mut classes = vec!["ui-keyboard".to_string(), state.tone_class.into()];

    if state.is_compact {
        classes.push("ui-keyboard--compact".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-keyboard--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
