pub use crate::button::normalize_optional_text;

pub const DEFAULT_LABEL: &str = "Slider";
pub const DEFAULT_MIN: f64 = 0.0;
pub const DEFAULT_MAX: f64 = 100.0;
pub const DEFAULT_STEP: f64 = 1.0;
pub const MIN_RANGE: f64 = 0.000_001;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SliderPhase {
    Enabled,
    Disabled,
}

impl SliderPhase {
    pub fn class_name(self) -> &'static str {
        match self {
            Self::Enabled => "ui-slider--state-enabled",
            Self::Disabled => "ui-slider--state-disabled",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Enabled => "enabled",
            Self::Disabled => "disabled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SliderStateInput {
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub is_disabled: bool,
    pub has_custom_motion: bool,
    pub has_custom_class_name: bool,
    pub has_custom_label: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SliderState {
    pub phase: SliderPhase,
    pub phase_class: &'static str,
    pub phase_attr: &'static str,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub value: f64,
    pub value_percent: f64,
    pub is_enabled: bool,
    pub is_disabled: bool,
    pub has_custom_motion: bool,
    pub has_custom_class_name: bool,
    pub has_custom_label: bool,
    pub motion_source_class: &'static str,
    pub motion_source_attr: &'static str,
    pub label_source_class: &'static str,
    pub label_source_attr: &'static str,
    pub class_source_attr: &'static str,
}

pub fn source_attr_from_presence(is_custom: bool) -> &'static str {
    if is_custom { "custom" } else { "default" }
}

pub fn resolve_label(value: String) -> (String, bool) {
    let trimmed = value.trim();

    if trimmed.is_empty() {
        return (DEFAULT_LABEL.into(), false);
    }

    let label = trimmed.into();
    let has_custom_label = label != DEFAULT_LABEL;
    (label, has_custom_label)
}

pub fn sanitize_bounds(min: f64, max: f64) -> (f64, f64) {
    let mut lower = if min.is_finite() { min } else { DEFAULT_MIN };
    let mut upper = if max.is_finite() { max } else { DEFAULT_MAX };

    if lower > upper {
        std::mem::swap(&mut lower, &mut upper);
    }

    if (upper - lower).abs() < MIN_RANGE {
        (DEFAULT_MIN, DEFAULT_MAX)
    } else {
        (lower, upper)
    }
}

pub fn sanitize_step(step: f64, min: f64, max: f64) -> f64 {
    let range = (max - min).abs().max(DEFAULT_STEP);

    if step.is_finite() && step > 0.0 {
        step.min(range)
    } else {
        DEFAULT_STEP.min(range)
    }
}

pub fn parse_value(value: &str) -> Option<f64> {
    let trimmed = value.trim();
    (!trimmed.is_empty())
        .then(|| trimmed.parse::<f64>().ok())
        .flatten()
}

fn round_to_precision(value: f64) -> f64 {
    (value * 1_000_000.0).round() / 1_000_000.0
}

pub fn sanitize_value(value: f64, min: f64, max: f64, step: f64) -> f64 {
    let fallback = min;
    let value = if value.is_finite() { value } else { fallback };
    let clamped = value.clamp(min, max);

    let step = sanitize_step(step, min, max);
    let snapped = min + ((clamped - min) / step).round() * step;

    round_to_precision(snapped).clamp(min, max)
}

pub fn resolve_percent(value: f64, min: f64, max: f64) -> f64 {
    let range = (max - min).abs().max(MIN_RANGE);
    let percent = ((value - min) / range) * 100.0;

    if percent.is_finite() {
        percent.clamp(0.0, 100.0)
    } else {
        0.0
    }
}

pub fn resolve_state(input: SliderStateInput) -> SliderState {
    let (min, max) = sanitize_bounds(input.min, input.max);
    let step = sanitize_step(input.step, min, max);
    let value = sanitize_value(input.value, min, max, step);
    let value_percent = resolve_percent(value, min, max);

    let phase = if input.is_disabled {
        SliderPhase::Disabled
    } else {
        SliderPhase::Enabled
    };

    let (motion_source_class, motion_source_attr) = if input.has_custom_motion {
        ("ui-slider--motion-custom", "custom")
    } else {
        ("ui-slider--motion-default", "default")
    };

    let (label_source_class, label_source_attr) = if input.has_custom_label {
        ("ui-slider--label-custom", "custom")
    } else {
        ("ui-slider--label-default", "default")
    };

    SliderState {
        phase,
        phase_class: phase.class_name(),
        phase_attr: phase.as_attr(),
        min,
        max,
        step,
        value,
        value_percent,
        is_enabled: !input.is_disabled,
        is_disabled: input.is_disabled,
        has_custom_motion: input.has_custom_motion,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_label: input.has_custom_label,
        motion_source_class,
        motion_source_attr,
        label_source_class,
        label_source_attr,
        class_source_attr: source_attr_from_presence(input.has_custom_class_name),
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: SliderState) -> String {
    let mut classes = vec![
        "ui-slider".to_string(),
        state.phase_class.into(),
        state.motion_source_class.into(),
        state.label_source_class.into(),
    ];

    if state.has_custom_class_name {
        classes.push("ui-slider--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "test/slider.rs"]
mod tests;
