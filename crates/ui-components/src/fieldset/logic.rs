use crate::fieldset::{FieldsetState, FieldsetStateInput};

pub const DEFAULT_ARIA_LABEL: &str = "Fieldset";
pub const DEFAULT_ERROR_MESSAGE: &str = "Invalid value";

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

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.to_string(), false)
}

pub fn normalize_error_message(value: Option<String>, invalid: bool) -> (Option<String>, bool) {
    if !invalid {
        return (None, false);
    }

    if let Some(message) = normalize_optional_text(value) {
        return (Some(message), true);
    }

    (Some(DEFAULT_ERROR_MESSAGE.to_string()), false)
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

pub fn compose_class_name(base_class_name: Option<String>, state: FieldsetState) -> String {
    let mut classes = vec![
        "ui-fieldset".to_string(),
        state.orientation_class.to_string(),
        state.tone_class.to_string(),
    ];

    if state.is_required {
        classes.push("ui-fieldset--required".to_string());
    }

    if state.is_disabled {
        classes.push("ui-fieldset--disabled".to_string());
    }

    if state.is_invalid {
        classes.push("ui-fieldset--invalid".to_string());
    }

    if state.has_legend {
        classes.push("ui-fieldset--has-legend".to_string());
    }

    if state.has_description {
        classes.push("ui-fieldset--has-description".to_string());
    }

    if state.has_error_message {
        classes.push("ui-fieldset--has-error".to_string());
    }

    if state.has_actions {
        classes.push("ui-fieldset--has-actions".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-fieldset--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
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
        assert_eq!(message, Some(DEFAULT_ERROR_MESSAGE.to_string()));
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

    #[test]
    fn compose_class_name_appends_state_and_custom_class() {
        let state = resolve_state(FieldsetStateInput {
            orientation: FieldsetOrientation::Vertical,
            tone: FieldsetTone::Default,
            required: true,
            disabled: true,
            invalid: false,
            has_legend: true,
            has_description: true,
            has_error_message: false,
            has_actions: true,
            has_custom_aria_label: false,
            has_custom_error_message: false,
            has_custom_class_name: true,
        });

        let class_name = compose_class_name(Some("docs-fieldset-custom".to_string()), state);

        for expected in [
            "ui-fieldset",
            "ui-fieldset--orientation-vertical",
            "ui-fieldset--tone-default",
            "ui-fieldset--required",
            "ui-fieldset--disabled",
            "ui-fieldset--has-legend",
            "ui-fieldset--has-description",
            "ui-fieldset--has-actions",
            "ui-fieldset--custom-class",
            "docs-fieldset-custom",
        ] {
            assert!(
                class_name.contains(expected),
                "expected class `{expected}` in `{class_name}`"
            );
        }
    }
}
