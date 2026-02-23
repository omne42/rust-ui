use leptos::prelude::{Callback, Signal};

pub use ui_state_primitives::progress::{
    ProgressMode, ProgressPhase, ProgressRange, ProgressStateInput, ProgressValueAxisInput,
    ProgressValueAxisState, clamp_to_range, compose_class_name, normalize_optional_text,
    normalize_progress, resolve_aria_label, resolve_phase, resolve_state, resolve_value_axis,
    resolve_value_label,
};

pub const DEFAULT_MIN: f64 = 0.0;
pub const DEFAULT_MAX: f64 = 100.0;

#[derive(Clone)]
pub struct ProgressValueAxis {
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
) -> ProgressValueAxis {
    let state: ProgressValueAxisState = resolve_value_axis(ProgressValueAxisInput {
        is_controlled: value.is_some(),
        has_default_value: default_value.is_some(),
        has_on_value_change: on_value_change.is_some(),
    });

    ProgressValueAxis {
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

pub fn normalize_mode(is_indeterminate: bool) -> ProgressMode {
    ui_state_primitives::progress::normalize_mode(is_indeterminate)
}

pub fn normalize_range(min: Option<f64>, max: Option<f64>) -> ProgressRange {
    let min = min.unwrap_or(DEFAULT_MIN);
    let max = max.unwrap_or(DEFAULT_MAX);
    ProgressRange::sanitized(min, max)
}

pub fn normalize_progress_value(progress: Option<f64>) -> f64 {
    progress.unwrap_or(0.0)
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProgressRenderInput {
    pub clamped_value: Option<f64>,
    pub normalized_progress: Option<f64>,
    pub mode: ProgressMode,
    pub value_label_override: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProgressRenderState {
    pub mode: ProgressMode,
    pub phase: ProgressPhase,
    pub is_indeterminate: bool,
    pub progress_value: f64,
    pub aria_value_now: Option<f64>,
    pub value_label_text: Option<String>,
}

pub fn resolve_render_state(input: ProgressRenderInput) -> ProgressRenderState {
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

    ProgressRenderState {
        mode: input.mode,
        phase,
        is_indeterminate,
        progress_value,
        aria_value_now: input.clamped_value,
        value_label_text,
    }
}

#[cfg(test)]
pub use ui_state_primitives::progress::DEFAULT_ARIA_LABEL;

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
