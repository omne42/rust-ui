pub const DEFAULT_ARIA_LABEL: &str = "Progress";

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProgressRange {
    pub min: f64,
    pub max: f64,
}

impl ProgressRange {
    pub fn sanitized(min: f64, max: f64) -> Self {
        let mut min = if min.is_finite() { min } else { 0.0 };
        let mut max = if max.is_finite() { max } else { 1.0 };
        if max <= min {
            (min, max) = (0.0, 1.0);
        }
        Self { min, max }
    }

    pub fn span(self) -> f64 {
        (self.max - self.min).max(f64::EPSILON)
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn resolve_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        let is_custom = label != DEFAULT_ARIA_LABEL;
        return (label, is_custom);
    }

    (DEFAULT_ARIA_LABEL.into(), false)
}

pub fn resolve_value_label(value: Option<String>) -> (Option<String>, bool) {
    let value = normalize_optional_text(value);
    let has_custom_value_label = value.is_some();
    (value, has_custom_value_label)
}

pub fn clamp_to_range(value: f64, range: ProgressRange) -> f64 {
    if !value.is_finite() {
        return range.min;
    }
    value.clamp(range.min, range.max)
}

pub fn normalize_progress(value: f64, range: ProgressRange) -> f64 {
    ((value - range.min) / range.span()).clamp(0.0, 1.0)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProgressPhase {
    Determinate,
    Indeterminate,
}

impl ProgressPhase {
    pub fn class_name(self) -> &'static str {
        match self {
            ProgressPhase::Determinate => "ui-progress--state-determinate",
            ProgressPhase::Indeterminate => "ui-progress--state-indeterminate",
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            ProgressPhase::Determinate => "determinate",
            ProgressPhase::Indeterminate => "indeterminate",
        }
    }
}

pub fn resolve_phase(is_indeterminate: bool) -> ProgressPhase {
    if is_indeterminate {
        ProgressPhase::Indeterminate
    } else {
        ProgressPhase::Determinate
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProgressStateInput {
    pub has_custom_aria_label: bool,
    pub has_custom_value_label: bool,
    pub has_custom_motion: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProgressState {
    pub has_custom_aria_label: bool,
    pub has_custom_value_label: bool,
    pub has_custom_motion: bool,
    pub has_custom_class_name: bool,
    pub label_source_class: &'static str,
    pub value_label_source_class: &'static str,
    pub motion_source_class: &'static str,
    pub label_source_attr: &'static str,
    pub value_label_source_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub class_source_attr: &'static str,
}

pub fn resolve_state(input: ProgressStateInput) -> ProgressState {
    let (label_source_class, label_source_attr) = if input.has_custom_aria_label {
        ("ui-progress--label-custom", "custom")
    } else {
        ("ui-progress--label-default", "default")
    };

    let (value_label_source_class, value_label_source_attr) = if input.has_custom_value_label {
        ("ui-progress--value-label-custom", "custom")
    } else {
        ("ui-progress--value-label-auto", "auto")
    };

    let (motion_source_class, motion_source_attr) = if input.has_custom_motion {
        ("ui-progress--motion-custom", "custom")
    } else {
        ("ui-progress--motion-default", "default")
    };

    let class_source_attr = if input.has_custom_class_name {
        "custom"
    } else {
        "default"
    };

    ProgressState {
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_value_label: input.has_custom_value_label,
        has_custom_motion: input.has_custom_motion,
        has_custom_class_name: input.has_custom_class_name,
        label_source_class,
        value_label_source_class,
        motion_source_class,
        label_source_attr,
        value_label_source_attr,
        motion_source_attr,
        class_source_attr,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: ProgressState) -> String {
    let mut classes = vec![
        "ui-progress".to_string(),
        state.label_source_class.into(),
        state.value_label_source_class.into(),
        state.motion_source_class.into(),
    ];

    if state.has_custom_class_name {
        classes.push("ui-progress--custom-class".to_string());
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
    fn range_sanitizes_invalid_bounds() {
        assert_eq!(
            ProgressRange::sanitized(10.0, 2.0),
            ProgressRange { min: 0.0, max: 1.0 }
        );
        assert_eq!(
            ProgressRange::sanitized(f64::NAN, f64::INFINITY),
            ProgressRange { min: 0.0, max: 1.0 }
        );
    }

    #[test]
    fn normalize_optional_text_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  docs-progress  ".to_string())),
            Some("docs-progress".to_string())
        );
    }

    #[test]
    fn resolve_aria_label_defaults_and_detects_custom_source() {
        assert_eq!(resolve_aria_label(None), (DEFAULT_ARIA_LABEL.into(), false));
        assert_eq!(
            resolve_aria_label(Some("\n\t".to_string())),
            (DEFAULT_ARIA_LABEL.into(), false)
        );
        assert_eq!(
            resolve_aria_label(Some("  Progress  ".to_string())),
            (DEFAULT_ARIA_LABEL.into(), false)
        );
        assert_eq!(
            resolve_aria_label(Some("  Uploading dataset  ".to_string())),
            ("Uploading dataset".to_string(), true)
        );
    }

    #[test]
    fn resolve_value_label_reports_source() {
        assert_eq!(resolve_value_label(None), (None, false));
        assert_eq!(resolve_value_label(Some("  ".to_string())), (None, false));
        assert_eq!(
            resolve_value_label(Some("  72 complete  ".to_string())),
            (Some("72 complete".to_string()), true)
        );
    }

    #[test]
    fn clamp_and_normalize_are_consistent() {
        let range = ProgressRange::sanitized(0.0, 100.0);
        let value = clamp_to_range(25.0, range);
        assert_eq!(value, 25.0);
        assert!((normalize_progress(value, range) - 0.25).abs() < 1e-9);
    }

    #[test]
    fn clamp_treats_non_finite_as_min() {
        let range = ProgressRange::sanitized(10.0, 20.0);
        assert_eq!(clamp_to_range(f64::NAN, range), 10.0);
        assert_eq!(clamp_to_range(f64::INFINITY, range), 10.0);
        assert_eq!(clamp_to_range(f64::NEG_INFINITY, range), 10.0);
    }

    #[test]
    fn phase_mapping_is_stable() {
        assert_eq!(
            ProgressPhase::Determinate.class_name(),
            "ui-progress--state-determinate"
        );
        assert_eq!(
            ProgressPhase::Indeterminate.class_name(),
            "ui-progress--state-indeterminate"
        );
        assert_eq!(ProgressPhase::Determinate.as_str(), "determinate");
        assert_eq!(ProgressPhase::Indeterminate.as_str(), "indeterminate");
    }

    #[test]
    fn resolve_state_tracks_source_contracts() {
        let state = resolve_state(ProgressStateInput {
            has_custom_aria_label: true,
            has_custom_value_label: false,
            has_custom_motion: true,
            has_custom_class_name: true,
        });

        assert_eq!(state.label_source_class, "ui-progress--label-custom");
        assert_eq!(
            state.value_label_source_class,
            "ui-progress--value-label-auto"
        );
        assert_eq!(state.motion_source_class, "ui-progress--motion-custom");
        assert_eq!(state.label_source_attr, "custom");
        assert_eq!(state.value_label_source_attr, "auto");
        assert_eq!(state.motion_source_attr, "custom");
        assert_eq!(state.class_source_attr, "custom");
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("custom".to_string()),
            resolve_state(ProgressStateInput {
                has_custom_aria_label: false,
                has_custom_value_label: true,
                has_custom_motion: true,
                has_custom_class_name: true,
            }),
        );

        for token in [
            "ui-progress",
            "ui-progress--value-label-custom",
            "ui-progress--motion-custom",
            "ui-progress--custom-class",
            "custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
