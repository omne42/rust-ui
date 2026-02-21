#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FieldsetOrientation {
    #[default]
    Vertical,
    Horizontal,
}

impl FieldsetOrientation {
    pub fn class_name(self) -> &'static str {
        match self {
            FieldsetOrientation::Vertical => "ui-fieldset--orientation-vertical",
            FieldsetOrientation::Horizontal => "ui-fieldset--orientation-horizontal",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            FieldsetOrientation::Vertical => "vertical",
            FieldsetOrientation::Horizontal => "horizontal",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FieldsetTone {
    #[default]
    Default,
    Muted,
}

impl FieldsetTone {
    pub fn class_name(self) -> &'static str {
        match self {
            FieldsetTone::Default => "ui-fieldset--tone-default",
            FieldsetTone::Muted => "ui-fieldset--tone-muted",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            FieldsetTone::Default => "default",
            FieldsetTone::Muted => "muted",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FieldsetMessageKind {
    #[default]
    None,
    Description,
    Error,
}

impl FieldsetMessageKind {
    pub fn as_attr(self) -> &'static str {
        match self {
            FieldsetMessageKind::None => "none",
            FieldsetMessageKind::Description => "description",
            FieldsetMessageKind::Error => "error",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FieldsetDataState {
    #[default]
    Default,
    Required,
    Disabled,
    Invalid,
    InvalidDisabled,
    Horizontal,
    Muted,
}

impl FieldsetDataState {
    pub fn as_attr(self) -> &'static str {
        match self {
            FieldsetDataState::Default => "default",
            FieldsetDataState::Required => "required",
            FieldsetDataState::Disabled => "disabled",
            FieldsetDataState::Invalid => "invalid",
            FieldsetDataState::InvalidDisabled => "invalid-disabled",
            FieldsetDataState::Horizontal => "horizontal",
            FieldsetDataState::Muted => "muted",
        }
    }
}

pub const DEFAULT_ARIA_LABEL: &str = "Fieldset";
pub const DEFAULT_ERROR_MESSAGE: &str = "Invalid value";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldsetStateInput {
    pub orientation: FieldsetOrientation,
    pub tone: FieldsetTone,
    pub required: bool,
    pub disabled: bool,
    pub invalid: bool,
    pub has_legend: bool,
    pub has_description: bool,
    pub has_error_message: bool,
    pub has_actions: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_error_message: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldsetBooleanAxisInput {
    pub value: Option<bool>,
    pub default_value: Option<bool>,
    pub has_on_change: bool,
    pub value_source_attr: &'static str,
    pub default_source_attr: &'static str,
    pub change_source_attr: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldsetBooleanAxis {
    pub controlled_value: Option<bool>,
    pub initial_value: bool,
    pub value_source_attr: &'static str,
    pub control_mode_attr: &'static str,
    pub change_source_attr: &'static str,
}

pub fn normalize_boolean_axis(input: FieldsetBooleanAxisInput) -> FieldsetBooleanAxis {
    let value_source_attr = if input.value.is_some() {
        input.value_source_attr
    } else if input.default_value.is_some() {
        input.default_source_attr
    } else {
        "default"
    };
    let control_mode_attr = if input.value.is_some() {
        "controlled"
    } else {
        "uncontrolled"
    };
    let change_source_attr = if input.has_on_change {
        input.change_source_attr
    } else {
        "none"
    };

    FieldsetBooleanAxis {
        controlled_value: input.value,
        initial_value: input.default_value.unwrap_or(false),
        value_source_attr,
        control_mode_attr,
        change_source_attr,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldsetState {
    pub orientation: FieldsetOrientation,
    pub orientation_class: &'static str,
    pub orientation_attr: &'static str,
    pub tone: FieldsetTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub is_required: bool,
    pub is_disabled: bool,
    pub is_invalid: bool,
    pub has_legend: bool,
    pub has_description: bool,
    pub has_error_message: bool,
    pub has_actions: bool,
    pub message_kind: FieldsetMessageKind,
    pub data_state: FieldsetDataState,
    pub message_kind_attr: &'static str,
    pub data_state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub error_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_aria_label: bool,
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

pub fn resolve_state(input: FieldsetStateInput) -> FieldsetState {
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

    let message_kind = if input.has_error_message {
        FieldsetMessageKind::Error
    } else if input.has_description {
        FieldsetMessageKind::Description
    } else {
        FieldsetMessageKind::None
    };

    let data_state = if input.invalid && input.disabled {
        FieldsetDataState::InvalidDisabled
    } else if input.invalid {
        FieldsetDataState::Invalid
    } else if input.disabled {
        FieldsetDataState::Disabled
    } else if input.required {
        FieldsetDataState::Required
    } else if input.orientation == FieldsetOrientation::Horizontal {
        FieldsetDataState::Horizontal
    } else if input.tone == FieldsetTone::Muted {
        FieldsetDataState::Muted
    } else {
        FieldsetDataState::Default
    };

    FieldsetState {
        orientation: input.orientation,
        orientation_class: input.orientation.class_name(),
        orientation_attr: input.orientation.as_attr(),
        tone: input.tone,
        tone_class: input.tone.class_name(),
        tone_attr: input.tone.as_attr(),
        is_required: input.required,
        is_disabled: input.disabled,
        is_invalid: input.invalid,
        has_legend: input.has_legend,
        has_description: input.has_description,
        has_error_message: input.has_error_message,
        has_actions: input.has_actions,
        message_kind,
        data_state,
        message_kind_attr: message_kind.as_attr(),
        data_state_attr: data_state.as_attr(),
        aria_source_attr,
        error_source_attr,
        class_source_attr,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
    }
}

#[cfg(test)]
#[path = "test/fieldset.rs"]
mod tests;
