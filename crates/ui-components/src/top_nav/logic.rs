use crate::top_nav::{DEFAULT_LABEL, TopNavState, TopNavStateInput};

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

pub fn normalize_default_selected_id(value: Option<String>) -> (String, bool) {
    if let Some(default_selected_id) = normalize_optional_text(value) {
        return (default_selected_id, true);
    }

    (String::new(), false)
}

pub fn resolve_state(input: TopNavStateInput) -> TopNavState {
    TopNavState {
        state_attr: if input.is_controlled {
            "controlled"
        } else if input.has_default_selected_id {
            "uncontrolled-default"
        } else {
            "uncontrolled"
        },
        selection_mode_attr: if input.is_controlled {
            "controlled"
        } else {
            "uncontrolled"
        },
        default_selection_attr: if input.has_default_selected_id {
            "explicit"
        } else {
            "none"
        },
        focus_activation_attr: if input.activate_on_focus {
            "focus"
        } else {
            "manual"
        },
        label_source_attr: if input.has_custom_label {
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
        has_default_selected_id: input.has_default_selected_id,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_motion: input.has_custom_motion,
    }
}

pub fn compose_class_name(class_name: Option<String>, state: TopNavState) -> String {
    let mut classes = vec![
        "ui-top-nav".to_string(),
        format!("ui-top-nav--state-{}", state.state_attr),
        format!("ui-top-nav--mode-{}", state.selection_mode_attr),
        format!("ui-top-nav--focus-{}", state.focus_activation_attr),
    ];

    if state.has_default_selected_id {
        classes.push("ui-top-nav--has-default-selection".to_string());
    }

    if state.has_custom_motion {
        classes.push("ui-top-nav--custom-motion".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-top-nav--custom-class".to_string());
        if let Some(class_name) = class_name {
            classes.push(class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_label_and_default_selected_id_track_sources() {
        assert_eq!(
            normalize_label(Some("  Main navigation  ".to_string())),
            ("Main navigation".to_string(), true)
        );
        assert_eq!(normalize_label(None), (DEFAULT_LABEL.to_string(), false));

        assert_eq!(
            normalize_default_selected_id(Some("  docs  ".to_string())),
            ("docs".to_string(), true)
        );
        assert_eq!(normalize_default_selected_id(None), (String::new(), false));
    }

    #[test]
    fn resolve_state_tracks_mode_sources_focus_and_motion_contracts() {
        let state = resolve_state(TopNavStateInput {
            is_controlled: false,
            has_default_selected_id: true,
            activate_on_focus: false,
            has_custom_label: true,
            has_custom_class_name: false,
            has_custom_motion: true,
        });

        assert_eq!(state.state_attr, "uncontrolled-default");
        assert_eq!(state.selection_mode_attr, "uncontrolled");
        assert_eq!(state.default_selection_attr, "explicit");
        assert_eq!(state.focus_activation_attr, "manual");
        assert_eq!(state.label_source_attr, "custom");
        assert_eq!(state.class_source_attr, "default");
        assert_eq!(state.motion_source_attr, "custom");
    }

    #[test]
    fn compose_class_name_includes_state_mode_and_custom_markers() {
        let state = resolve_state(TopNavStateInput {
            is_controlled: true,
            has_default_selected_id: false,
            activate_on_focus: true,
            has_custom_label: false,
            has_custom_class_name: true,
            has_custom_motion: true,
        });

        let class_name = compose_class_name(Some("docs-top-nav".to_string()), state);

        for token in [
            "ui-top-nav",
            "ui-top-nav--state-controlled",
            "ui-top-nav--mode-controlled",
            "ui-top-nav--focus-focus",
            "ui-top-nav--custom-motion",
            "ui-top-nav--custom-class",
            "docs-top-nav",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
