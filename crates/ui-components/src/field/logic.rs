use crate::field::{FieldState, FieldStateInput};

pub const DEFAULT_ARIA_LABEL: &str = "Field";
pub const DEFAULT_ERROR_MESSAGE: &str = "Invalid value";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FieldOrientation {
    #[default]
    Vertical,
    Horizontal,
}

impl FieldOrientation {
    pub fn class_name(self) -> &'static str {
        match self {
            FieldOrientation::Vertical => "ui-field--orientation-vertical",
            FieldOrientation::Horizontal => "ui-field--orientation-horizontal",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            FieldOrientation::Vertical => "vertical",
            FieldOrientation::Horizontal => "horizontal",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FieldTone {
    #[default]
    Default,
    Muted,
}

impl FieldTone {
    pub fn class_name(self) -> &'static str {
        match self {
            FieldTone::Default => "ui-field--tone-default",
            FieldTone::Muted => "ui-field--tone-muted",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            FieldTone::Default => "default",
            FieldTone::Muted => "muted",
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

pub fn resolve_state(input: FieldStateInput) -> FieldState {
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
    } else if input.orientation == FieldOrientation::Horizontal {
        "horizontal"
    } else if input.tone == FieldTone::Muted {
        "muted"
    } else {
        "default"
    };

    FieldState {
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
        message_kind_attr,
        data_state_attr,
        aria_source_attr,
        error_source_attr,
        class_source_attr,
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: FieldState) -> String {
    let mut classes = vec![
        "ui-field".to_string(),
        state.orientation_class.to_string(),
        state.tone_class.to_string(),
    ];

    if state.is_required {
        classes.push("ui-field--required".to_string());
    }

    if state.is_disabled {
        classes.push("ui-field--disabled".to_string());
    }

    if state.is_invalid {
        classes.push("ui-field--invalid".to_string());
    }

    if state.has_label {
        classes.push("ui-field--has-label".to_string());
    }

    if state.has_description {
        classes.push("ui-field--has-description".to_string());
    }

    if state.has_error_message {
        classes.push("ui-field--has-error".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-field--custom-class".to_string());
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
    fn field_enums_map_to_stable_class_and_attr_contracts() {
        assert_eq!(
            FieldOrientation::Vertical.class_name(),
            "ui-field--orientation-vertical"
        );
        assert_eq!(FieldOrientation::Horizontal.as_attr(), "horizontal");

        assert_eq!(FieldTone::Default.class_name(), "ui-field--tone-default");
        assert_eq!(FieldTone::Muted.as_attr(), "muted");
    }

    #[test]
    fn normalize_optional_text_trims_and_drops_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  docs-field  ".to_string())),
            Some("docs-field".to_string())
        );
    }

    #[test]
    fn normalize_helpers_apply_expected_fallbacks() {
        let (aria_label, custom_aria_label) =
            normalize_aria_label(Some("  Profile Field  ".to_string()));
        assert_eq!(aria_label, "Profile Field");
        assert!(custom_aria_label);

        let (aria_label, custom_aria_label) = normalize_aria_label(None);
        assert_eq!(aria_label, DEFAULT_ARIA_LABEL);
        assert!(!custom_aria_label);

        let (error_message, custom_error_message) =
            normalize_error_message(Some("  Required value  ".to_string()), true);
        assert_eq!(error_message, Some("Required value".to_string()));
        assert!(custom_error_message);

        let (error_message, custom_error_message) = normalize_error_message(None, true);
        assert_eq!(error_message, Some(DEFAULT_ERROR_MESSAGE.to_string()));
        assert!(!custom_error_message);

        let (error_message, custom_error_message) =
            normalize_error_message(Some("ignored".to_string()), false);
        assert_eq!(error_message, None);
        assert!(!custom_error_message);
    }

    #[test]
    fn resolve_state_tracks_flags_sources_and_message_kind() {
        let state = resolve_state(FieldStateInput {
            orientation: FieldOrientation::Horizontal,
            tone: FieldTone::Muted,
            required: true,
            disabled: false,
            invalid: true,
            has_label: true,
            has_description: true,
            has_error_message: true,
            has_custom_aria_label: true,
            has_custom_error_message: false,
            has_custom_class_name: false,
        });

        assert_eq!(state.orientation_attr, "horizontal");
        assert_eq!(state.tone_attr, "muted");
        assert!(state.is_required);
        assert!(state.is_invalid);
        assert_eq!(state.message_kind_attr, "error");
        assert_eq!(state.error_source_attr, "default");
        assert_eq!(state.aria_source_attr, "custom");
        assert_eq!(state.class_source_attr, "default");
        assert_eq!(state.data_state_attr, "invalid");
    }

    #[test]
    fn compose_class_name_includes_state_and_custom_markers() {
        let state = resolve_state(FieldStateInput {
            orientation: FieldOrientation::Vertical,
            tone: FieldTone::Default,
            required: true,
            disabled: false,
            invalid: false,
            has_label: true,
            has_description: true,
            has_error_message: false,
            has_custom_aria_label: false,
            has_custom_error_message: false,
            has_custom_class_name: true,
        });

        let class_name = compose_class_name(Some("docs-field-custom".to_string()), state);

        for token in [
            "ui-field",
            "ui-field--orientation-vertical",
            "ui-field--tone-default",
            "ui-field--required",
            "ui-field--has-label",
            "ui-field--has-description",
            "ui-field--custom-class",
            "docs-field-custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class should include `{token}`"
            );
        }
    }
}
