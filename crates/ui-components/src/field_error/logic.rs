use crate::field_error::{FieldErrorState, FieldErrorStateInput};

pub const DEFAULT_ARIA_LABEL: &str = "FieldError";
pub const DEFAULT_MESSAGE: &str = "Invalid value";

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

pub fn normalize_message(value: Option<String>, visible: bool) -> (Option<String>, bool) {
    if !visible {
        return (None, false);
    }

    if let Some(message) = normalize_optional_text(value) {
        return (Some(message), true);
    }

    (Some(DEFAULT_MESSAGE.to_string()), false)
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

    let data_state_attr = if !is_visible {
        "hidden"
    } else if input.disabled {
        "disabled"
    } else {
        "visible"
    };

    let aria_source_attr = if input.has_custom_aria_label {
        "custom"
    } else {
        "default"
    };

    let message_source_attr = if !input.has_message {
        "none"
    } else if input.has_custom_message {
        "custom"
    } else {
        "default"
    };

    let class_source_attr = if input.has_custom_class_name {
        "custom"
    } else {
        "default"
    };

    FieldErrorState {
        tone,
        tone_class: tone.class_name(),
        tone_attr: tone.as_attr(),
        is_visible,
        is_disabled: input.disabled,
        show_icon,
        has_message: input.has_message,
        data_state_attr,
        aria_source_attr,
        message_source_attr,
        class_source_attr,
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: FieldErrorState) -> String {
    let mut classes = vec!["ui-field-error".to_string(), state.tone_class.to_string()];

    if state.is_visible {
        classes.push("ui-field-error--visible".to_string());
    }

    if state.is_disabled {
        classes.push("ui-field-error--disabled".to_string());
    }

    if state.show_icon {
        classes.push("ui-field-error--with-icon".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-field-error--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field_error::FieldErrorStateInput;

    #[test]
    fn tone_contract_is_stable() {
        assert_eq!(
            FieldErrorTone::Auto.class_name(),
            "ui-field-error--tone-auto"
        );
        assert_eq!(
            FieldErrorTone::Neutral.class_name(),
            "ui-field-error--tone-neutral"
        );
        assert_eq!(
            FieldErrorTone::Negative.class_name(),
            "ui-field-error--tone-negative"
        );

        assert_eq!(FieldErrorTone::Auto.as_attr(), "auto");
        assert_eq!(FieldErrorTone::Neutral.as_attr(), "neutral");
        assert_eq!(FieldErrorTone::Negative.as_attr(), "negative");
    }

    #[test]
    fn normalize_helpers_trim_and_fallback() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  bad email  ".to_string())),
            Some("bad email".to_string())
        );

        let (label, custom_label) = normalize_aria_label(Some("  Email error  ".to_string()));
        assert_eq!(label, "Email error");
        assert!(custom_label);

        let (label, custom_label) = normalize_aria_label(None);
        assert_eq!(label, DEFAULT_ARIA_LABEL);
        assert!(!custom_label);

        let (message, custom_message) = normalize_message(Some("  Required  ".to_string()), true);
        assert_eq!(message, Some("Required".to_string()));
        assert!(custom_message);

        let (message, custom_message) = normalize_message(None, true);
        assert_eq!(message, Some(DEFAULT_MESSAGE.to_string()));
        assert!(!custom_message);

        let (message, custom_message) = normalize_message(Some("ignored".to_string()), false);
        assert_eq!(message, None);
        assert!(!custom_message);
    }

    #[test]
    fn resolve_state_tracks_visibility_and_sources() {
        let state = resolve_state(FieldErrorStateInput {
            tone: FieldErrorTone::Auto,
            visible: true,
            disabled: false,
            show_icon: true,
            has_message: true,
            has_custom_aria_label: true,
            has_custom_message: false,
            has_custom_class_name: false,
        });

        assert!(state.is_visible);
        assert_eq!(state.tone_attr, "negative");
        assert_eq!(state.data_state_attr, "visible");
        assert_eq!(state.aria_source_attr, "custom");
        assert_eq!(state.message_source_attr, "default");
        assert_eq!(state.class_source_attr, "default");
        assert!(state.show_icon);
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("docs-field-error".to_string()),
            resolve_state(FieldErrorStateInput {
                tone: FieldErrorTone::Negative,
                visible: true,
                disabled: true,
                show_icon: true,
                has_message: true,
                has_custom_aria_label: false,
                has_custom_message: true,
                has_custom_class_name: true,
            }),
        );

        for token in [
            "ui-field-error",
            "ui-field-error--tone-negative",
            "ui-field-error--visible",
            "ui-field-error--disabled",
            "ui-field-error--with-icon",
            "ui-field-error--custom-class",
            "docs-field-error",
        ] {
            assert!(
                class_name.contains(token),
                "composed class should contain `{token}`"
            );
        }
    }
}
