use crate::sidenav::{SidenavState, SidenavStateInput};

pub const DEFAULT_ARIA_LABEL: &str = "Sidenav";
pub const DEFAULT_TRIGGER_LABEL: &str = "Toggle sidenav";
pub const DEFAULT_SHORTCUT_KEY: &str = "b";

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

pub fn normalize_trigger_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_TRIGGER_LABEL.to_string(), false)
}

pub fn normalize_shortcut_key(value: Option<String>, enable_shortcut: bool) -> (String, bool) {
    if !enable_shortcut {
        return (String::new(), false);
    }

    if let Some(value) = normalize_optional_text(value) {
        let normalized = value.to_ascii_lowercase();
        let mut chars = normalized.chars();
        let first = chars.next().unwrap_or('b');
        return (first.to_string(), true);
    }

    (DEFAULT_SHORTCUT_KEY.to_string(), false)
}

pub fn resolve_state(input: SidenavStateInput) -> SidenavState {
    SidenavState {
        is_disabled: input.disabled,
        show_trigger: input.show_trigger,
        enable_shortcut: input.enable_shortcut,
        is_controlled: input.is_controlled,
        initial_open: input.initial_open,
        has_custom_shortcut_key: input.has_custom_shortcut_key,
        has_custom_trigger_label: input.has_custom_trigger_label,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_open_handler: input.has_custom_open_handler,
        state_attr: if input.disabled { "disabled" } else { "ready" },
        open_mode_attr: if input.is_controlled {
            "controlled"
        } else {
            "uncontrolled"
        },
        initial_open_attr: if input.initial_open { "open" } else { "closed" },
        trigger_mode_attr: if input.show_trigger {
            "visible"
        } else {
            "hidden"
        },
        shortcut_mode_attr: if input.enable_shortcut {
            "enabled"
        } else {
            "disabled"
        },
        label_source_attr: if input.has_custom_aria_label {
            "custom"
        } else {
            "default"
        },
        trigger_source_attr: if input.has_custom_trigger_label {
            "custom"
        } else {
            "default"
        },
        shortcut_source_attr: if input.has_custom_shortcut_key {
            "custom"
        } else {
            "default"
        },
        class_source_attr: if input.has_custom_class_name {
            "custom"
        } else {
            "default"
        },
        handler_source_attr: if input.has_custom_open_handler {
            "custom"
        } else {
            "default"
        },
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: SidenavState) -> String {
    let mut classes = vec!["ui-sidenav".to_string()];

    if state.is_disabled {
        classes.push("ui-sidenav--disabled".to_string());
    }

    if state.is_controlled {
        classes.push("ui-sidenav--controlled".to_string());
    } else {
        classes.push("ui-sidenav--uncontrolled".to_string());
    }

    if state.show_trigger {
        classes.push("ui-sidenav--trigger-visible".to_string());
    } else {
        classes.push("ui-sidenav--trigger-hidden".to_string());
    }

    if state.enable_shortcut {
        classes.push("ui-sidenav--shortcut-enabled".to_string());
    } else {
        classes.push("ui-sidenav--shortcut-disabled".to_string());
    }

    if state.initial_open {
        classes.push("ui-sidenav--default-open".to_string());
    } else {
        classes.push("ui-sidenav--default-closed".to_string());
    }

    if state.has_custom_open_handler {
        classes.push("ui-sidenav--custom-handler".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-sidenav--custom-class".to_string());
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
            normalize_optional_text(Some("  docs-sidenav  ".to_string())),
            Some("docs-sidenav".to_string())
        );
    }

    #[test]
    fn normalize_labels_use_default_when_missing() {
        let (aria, aria_custom) = normalize_aria_label(Some("  Main nav  ".to_string()));
        assert_eq!(aria, "Main nav");
        assert!(aria_custom);

        let (aria, aria_custom) = normalize_aria_label(None);
        assert_eq!(aria, DEFAULT_ARIA_LABEL);
        assert!(!aria_custom);

        let (trigger, trigger_custom) = normalize_trigger_label(Some("  Toggle  ".to_string()));
        assert_eq!(trigger, "Toggle");
        assert!(trigger_custom);

        let (trigger, trigger_custom) = normalize_trigger_label(None);
        assert_eq!(trigger, DEFAULT_TRIGGER_LABEL);
        assert!(!trigger_custom);
    }

    #[test]
    fn normalize_shortcut_key_respects_enable_and_custom_source() {
        assert_eq!(
            normalize_shortcut_key(Some("  n  ".to_string()), true),
            ("n".to_string(), true)
        );
        assert_eq!(
            normalize_shortcut_key(Some("Nav".to_string()), true),
            ("n".to_string(), true)
        );
        assert_eq!(
            normalize_shortcut_key(None, true),
            (DEFAULT_SHORTCUT_KEY.to_string(), false)
        );
        assert_eq!(
            normalize_shortcut_key(Some("x".to_string()), false),
            ("".to_string(), false)
        );
    }

    #[test]
    fn resolve_state_tracks_mode_and_source_markers() {
        let state = resolve_state(SidenavStateInput {
            disabled: true,
            show_trigger: false,
            enable_shortcut: true,
            is_controlled: true,
            initial_open: false,
            has_custom_shortcut_key: true,
            has_custom_trigger_label: true,
            has_custom_aria_label: false,
            has_custom_class_name: true,
            has_custom_open_handler: true,
        });

        assert!(state.is_disabled);
        assert!(state.is_controlled);
        assert!(!state.show_trigger);
        assert_eq!(state.state_attr, "disabled");
        assert_eq!(state.open_mode_attr, "controlled");
        assert_eq!(state.initial_open_attr, "closed");
        assert_eq!(state.trigger_mode_attr, "hidden");
        assert_eq!(state.shortcut_mode_attr, "enabled");
        assert_eq!(state.label_source_attr, "default");
        assert_eq!(state.trigger_source_attr, "custom");
        assert_eq!(state.shortcut_source_attr, "custom");
        assert_eq!(state.class_source_attr, "custom");
        assert_eq!(state.handler_source_attr, "custom");
    }

    #[test]
    fn compose_class_name_includes_state_and_custom_markers() {
        let class_name = compose_class_name(
            Some("docs-sidenav-custom".to_string()),
            resolve_state(SidenavStateInput {
                disabled: false,
                show_trigger: true,
                enable_shortcut: false,
                is_controlled: false,
                initial_open: true,
                has_custom_shortcut_key: false,
                has_custom_trigger_label: false,
                has_custom_aria_label: true,
                has_custom_class_name: true,
                has_custom_open_handler: true,
            }),
        );

        for token in [
            "ui-sidenav",
            "ui-sidenav--uncontrolled",
            "ui-sidenav--trigger-visible",
            "ui-sidenav--shortcut-disabled",
            "ui-sidenav--default-open",
            "ui-sidenav--custom-handler",
            "ui-sidenav--custom-class",
            "docs-sidenav-custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
