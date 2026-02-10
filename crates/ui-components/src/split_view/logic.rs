use crate::resizable::{
    DEFAULT_ARIA_LABEL, DEFAULT_MAX_SPLIT_PERCENT, DEFAULT_MIN_SPLIT_PERCENT,
    DEFAULT_SPLIT_PERCENT, ResizableOrientation,
};
use crate::split_view::{SplitViewState, SplitViewStateInput};

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(value) = normalize_optional_text(value) {
        return (value, true);
    }

    (DEFAULT_ARIA_LABEL.to_string(), false)
}

pub fn resolve_state(input: SplitViewStateInput) -> SplitViewState {
    SplitViewState {
        orientation: input.orientation,
        is_disabled: input.disabled,
        is_enabled: !input.disabled,
        with_handle: input.with_handle,
        is_controlled: input.is_controlled,
        has_custom_default_split: input.has_custom_default_split,
        has_custom_bounds: input.has_custom_bounds,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_change_handler: input.has_custom_change_handler,
        orientation_attr: match input.orientation {
            ResizableOrientation::Horizontal => "horizontal",
            ResizableOrientation::Vertical => "vertical",
        },
        state_attr: if input.disabled {
            "disabled"
        } else {
            "enabled"
        },
        split_mode_attr: if input.is_controlled {
            "controlled"
        } else {
            "uncontrolled"
        },
        handle_attr: if input.with_handle {
            "with-handle"
        } else {
            "plain"
        },
        default_split_source_attr: if input.has_custom_default_split {
            "custom"
        } else {
            "default"
        },
        bounds_source_attr: if input.has_custom_bounds {
            "custom"
        } else {
            "default"
        },
        label_source_attr: if input.has_custom_aria_label {
            "custom"
        } else {
            "default"
        },
        class_source_attr: if input.has_custom_class_name {
            "custom"
        } else {
            "default"
        },
        handler_source_attr: if input.has_custom_change_handler {
            "custom"
        } else {
            "default"
        },
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: SplitViewState) -> String {
    let mut classes = vec!["ui-split-view".to_string()];

    match state.orientation {
        ResizableOrientation::Horizontal => classes.push("ui-split-view--horizontal".to_string()),
        ResizableOrientation::Vertical => classes.push("ui-split-view--vertical".to_string()),
    }

    if state.is_disabled {
        classes.push("ui-split-view--disabled".to_string());
    }

    if state.with_handle {
        classes.push("ui-split-view--with-handle".to_string());
    }

    if state.is_controlled {
        classes.push("ui-split-view--controlled".to_string());
    } else {
        classes.push("ui-split-view--uncontrolled".to_string());
    }

    if state.has_custom_default_split {
        classes.push("ui-split-view--custom-default".to_string());
    }

    if state.has_custom_bounds {
        classes.push("ui-split-view--custom-bounds".to_string());
    }

    if state.has_custom_aria_label {
        classes.push("ui-split-view--custom-label".to_string());
    }

    if state.has_custom_change_handler {
        classes.push("ui-split-view--custom-handler".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-split-view--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

pub fn default_split_percent(value: Option<f64>) -> f64 {
    value.unwrap_or(DEFAULT_SPLIT_PERCENT)
}

pub fn has_custom_bounds(min_split_percent: f64, max_split_percent: f64) -> bool {
    min_split_percent != DEFAULT_MIN_SPLIT_PERCENT || max_split_percent != DEFAULT_MAX_SPLIT_PERCENT
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_helpers_trim_and_fallback() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("  \n ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  split panel  ".to_string())),
            Some("split panel".to_string())
        );

        let (label, custom) = normalize_aria_label(Some("  Region split  ".to_string()));
        assert_eq!(label, "Region split");
        assert!(custom);

        let (label, custom) = normalize_aria_label(None);
        assert_eq!(label, DEFAULT_ARIA_LABEL);
        assert!(!custom);
    }

    #[test]
    fn resolve_state_tracks_sources_and_modes() {
        let state = resolve_state(SplitViewStateInput {
            orientation: ResizableOrientation::Vertical,
            disabled: false,
            with_handle: true,
            is_controlled: true,
            has_custom_default_split: true,
            has_custom_bounds: false,
            has_custom_aria_label: true,
            has_custom_class_name: false,
            has_custom_change_handler: true,
        });

        assert_eq!(state.orientation_attr, "vertical");
        assert_eq!(state.state_attr, "enabled");
        assert_eq!(state.split_mode_attr, "controlled");
        assert_eq!(state.handle_attr, "with-handle");
        assert_eq!(state.default_split_source_attr, "custom");
        assert_eq!(state.bounds_source_attr, "default");
        assert_eq!(state.label_source_attr, "custom");
        assert_eq!(state.class_source_attr, "default");
        assert_eq!(state.handler_source_attr, "custom");
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("docs-split-view-state".to_string()),
            resolve_state(SplitViewStateInput {
                orientation: ResizableOrientation::Horizontal,
                disabled: true,
                with_handle: true,
                is_controlled: false,
                has_custom_default_split: true,
                has_custom_bounds: true,
                has_custom_aria_label: false,
                has_custom_class_name: true,
                has_custom_change_handler: true,
            }),
        );

        for token in [
            "ui-split-view",
            "ui-split-view--horizontal",
            "ui-split-view--disabled",
            "ui-split-view--with-handle",
            "ui-split-view--uncontrolled",
            "ui-split-view--custom-default",
            "ui-split-view--custom-bounds",
            "ui-split-view--custom-handler",
            "ui-split-view--custom-class",
            "docs-split-view-state",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }

    #[test]
    fn defaults_and_bounds_source_detection_follow_resizable_contract() {
        assert_eq!(default_split_percent(None), DEFAULT_SPLIT_PERCENT);
        assert_eq!(default_split_percent(Some(42.0)), 42.0);

        assert!(!has_custom_bounds(
            DEFAULT_MIN_SPLIT_PERCENT,
            DEFAULT_MAX_SPLIT_PERCENT
        ));
        assert!(has_custom_bounds(20.0, DEFAULT_MAX_SPLIT_PERCENT));
        assert!(has_custom_bounds(DEFAULT_MIN_SPLIT_PERCENT, 80.0));
    }
}
