use super::{SwitchGroupIds, SwitchGroupState, SwitchGroupStateInput};

pub const DEFAULT_LABEL: &str = "Switches";
pub const DEFAULT_ARIA_LABEL: &str = "SwitchGroup";
pub const DEFAULT_ERROR_MESSAGE: &str = "Invalid selection";

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

pub fn resolve_ids(id_base: String) -> SwitchGroupIds {
    let normalized = id_base.trim();
    let root_id = if normalized.is_empty() {
        "switch-group".to_string()
    } else {
        normalized.to_string()
    };

    SwitchGroupIds {
        label_id: format!("{root_id}-label"),
        description_id: format!("{root_id}-description"),
        error_id: format!("{root_id}-error"),
        root_id,
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_LABEL.to_string(), false)
}

pub fn normalize_description(value: Option<String>) -> Option<String> {
    normalize_optional_text(value)
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

pub fn compose_class_name(base_class_name: Option<String>, state: SwitchGroupState) -> String {
    let mut classes = vec![
        "ui-switch-group".to_string(),
        state.orientation_class.to_string(),
        state.tone_class.to_string(),
    ];

    if state.is_required {
        classes.push("ui-switch-group--required".to_string());
    }

    if state.is_disabled {
        classes.push("ui-switch-group--disabled".to_string());
    }

    if state.is_invalid {
        classes.push("ui-switch-group--invalid".to_string());
    }

    if state.has_description {
        classes.push("ui-switch-group--has-description".to_string());
    }

    if state.shows_error {
        classes.push("ui-switch-group--has-error".to_string());
    }

    if state.label_source_attr == "custom" {
        classes.push("ui-switch-group--label-custom".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-switch-group--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::SwitchGroupStateInput;
    use super::*;

    #[test]
    fn orientation_and_tone_contracts_are_stable() {
        assert_eq!(
            SwitchGroupOrientation::Vertical.class_name(),
            "ui-switch-group--orientation-vertical"
        );
        assert_eq!(
            SwitchGroupOrientation::Horizontal.class_name(),
            "ui-switch-group--orientation-horizontal"
        );

        assert_eq!(SwitchGroupOrientation::Vertical.as_attr(), "vertical");
        assert_eq!(SwitchGroupOrientation::Horizontal.as_attr(), "horizontal");

        assert_eq!(
            SwitchGroupTone::Default.class_name(),
            "ui-switch-group--tone-default"
        );
        assert_eq!(
            SwitchGroupTone::Muted.class_name(),
            "ui-switch-group--tone-muted"
        );

        assert_eq!(SwitchGroupTone::Default.as_attr(), "default");
        assert_eq!(SwitchGroupTone::Muted.as_attr(), "muted");
    }

    #[test]
    fn resolve_ids_uses_trimmed_or_fallback_base() {
        assert_eq!(
            resolve_ids("  notifications  ".to_string()),
            SwitchGroupIds {
                root_id: "notifications".to_string(),
                label_id: "notifications-label".to_string(),
                description_id: "notifications-description".to_string(),
                error_id: "notifications-error".to_string(),
            }
        );

        assert_eq!(
            resolve_ids("   ".to_string()).root_id,
            "switch-group".to_string()
        );
    }

    #[test]
    fn normalize_helpers_trim_and_fallback() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  digest  ".to_string())),
            Some("digest".to_string())
        );

        let (label, custom_label) = normalize_label(Some("  Alerts  ".to_string()));
        assert_eq!(label, "Alerts");
        assert!(custom_label);

        let (label, custom_label) = normalize_label(None);
        assert_eq!(label, DEFAULT_LABEL);
        assert!(!custom_label);

        assert_eq!(
            normalize_description(Some("  Switch group helper  ".to_string())),
            Some("Switch group helper".to_string())
        );

        let (aria, custom_aria) = normalize_aria_label(Some("  Channels  ".to_string()));
        assert_eq!(aria, "Channels");
        assert!(custom_aria);

        let (aria, custom_aria) = normalize_aria_label(None);
        assert_eq!(aria, DEFAULT_ARIA_LABEL);
        assert!(!custom_aria);

        let (error, custom_error) =
            normalize_error_message(Some("  Choose one  ".to_string()), true);
        assert_eq!(error, Some("Choose one".to_string()));
        assert!(custom_error);

        let (error, custom_error) = normalize_error_message(None, true);
        assert_eq!(error, Some(DEFAULT_ERROR_MESSAGE.to_string()));
        assert!(!custom_error);

        let (error, custom_error) = normalize_error_message(None, false);
        assert_eq!(error, None);
        assert!(!custom_error);
    }

    #[test]
    fn resolve_state_tracks_markers_and_sources() {
        let state = resolve_state(SwitchGroupStateInput {
            orientation: SwitchGroupOrientation::Horizontal,
            tone: SwitchGroupTone::Muted,
            required: true,
            disabled: false,
            invalid: true,
            has_label: true,
            has_description: true,
            has_error_message: true,
            has_custom_label: true,
            has_custom_aria_label: false,
            has_custom_error_message: true,
            has_custom_class_name: true,
        });

        assert_eq!(state.orientation_attr, "horizontal");
        assert_eq!(state.tone_attr, "muted");
        assert!(state.is_required);
        assert!(state.is_invalid);
        assert!(state.has_messages);
        assert!(state.shows_error);
        assert_eq!(state.message_kind_attr, "error");
        assert_eq!(state.data_state_attr, "invalid");
        assert_eq!(state.label_source_attr, "custom");
        assert_eq!(state.aria_source_attr, "default");
        assert_eq!(state.error_source_attr, "custom");
        assert_eq!(state.class_source_attr, "custom");
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("docs-switch-group".to_string()),
            resolve_state(SwitchGroupStateInput {
                orientation: SwitchGroupOrientation::Horizontal,
                tone: SwitchGroupTone::Muted,
                required: true,
                disabled: true,
                invalid: true,
                has_label: true,
                has_description: true,
                has_error_message: true,
                has_custom_label: true,
                has_custom_aria_label: false,
                has_custom_error_message: false,
                has_custom_class_name: true,
            }),
        );

        for token in [
            "ui-switch-group",
            "ui-switch-group--orientation-horizontal",
            "ui-switch-group--tone-muted",
            "ui-switch-group--required",
            "ui-switch-group--disabled",
            "ui-switch-group--invalid",
            "ui-switch-group--has-description",
            "ui-switch-group--has-error",
            "ui-switch-group--label-custom",
            "ui-switch-group--custom-class",
            "docs-switch-group",
        ] {
            assert!(
                class_name.contains(token),
                "composed class should include `{token}`"
            );
        }
    }
}
