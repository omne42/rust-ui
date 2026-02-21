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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KeyboardStateInput {
    pub tone: KeyboardTone,
    pub compact: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KeyboardState {
    pub tone: KeyboardTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub is_compact: bool,
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

#[cfg(test)]
#[path = "test/keyboard.rs"]
mod tests;
