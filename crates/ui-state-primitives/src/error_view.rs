pub const DEFAULT_ARIA_LABEL: &str = "Error view";
pub const DEFAULT_MESSAGE: &str = "Invalid value";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ErrorViewTone {
    Neutral,
    #[default]
    Negative,
}

impl ErrorViewTone {
    pub fn class_name(self) -> &'static str {
        match self {
            ErrorViewTone::Neutral => "ui-error-view--tone-neutral",
            ErrorViewTone::Negative => "ui-error-view--tone-negative",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            ErrorViewTone::Neutral => "neutral",
            ErrorViewTone::Negative => "negative",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ErrorViewStateInput {
    pub tone: ErrorViewTone,
    pub is_invalid: bool,
    pub compact: bool,
    pub bordered: bool,
    pub has_icon: bool,
    pub has_actions: bool,
    pub has_children: bool,
    pub has_custom_message: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ErrorViewState {
    pub tone: ErrorViewTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub is_visible: bool,
    pub is_hidden: bool,
    pub state_class: &'static str,
    pub state_attr: &'static str,
    pub is_compact: bool,
    pub is_bordered: bool,
    pub has_icon: bool,
    pub has_actions: bool,
    pub has_children: bool,
    pub content_attr: &'static str,
    pub message_source_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub has_custom_class_name: bool,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_message(value: Option<String>) -> (String, bool) {
    if let Some(message) = normalize_optional_text(value) {
        return (message, true);
    }

    (DEFAULT_MESSAGE.into(), false)
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.into(), false)
}

pub fn resolve_state(input: ErrorViewStateInput) -> ErrorViewState {
    let (state_class, state_attr) = if input.is_invalid {
        ("ui-error-view--visible", "visible")
    } else {
        ("ui-error-view--hidden", "hidden")
    };

    let message_source_attr = if input.has_children {
        "none"
    } else if input.has_custom_message {
        "custom"
    } else {
        "default"
    };

    let content_attr = if input.has_children {
        "children"
    } else {
        "text"
    };

    ErrorViewState {
        tone: input.tone,
        tone_class: input.tone.class_name(),
        tone_attr: input.tone.as_attr(),
        is_visible: input.is_invalid,
        is_hidden: !input.is_invalid,
        state_class,
        state_attr,
        is_compact: input.compact,
        is_bordered: input.bordered,
        has_icon: input.has_icon,
        has_actions: input.has_actions,
        has_children: input.has_children,
        content_attr,
        message_source_attr,
        aria_source_attr: if input.has_custom_aria_label {
            "custom"
        } else {
            "default"
        },
        class_source_attr: if input.has_custom_class_name {
            "custom"
        } else {
            "default"
        },
        motion_source_attr: if input.has_custom_motion {
            "custom"
        } else {
            "default"
        },
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: ErrorViewState) -> String {
    let mut classes = vec![
        "ui-error-view".to_string(),
        state.tone_class.into(),
        state.state_class.into(),
    ];

    if state.is_compact {
        classes.push("ui-error-view--compact".to_string());
    }

    if state.is_bordered {
        classes.push("ui-error-view--bordered".to_string());
    }

    if state.has_icon {
        classes.push("ui-error-view--with-icon".to_string());
    }

    if state.has_actions {
        classes.push("ui-error-view--with-actions".to_string());
    }

    if state.has_children {
        classes.push("ui-error-view--with-children".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-error-view--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "test/error_view.rs"]
mod tests;
