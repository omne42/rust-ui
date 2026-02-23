#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SwitchGroupOrientation {
    #[default]
    Vertical,
    Horizontal,
}

impl SwitchGroupOrientation {
    pub fn class_name(self) -> &'static str {
        match self {
            SwitchGroupOrientation::Vertical => "ui-switch-group--orientation-vertical",
            SwitchGroupOrientation::Horizontal => "ui-switch-group--orientation-horizontal",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            SwitchGroupOrientation::Vertical => "vertical",
            SwitchGroupOrientation::Horizontal => "horizontal",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SwitchGroupTone {
    #[default]
    Default,
    Muted,
}

impl SwitchGroupTone {
    pub fn class_name(self) -> &'static str {
        match self {
            SwitchGroupTone::Default => "ui-switch-group--tone-default",
            SwitchGroupTone::Muted => "ui-switch-group--tone-muted",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            SwitchGroupTone::Default => "default",
            SwitchGroupTone::Muted => "muted",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SwitchGroupStateInput {
    pub orientation: SwitchGroupOrientation,
    pub tone: SwitchGroupTone,
    pub required: bool,
    pub disabled: bool,
    pub invalid: bool,
    pub has_label: bool,
    pub has_description: bool,
    pub has_error_message: bool,
    pub has_custom_label: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_error_message: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SwitchGroupState {
    pub orientation: SwitchGroupOrientation,
    pub orientation_class: &'static str,
    pub orientation_attr: &'static str,
    pub tone: SwitchGroupTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub is_required: bool,
    pub is_disabled: bool,
    pub is_invalid: bool,
    pub has_label: bool,
    pub has_description: bool,
    pub has_error_message: bool,
    pub shows_error: bool,
    pub has_messages: bool,
    pub message_kind_attr: &'static str,
    pub data_state_attr: &'static str,
    pub label_source_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub error_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}

pub fn resolve_state(input: SwitchGroupStateInput) -> SwitchGroupState {
    let shows_error = input.invalid && input.has_error_message;
    let has_messages = input.has_description || shows_error;

    let label_source_attr = if input.has_custom_label {
        "custom"
    } else {
        "default"
    };

    let aria_source_attr = if input.has_custom_aria_label {
        "custom"
    } else {
        "default"
    };

    let error_source_attr = if !input.has_error_message {
        "none"
    } else if input.has_custom_error_message {
        "custom"
    } else {
        "default"
    };

    let class_source_attr = if input.has_custom_class_name {
        "custom"
    } else {
        "default"
    };

    let message_kind_attr = if shows_error {
        "error"
    } else if input.has_description {
        "description"
    } else {
        "none"
    };

    let data_state_attr = if input.invalid && input.disabled {
        "invalid-disabled"
    } else if input.invalid {
        "invalid"
    } else if input.disabled {
        "disabled"
    } else if input.required {
        "required"
    } else if input.orientation == SwitchGroupOrientation::Horizontal {
        "horizontal"
    } else if input.tone == SwitchGroupTone::Muted {
        "muted"
    } else {
        "default"
    };

    SwitchGroupState {
        orientation: input.orientation,
        orientation_class: input.orientation.class_name(),
        orientation_attr: input.orientation.as_attr(),
        tone: input.tone,
        tone_class: input.tone.class_name(),
        tone_attr: input.tone.as_attr(),
        is_required: input.required,
        is_disabled: input.disabled,
        is_invalid: input.invalid,
        has_label: input.has_label,
        has_description: input.has_description,
        has_error_message: input.has_error_message,
        shows_error,
        has_messages,
        message_kind_attr,
        data_state_attr,
        label_source_attr,
        aria_source_attr,
        error_source_attr,
        class_source_attr,
        has_custom_class_name: input.has_custom_class_name,
    }
}

#[cfg(test)]
#[path = "test/switch_group.rs"]
mod tests;
