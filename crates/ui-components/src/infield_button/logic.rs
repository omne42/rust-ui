use crate::infield_button::{InfieldButtonState, InfieldButtonStateInput};

pub const DEFAULT_ARIA_LABEL: &str = "InfieldButton";

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

pub fn resolve_state(input: InfieldButtonStateInput) -> InfieldButtonState {
    let data_state_attr = if input.disabled && input.invalid {
        "invalid-disabled"
    } else if input.disabled {
        "disabled"
    } else if input.invalid {
        "invalid"
    } else if input.forced_active {
        "active"
    } else if input.quiet {
        "quiet"
    } else {
        "default"
    };

    InfieldButtonState {
        is_quiet: input.quiet,
        is_invalid: input.invalid,
        is_disabled: input.disabled,
        is_forced_active: input.forced_active,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_press_handler: input.has_custom_press_handler,
        quiet_attr: if input.quiet { "true" } else { "false" },
        invalid_attr: if input.invalid { "true" } else { "false" },
        disabled_attr: if input.disabled { "true" } else { "false" },
        active_mode_attr: if input.forced_active {
            "forced"
        } else {
            "interactive"
        },
        data_state_attr,
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
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: InfieldButtonState) -> String {
    let mut classes = vec!["ui-infield-button".to_string()];

    if state.is_quiet {
        classes.push("ui-infield-button--quiet".to_string());
    }

    if state.is_invalid {
        classes.push("ui-infield-button--invalid".to_string());
    }

    if state.is_disabled {
        classes.push("ui-infield-button--disabled".to_string());
    }

    if state.is_forced_active {
        classes.push("ui-infield-button--active".to_string());
    }

    if state.has_custom_press_handler {
        classes.push("ui-infield-button--custom-handler".to_string());
    }

    if state.has_custom_aria_label {
        classes.push("ui-infield-button--custom-aria-label".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-infield-button--custom-class".to_string());
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
    fn normalize_optional_text_trims_and_drops_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  docs-infield-button  ".to_string())),
            Some("docs-infield-button".to_string())
        );
    }

    #[test]
    fn normalize_aria_label_uses_fallback_when_missing() {
        let (label, custom) = normalize_aria_label(Some("  Open picker  ".to_string()));
        assert_eq!(label, "Open picker");
        assert!(custom);

        let (label, custom) = normalize_aria_label(None);
        assert_eq!(label, DEFAULT_ARIA_LABEL);
        assert!(!custom);
    }

    #[test]
    fn resolve_state_tracks_quiet_invalid_disabled_sources() {
        let state = resolve_state(InfieldButtonStateInput {
            quiet: true,
            invalid: true,
            disabled: false,
            forced_active: true,
            has_custom_aria_label: true,
            has_custom_class_name: false,
            has_custom_press_handler: true,
        });

        assert!(state.is_quiet);
        assert!(state.is_invalid);
        assert!(!state.is_disabled);
        assert!(state.is_forced_active);
        assert_eq!(state.quiet_attr, "true");
        assert_eq!(state.invalid_attr, "true");
        assert_eq!(state.disabled_attr, "false");
        assert_eq!(state.active_mode_attr, "forced");
        assert_eq!(state.data_state_attr, "invalid");
        assert_eq!(state.aria_source_attr, "custom");
        assert_eq!(state.class_source_attr, "default");
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("docs-infield-button-custom".to_string()),
            resolve_state(InfieldButtonStateInput {
                quiet: true,
                invalid: true,
                disabled: true,
                forced_active: true,
                has_custom_aria_label: true,
                has_custom_class_name: true,
                has_custom_press_handler: true,
            }),
        );

        for expected in [
            "ui-infield-button",
            "ui-infield-button--quiet",
            "ui-infield-button--invalid",
            "ui-infield-button--disabled",
            "ui-infield-button--active",
            "ui-infield-button--custom-handler",
            "ui-infield-button--custom-aria-label",
            "ui-infield-button--custom-class",
            "docs-infield-button-custom",
        ] {
            assert!(
                class_name.contains(expected),
                "expected composed class name to contain `{expected}`"
            );
        }
    }
}
