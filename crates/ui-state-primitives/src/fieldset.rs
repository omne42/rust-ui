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

    let message_kind_attr = if input.has_error_message {
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
    } else if input.orientation == FieldsetOrientation::Horizontal {
        "horizontal"
    } else if input.tone == FieldsetTone::Muted {
        "muted"
    } else {
        "default"
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
        message_kind_attr,
        data_state_attr,
        aria_source_attr,
        error_source_attr,
        class_source_attr,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orientation_and_tone_contracts_are_stable() {
        assert_eq!(
            FieldsetOrientation::Vertical.class_name(),
            "ui-fieldset--orientation-vertical"
        );
        assert_eq!(
            FieldsetOrientation::Horizontal.class_name(),
            "ui-fieldset--orientation-horizontal"
        );
        assert_eq!(FieldsetOrientation::Vertical.as_attr(), "vertical");
        assert_eq!(FieldsetOrientation::Horizontal.as_attr(), "horizontal");

        assert_eq!(
            FieldsetTone::Default.class_name(),
            "ui-fieldset--tone-default"
        );
        assert_eq!(FieldsetTone::Muted.class_name(), "ui-fieldset--tone-muted");
        assert_eq!(FieldsetTone::Default.as_attr(), "default");
        assert_eq!(FieldsetTone::Muted.as_attr(), "muted");
    }

    #[test]
    fn normalize_helpers_trim_and_fallback() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("   \n\t ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  Billing details  ".to_string())),
            Some("Billing details".to_string())
        );

        let (label, custom) = normalize_aria_label(Some("  Payment group  ".to_string()));
        assert_eq!(label, "Payment group");
        assert!(custom);

        let (label, custom) = normalize_aria_label(None);
        assert_eq!(label, DEFAULT_ARIA_LABEL);
        assert!(!custom);
    }

    #[test]
    fn error_message_normalization_respects_invalid_state() {
        let (message, custom) =
            normalize_error_message(Some("  Missing value  ".to_string()), true);
        assert_eq!(message, Some("Missing value".to_string()));
        assert!(custom);

        let (message, custom) = normalize_error_message(None, true);
        assert_eq!(message, Some(DEFAULT_ERROR_MESSAGE.into()));
        assert!(!custom);

        let (message, custom) = normalize_error_message(Some("Ignored".to_string()), false);
        assert_eq!(message, None);
        assert!(!custom);
    }

    #[test]
    fn resolve_state_tracks_sources_and_priorities() {
        let state = resolve_state(FieldsetStateInput {
            orientation: FieldsetOrientation::Horizontal,
            tone: FieldsetTone::Muted,
            required: true,
            disabled: false,
            invalid: true,
            has_legend: true,
            has_description: false,
            has_error_message: true,
            has_actions: true,
            has_custom_aria_label: true,
            has_custom_error_message: false,
            has_custom_class_name: true,
        });

        assert_eq!(state.orientation_attr, "horizontal");
        assert_eq!(state.tone_attr, "muted");
        assert_eq!(state.message_kind_attr, "error");
        assert_eq!(state.data_state_attr, "invalid");
        assert_eq!(state.aria_source_attr, "custom");
        assert_eq!(state.error_source_attr, "default");
        assert_eq!(state.class_source_attr, "custom");
        assert!(state.has_actions);
    }
}
