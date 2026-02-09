use crate::command_dialog::{DEFAULT_ID_BASE, DEFAULT_TITLE};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CommandDialogStateInput {
    pub is_open: bool,
    pub has_description: bool,
    pub close_on_action: bool,
    pub disabled: bool,
    pub is_controlled: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CommandDialogState {
    pub is_open: bool,
    pub has_description: bool,
    pub close_on_action: bool,
    pub disabled: bool,
    pub enabled: bool,
    pub is_controlled: bool,
    pub is_uncontrolled: bool,
    pub has_custom_class_name: bool,
    pub state_attr: &'static str,
    pub description_attr: &'static str,
    pub close_on_action_attr: &'static str,
    pub class_source_attr: &'static str,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_id_base(value: Option<String>) -> String {
    normalize_optional_text(value).unwrap_or_else(|| DEFAULT_ID_BASE.to_string())
}

pub fn normalize_title(value: Option<String>) -> String {
    normalize_optional_text(value).unwrap_or_else(|| DEFAULT_TITLE.to_string())
}

pub fn resolve_state(input: CommandDialogStateInput) -> CommandDialogState {
    let enabled = !input.disabled;
    let is_uncontrolled = !input.is_controlled;

    CommandDialogState {
        is_open: input.is_open,
        has_description: input.has_description,
        close_on_action: input.close_on_action,
        disabled: input.disabled,
        enabled,
        is_controlled: input.is_controlled,
        is_uncontrolled,
        has_custom_class_name: input.has_custom_class_name,
        state_attr: if input.is_open { "open" } else { "closed" },
        description_attr: if input.has_description {
            "present"
        } else {
            "absent"
        },
        close_on_action_attr: if input.close_on_action {
            "true"
        } else {
            "false"
        },
        class_source_attr: if input.has_custom_class_name {
            "custom"
        } else {
            "default"
        },
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: CommandDialogState) -> String {
    let mut classes = vec!["ui-command-dialog".to_string()];

    if state.is_open {
        classes.push("ui-command-dialog--open".to_string());
    } else {
        classes.push("ui-command-dialog--closed".to_string());
    }

    if state.close_on_action {
        classes.push("ui-command-dialog--close-on-action".to_string());
    } else {
        classes.push("ui-command-dialog--persistent".to_string());
    }

    if state.disabled {
        classes.push("ui-command-dialog--disabled".to_string());
    }

    if state.is_controlled {
        classes.push("ui-command-dialog--controlled".to_string());
    } else {
        classes.push("ui-command-dialog--uncontrolled".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-command-dialog--custom-class".to_string());
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
    fn normalize_helpers_trim_and_fallback() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("  ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some(" docs-dialog ".to_string())),
            Some("docs-dialog".to_string())
        );

        assert_eq!(normalize_id_base(None), DEFAULT_ID_BASE);
        assert_eq!(
            normalize_id_base(Some(" docs-command-dialog ".to_string())),
            "docs-command-dialog"
        );

        assert_eq!(normalize_title(None), DEFAULT_TITLE);
        assert_eq!(
            normalize_title(Some(" Quick Actions ".to_string())),
            "Quick Actions"
        );
    }

    #[test]
    fn resolve_state_tracks_flags() {
        let state = resolve_state(CommandDialogStateInput {
            is_open: true,
            has_description: true,
            close_on_action: false,
            disabled: true,
            is_controlled: true,
            has_custom_class_name: true,
        });

        assert_eq!(state.state_attr, "open");
        assert_eq!(state.description_attr, "present");
        assert_eq!(state.close_on_action_attr, "false");
        assert!(!state.enabled);
        assert!(state.is_controlled);
        assert_eq!(state.class_source_attr, "custom");
    }

    #[test]
    fn compose_class_name_contains_state_markers() {
        let class_name = compose_class_name(
            Some("docs-command-dialog".to_string()),
            resolve_state(CommandDialogStateInput {
                is_open: false,
                has_description: false,
                close_on_action: true,
                disabled: false,
                is_controlled: false,
                has_custom_class_name: true,
            }),
        );

        for token in [
            "ui-command-dialog",
            "ui-command-dialog--closed",
            "ui-command-dialog--close-on-action",
            "ui-command-dialog--uncontrolled",
            "ui-command-dialog--custom-class",
            "docs-command-dialog",
        ] {
            assert!(
                class_name.contains(token),
                "composed class should include `{token}`"
            );
        }
    }
}
