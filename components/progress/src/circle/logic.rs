use leptos::prelude::{Callback, Signal};

pub use ui_state_primitives::progress_circle::{
    ProgressCircleMetrics, ProgressCircleMetricsInput, ProgressCircleMode, ProgressCirclePhase,
    ProgressCircleRange, ProgressCircleStateInput, ProgressCircleValueAxisInput,
    ProgressCircleValueAxisState, clamp_to_range, compose_class_name, normalize_optional_text,
    normalize_progress, resolve_aria_label, resolve_metrics, resolve_phase, resolve_state,
    resolve_value_axis, resolve_value_label,
};

pub const DEFAULT_MIN: f64 = 0.0;
pub const DEFAULT_MAX: f64 = 100.0;

#[derive(Clone)]
pub struct ProgressCircleSvgTemplate {
    pub size_attr: String,
    pub view_box_attr: String,
    pub center_attr: String,
    pub radius_attr: String,
    pub stroke_width_attr: String,
    pub circumference_attr: String,
}

pub fn build_progress_circle_svg_template(
    size_px: f64,
    radius_px: f64,
    stroke_width_px: f64,
    circumference: f64,
) -> ProgressCircleSvgTemplate {
    let size_attr = size_px.to_string();
    ProgressCircleSvgTemplate {
        view_box_attr: format!("0 0 {} {}", size_px, size_px),
        center_attr: (size_px / 2.0).to_string(),
        radius_attr: radius_px.to_string(),
        stroke_width_attr: stroke_width_px.to_string(),
        circumference_attr: circumference.to_string(),
        size_attr,
    }
}

#[derive(Clone)]
pub struct ProgressCircleValueAxis {
    pub value: Option<Signal<Option<f64>>>,
    pub default_value: Option<f64>,
    pub on_value_change: Option<Callback<Option<f64>>>,
    pub is_controlled: bool,
    pub has_custom_default_value: bool,
    pub has_custom_on_value_change: bool,
    pub mode_attr: &'static str,
    pub value_source_attr: &'static str,
    pub default_value_source_attr: &'static str,
    pub value_change_source_attr: &'static str,
}

pub fn normalize_value_axis(
    value: Option<Signal<Option<f64>>>,
    default_value: Option<f64>,
    on_value_change: Option<Callback<Option<f64>>>,
) -> ProgressCircleValueAxis {
    let state: ProgressCircleValueAxisState = resolve_value_axis(ProgressCircleValueAxisInput {
        is_controlled: value.is_some(),
        has_default_value: default_value.is_some(),
        has_on_value_change: on_value_change.is_some(),
    });

    ProgressCircleValueAxis {
        value,
        default_value,
        on_value_change,
        is_controlled: state.is_controlled,
        has_custom_default_value: state.has_default_value,
        has_custom_on_value_change: state.has_on_value_change,
        mode_attr: state.mode_attr,
        value_source_attr: state.value_source_attr,
        default_value_source_attr: state.default_value_source_attr,
        value_change_source_attr: state.value_change_source_attr,
    }
}

pub fn normalize_mode(is_indeterminate: bool) -> ProgressCircleMode {
    ui_state_primitives::progress_circle::normalize_mode(is_indeterminate)
}

pub fn normalize_range(min: Option<f64>, max: Option<f64>) -> ProgressCircleRange {
    let min = min.unwrap_or(DEFAULT_MIN);
    let max = max.unwrap_or(DEFAULT_MAX);
    ProgressCircleRange::sanitized(min, max)
}

pub fn normalize_progress_value(progress: Option<f64>) -> f64 {
    progress.unwrap_or(0.0)
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProgressCircleKernelInput {
    pub clamped_value: Option<f64>,
    pub normalized_progress: Option<f64>,
    pub mode: ProgressCircleMode,
    pub value_label_override: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProgressCircleKernelState {
    pub mode: ProgressCircleMode,
    pub phase: ProgressCirclePhase,
    pub is_indeterminate: bool,
    pub progress_value: f64,
    pub aria_value_now: Option<f64>,
    pub value_label_text: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProgressCircleStrokeInput {
    pub circumference: f64,
    pub is_indeterminate: bool,
    pub animated_progress: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProgressCircleStrokeState {
    pub dasharray: String,
    pub dashoffset: String,
}

pub fn resolve_kernel_state(input: ProgressCircleKernelInput) -> ProgressCircleKernelState {
    let is_indeterminate = input.mode.is_indeterminate() || input.normalized_progress.is_none();
    let phase = resolve_phase(is_indeterminate);
    let progress_value = normalize_progress_value(input.normalized_progress);

    let value_label_text = if is_indeterminate {
        None
    } else if let Some(value_label_override) = input.value_label_override {
        Some(value_label_override)
    } else {
        input
            .normalized_progress
            .map(|progress| format!("{:.0}%", progress * 100.0))
    };

    ProgressCircleKernelState {
        mode: input.mode,
        phase,
        is_indeterminate,
        progress_value,
        aria_value_now: input.clamped_value,
        value_label_text,
    }
}

pub fn resolve_stroke_state(input: ProgressCircleStrokeInput) -> ProgressCircleStrokeState {
    let progress = if input.is_indeterminate {
        0.25
    } else {
        input.animated_progress
    };

    let dasharray = if input.is_indeterminate {
        (input.circumference * 0.25).to_string()
    } else {
        input.circumference.to_string()
    };
    let dashoffset = (input.circumference * (1.0 - progress)).to_string();

    ProgressCircleStrokeState {
        dasharray,
        dashoffset,
    }
}

#[cfg(test)]
pub use ui_state_primitives::progress_circle::{
    DEFAULT_ARIA_LABEL, DEFAULT_SIZE_PX, DEFAULT_STROKE_WIDTH_PX, sanitize_dimension,
};

#[cfg(test)]
#[path = "../../test/circle/logic.rs"]
mod tests;
