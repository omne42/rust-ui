#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FieldErrorTone {
    #[default]
    Auto,
    Neutral,
    Negative,
}

impl FieldErrorTone {
    pub fn class_name(self) -> &'static str {
        match self {
            FieldErrorTone::Auto => "ui-field-error--tone-auto",
            FieldErrorTone::Neutral => "ui-field-error--tone-neutral",
            FieldErrorTone::Negative => "ui-field-error--tone-negative",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            FieldErrorTone::Auto => "auto",
            FieldErrorTone::Neutral => "neutral",
            FieldErrorTone::Negative => "negative",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldErrorDataState {
    Hidden,
    Disabled,
    Visible,
}

impl FieldErrorDataState {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Hidden => "hidden",
            Self::Disabled => "disabled",
            Self::Visible => "visible",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldErrorSource {
    Default,
    Custom,
}

impl FieldErrorSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Custom => "custom",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldErrorMessageSource {
    None,
    Default,
    Custom,
}

impl FieldErrorMessageSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Default => "default",
            Self::Custom => "custom",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldErrorStateInput {
    pub tone: FieldErrorTone,
    pub visible: bool,
    pub disabled: bool,
    pub show_icon: bool,
    pub has_message: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_message: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldErrorState {
    pub tone: FieldErrorTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub is_visible: bool,
    pub is_disabled: bool,
    pub show_icon: bool,
    pub has_message: bool,
    pub data_state: FieldErrorDataState,
    pub aria_source: FieldErrorSource,
    pub message_source: FieldErrorMessageSource,
    pub class_source: FieldErrorSource,
    pub has_custom_class_name: bool,
}

pub fn resolve_effective_tone(requested_tone: FieldErrorTone, is_visible: bool) -> FieldErrorTone {
    match requested_tone {
        FieldErrorTone::Neutral => FieldErrorTone::Neutral,
        FieldErrorTone::Negative => FieldErrorTone::Negative,
        FieldErrorTone::Auto if is_visible => FieldErrorTone::Negative,
        FieldErrorTone::Auto => FieldErrorTone::Neutral,
    }
}

pub fn resolve_state(input: FieldErrorStateInput) -> FieldErrorState {
    let is_visible = input.visible && input.has_message;
    let tone = resolve_effective_tone(input.tone, is_visible);
    let show_icon = input.show_icon && is_visible;

    let data_state = if !is_visible {
        FieldErrorDataState::Hidden
    } else if input.disabled {
        FieldErrorDataState::Disabled
    } else {
        FieldErrorDataState::Visible
    };

    let aria_source = if input.has_custom_aria_label {
        FieldErrorSource::Custom
    } else {
        FieldErrorSource::Default
    };

    let message_source = if !input.has_message {
        FieldErrorMessageSource::None
    } else if input.has_custom_message {
        FieldErrorMessageSource::Custom
    } else {
        FieldErrorMessageSource::Default
    };

    let class_source = if input.has_custom_class_name {
        FieldErrorSource::Custom
    } else {
        FieldErrorSource::Default
    };

    FieldErrorState {
        tone,
        tone_class: tone.class_name(),
        tone_attr: tone.as_attr(),
        is_visible,
        is_disabled: input.disabled,
        show_icon,
        has_message: input.has_message,
        data_state,
        aria_source,
        message_source,
        class_source,
        has_custom_class_name: input.has_custom_class_name,
    }
}

#[cfg(test)]
#[path = "test/field_error.rs"]
mod tests;
