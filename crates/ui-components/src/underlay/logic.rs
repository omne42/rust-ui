use crate::underlay::{UnderlayState, UnderlayStateInput};

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn resolve_state(input: UnderlayStateInput) -> UnderlayState {
    let is_open = input.open && !input.disabled;
    let is_interactive = is_open && input.has_on_close;

    let data_state_attr = if input.disabled {
        "disabled"
    } else if is_open {
        "open"
    } else {
        "closed"
    };

    UnderlayState {
        is_open,
        is_transparent: input.transparent,
        is_disabled: input.disabled,
        is_interactive,
        data_state_attr,
        tone_attr: if input.transparent {
            "transparent"
        } else {
            "scrim"
        },
        close_mode_attr: if is_interactive {
            "interactive"
        } else {
            "static"
        },
        class_source_attr: if input.has_custom_class_name {
            "custom"
        } else {
            "default"
        },
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: UnderlayState) -> String {
    let mut classes = vec!["ui-underlay".to_string()];

    if state.is_open {
        classes.push("ui-underlay--open".to_string());
    }

    if state.is_transparent {
        classes.push("ui-underlay--transparent".to_string());
    }

    if state.is_disabled {
        classes.push("ui-underlay--disabled".to_string());
    }

    if state.is_interactive {
        classes.push("ui-underlay--interactive".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-underlay--custom-class".to_string());
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
    fn resolve_state_tracks_open_transparent_disabled_and_interactivity() {
        let open = resolve_state(UnderlayStateInput {
            open: true,
            transparent: false,
            disabled: false,
            has_on_close: true,
            has_custom_class_name: true,
        });

        assert!(open.is_open);
        assert!(open.is_interactive);
        assert_eq!(open.data_state_attr, "open");
        assert_eq!(open.tone_attr, "scrim");
        assert_eq!(open.close_mode_attr, "interactive");
        assert_eq!(open.class_source_attr, "custom");

        let closed = resolve_state(UnderlayStateInput {
            open: false,
            transparent: true,
            disabled: false,
            has_on_close: true,
            has_custom_class_name: false,
        });

        assert!(!closed.is_open);
        assert!(!closed.is_interactive);
        assert_eq!(closed.data_state_attr, "closed");
        assert_eq!(closed.tone_attr, "transparent");

        let disabled = resolve_state(UnderlayStateInput {
            open: true,
            transparent: false,
            disabled: true,
            has_on_close: true,
            has_custom_class_name: false,
        });

        assert!(!disabled.is_open);
        assert!(!disabled.is_interactive);
        assert_eq!(disabled.data_state_attr, "disabled");
        assert!(disabled.is_disabled);
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let state = resolve_state(UnderlayStateInput {
            open: true,
            transparent: true,
            disabled: false,
            has_on_close: true,
            has_custom_class_name: true,
        });

        let class_name = compose_class_name(Some("docs-underlay".to_string()), state);
        assert!(class_name.contains("ui-underlay"));
        assert!(class_name.contains("ui-underlay--open"));
        assert!(class_name.contains("ui-underlay--transparent"));
        assert!(class_name.contains("ui-underlay--interactive"));
        assert!(class_name.contains("ui-underlay--custom-class"));
        assert!(class_name.contains("docs-underlay"));
    }
}
