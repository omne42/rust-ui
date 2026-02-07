#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AutoHeightStateInput {
    pub animate_height: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AutoHeightState {
    pub overflow_hidden: bool,
    pub animate_height: bool,
    pub is_static: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn resolve_state(input: AutoHeightStateInput) -> AutoHeightState {
    AutoHeightState {
        overflow_hidden: true,
        animate_height: input.animate_height,
        is_static: !input.animate_height,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_motion: input.has_custom_motion,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: AutoHeightState) -> String {
    let mut classes = vec!["ui-auto-height".to_string()];

    if state.animate_height {
        classes.push("ui-auto-height--animated".to_string());
    }
    if state.is_static {
        classes.push("ui-auto-height--static".to_string());
    }
    if state.has_custom_motion {
        classes.push("ui-auto-height--custom-motion".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-auto-height--custom-class".to_string());
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
    fn default_state_is_overflow_hidden() {
        let state = resolve_state(AutoHeightStateInput {
            animate_height: true,
            has_custom_class_name: false,
            has_custom_motion: false,
        });

        assert!(state.overflow_hidden);
        assert!(state.animate_height);
        assert!(!state.is_static);
        assert!(!state.has_custom_class_name);
        assert!(!state.has_custom_motion);
    }

    #[test]
    fn normalize_optional_text_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("  \n\t".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  docs-auto-height  ".to_string())),
            Some("docs-auto-height".to_string())
        );
    }

    #[test]
    fn resolve_state_tracks_static_and_custom_flags() {
        let state = resolve_state(AutoHeightStateInput {
            animate_height: false,
            has_custom_class_name: true,
            has_custom_motion: true,
        });

        assert!(state.overflow_hidden);
        assert!(!state.animate_height);
        assert!(state.is_static);
        assert!(state.has_custom_class_name);
        assert!(state.has_custom_motion);
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("custom".to_string()),
            resolve_state(AutoHeightStateInput {
                animate_height: false,
                has_custom_class_name: true,
                has_custom_motion: true,
            }),
        );

        for token in [
            "ui-auto-height",
            "ui-auto-height--static",
            "ui-auto-height--custom-motion",
            "ui-auto-height--custom-class",
            "custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
