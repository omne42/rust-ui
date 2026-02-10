use crate::picker::{PickerState, PickerStateInput};

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn resolve_state(input: PickerStateInput) -> PickerState {
    PickerState {
        is_disabled: input.disabled,
        has_items: input.has_items,
        has_selection: input.has_selection,
        has_disabled_indices: input.has_disabled_indices,
        is_controlled: input.is_controlled,
        default_open: input.default_open,
        has_custom_placeholder: input.has_custom_placeholder,
        has_custom_open_handler: input.has_custom_open_handler,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_placement: input.has_custom_placement,
        has_custom_motion: input.has_custom_motion,
        state_attr: if input.disabled {
            "disabled"
        } else if !input.has_items {
            "empty"
        } else {
            "ready"
        },
        selection_attr: if input.has_selection {
            "selected"
        } else {
            "none"
        },
        disabled_options_attr: if input.has_disabled_indices {
            "present"
        } else {
            "none"
        },
        open_mode_attr: if input.is_controlled {
            "controlled"
        } else {
            "uncontrolled"
        },
        initial_open_attr: if input.default_open { "open" } else { "closed" },
        placeholder_source_attr: if input.has_custom_placeholder {
            "custom"
        } else {
            "default"
        },
        handler_source_attr: if input.has_custom_open_handler {
            "custom"
        } else {
            "default"
        },
        class_source_attr: if input.has_custom_class_name {
            "custom"
        } else {
            "default"
        },
        placement_source_attr: if input.has_custom_placement {
            "custom"
        } else {
            "default"
        },
        motion_source_attr: if input.has_custom_motion {
            "custom"
        } else {
            "default"
        },
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: PickerState) -> String {
    let mut classes = vec!["ui-picker".to_string()];

    if state.is_disabled {
        classes.push("ui-picker--disabled".to_string());
    }

    if state.is_controlled {
        classes.push("ui-picker--controlled".to_string());
    } else {
        classes.push("ui-picker--uncontrolled".to_string());
    }

    if state.has_selection {
        classes.push("ui-picker--has-selection".to_string());
    }

    if state.has_custom_motion {
        classes.push("ui-picker--custom-motion".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-picker--custom-class".to_string());
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
    fn normalize_optional_text_trims_and_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("  ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  Choose region  ".to_string())),
            Some("Choose region".to_string())
        );
    }

    #[test]
    fn resolve_state_tracks_sources_and_modes() {
        let state = resolve_state(PickerStateInput {
            disabled: false,
            has_items: true,
            has_selection: true,
            has_disabled_indices: true,
            is_controlled: true,
            default_open: false,
            has_custom_placeholder: true,
            has_custom_open_handler: true,
            has_custom_class_name: false,
            has_custom_placement: true,
            has_custom_motion: false,
        });

        assert_eq!(state.state_attr, "ready");
        assert_eq!(state.selection_attr, "selected");
        assert_eq!(state.disabled_options_attr, "present");
        assert_eq!(state.open_mode_attr, "controlled");
        assert_eq!(state.initial_open_attr, "closed");
        assert_eq!(state.placeholder_source_attr, "custom");
        assert_eq!(state.handler_source_attr, "custom");
        assert_eq!(state.class_source_attr, "default");
        assert_eq!(state.placement_source_attr, "custom");
        assert_eq!(state.motion_source_attr, "default");
    }

    #[test]
    fn compose_class_name_includes_state_and_custom_markers() {
        let class_name = compose_class_name(
            Some("docs-picker-state".to_string()),
            resolve_state(PickerStateInput {
                disabled: true,
                has_items: false,
                has_selection: false,
                has_disabled_indices: false,
                is_controlled: false,
                default_open: true,
                has_custom_placeholder: false,
                has_custom_open_handler: false,
                has_custom_class_name: true,
                has_custom_placement: false,
                has_custom_motion: true,
            }),
        );

        for token in [
            "ui-picker",
            "ui-picker--disabled",
            "ui-picker--uncontrolled",
            "ui-picker--custom-motion",
            "ui-picker--custom-class",
            "docs-picker-state",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
