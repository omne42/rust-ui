use std::f64::consts::PI;

pub const DEFAULT_ARIA_LABEL: &str = "Progress";
pub const DEFAULT_SIZE_PX: f64 = 24.0;
pub const DEFAULT_STROKE_WIDTH_PX: f64 = 3.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProgressCircleRange {
    pub min: f64,
    pub max: f64,
}

impl ProgressCircleRange {
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
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn resolve_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        let is_custom = label != DEFAULT_ARIA_LABEL;
        return (label, is_custom);
    }

    (DEFAULT_ARIA_LABEL.to_string(), false)
}

pub fn resolve_value_label(value: Option<String>) -> (Option<String>, bool) {
    let value = normalize_optional_text(value);
    let has_custom_value_label = value.is_some();
    (value, has_custom_value_label)
}

pub fn sanitize_dimension(value: Option<f64>, fallback: f64) -> (f64, bool) {
    if let Some(value) = value.filter(|value| value.is_finite() && *value > 0.0) {
        return (value, true);
    }

    (fallback, false)
}

pub fn clamp_to_range(value: f64, range: ProgressCircleRange) -> f64 {
    if !value.is_finite() {
        return range.min;
    }
    value.clamp(range.min, range.max)
}

pub fn normalize_progress(value: f64, range: ProgressCircleRange) -> f64 {
    ((value - range.min) / range.span()).clamp(0.0, 1.0)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProgressCircleMetrics {
    pub size_px: f64,
    pub stroke_width_px: f64,
    pub radius_px: f64,
    pub circumference: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProgressCircleMetricsInput {
    pub size_px: Option<f64>,
    pub stroke_width_px: Option<f64>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProgressCircleResolvedMetrics {
    pub metrics: ProgressCircleMetrics,
    pub has_custom_size: bool,
    pub has_custom_stroke_width: bool,
}

pub fn resolve_metrics(input: ProgressCircleMetricsInput) -> ProgressCircleResolvedMetrics {
    let (size_px, has_custom_size) = sanitize_dimension(input.size_px, DEFAULT_SIZE_PX);
    let (stroke_width_px, has_custom_stroke_width) =
        sanitize_dimension(input.stroke_width_px, DEFAULT_STROKE_WIDTH_PX);

    let radius_px = (size_px - stroke_width_px).max(1.0) / 2.0;
    let circumference = 2.0 * PI * radius_px;

    ProgressCircleResolvedMetrics {
        metrics: ProgressCircleMetrics {
            size_px,
            stroke_width_px,
            radius_px,
            circumference,
        },
        has_custom_size,
        has_custom_stroke_width,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProgressCirclePhase {
    Determinate,
    Indeterminate,
}

impl ProgressCirclePhase {
    pub fn class_name(self) -> &'static str {
        match self {
            ProgressCirclePhase::Determinate => "ui-progress-circle--state-determinate",
            ProgressCirclePhase::Indeterminate => "ui-progress-circle--state-indeterminate",
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            ProgressCirclePhase::Determinate => "determinate",
            ProgressCirclePhase::Indeterminate => "indeterminate",
        }
    }
}

pub fn resolve_phase(is_indeterminate: bool) -> ProgressCirclePhase {
    if is_indeterminate {
        ProgressCirclePhase::Indeterminate
    } else {
        ProgressCirclePhase::Determinate
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProgressCircleStateInput {
    pub has_custom_aria_label: bool,
    pub has_custom_value_label: bool,
    pub has_custom_size: bool,
    pub has_custom_stroke_width: bool,
    pub has_custom_motion: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProgressCircleState {
    pub has_custom_aria_label: bool,
    pub has_custom_value_label: bool,
    pub has_custom_size: bool,
    pub has_custom_stroke_width: bool,
    pub has_custom_motion: bool,
    pub has_custom_class_name: bool,
    pub label_source_class: &'static str,
    pub value_label_source_class: &'static str,
    pub size_source_class: &'static str,
    pub stroke_source_class: &'static str,
    pub motion_source_class: &'static str,
    pub label_source_attr: &'static str,
    pub value_label_source_attr: &'static str,
    pub size_source_attr: &'static str,
    pub stroke_source_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub class_source_attr: &'static str,
}

pub fn resolve_state(input: ProgressCircleStateInput) -> ProgressCircleState {
    let (label_source_class, label_source_attr) = if input.has_custom_aria_label {
        ("ui-progress-circle--label-custom", "custom")
    } else {
        ("ui-progress-circle--label-default", "default")
    };

    let (value_label_source_class, value_label_source_attr) = if input.has_custom_value_label {
        ("ui-progress-circle--value-label-custom", "custom")
    } else {
        ("ui-progress-circle--value-label-auto", "auto")
    };

    let (size_source_class, size_source_attr) = if input.has_custom_size {
        ("ui-progress-circle--size-custom", "custom")
    } else {
        ("ui-progress-circle--size-default", "default")
    };

    let (stroke_source_class, stroke_source_attr) = if input.has_custom_stroke_width {
        ("ui-progress-circle--stroke-custom", "custom")
    } else {
        ("ui-progress-circle--stroke-default", "default")
    };

    let (motion_source_class, motion_source_attr) = if input.has_custom_motion {
        ("ui-progress-circle--motion-custom", "custom")
    } else {
        ("ui-progress-circle--motion-default", "default")
    };

    let class_source_attr = if input.has_custom_class_name {
        "custom"
    } else {
        "default"
    };

    ProgressCircleState {
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_value_label: input.has_custom_value_label,
        has_custom_size: input.has_custom_size,
        has_custom_stroke_width: input.has_custom_stroke_width,
        has_custom_motion: input.has_custom_motion,
        has_custom_class_name: input.has_custom_class_name,
        label_source_class,
        value_label_source_class,
        size_source_class,
        stroke_source_class,
        motion_source_class,
        label_source_attr,
        value_label_source_attr,
        size_source_attr,
        stroke_source_attr,
        motion_source_attr,
        class_source_attr,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: ProgressCircleState) -> String {
    let mut classes = vec![
        "ui-progress-circle".to_string(),
        state.label_source_class.to_string(),
        state.value_label_source_class.to_string(),
        state.size_source_class.to_string(),
        state.stroke_source_class.to_string(),
        state.motion_source_class.to_string(),
    ];

    if state.has_custom_class_name {
        classes.push("ui-progress-circle--custom-class".to_string());
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
        assert_eq!(normalize_optional_text(Some("\n\t ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  docs-progress-circle  ".to_string())),
            Some("docs-progress-circle".to_string())
        );
    }

    #[test]
    fn resolve_aria_label_defaults_and_trims_values() {
        assert_eq!(
            resolve_aria_label(None),
            (DEFAULT_ARIA_LABEL.to_string(), false)
        );
        assert_eq!(
            resolve_aria_label(Some("\n\t".to_string())),
            (DEFAULT_ARIA_LABEL.to_string(), false)
        );
        assert_eq!(
            resolve_aria_label(Some(" Progress ".to_string())),
            (DEFAULT_ARIA_LABEL.to_string(), false)
        );
        assert_eq!(
            resolve_aria_label(Some("  Syncing mailbox  ".to_string())),
            ("Syncing mailbox".to_string(), true)
        );
    }

    #[test]
    fn resolve_value_label_reports_source() {
        assert_eq!(resolve_value_label(None), (None, false));
        assert_eq!(resolve_value_label(Some("  ".to_string())), (None, false));
        assert_eq!(
            resolve_value_label(Some("  42 percent  ".to_string())),
            (Some("42 percent".to_string()), true)
        );
    }

    #[test]
    fn sanitize_dimension_rejects_invalid_values() {
        assert_eq!(sanitize_dimension(None, 24.0), (24.0, false));
        assert_eq!(sanitize_dimension(Some(0.0), 24.0), (24.0, false));
        assert_eq!(sanitize_dimension(Some(-1.0), 24.0), (24.0, false));
        assert_eq!(sanitize_dimension(Some(f64::NAN), 24.0), (24.0, false));
        assert_eq!(sanitize_dimension(Some(26.0), 24.0), (26.0, true));
    }

    #[test]
    fn range_sanitizes_invalid_bounds() {
        assert_eq!(
            ProgressCircleRange::sanitized(10.0, 2.0),
            ProgressCircleRange { min: 0.0, max: 1.0 }
        );
        assert_eq!(
            ProgressCircleRange::sanitized(f64::NAN, f64::INFINITY),
            ProgressCircleRange { min: 0.0, max: 1.0 }
        );
    }

    #[test]
    fn clamp_and_normalize_are_consistent() {
        let range = ProgressCircleRange::sanitized(0.0, 100.0);
        let value = clamp_to_range(25.0, range);
        assert_eq!(value, 25.0);
        assert!((normalize_progress(value, range) - 0.25).abs() < 1e-9);
    }

    #[test]
    fn clamp_treats_non_finite_as_min() {
        let range = ProgressCircleRange::sanitized(10.0, 20.0);
        assert_eq!(clamp_to_range(f64::NAN, range), 10.0);
        assert_eq!(clamp_to_range(f64::INFINITY, range), 10.0);
        assert_eq!(clamp_to_range(f64::NEG_INFINITY, range), 10.0);
    }

    #[test]
    fn resolve_metrics_sanitizes_inputs_and_tracks_sources() {
        let resolved = resolve_metrics(ProgressCircleMetricsInput {
            size_px: Some(f64::NAN),
            stroke_width_px: Some(-1.0),
        });

        assert_eq!(resolved.metrics.size_px, DEFAULT_SIZE_PX);
        assert_eq!(resolved.metrics.stroke_width_px, DEFAULT_STROKE_WIDTH_PX);
        assert!(resolved.metrics.radius_px > 0.0);
        assert!(resolved.metrics.circumference > 0.0);
        assert!(!resolved.has_custom_size);
        assert!(!resolved.has_custom_stroke_width);
    }

    #[test]
    fn phase_mapping_is_stable() {
        assert_eq!(
            ProgressCirclePhase::Determinate.class_name(),
            "ui-progress-circle--state-determinate"
        );
        assert_eq!(
            ProgressCirclePhase::Indeterminate.class_name(),
            "ui-progress-circle--state-indeterminate"
        );
        assert_eq!(ProgressCirclePhase::Determinate.as_str(), "determinate");
        assert_eq!(ProgressCirclePhase::Indeterminate.as_str(), "indeterminate");
    }

    #[test]
    fn resolve_state_tracks_source_contracts() {
        let state = resolve_state(ProgressCircleStateInput {
            has_custom_aria_label: true,
            has_custom_value_label: true,
            has_custom_size: true,
            has_custom_stroke_width: false,
            has_custom_motion: true,
            has_custom_class_name: true,
        });

        assert_eq!(state.label_source_class, "ui-progress-circle--label-custom");
        assert_eq!(
            state.value_label_source_class,
            "ui-progress-circle--value-label-custom"
        );
        assert_eq!(state.size_source_class, "ui-progress-circle--size-custom");
        assert_eq!(
            state.stroke_source_class,
            "ui-progress-circle--stroke-default"
        );
        assert_eq!(
            state.motion_source_class,
            "ui-progress-circle--motion-custom"
        );
        assert_eq!(state.label_source_attr, "custom");
        assert_eq!(state.value_label_source_attr, "custom");
        assert_eq!(state.size_source_attr, "custom");
        assert_eq!(state.stroke_source_attr, "default");
        assert_eq!(state.motion_source_attr, "custom");
        assert_eq!(state.class_source_attr, "custom");
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("custom".to_string()),
            resolve_state(ProgressCircleStateInput {
                has_custom_aria_label: false,
                has_custom_value_label: true,
                has_custom_size: false,
                has_custom_stroke_width: true,
                has_custom_motion: true,
                has_custom_class_name: true,
            }),
        );

        for token in [
            "ui-progress-circle",
            "ui-progress-circle--value-label-custom",
            "ui-progress-circle--stroke-custom",
            "ui-progress-circle--motion-custom",
            "ui-progress-circle--custom-class",
            "custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
