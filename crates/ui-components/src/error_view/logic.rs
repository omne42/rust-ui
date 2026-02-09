use crate::error_view::{ErrorViewState, ErrorViewStateInput};

pub const DEFAULT_ARIA_LABEL: &str = "Error view";
pub const DEFAULT_MESSAGE: &str = "Invalid value";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ErrorViewTone {
    Neutral,
    #[default]
    Negative,
}

impl ErrorViewTone {
    pub fn class_name(self) -> &'static str {
        match self {
            ErrorViewTone::Neutral => "ui-error-view--tone-neutral",
            ErrorViewTone::Negative => "ui-error-view--tone-negative",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            ErrorViewTone::Neutral => "neutral",
            ErrorViewTone::Negative => "negative",
        }
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_message(value: Option<String>) -> (String, bool) {
    if let Some(message) = normalize_optional_text(value) {
        return (message, true);
    }

    (DEFAULT_MESSAGE.to_string(), false)
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.to_string(), false)
}

pub fn resolve_state(input: ErrorViewStateInput) -> ErrorViewState {
    let (state_class, state_attr) = if input.is_invalid {
        ("ui-error-view--visible", "visible")
    } else {
        ("ui-error-view--hidden", "hidden")
    };

    let message_source_attr = if input.has_children {
        "none"
    } else if input.has_custom_message {
        "custom"
    } else {
        "default"
    };

    let content_attr = if input.has_children {
        "children"
    } else {
        "text"
    };

    ErrorViewState {
        tone: input.tone,
        tone_class: input.tone.class_name(),
        tone_attr: input.tone.as_attr(),
        is_visible: input.is_invalid,
        is_hidden: !input.is_invalid,
        state_class,
        state_attr,
        is_compact: input.compact,
        is_bordered: input.bordered,
        has_icon: input.has_icon,
        has_actions: input.has_actions,
        has_children: input.has_children,
        content_attr,
        message_source_attr,
        aria_source_attr: if input.has_custom_aria_label {
            "custom"
        } else {
            "default"
        },
        class_source_attr: if input.has_custom_class_name {
            "custom"
        } else {
            "default"
        },
        motion_source_attr: if input.has_custom_motion {
            "custom"
        } else {
            "default"
        },
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: ErrorViewState) -> String {
    let mut classes = vec![
        "ui-error-view".to_string(),
        state.tone_class.to_string(),
        state.state_class.to_string(),
    ];

    if state.is_compact {
        classes.push("ui-error-view--compact".to_string());
    }

    if state.is_bordered {
        classes.push("ui-error-view--bordered".to_string());
    }

    if state.has_icon {
        classes.push("ui-error-view--with-icon".to_string());
    }

    if state.has_actions {
        classes.push("ui-error-view--with-actions".to_string());
    }

    if state.has_children {
        classes.push("ui-error-view--with-children".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-error-view--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error_view::ErrorViewStateInput;

    #[test]
    fn tone_contract_is_stable() {
        assert_eq!(
            ErrorViewTone::Negative.class_name(),
            "ui-error-view--tone-negative"
        );
        assert_eq!(
            ErrorViewTone::Neutral.class_name(),
            "ui-error-view--tone-neutral"
        );
        assert_eq!(ErrorViewTone::Negative.as_attr(), "negative");
        assert_eq!(ErrorViewTone::Neutral.as_attr(), "neutral");
    }

    #[test]
    fn normalize_helpers_trim_and_fallback() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("\n\t  ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  bad input  ".to_string())),
            Some("bad input".to_string())
        );

        let (message, custom_message) = normalize_message(Some("  Email invalid  ".to_string()));
        assert_eq!(message, "Email invalid");
        assert!(custom_message);

        let (message, custom_message) = normalize_message(None);
        assert_eq!(message, DEFAULT_MESSAGE);
        assert!(!custom_message);

        let (label, custom_label) = normalize_aria_label(Some("  field error  ".to_string()));
        assert_eq!(label, "field error");
        assert!(custom_label);

        let (label, custom_label) = normalize_aria_label(None);
        assert_eq!(label, DEFAULT_ARIA_LABEL);
        assert!(!custom_label);
    }

    #[test]
    fn resolve_state_tracks_visibility_and_sources() {
        let state = resolve_state(ErrorViewStateInput {
            tone: ErrorViewTone::Negative,
            is_invalid: true,
            compact: true,
            bordered: false,
            has_icon: true,
            has_actions: false,
            has_children: false,
            has_custom_message: true,
            has_custom_aria_label: false,
            has_custom_class_name: true,
            has_custom_motion: true,
        });

        assert!(state.is_visible);
        assert_eq!(state.state_attr, "visible");
        assert_eq!(state.message_source_attr, "custom");
        assert_eq!(state.aria_source_attr, "default");
        assert_eq!(state.class_source_attr, "custom");
        assert_eq!(state.motion_source_attr, "custom");
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("docs-error-view".to_string()),
            resolve_state(ErrorViewStateInput {
                tone: ErrorViewTone::Neutral,
                is_invalid: false,
                compact: true,
                bordered: true,
                has_icon: true,
                has_actions: true,
                has_children: true,
                has_custom_message: false,
                has_custom_aria_label: true,
                has_custom_class_name: true,
                has_custom_motion: false,
            }),
        );

        for token in [
            "ui-error-view",
            "ui-error-view--tone-neutral",
            "ui-error-view--hidden",
            "ui-error-view--compact",
            "ui-error-view--bordered",
            "ui-error-view--with-icon",
            "ui-error-view--with-actions",
            "ui-error-view--with-children",
            "ui-error-view--custom-class",
            "docs-error-view",
        ] {
            assert!(
                class_name.contains(token),
                "composed class should include `{token}`"
            );
        }
    }
}
