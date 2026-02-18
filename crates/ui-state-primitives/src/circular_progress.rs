pub const DEFAULT_ARIA_LABEL: &str = "Loading";

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CircularProgressStateInput {
    pub size_px: Option<f64>,
    pub thickness_px: Option<f64>,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CircularProgressState {
    pub style_vars: Option<String>,
    pub has_custom_size: bool,
    pub has_custom_thickness: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub size_source_attr: &'static str,
    pub thickness_source_attr: &'static str,
    pub label_source_attr: &'static str,
    pub class_source_attr: &'static str,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn resolve_aria_label(value: Option<String>, default_aria_label: &str) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        let is_custom = label != default_aria_label;
        return (label, is_custom);
    }

    (default_aria_label.to_string(), false)
}

pub fn sanitize_dimension(value: Option<f64>) -> Option<f64> {
    value.filter(|value| value.is_finite() && *value > 0.0)
}

pub fn compose_style_vars(size_px: Option<f64>, thickness_px: Option<f64>) -> Option<String> {
    let mut vars = Vec::new();

    if let Some(size_px) = size_px {
        vars.push(format!("--ui-cp-size: {size_px}px;"));
    }

    if let Some(thickness_px) = thickness_px {
        vars.push(format!("--ui-cp-thickness: {thickness_px}px;"));
    }

    (!vars.is_empty()).then(|| vars.join(" "))
}

pub fn resolve_state(input: CircularProgressStateInput) -> CircularProgressState {
    let size_px = sanitize_dimension(input.size_px);
    let thickness_px = sanitize_dimension(input.thickness_px);

    let has_custom_size = size_px.is_some();
    let has_custom_thickness = thickness_px.is_some();

    CircularProgressState {
        style_vars: compose_style_vars(size_px, thickness_px),
        has_custom_size,
        has_custom_thickness,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
        size_source_attr: if has_custom_size { "custom" } else { "default" },
        thickness_source_attr: if has_custom_thickness {
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
    }
}

pub fn compose_class_name(
    base_class_name: Option<String>,
    state: &CircularProgressState,
) -> String {
    let mut classes = vec![
        "ui-circular-progress".to_string(),
        "ui-circular-progress--state-indeterminate".to_string(),
    ];

    if state.has_custom_size {
        classes.push("ui-circular-progress--size-custom".to_string());
    }

    if state.has_custom_thickness {
        classes.push("ui-circular-progress--thickness-custom".to_string());
    }

    if state.has_custom_aria_label {
        classes.push("ui-circular-progress--label-custom".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-circular-progress--custom-class".to_string());
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
    fn normalize_optional_text_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("  \n\t".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  docs-cp  ".to_string())),
            Some("docs-cp".to_string())
        );
    }

    #[test]
    fn resolve_aria_label_defaults_and_detects_custom_source() {
        assert_eq!(
            resolve_aria_label(None, DEFAULT_ARIA_LABEL),
            (DEFAULT_ARIA_LABEL.to_string(), false)
        );
        assert_eq!(
            resolve_aria_label(Some("  ".to_string()), DEFAULT_ARIA_LABEL),
            (DEFAULT_ARIA_LABEL.to_string(), false)
        );
        assert_eq!(
            resolve_aria_label(Some("Loading".to_string()), DEFAULT_ARIA_LABEL),
            (DEFAULT_ARIA_LABEL.to_string(), false)
        );
        assert_eq!(
            resolve_aria_label(Some("  Syncing mailbox ".to_string()), DEFAULT_ARIA_LABEL),
            ("Syncing mailbox".to_string(), true)
        );
    }

    #[test]
    fn sanitize_dimension_rejects_invalid_values() {
        assert_eq!(sanitize_dimension(None), None);
        assert_eq!(sanitize_dimension(Some(-1.0)), None);
        assert_eq!(sanitize_dimension(Some(0.0)), None);
        assert_eq!(sanitize_dimension(Some(f64::NAN)), None);
        assert_eq!(sanitize_dimension(Some(f64::INFINITY)), None);
        assert_eq!(sanitize_dimension(Some(24.0)), Some(24.0));
    }

    #[test]
    fn resolve_state_tracks_source_flags() {
        let state = resolve_state(CircularProgressStateInput {
            size_px: Some(24.0),
            thickness_px: Some(3.0),
            has_custom_aria_label: true,
            has_custom_class_name: true,
        });

        assert_eq!(
            state.style_vars,
            Some("--ui-cp-size: 24px; --ui-cp-thickness: 3px;".to_string())
        );
        assert!(state.has_custom_size);
        assert!(state.has_custom_thickness);
        assert!(state.has_custom_aria_label);
        assert!(state.has_custom_class_name);
        assert_eq!(state.size_source_attr, "custom");
        assert_eq!(state.thickness_source_attr, "custom");
        assert_eq!(state.label_source_attr, "custom");
        assert_eq!(state.class_source_attr, "custom");
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("custom".to_string()),
            &resolve_state(CircularProgressStateInput {
                size_px: Some(22.0),
                thickness_px: None,
                has_custom_aria_label: true,
                has_custom_class_name: true,
            }),
        );

        for token in [
            "ui-circular-progress",
            "ui-circular-progress--state-indeterminate",
            "ui-circular-progress--size-custom",
            "ui-circular-progress--label-custom",
            "ui-circular-progress--custom-class",
            "custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
