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
        state.label_source_class.into(),
        state.value_label_source_class.into(),
        state.size_source_class.into(),
        state.stroke_source_class.into(),
        state.motion_source_class.into(),
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
#[path = "../../test/circle/logic.rs"]
mod tests;
