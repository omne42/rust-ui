use crate::dropzone::{DEFAULT_LABEL, DropzoneState, DropzoneStateInput};

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn resolve_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_LABEL.to_string(), false)
}

pub fn resolve_aria_label(label: &str, value: Option<String>) -> (String, &'static str) {
    if let Some(aria_label) = normalize_optional_text(value) {
        return (aria_label, "custom");
    }

    if label != DEFAULT_LABEL {
        return (label.to_string(), "label");
    }

    (DEFAULT_LABEL.to_string(), "default")
}

pub fn resolve_state(input: DropzoneStateInput) -> DropzoneState {
    DropzoneState {
        state_attr: if input.disabled {
            "disabled"
        } else {
            "enabled"
        },
        label_source_attr: if input.has_custom_label {
            "custom"
        } else {
            "default"
        },
        aria_source_attr: input.aria_source_attr,
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
        drop_handler_source_attr: if input.has_custom_drop_handler {
            "custom"
        } else {
            "default"
        },
        has_custom_label: input.has_custom_label,
        has_custom_aria: input.aria_source_attr == "custom",
        has_custom_drop_handler: input.has_custom_drop_handler,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_motion: input.has_custom_motion,
    }
}

pub fn compose_class_name(class_name: Option<String>, state: DropzoneState) -> String {
    let mut classes = vec![
        "ui-dropzone".to_string(),
        format!("ui-dropzone--state-{}", state.state_attr),
    ];

    if state.has_custom_label {
        classes.push("ui-dropzone--custom-label".to_string());
    }

    if state.has_custom_aria {
        classes.push("ui-dropzone--custom-aria".to_string());
    }

    if state.has_custom_drop_handler {
        classes.push("ui-dropzone--custom-drop-handler".to_string());
    }

    if state.has_custom_motion {
        classes.push("ui-dropzone--custom-motion".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-dropzone--custom-class".to_string());
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
    fn resolve_label_and_aria_sources_follow_fallback_order() {
        let (label, custom_label) = resolve_label(Some("  Upload assets  ".to_string()));
        assert_eq!(label, "Upload assets");
        assert!(custom_label);

        let (aria, source) = resolve_aria_label(&label, Some("  Upload zone  ".to_string()));
        assert_eq!(aria, "Upload zone");
        assert_eq!(source, "custom");

        let (aria, source) = resolve_aria_label(&label, None);
        assert_eq!(aria, "Upload assets");
        assert_eq!(source, "label");

        let (aria, source) = resolve_aria_label(DEFAULT_LABEL, None);
        assert_eq!(aria, DEFAULT_LABEL);
        assert_eq!(source, "default");
    }

    #[test]
    fn resolve_state_tracks_sources_and_motion_contracts() {
        let state = resolve_state(DropzoneStateInput {
            disabled: false,
            has_custom_label: true,
            aria_source_attr: "custom",
            has_custom_class_name: true,
            has_custom_motion: true,
            has_custom_drop_handler: false,
        });

        assert_eq!(state.state_attr, "enabled");
        assert_eq!(state.label_source_attr, "custom");
        assert_eq!(state.aria_source_attr, "custom");
        assert_eq!(state.class_source_attr, "custom");
        assert_eq!(state.motion_source_attr, "custom");
        assert_eq!(state.drop_handler_source_attr, "default");
        assert!(state.has_custom_label);
        assert!(state.has_custom_aria);
        assert!(!state.has_custom_drop_handler);
    }

    #[test]
    fn compose_class_name_includes_state_and_custom_markers() {
        let state = resolve_state(DropzoneStateInput {
            disabled: true,
            has_custom_label: true,
            aria_source_attr: "custom",
            has_custom_class_name: true,
            has_custom_motion: true,
            has_custom_drop_handler: true,
        });

        let class_name = compose_class_name(Some("docs-dropzone".to_string()), state);

        for token in [
            "ui-dropzone",
            "ui-dropzone--state-disabled",
            "ui-dropzone--custom-label",
            "ui-dropzone--custom-aria",
            "ui-dropzone--custom-drop-handler",
            "ui-dropzone--custom-motion",
            "ui-dropzone--custom-class",
            "docs-dropzone",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
