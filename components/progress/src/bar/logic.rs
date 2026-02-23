use leptos::prelude::Callback;

pub use ui_state_primitives::progress_bar::{
    DEFAULT_ARIA_LABEL, DEFAULT_MAX, ProgressBarMode, ProgressBarSize, ProgressBarStateInput,
    ProgressBarValueAxisInput, ProgressBarValueAxisState, ProgressBarVariant, compose_class_name,
    normalize_optional_text, resolve_aria_label, resolve_state, resolve_value_axis,
};

#[derive(Clone)]
pub struct ProgressBarValueAxis {
    pub value: Option<f64>,
    pub is_controlled: bool,
    pub has_custom_default_value: bool,
    pub has_custom_on_value_change: bool,
    pub mode_attr: &'static str,
    pub value_source_attr: &'static str,
    pub default_value_source_attr: &'static str,
    pub value_change_source_attr: &'static str,
}

pub fn normalize_value_axis(
    value: Option<f64>,
    default_value: Option<f64>,
    on_value_change: Option<Callback<Option<f64>>>,
) -> ProgressBarValueAxis {
    let state: ProgressBarValueAxisState = resolve_value_axis(ProgressBarValueAxisInput {
        is_controlled: value.is_some(),
        has_default_value: default_value.is_some(),
        has_on_value_change: on_value_change.is_some(),
    });
    let value = if state.is_controlled {
        value
    } else {
        default_value
    };

    ProgressBarValueAxis {
        value,
        is_controlled: state.is_controlled,
        has_custom_default_value: state.has_default_value,
        has_custom_on_value_change: state.has_on_value_change,
        mode_attr: state.mode_attr,
        value_source_attr: state.value_source_attr,
        default_value_source_attr: state.default_value_source_attr,
        value_change_source_attr: state.value_change_source_attr,
    }
}

pub fn normalize_mode(is_indeterminate: bool) -> ProgressBarMode {
    ui_state_primitives::progress_bar::normalize_mode(is_indeterminate)
}

pub fn normalize_max(max: Option<f64>) -> f64 {
    max.unwrap_or(DEFAULT_MAX)
}

#[cfg(test)]
pub use ui_state_primitives::progress_bar::{
    MIN_MAX, ProgressBarPhase, sanitize_max, sanitize_value,
};

#[cfg(test)]
#[path = "../../test/bar/logic.rs"]
mod tests;
