pub const DEFAULT_ARIA_LABEL: &str = "HelpText";
pub const DEFAULT_ERROR_MESSAGE: &str = "Invalid value";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum HelpTextTone {
    #[default]
    Auto,
    Neutral,
    Negative,
}

impl HelpTextTone {
    pub fn class_name(self) -> &'static str {
        match self {
            HelpTextTone::Auto => "ui-help-text--tone-auto",
            HelpTextTone::Neutral => "ui-help-text--tone-neutral",
            HelpTextTone::Negative => "ui-help-text--tone-negative",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            HelpTextTone::Auto => "auto",
            HelpTextTone::Neutral => "neutral",
            HelpTextTone::Negative => "negative",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HelpTextStateInput {
    pub tone: HelpTextTone,
    pub invalid: bool,
    pub disabled: bool,
    pub show_error_icon: bool,
    pub has_description: bool,
    pub has_error_message: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_error_message: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HelpTextMessageKind {
    None,
    Description,
    Error,
}

impl HelpTextMessageKind {
    pub const fn as_attr(self) -> &'static str {
        match self {
            HelpTextMessageKind::None => "none",
            HelpTextMessageKind::Description => "description",
            HelpTextMessageKind::Error => "error",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HelpTextDataState {
    Empty,
    Description,
    Disabled,
    Error,
    ErrorDisabled,
}

impl HelpTextDataState {
    pub const fn as_attr(self) -> &'static str {
        match self {
            HelpTextDataState::Empty => "empty",
            HelpTextDataState::Description => "description",
            HelpTextDataState::Disabled => "disabled",
            HelpTextDataState::Error => "error",
            HelpTextDataState::ErrorDisabled => "error-disabled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HelpTextSourceAttr {
    Default,
    Custom,
}

impl HelpTextSourceAttr {
    pub const fn as_attr(self) -> &'static str {
        match self {
            HelpTextSourceAttr::Default => "default",
            HelpTextSourceAttr::Custom => "custom",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HelpTextErrorSourceAttr {
    None,
    Default,
    Custom,
}

impl HelpTextErrorSourceAttr {
    pub const fn as_attr(self) -> &'static str {
        match self {
            HelpTextErrorSourceAttr::None => "none",
            HelpTextErrorSourceAttr::Default => "default",
            HelpTextErrorSourceAttr::Custom => "custom",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HelpTextState {
    pub tone: HelpTextTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub is_invalid: bool,
    pub is_disabled: bool,
    pub show_error_icon: bool,
    pub has_description: bool,
    pub has_error_message: bool,
    pub message_kind: HelpTextMessageKind,
    pub data_state: HelpTextDataState,
    pub aria_source: HelpTextSourceAttr,
    pub error_source: HelpTextErrorSourceAttr,
    pub class_source: HelpTextSourceAttr,
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

pub fn normalize_error_message(value: Option<String>, invalid: bool) -> (Option<String>, bool) {
    if !invalid {
        return (None, false);
    }

    if let Some(message) = normalize_optional_text(value) {
        return (Some(message), true);
    }

    (Some(DEFAULT_ERROR_MESSAGE.into()), false)
}

pub fn resolve_effective_tone(
    requested_tone: HelpTextTone,
    invalid: bool,
    has_error_message: bool,
) -> HelpTextTone {
    match requested_tone {
        HelpTextTone::Neutral => HelpTextTone::Neutral,
        HelpTextTone::Negative => HelpTextTone::Negative,
        HelpTextTone::Auto if invalid && has_error_message => HelpTextTone::Negative,
        HelpTextTone::Auto => HelpTextTone::Neutral,
    }
}

pub fn resolve_state(input: HelpTextStateInput) -> HelpTextState {
    let message_kind = if input.has_error_message && input.invalid {
        HelpTextMessageKind::Error
    } else if input.has_description {
        HelpTextMessageKind::Description
    } else {
        HelpTextMessageKind::None
    };

    let tone = resolve_effective_tone(input.tone, input.invalid, input.has_error_message);
    let show_error_icon = input.show_error_icon && message_kind == HelpTextMessageKind::Error;

    let data_state = if message_kind == HelpTextMessageKind::Error && input.disabled {
        HelpTextDataState::ErrorDisabled
    } else if message_kind == HelpTextMessageKind::Error {
        HelpTextDataState::Error
    } else if input.disabled {
        HelpTextDataState::Disabled
    } else if message_kind == HelpTextMessageKind::Description {
        HelpTextDataState::Description
    } else {
        HelpTextDataState::Empty
    };

    let aria_source = if input.has_custom_aria_label {
        HelpTextSourceAttr::Custom
    } else {
        HelpTextSourceAttr::Default
    };

    let error_source = if !input.has_error_message {
        HelpTextErrorSourceAttr::None
    } else if input.has_custom_error_message {
        HelpTextErrorSourceAttr::Custom
    } else {
        HelpTextErrorSourceAttr::Default
    };

    let class_source = if input.has_custom_class_name {
        HelpTextSourceAttr::Custom
    } else {
        HelpTextSourceAttr::Default
    };

    HelpTextState {
        tone,
        tone_class: tone.class_name(),
        tone_attr: tone.as_attr(),
        is_invalid: input.invalid,
        is_disabled: input.disabled,
        show_error_icon,
        has_description: input.has_description,
        has_error_message: input.has_error_message,
        message_kind,
        data_state,
        aria_source,
        error_source,
        class_source,
        has_custom_class_name: input.has_custom_class_name,
    }
}

#[cfg(test)]
#[path = "test/help_text.rs"]
mod tests;
