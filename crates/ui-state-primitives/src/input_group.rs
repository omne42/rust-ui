pub use crate::button::normalize_optional_text;

pub const DEFAULT_ARIA_LABEL: &str = "Input group";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputGroupPhase {
    Enabled,
    Disabled,
}

impl InputGroupPhase {
    pub fn class_name(self) -> &'static str {
        match self {
            InputGroupPhase::Enabled => "ui-input-group--state-enabled",
            InputGroupPhase::Disabled => "ui-input-group--state-disabled",
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            InputGroupPhase::Enabled => "enabled",
            InputGroupPhase::Disabled => "disabled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputGroupAttachment {
    Attached,
    Detached,
}

impl InputGroupAttachment {
    pub fn class_name(self) -> &'static str {
        match self {
            InputGroupAttachment::Attached => "ui-input-group--attached",
            InputGroupAttachment::Detached => "ui-input-group--detached",
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            InputGroupAttachment::Attached => "attached",
            InputGroupAttachment::Detached => "detached",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InputGroupStateInput {
    pub disabled: bool,
    pub invalid: bool,
    pub attached: bool,
    pub has_start_content: bool,
    pub has_end_content: bool,
    pub has_custom_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InputGroupState {
    pub phase: InputGroupPhase,
    pub attachment: InputGroupAttachment,
    pub phase_class: &'static str,
    pub attachment_class: &'static str,
    pub phase_attr: &'static str,
    pub attachment_attr: &'static str,
    pub is_enabled: bool,
    pub is_disabled: bool,
    pub is_invalid: bool,
    pub is_attached: bool,
    pub is_detached: bool,
    pub has_start_content: bool,
    pub has_end_content: bool,
    pub has_custom_label: bool,
    pub has_custom_class_name: bool,
    pub label_source_class: &'static str,
    pub label_source_attr: &'static str,
    pub class_source_attr: &'static str,
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    let Some(value) = value else {
        return (DEFAULT_ARIA_LABEL.into(), false);
    };

    let trimmed = value.trim();
    if trimmed.is_empty() {
        return (DEFAULT_ARIA_LABEL.into(), false);
    }

    let label = trimmed.into();
    let has_custom_label = label != DEFAULT_ARIA_LABEL;

    (label, has_custom_label)
}

pub fn resolve_state(input: InputGroupStateInput) -> InputGroupState {
    let phase = if input.disabled {
        InputGroupPhase::Disabled
    } else {
        InputGroupPhase::Enabled
    };

    let attachment = if input.attached {
        InputGroupAttachment::Attached
    } else {
        InputGroupAttachment::Detached
    };

    let (label_source_class, label_source_attr) = if input.has_custom_label {
        ("ui-input-group--label-custom", "custom")
    } else {
        ("ui-input-group--label-default", "default")
    };

    let class_source_attr = if input.has_custom_class_name {
        "custom"
    } else {
        "default"
    };

    InputGroupState {
        phase,
        attachment,
        phase_class: phase.class_name(),
        attachment_class: attachment.class_name(),
        phase_attr: phase.as_str(),
        attachment_attr: attachment.as_str(),
        is_enabled: !input.disabled,
        is_disabled: input.disabled,
        is_invalid: input.invalid,
        is_attached: input.attached,
        is_detached: !input.attached,
        has_start_content: input.has_start_content,
        has_end_content: input.has_end_content,
        has_custom_label: input.has_custom_label,
        has_custom_class_name: input.has_custom_class_name,
        label_source_class,
        label_source_attr,
        class_source_attr,
    }
}

#[cfg(test)]
#[path = "test/input_group.rs"]
mod tests;
